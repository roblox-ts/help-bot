import Discord from "discord.js";
import { createDiscordEventListener } from "../util/createDiscordEventListener";
import { processThread } from "../jobs/processThread";

export default createDiscordEventListener({
	event: Discord.Events.ThreadCreate,
	listener: async event => processThread(event),
});
