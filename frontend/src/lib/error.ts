export type AppError =
	| { type: "NETWORK_ERROR"; error: unknown }
	| { type: "HTTP_ERROR"; status: number; statusText: string }
	| { type: "VALIDATION_ERROR"; error: string }
	| { type: "PARSE_ERROR"; error: unknown };
