import csv
import os
import sys
import psycopg2
from dotenv import load_dotenv

CSV_FILE_PATH = "../data/swiss_towns.csv"
COUNTRY_CODE = "CH"


def connect_db():
    load_dotenv()
    db_url = os.getenv("DATABASE_URL")
    if not db_url:
        print("Error: DATABASE_URL environment variable not set.", file=sys.stderr)
        sys.exit(1)
    try:
        conn = psycopg2.connect(db_url)
        print("Database connection established successfully.")
        return conn
    except psycopg2.OperationalError as e:
        print(f"Error connecting to the database: {e}", file=sys.stderr)
        sys.exit(1)


def populate_from_csv(conn, csv_path):
    print(f"Attempting to populate CanonicalCity table from {csv_path}...")
    inserted_count = 0
    skipped_count = 0
    error_count = 0

    insert_sql = """
        INSERT INTO "CanonicalCity" ("canonicalName", "countryCode", "canton", "longitude", "latitude", "postalCode")
        VALUES (%s, %s, %s, %s, %s, %s)
        ON CONFLICT ("canonicalName", "countryCode", COALESCE("canton", '')) DO NOTHING;
    """

    try:
        with open(csv_path, mode='r', encoding='utf-8') as infile, conn.cursor() as cur:
            reader = csv.reader(infile, delimiter=';')
            header = next(reader)
            print(f"CSV Header: {header}")
            print("Processing CSV rows...")

            NAME_IDX = 0
            POSTAL_IDX = 1
            CANTON_IDX = 5
            LON_IDX = 6
            LAT_IDX = 7

            for row in reader:
                try:
                    name = row[NAME_IDX].strip()
                    canton = row[CANTON_IDX].strip() if row[CANTON_IDX] else None
                    lon_str = row[LON_IDX].strip()
                    lat_str = row[LAT_IDX].strip()
                    postal_code = row[POSTAL_IDX].strip() if row[POSTAL_IDX] else None

                    try:
                        lon = float(lon_str)
                        lat = float(lat_str)
                    except ValueError:
                        print(f"Skipping row due to invalid coordinate format: {row}")
                        skipped_count += 1
                        continue

                    cur.execute(insert_sql, (name, COUNTRY_CODE, canton, lon, lat, postal_code))
                    if cur.rowcount > 0:
                        inserted_count += 1
                    else:
                        pass

                except IndexError:
                    print(f"Skipping row due to unexpected format (IndexError): {row}")
                    skipped_count += 1
                except Exception as e:
                    print(f"Error processing row {row}: {e}", file=sys.stderr)
                    error_count += 1
                    conn.rollback()

        conn.commit()
        print(f"CSV processing complete.")
        print(f"Rows processed: {inserted_count + skipped_count + error_count}")
        print(f"Rows successfully inserted: {inserted_count}")
        print(f"Rows skipped (missing data/duplicates/invalid format): {skipped_count}")
        print(f"Rows with processing errors: {error_count}")

    except FileNotFoundError:
        print(f"Error: CSV file not found at {csv_path}", file=sys.stderr)
        sys.exit(1)
    except Exception as e:
        print(f"An unexpected error occurred during CSV processing: {e}", file=sys.stderr)
        conn.rollback()
        sys.exit(1)


def main():
    print("Starting city population script...")
    conn = connect_db()
    try:
        script_dir = os.path.dirname(__file__)
        absolute_csv_path = os.path.join(script_dir, CSV_FILE_PATH)
        absolute_csv_path = os.path.normpath(absolute_csv_path) # clean up path separators
        populate_from_csv(conn, absolute_csv_path)
    finally:
        if conn:
            conn.close()
            print("Database connection closed.")
    print("Script finished.")

if __name__ == "__main__":
    main()
