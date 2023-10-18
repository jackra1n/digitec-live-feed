import { Elysia } from "elysia";
import { Cron } from "croner";
import { PrismaClient } from "@prisma/client";
import { fetchFeedItems } from "./fetch";
import { convertDisplayPrices, convertSocialShoppingItems } from "./convertToPrisma";


const prisma = new PrismaClient();

const fetchJob = Cron("*/30 * * * * *", async () => {
    let items = await fetchFeedItems();

    const socialShoppingItems = convertSocialShoppingItems(items);
    const displayPrices = convertDisplayPrices(items);

    try {
      await prisma.socialShoppingItem.createMany({
        data: socialShoppingItems,
        skipDuplicates: true,
      });
      
      await prisma.displayPrice.createMany({
        data: displayPrices,
        skipDuplicates: true,
      });
    } catch (error) {
      console.error("Error saving to database: ", error);
    }


});

const app = new Elysia().get("/", () => "Hello Elysia").listen(3000);

console.log(
  `🦊 Elysia is running at ${app.server?.hostname}:${app.server?.port}`
);
