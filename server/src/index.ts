import { Hono } from "hono";
import { Cron } from "croner";
import { PrismaClient, SocialShoppingItem } from "@prisma/client";
import { fetchFeedItems } from "./fetch";
import { convertDisplayPrices, convertSocialShoppingItems } from "./convertToPrisma";


const prisma = new PrismaClient();
const app = new Hono();

app.get('/', (c) => c.text('Hello Hono!'));
app.get('/api/hello', (c) => {
  return c.json({
    ok: true,
    message: 'Hello Hono!',
  })
});

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

export default app;
