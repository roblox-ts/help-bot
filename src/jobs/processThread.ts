import Discord from "discord.js";
import { HELP_CHANNEL_ID, SOLVED_TAG_ID, UNSOLVED_TAG_ID } from "../constants";
import log from "@osyris/log";

export async function processThread(channel: Discord.AnyThreadChannel) {
	if (HELP_CHANNEL_ID !== channel.parentId) return;

	const hasSolved = channel.appliedTags.includes(SOLVED_TAG_ID);
	const hasUnsolved = channel.appliedTags.includes(UNSOLVED_TAG_ID);

	const threadName = channel.name;

	const { ownerId, appliedTags } = channel;
	const metadata = { threadName, ownerId, appliedTags };

	if (hasSolved && hasUnsolved) {
		log.info(`Removing unsolved tag from existing thread: "${threadName}"`, metadata);
		await channel.setAppliedTags(appliedTags.filter(tag => tag !== UNSOLVED_TAG_ID));
	} else if (!hasSolved && !hasUnsolved) {
		log.info(`Adding unsolved tag to existing thread: "${threadName}"`, metadata);
		await channel.setAppliedTags([...appliedTags, UNSOLVED_TAG_ID]);
	}
}
