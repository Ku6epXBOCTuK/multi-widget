import type { Activity, ActivityId } from "$lib/generated-types";
import { getAllActivities } from "$lib/network";
import type { ProcessActivity } from "$lib/network_stream";

export class ActivitiesState implements ProcessActivity {
	#activities = $state<Activity[]>([]);

	constructor() {}

	init(data: Activity[]) {
		this.#activities = data;
	}

	createActivity(activity: Activity): void {
		this.#activities.push(activity);
	}

	updateActivity(updated: Activity): void {
		this.#activities = this.#activities.map((a) =>
			a.id === updated.id ? updated : a,
		);
	}

	deleteActivity(id: ActivityId): void {
		this.#activities = this.#activities.filter((a) => a.id !== id);
	}

	get activities() {
		return this.#activities;
	}
}

export const activitiesState = new ActivitiesState();

await getAllActivities().andTee((fixtureActivities) => {
	activitiesState.init(fixtureActivities);
});
