import { PUBLIC_BASE_URL } from "$env/static/public";
import { NetworkStream, type ProcessActivity } from "./network_stream";

const processActivity: ProcessActivity = {
	createActivity: (activity) => {
		console.log("create activity", activity);
	},
	updateActivity: (activity) => {
		console.log("update activity", activity);
	},
	deleteActivity: (activityId) => {
		console.log("delete activity", activityId);
	},
};

export const networkStream = new NetworkStream(
	processActivity,
	`${PUBLIC_BASE_URL}/sse`,
);
