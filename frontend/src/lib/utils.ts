import { err, ok, type Result } from "neverthrow";
import type { AppError } from "./error";

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

export function parseJson<T>(data: string): Result<T, AppError> {
	try {
		return ok(JSON.parse(data));
	} catch (e) {
		return err({ type: "PARSE_ERROR", error: e });
	}
}
