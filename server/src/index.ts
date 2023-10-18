import { Elysia } from "elysia";
import { Cron } from "croner";
import { PrismaClient } from "@prisma/client";
import { fetchFeedItems } from "./fetch";


const prisma = new PrismaClient();

const fetchJob = Cron("*/30 * * * * *", async () => {
    let items = await fetchFeedItems();
    
    const createMany = await prisma.socialShoppingItem.createMany({
        data: items,
        skipDuplicates: true,
    });

    console.log(`🦊 ${createMany.count} items created`);

});

const app = new Elysia().get("/", () => "Hello Elysia").listen(3000);

console.log(
  `🦊 Elysia is running at ${app.server?.hostname}:${app.server?.port}`
);
