-- CreateTable
CREATE TABLE "SocialShoppingItem" (
    "id" INTEGER NOT NULL,
    "username" TEXT NOT NULL,
    "city" TEXT NOT NULL,
    "date" TIMESTAMP(3) NOT NULL,
    "imageUrl" TEXT,
    "brand" TEXT,
    "product" TEXT,
    "oAuthProviderName" TEXT,
    "targetUsername" TEXT,
    "quote" TEXT,
    "voteTypeId" INTEGER,
    "productTypeName" TEXT,
    "socialShoppingTransactionTypeId" INTEGER NOT NULL,
    "url" TEXT NOT NULL,
    "rating" DECIMAL(65,30),
    "searchString" TEXT,

    CONSTRAINT "SocialShoppingItem_pkey" PRIMARY KEY ("id")
);
