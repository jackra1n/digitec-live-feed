-- Add migration script here
BEGIN;

CREATE TABLE "CanonicalCity" (
    "id" SERIAL PRIMARY KEY,
    "canonicalName" TEXT NOT NULL UNIQUE,
    "isSwiss" BOOLEAN NOT NULL DEFAULT FALSE,
    "canton" TEXT,
    "countryCode" CHAR(2),
    "createdAt" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE "RawCityNameMap" (
    "rawInput" TEXT PRIMARY KEY,
    "canonicalCityId" INTEGER,
    "mappingStatus" TEXT NOT NULL DEFAULT 'unprocessed',
    "createdAt" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT "RawCityNameMap_canonicalCityId_fkey"
        FOREIGN KEY ("canonicalCityId") REFERENCES "CanonicalCity"("id")
        ON DELETE SET NULL
        ON UPDATE CASCADE
);

INSERT INTO "RawCityNameMap" ("rawInput", "mappingStatus")
SELECT DISTINCT "cityName", 'unprocessed'
FROM "SocialShoppingItem"
WHERE "cityName" IS NOT NULL
ON CONFLICT ("rawInput") DO NOTHING;

ALTER TABLE "SocialShoppingItem"
ADD CONSTRAINT "SocialShoppingItem_cityName_fkey"
    FOREIGN KEY ("cityName") REFERENCES "RawCityNameMap"("rawInput")
    ON DELETE RESTRICT
    ON UPDATE CASCADE;

CREATE INDEX IF NOT EXISTS idx_canonical_city_swiss ON "CanonicalCity" ("isSwiss");
CREATE INDEX IF NOT EXISTS idx_canonical_city_country ON "CanonicalCity" ("countryCode");
CREATE INDEX IF NOT EXISTS idx_canonical_city_canton ON "CanonicalCity" ("canton") WHERE "isSwiss" = TRUE;
CREATE INDEX IF NOT EXISTS idx_raw_city_name_map_canonical_id ON "RawCityNameMap" ("canonicalCityId");
CREATE INDEX IF NOT EXISTS idx_raw_city_name_map_status ON "RawCityNameMap" ("mappingStatus");

COMMIT;