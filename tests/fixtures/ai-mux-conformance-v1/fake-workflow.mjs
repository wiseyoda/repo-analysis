#!/usr/bin/env node

import { readFileSync } from "node:fs";

const [mode = "success"] = process.argv.slice(2);
if (mode === "crash") {
  process.stderr.write("intentional fake workflow crash\n");
  process.exit(29);
}
const lines = readFileSync(0, "utf8").trim().split("\n").map(JSON.parse);
if (lines.length !== 2 || lines[0].type !== "initialize" || lines[1].type !== "run.start") {
  process.stderr.write("fake workflow requires initialize and run.start\n");
  process.exit(2);
}
const [initialize, start] = lines;
const release = JSON.parse(
  readFileSync(new URL("./protocol-release.json", import.meta.url), "utf8"),
);
if (initialize.schemaHash !== release.schemaPackageSha256) {
  process.stderr.write("unsupported protocol schema hash\n");
  process.exit(2);
}
const base = {
  protocolVersion: "1.0",
  schemaHash: initialize.schemaHash,
  timestamp: "2026-07-27T05:00:00Z",
  extensionId: "ai-mux.fixture-workflow",
  installedVersion: "1.0.0",
};
const message = (type, messageId, requestId, payload) => ({
  ...base,
  messageId,
  type,
  rootRunId: start.rootRunId,
  runId: start.runId,
  requestId,
  payload,
});
const output = [
  message(
    "initialized",
    "019fa10b-5279-72b2-bfb3-adbf00d3ee51",
    initialize.requestId,
    {
      extensionId: "ai-mux.fixture-workflow",
      extensionVersion: "1.0.0",
      selectedProtocolVersion: "1.0",
      capabilities: [],
      commands: ["fixture.run"],
      workflows: ["fixture.workflow.v1"],
      health: "healthy",
    },
  ),
];
if (mode === "partial") {
  output.push(message(
    "warning.raised",
    "019fa10b-5279-72b2-bfb3-adbf00d3ee52",
    start.requestId,
    { code: "FIXTURE_PARTIAL", message: "intentional partial result" },
  ));
}
if (mode === "checkpoint" || mode === "approval") {
  output.push(message(
    mode === "checkpoint" ? "checkpoint.save" : "approval.request",
    "019fa10b-5279-72b2-bfb3-adbf00d3ee53",
    start.requestId,
    mode === "checkpoint"
      ? { checkpointKey: "fixture-stage-1", idempotencyKey: start.runId }
      : { approvalId: "fixture-approval-1", sideEffect: "repository.write" },
  ));
} else {
  output.push(message(
    "agent.start",
    "019fa10b-5279-72b2-bfb3-adbf00d3ee54",
    start.requestId,
    {
      idempotencyKey: `${start.runId}:agent`,
      stageId: "fixture.agent",
      role: "reviewer",
      sideEffect: "none",
      permissions: ["repository.read"],
      prompt: "Return the deterministic fixture result.",
    },
  ));
}
process.stdout.write(`${output.map(JSON.stringify).join("\n")}\n`);
