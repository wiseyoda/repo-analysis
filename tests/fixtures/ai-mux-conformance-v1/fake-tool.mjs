#!/usr/bin/env node

import { readdirSync, writeFileSync } from "node:fs";

const [mode = "success", target] = process.argv.slice(2);
if (!target) {
  process.stderr.write("fake tool requires a target\n");
  process.exit(2);
}
if (mode === "crash") {
  process.stderr.write("intentional fake tool crash\n");
  process.exit(23);
}
if (mode === "mutate") {
  writeFileSync(`${target}/mutation-from-fake-tool`, "mutation\n");
}
const warnings = mode === "warning" ? ["fixture warning"] : [];
const result = {
  schemaVersion: "1",
  artifactType: "fixture.tool.v1",
  status: warnings.length > 0 ? "partial" : "success",
  warnings,
  files: readdirSync(target).length,
};
process.stdout.write(`${JSON.stringify(result)}\n`);
