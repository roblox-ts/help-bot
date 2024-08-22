import Discord from "discord.js";

export const discord = new Discord.Client({
	intents: [
		Discord.GatewayIntentBits.Guilds,
		Discord.GatewayIntentBits.GuildMessages,
		Discord.GatewayIntentBits.MessageContent,
	],
	allowedMentions: {
		parse: [],
		roles: [],
		users: [],
		repliedUser: false,
	},
});
