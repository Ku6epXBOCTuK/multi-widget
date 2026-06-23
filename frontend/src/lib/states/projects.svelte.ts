import type { Activity } from "$lib/generated-types";
import { getAllActivities } from "$lib/network";

let activities = $state<Activity[]>([]);

export function getActivities() {
	return {
		get all() {
			return activities;
		},
		init(data: Activity[]) {
			activities = data;
		},
		add(activity: Activity) {
			activities = [...activities, activity];
		},
		update(updated: Activity) {
			activities = activities.map((a) => (a.id === updated.id ? updated : a));
		},
		remove(id: number) {
			activities = activities.filter((a) => a.id !== id);
		},
		childrenOf(parentId: number) {
			return activities.filter((a) => a.parent_id === parentId);
		},
	};
}

export const activityStore = getActivities();

await getAllActivities().andTee((fixtureActivities) => {
	activityStore.init(fixtureActivities);
});
