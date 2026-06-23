import { TypeScriptToTypeBox } from "@sinclair/typebox-codegen";
import fs from "fs";
import path from "path";

const sourcePath = path.resolve("./src/lib/generated-types.ts");
const outputPath = path.resolve("./src/lib/generated-schemas.ts");

try {
	const tsCode = fs.readFileSync(sourcePath, "utf8");

	const typeBoxCode = TypeScriptToTypeBox.Generate(tsCode);

	fs.writeFileSync(outputPath, typeBoxCode, "utf8");
	console.log("> TypeBox schemas generated.");
} catch (error) {
	console.error("> Generate schemas error:", error);
}
