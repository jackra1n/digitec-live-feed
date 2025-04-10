-- Add migration script here
BEGIN;

-- 1. Create Brand Table
CREATE TABLE "Brand" (
    "id" SERIAL PRIMARY KEY,
    "name" TEXT NOT NULL UNIQUE,
    "createdAt" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_brand_name ON "Brand" ("name");

-- 2. Populate Brand Table
INSERT INTO "Brand" ("name")
SELECT DISTINCT "brandName"
FROM "SocialShoppingItem"
WHERE "brandName" IS NOT NULL
ON CONFLICT ("name") DO NOTHING;

-- 3. Create ProductType Table
CREATE TABLE "ProductType" (
    "id" SERIAL PRIMARY KEY,
    "name" TEXT NOT NULL UNIQUE,
    "createdAt" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_product_type_name ON "ProductType" ("name");

-- 4. Populate ProductType Table
INSERT INTO "ProductType" ("name")
SELECT DISTINCT "productTypeName"
FROM "SocialShoppingItem"
WHERE "productTypeName" IS NOT NULL
ON CONFLICT ("name") DO NOTHING;

-- 5. Add Foreign Key Columns to SocialShoppingItem
ALTER TABLE "SocialShoppingItem"
ADD COLUMN "brandId" INTEGER,
ADD COLUMN "productTypeId" INTEGER;

-- 6. Update brandId in SocialShoppingItem
UPDATE "SocialShoppingItem" ssi
SET "brandId" = b.id
FROM "Brand" b
WHERE ssi."brandName" = b.name;

-- 7. Update productTypeId in SocialShoppingItem
UPDATE "SocialShoppingItem" ssi
SET "productTypeId" = pt.id
FROM "ProductType" pt
WHERE ssi."productTypeName" = pt.name;

-- 8. Add Foreign Key Constraints
ALTER TABLE "SocialShoppingItem"
ADD CONSTRAINT "SocialShoppingItem_brandId_fkey"
FOREIGN KEY ("brandId") REFERENCES "Brand"("id")
ON DELETE SET NULL -- Or RESTRICT, depending on desired behavior
ON UPDATE CASCADE;

ALTER TABLE "SocialShoppingItem"
ADD CONSTRAINT "SocialShoppingItem_productTypeId_fkey"
FOREIGN KEY ("productTypeId") REFERENCES "ProductType"("id")
ON DELETE SET NULL -- Or RESTRICT
ON UPDATE CASCADE;

-- 9. Add Indexes for new Foreign Keys
CREATE INDEX IF NOT EXISTS idx_social_shopping_item_brand_id ON "SocialShoppingItem" ("brandId");
CREATE INDEX IF NOT EXISTS idx_social_shopping_item_product_type_id ON "SocialShoppingItem" ("productTypeId");

-- 10. Create or Ensure the updatedAt trigger function exists
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
   NEW."updatedAt" = now();
   RETURN NEW;
END;
$$ language 'plpgsql';

-- 11. Apply the trigger to the new tables
CREATE TRIGGER update_brand_updated_at
BEFORE UPDATE ON "Brand"
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_product_type_updated_at
BEFORE UPDATE ON "ProductType"
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

-- Note on dropping old columns:
-- It's highly recommended to verify the data integrity and application functionality
-- THOROUGHLY before dropping the original columns.
-- Keep them for a transition period if possible.

-- Consider running these DROP commands in a separate, later migration:
-- ALTER TABLE "SocialShoppingItem" DROP COLUMN "brandName";
-- ALTER TABLE "SocialShoppingItem" DROP COLUMN "productTypeName";

-- Also, drop the old indexes if you drop the columns:
-- DROP INDEX IF EXISTS idx_social_shopping_item_brand_name;
-- DROP INDEX IF EXISTS idx_social_shopping_item_product_type;

COMMIT;