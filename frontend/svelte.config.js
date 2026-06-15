import adapter from "@sveltejs/adapter-static";

/** @type {import('@sveltejs/kit').Config} */
const config = {
	kit: {
		adapter: adapter(),
		output: {
			bundleStrategy: "inline",
		},
		alias: {
			$cmp: "src/components",
		},
		router: {
			type: "hash",
		},
	},
	compilerOptions: { runes: true },
};

export default config;
