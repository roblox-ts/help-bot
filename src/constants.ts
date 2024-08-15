import assert from "node:assert";
import path from "node:path";

assert(Bun.env.DISCORD_TOKEN);
export const DISCORD_TOKEN = Bun.env.DISCORD_TOKEN;

assert(Bun.env.HELP_CHANNEL_ID);
export const HELP_CHANNEL_ID = Bun.env.HELP_CHANNEL_ID;

assert(Bun.env.UNSOLVED_TAG_ID);
export const UNSOLVED_TAG_ID = Bun.env.UNSOLVED_TAG_ID;

assert(Bun.env.SOLVED_TAG_ID);
export const SOLVED_TAG_ID = Bun.env.SOLVED_TAG_ID;

export const PROJECT_FOLDER = path.join(__dirname, "..");
export const EVENT_FOLDER = path.join(PROJECT_FOLDER, "src", "events");
