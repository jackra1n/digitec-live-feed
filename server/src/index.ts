import { Hono } from 'hono';
import { Cron } from 'croner';
import { prettyJSON } from 'hono/pretty-json';
import { PrismaClient } from '@prisma/client';
import { fetchFeedItems } from './fetch.js';
import { convertDisplayPrices, convertSocialShoppingItems } from './convertToPrisma.js';


console.log('Starting server...');

const prisma = new PrismaClient();
const app = new Hono();

app.use('*', prettyJSON());

app.get('/', (c) => c.text('Hello Hono!'));

app.get('/api/v1/live-feed/', async (c) => {
    try {
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
    } catch (error) {
        console.error(`Error fetching live feed: ${error}`);
        return c.json({ error: 'Error fetching live feed' }, 500);
    }
});

const fetchJob = new Cron("*/30 * * * * *", async () => {
    let items = [];
    try {
        items = await fetchFeedItems();
    } catch (error) {
        console.error(`Error fetching feed items: ${error}`);
    }

    const socialShoppingItems = convertSocialShoppingItems(items);
    const displayPrices = convertDisplayPrices(items);

    try {
        await prisma.socialShoppingItem.createMany({
            data: socialShoppingItems,
            skipDuplicates: true
        });
    
        await prisma.displayPrice.createMany({
            data: displayPrices,
            skipDuplicates: true
        });
    } catch (error) {
        console.error(`Error saving to database: ${error}`);
    }
});

export default { 
    port: 4794, 
    fetch: app.fetch, 
}

console.log('Server started!');
