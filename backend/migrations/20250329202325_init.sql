CREATE TABLE "SocialShoppingItem" (
    "id" INTEGER NOT NULL,
    "userName" TEXT NOT NULL,
    "cityName" TEXT,
    "dateTime" TIMESTAMPTZ NOT NULL,
    "imageUrl" TEXT,
    "brandName" TEXT,
    "fullProductName" TEXT,
    "oAuthProviderName" TEXT,
    "targetUserName" TEXT,
    "quote" TEXT,
    "voteTypeId" INTEGER,
    "productTypeName" TEXT,
    "socialShoppingTransactionTypeId" INTEGER NOT NULL,
    "url" TEXT NOT NULL,
    "rating" INTEGER,
    "searchString" TEXT,

    "createdAt" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "SocialShoppingItem_pkey" PRIMARY KEY ("id"),

    CONSTRAINT "SocialShoppingItem_userName_dateTime_sstTypeId_url_uq"
        UNIQUE ("userName", "dateTime", "socialShoppingTransactionTypeId", "url")
);

CREATE TABLE "DisplayPrice" (
    "id" SERIAL NOT NULL,
    "socialShoppingItemId" INTEGER NOT NULL,
    "amountInclusive" DECIMAL(65,30) NOT NULL,
    "amountExclusive" DECIMAL(65,30) NOT NULL,
    "currency" TEXT NOT NULL,

    "createdAt" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "DisplayPrice_pkey" PRIMARY KEY ("id")
);

-- Create Indexes
CREATE UNIQUE INDEX "DisplayPrice_socialShoppingItemId_key"
ON "DisplayPrice"("socialShoppingItemId");


-- Add Foreign Key Constraint
ALTER TABLE "DisplayPrice"
ADD CONSTRAINT "DisplayPrice_socialShoppingItemId_fkey"
FOREIGN KEY ("socialShoppingItemId") REFERENCES "SocialShoppingItem"("id")
ON DELETE RESTRICT
ON UPDATE CASCADE;

-- Add indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_social_shopping_item_date_time ON "SocialShoppingItem" ("dateTime" DESC);
CREATE INDEX IF NOT EXISTS idx_social_shopping_item_user_name ON "SocialShoppingItem" ("userName");
CREATE INDEX IF NOT EXISTS idx_social_shopping_item_city_name ON "SocialShoppingItem" ("cityName");
CREATE INDEX IF NOT EXISTS idx_social_shopping_item_brand_name ON "SocialShoppingItem" ("brandName");
CREATE INDEX IF NOT EXISTS idx_social_shopping_item_product_type ON "SocialShoppingItem" ("productTypeName");
CREATE INDEX IF NOT EXISTS idx_social_shopping_item_transaction_type ON "SocialShoppingItem" ("socialShoppingTransactionTypeId");

-- Add index for the foreign key relationship
CREATE INDEX IF NOT EXISTS idx_display_price_social_shopping_item_id ON "DisplayPrice" ("socialShoppingItemId");

-- Add index for the unique constraint
CREATE INDEX IF NOT EXISTS idx_social_shopping_item_unique_constraint ON "SocialShoppingItem" 
    ("userName", "dateTime", "socialShoppingTransactionTypeId", "url");

-- Add index for the vote type
CREATE INDEX IF NOT EXISTS idx_social_shopping_item_vote_type ON "SocialShoppingItem" ("voteTypeId");

-- Add index for the search string
CREATE INDEX IF NOT EXISTS idx_social_shopping_item_search_string ON "SocialShoppingItem" ("searchString");