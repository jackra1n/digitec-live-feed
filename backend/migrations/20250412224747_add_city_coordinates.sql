BEGIN;

-- Add longitude and latitude columns to the CanonicalCity table
-- Using NUMERIC type for precise coordinate storage
ALTER TABLE "CanonicalCity" 
ADD COLUMN "longitude" NUMERIC(9,6),
ADD COLUMN "latitude" NUMERIC(8,6),
ADD COLUMN "postalCode" VARCHAR(10);

-- Drop the existing unique constraint on canonicalName
ALTER TABLE "CanonicalCity" DROP CONSTRAINT IF EXISTS "CanonicalCity_canonicalName_key";

-- Make countryCode NOT NULL
ALTER TABLE "CanonicalCity" ALTER COLUMN "countryCode" SET NOT NULL;

-- Create a functional index that handles NULL cantons correctly for uniqueness check
-- This approach lets us keep cantons as NULL when they're not available
CREATE UNIQUE INDEX "CanonicalCity_name_location_idx" ON "CanonicalCity" 
("canonicalName", "countryCode", (COALESCE("canton", '')));

-- Create a spatial index on the coordinates for faster geospatial queries
CREATE INDEX IF NOT EXISTS idx_canonical_city_coordinates 
ON "CanonicalCity" ("longitude", "latitude")
WHERE "longitude" IS NOT NULL AND "latitude" IS NOT NULL;

-- Automatically update the updatedAt timestamp when coordinates are modified
CREATE OR REPLACE FUNCTION update_city_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW."updatedAt" = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_canonical_city_timestamp
BEFORE UPDATE ON "CanonicalCity"
FOR EACH ROW
WHEN (OLD."longitude" IS DISTINCT FROM NEW."longitude" OR 
      OLD."latitude" IS DISTINCT FROM NEW."latitude" OR
      OLD."postalCode" IS DISTINCT FROM NEW."postalCode")
EXECUTE FUNCTION update_city_timestamp();

COMMIT;
