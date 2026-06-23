import { match } from "ts-pattern";
import type { Activity, ActivityId, StreamEvent } from "./generated-types";
import { parseJson } from "./utils";

export interface ProcessActivity {
	createActivity: (_: Activity) => void;
	updateActivity: (_: Activity) => void;
	deleteActivity: (_: ActivityId) => void;
}

export class NetworkStream {
	#processActivity: ProcessActivity;
	#eventSource: EventSource | null = null;
	#url: string;

	constructor(processActivity: ProcessActivity, url: string) {
		this.#processActivity = processActivity;
		this.#url = url;
	}

	run() {
		// TODO: проверить, нужен ли мне fetchEventSource? там есть авторизация
		// и другие возможности
		this.#eventSource = new EventSource(this.#url);

		this.#eventSource.onopen = () => {
			console.log("SSE connected...");
		};

		this.#eventSource.onmessage = (event) => {
			parseJson<StreamEvent>(event.data)
				.andTee((parsed) => {
					this.#processEvent(parsed);
				})
				.orTee(() => {
					console.error("Error: received event is not a valid JSON");
				});
		};

		this.#eventSource.onerror = (_) => {
			console.error("SSE or network error...");
			// TODO: need investigate reconnection flow???
		};
	}

	disconnect() {
		if (this.#eventSource) {
			this.#eventSource.close();
			console.log("SSE connection closed");
		}
	}

	#processEvent(event: StreamEvent) {
		console.log("Received event:", event);
		match(event)
			.with({ variant: "createActivity" }, (e) => {
				this.#processActivity.createActivity(e.payload);
			})
			.with({ variant: "updateActivity" }, (e) => {
				this.#processActivity.updateActivity(e.payload);
			})
			.with({ variant: "deleteActivity" }, (e) => {
				this.#processActivity.deleteActivity(e.payload);
			})
			.exhaustive();
	}
}
