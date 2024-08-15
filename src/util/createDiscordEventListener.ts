import type Discord from "discord.js";

export interface DiscordEventListener<E extends keyof Discord.ClientEvents = keyof Discord.ClientEvents> {
	event: E;
	once?: boolean;
	listener: (...args: Discord.ClientEvents[E]) => void;
}

export function createDiscordEventListener<E extends keyof Discord.ClientEvents>(listener: DiscordEventListener<E>) {
	return listener;
}
