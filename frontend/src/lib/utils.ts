import { PUBLIC_BASE_URL } from "$env/static/public";
import { errAsync, ResultAsync } from "neverthrow";
import type { Activity } from "./generated-bindings";

export function getNoun(
	number: number,
	one: string,
	two: string,
	five: string,
): string {
	let n = Math.abs(number);
	// eslint-disable-next-line @typescript-eslint/no-magic-numbers
	n %= 100;
	// eslint-disable-next-line @typescript-eslint/no-magic-numbers
	if (n >= 5 && n <= 20) return five;
	// eslint-disable-next-line @typescript-eslint/no-magic-numbers
	n %= 10;
	if (n === 1) return one;
	// eslint-disable-next-line @typescript-eslint/no-magic-numbers
	if (n >= 2 && n <= 4) return two;
	return five;
}

type FetchError =
	| { type: "NETWORK_ERROR"; error: unknown }
	| { type: "HTTP_ERROR"; status: number; statusText: string }
	| { type: "PARSE_ERROR"; error: unknown };

export function appFetch<T>(
	url: string,
	options?: RequestInit,
): ResultAsync<T, FetchError> {
	return ResultAsync.fromPromise(
		fetch(url, options),
		(error): FetchError => ({ type: "NETWORK_ERROR", error }),
	).andThen((response) => {
		if (!response.ok) {
			return errAsync<T, FetchError>({
				type: "HTTP_ERROR",
				status: response.status,
				statusText: response.statusText,
			});
		}

		return ResultAsync.fromPromise(
			response.json() as Promise<T>,
			(error): FetchError => ({ type: "PARSE_ERROR", error }),
		);
	});
}

export function getAllActivities(): ResultAsync<Activity[], FetchError> {
	return appFetch<Activity[]>(`${PUBLIC_BASE_URL}/activities`);
}
