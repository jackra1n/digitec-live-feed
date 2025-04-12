-- Add migration script here

-- 1. First, drop the unique constraint on DisplayPrice.socialShoppingItemId
ALTER TABLE "DisplayPrice" 
DROP CONSTRAINT IF EXISTS "DisplayPrice_socialShoppingItemId_key";
DROP INDEX IF EXISTS "DisplayPrice_socialShoppingItemId_key";

-- 2. Drop the foreign key constraint from DisplayPrice referencing the old ID
ALTER TABLE "DisplayPrice"
DROP CONSTRAINT IF EXISTS "DisplayPrice_socialShoppingItemId_fkey";

-- 3. Drop the old primary key constraint on SocialShoppingItem
ALTER TABLE "SocialShoppingItem"
DROP CONSTRAINT IF EXISTS "SocialShoppingItem_pkey";

-- 4. Rename the existing 'id' column (which comes from the API)
ALTER TABLE "SocialShoppingItem"
RENAME COLUMN "id" TO "api_id";

-- 5. Add a new auto-incrementing primary key column
ALTER TABLE "SocialShoppingItem"
ADD COLUMN "id" BIGSERIAL PRIMARY KEY;

-- 6. Update DisplayPrice to use the new internal SocialShoppingItem ID type
ALTER TABLE "DisplayPrice"
ALTER COLUMN "socialShoppingItemId" TYPE BIGINT;

-- 7. Create a temporary table to identify any duplicate records
CREATE TEMPORARY TABLE temp_duplicate_check AS
SELECT
    dp."socialShoppingItemId" as old_id,
    COUNT(*) as record_count,
    array_agg(dp.id) as dp_ids
FROM "DisplayPrice" dp
GROUP BY dp."socialShoppingItemId"
HAVING COUNT(*) > 1;

-- 8. Output the duplicate records so they can be inspected (optional)
-- You can check these with: SELECT * FROM temp_duplicate_data;
CREATE TEMPORARY TABLE temp_duplicate_data AS
SELECT dp.*
FROM "DisplayPrice" dp
JOIN temp_duplicate_check tdc ON dp."socialShoppingItemId" = tdc.old_id;

-- 9. Update DisplayPrice records with the new SocialShoppingItem ID
-- This update works because there's a 1:1 mapping from old api_id to new internal id
UPDATE "DisplayPrice" dp
SET "socialShoppingItemId" = ssi.id
FROM "SocialShoppingItem" ssi
WHERE dp."socialShoppingItemId" = ssi."api_id";

-- Drop the temporary tables
DROP TABLE temp_duplicate_check;
DROP TABLE temp_duplicate_data;

-- 10. Re-add the unique constraint
CREATE UNIQUE INDEX "DisplayPrice_socialShoppingItemId_key"
ON "DisplayPrice"("socialShoppingItemId");

-- 11. Re-add the foreign key constraint referencing the new 'id'
ALTER TABLE "DisplayPrice"
ADD CONSTRAINT "DisplayPrice_socialShoppingItemId_fkey"
FOREIGN KEY ("socialShoppingItemId") REFERENCES "SocialShoppingItem"("id")
ON DELETE RESTRICT
ON UPDATE CASCADE;

-- 12. Index the api_id column if you query by it
CREATE INDEX IF NOT EXISTS idx_social_shopping_item_api_id ON "SocialShoppingItem" ("api_id");