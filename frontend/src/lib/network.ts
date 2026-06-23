import { PUBLIC_BASE_URL } from "$env/static/public";
import { errAsync, ResultAsync } from "neverthrow";
import type { AppError } from "./error";
import type { Activity } from "./generated-types";

export function appFetch<T>(
	url: string,
	options?: RequestInit,
): ResultAsync<T, AppError> {
	return ResultAsync.fromPromise(
		fetch(url, options),
		(error): AppError => ({ type: "NETWORK_ERROR", error }),
	).andThen((response) => {
		if (!response.ok) {
			return errAsync<T, AppError>({
				type: "HTTP_ERROR",
				status: response.status,
				statusText: response.statusText,
			});
		}

		return ResultAsync.fromPromise(
			response.json() as Promise<T>,
			(error): AppError => ({ type: "PARSE_ERROR", error }),
		);
	});
}

export function getAllActivities(): ResultAsync<Activity[], AppError> {
	return appFetch<Activity[]>(`${PUBLIC_BASE_URL}/activities`);
}
