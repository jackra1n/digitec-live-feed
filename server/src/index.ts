import { Elysia } from "elysia";
import { Cron } from "croner";
import { fetchFeedItems } from "./fetch";
import postgres from "postgres";


const sql = postgres(process.env.DATABASE_URL);

const fetchJob = Cron("*/30 * * * * *", async () => {
    let items = await fetchFeedItems();
});

const app = new Elysia().get("/", () => "Hello Elysia").listen(3000);

console.log(
  `🦊 Elysia is running at ${app.server?.hostname}:${app.server?.port}`
);
