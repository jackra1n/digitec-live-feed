CREATE TABLE "SocialShoppingItem" (
    "id" INTEGER NOT NULL,
    "userName" TEXT NOT NULL,
    "cityName" TEXT,
    "dateTime" TIMESTAMP(3) WITH TIME ZONE NOT NULL,
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

    "createdAt" TIMESTAMP(3) WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "SocialShoppingItem_pkey" PRIMARY KEY ("id")
);

CREATE TABLE "DisplayPrice" (
    "id" SERIAL NOT NULL,
    "socialShoppingItemId" INTEGER NOT NULL,
    "amountInclusive" DECIMAL(65,30) NOT NULL,
    "amountExclusive" DECIMAL(65,30) NOT NULL,
    "currency" TEXT NOT NULL,

    "createdAt" TIMESTAMP(3) WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "DisplayPrice_pkey" PRIMARY KEY ("id")
);

-- Create Indexes
CREATE UNIQUE INDEX "SocialShoppingItem_userName_dateTime_socialShoppingTransact_key"
ON "SocialShoppingItem"("userName", "dateTime", "socialShoppingTransactionTypeId", "url");

CREATE UNIQUE INDEX "DisplayPrice_socialShoppingItemId_key"
ON "DisplayPrice"("socialShoppingItemId");


-- Add Foreign Key Constraint
ALTER TABLE "DisplayPrice"
ADD CONSTRAINT "DisplayPrice_socialShoppingItemId_fkey"
FOREIGN KEY ("socialShoppingItemId") REFERENCES "SocialShoppingItem"("id")
ON DELETE RESTRICT
ON UPDATE CASCADE;