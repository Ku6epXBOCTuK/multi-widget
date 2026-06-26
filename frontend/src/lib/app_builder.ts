import { PUBLIC_BASE_URL } from "$env/static/public";
import { registerGlobalLogger } from "./logger";
import { NetworkStream } from "./network_stream";
import { activitiesState } from "./states/projects.svelte";

export const networkStream = new NetworkStream(
	activitiesState,
	`${PUBLIC_BASE_URL}/sse`,
);

registerGlobalLogger();
