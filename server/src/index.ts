import { Hono } from 'hono';
import { Cron } from 'croner';
import { PrismaClient } from '@prisma/client';
import { fetchFeedItems } from './fetch';
import { convertDisplayPrices, convertSocialShoppingItems } from './convertToPrisma';
import { prettyJSON } from 'hono/pretty-json';


const prisma = new PrismaClient();
const app = new Hono();

app.use('*', prettyJSON());
app.get('/', (c) => c.text('Hello Hono!'));
app.get('/api/v1/live-feed/', async (c) => {
  const items = await prisma.socialShoppingItem.findMany({
    include: {
      displayPrice: true,
    },
    take: 6,
    orderBy: {
      dateTime: 'desc',
    },
  });
  return c.json(items);
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
      console.error('Error saving to database: ', error);
    }
});

export default app;
