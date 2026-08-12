#!/usr/bin/env node

import {
  cpSync,
  mkdirSync,
  symlinkSync,
  writeFileSync,
} from "node:fs";
import { spawnSync } from "node:child_process";

const [target] = process.argv.slice(2);
if (!target) {
  process.stderr.write("materializer requires a target\n");
  process.exit(2);
}
mkdirSync(target, { recursive: true });
cpSync(new URL("./repository/", import.meta.url), target, { recursive: true });
symlinkSync("src/query.ts", `${target}/query-link`);
writeFileSync(`${target}/large.fixture`, "x".repeat(65_536));
const git = (...args) => spawnSync("git", args, { cwd: target, stdio: "ignore" });
git("init", "-q", "-b", "main");
git("add", ".");
git(
  "-c",
  "user.name=Fixture",
  "-c",
  "user.email=fixture@example.invalid",
  "commit",
  "-qm",
  "fixture",
);
writeFileSync(`${target}/dirty.txt`, "dirty working tree\n");
