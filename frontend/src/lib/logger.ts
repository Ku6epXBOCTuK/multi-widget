export const LogLevel = {
	VERBOSE: 0,
	DEBUG: 1,
	INFO: 2,
	WARN: 3,
	ERROR: 4,
} as const;

export type LogLevelType = (typeof LogLevel)[keyof typeof LogLevel];

class Logger {
	#currentLever: LogLevelType = LogLevel.INFO;
	constructor() {}

	print(level: LogLevelType, ...args: unknown[]) {
		if (level >= this.#currentLever) {
			switch (level) {
				case LogLevel.VERBOSE:
					// eslint-disable-next-line no-console
					console.log(...args);
					break;
				case LogLevel.DEBUG:
					// eslint-disable-next-line no-console
					console.debug(...args);
					break;
				case LogLevel.INFO:
					// eslint-disable-next-line no-console
					console.info(...args);
					break;
				case LogLevel.WARN:
					console.warn(...args);
					break;
				case LogLevel.ERROR:
					console.error(...args);
					break;
			}
		}
	}

	log(...args: unknown[]) {
		this.print(LogLevel.INFO, ...args);
	}

	warn(...args: unknown[]) {
		this.print(LogLevel.WARN, ...args);
	}

	debug(...args: unknown[]) {
		this.print(LogLevel.DEBUG, ...args);
	}

	error(...args: unknown[]) {
		this.print(LogLevel.ERROR, ...args);
	}

	verbose(...args: unknown[]) {
		this.print(LogLevel.VERBOSE, ...args);
	}

	setLevel(level: LogLevelType) {
		this.#currentLever = level;
	}
}
export const logger = new Logger();

export function registerGlobalLogger() {
	globalThis.logger = logger;
}

declare global {
	interface Window {
		logger: Logger;
	}
	var logger: Logger;
}
