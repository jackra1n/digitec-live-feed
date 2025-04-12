-- Add migration script here
ALTER TABLE "SocialShoppingItem" DROP COLUMN IF EXISTS "brandName";
ALTER TABLE "SocialShoppingItem" DROP COLUMN IF EXISTS "productTypeName";

DROP INDEX IF EXISTS idx_social_shopping_item_brand_name;
DROP INDEX IF EXISTS idx_social_shopping_item_product_type;