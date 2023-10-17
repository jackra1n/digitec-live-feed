import { Elysia } from "elysia";
import { Cron } from "croner";

const fetchJob = Cron("*/30 * * * * *", () => {
  console.log("fetching data");
});

const app = new Elysia().get("/", () => "Hello Elysia").listen(3000);

console.log(
  `🦊 Elysia is running at ${app.server?.hostname}:${app.server?.port}`
);
