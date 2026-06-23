import { PUBLIC_BASE_URL } from "$env/static/public";
import { registerGlobalLogger } from "./logger";
import { NetworkStream, type ProcessActivity } from "./network_stream";

const processActivity: ProcessActivity = {
	createActivity: (activity) => {
		logger.log("create activity", activity);
	},
	updateActivity: (activity) => {
		logger.log("update activity", activity);
	},
	deleteActivity: (activityId) => {
		logger.log("delete activity", activityId);
	},
};

export const networkStream = new NetworkStream(
	processActivity,
	`${PUBLIC_BASE_URL}/sse`,
);

registerGlobalLogger();
