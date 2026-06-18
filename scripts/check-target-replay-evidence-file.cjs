#!/usr/bin/env node
"use strict";

const fs = require("node:fs");
const path = require("node:path");
const {
  sha256Text,
  stableStringify,
  validateTargetReplayEvidence,
} = require("./cleanroom-branch-coverage-check.cjs");

function argValue(name) {
  const index = process.argv.indexOf(name);
  if (index === -1) return undefined;
  const value = process.argv[index + 1];
  if (value === undefined || value.startsWith("--")) {
    throw new Error(`${name} requires a value`);
  }
  return value;
}

function readJson(filePath) {
  return JSON.parse(fs.readFileSync(filePath, "utf8"));
}

function manifestSourceCommit(rootPath) {
  const manifest = fs.readFileSync(
    path.join(rootPath, "cleanroom-input/MANIFEST.md"),
    "utf8",
  );
  const match = manifest.match(/Source commit SHA: ([0-9a-f]{40})/);
  if (match === null) {
    throw new Error("cleanroom-input/MANIFEST.md has no Source commit SHA");
  }
  return match[1];
}

function usage() {
  return [
    "Usage:",
    "  node scripts/check-target-replay-evidence-file.cjs --driver <driver.mbt.qnt> --evidence <evidence.json>",
    "",
    "This is a diagnostic per-file replay evidence check. It does not replace",
    "node scripts/check-cleanroom-harness.cjs and does not close work-loop acceptance.",
  ].join("\n");
}

function main() {
  const rootPath = process.cwd();
  const driverPath = argValue("--driver");
  const evidencePath = argValue("--evidence");
  if (driverPath === undefined || evidencePath === undefined) {
    console.error(usage());
    process.exit(2);
  }

  const inventory = readJson(
    path.join(rootPath, "cleanroom-input/branch-coverage/source-branch-inventory.json"),
  );
  const profile = readJson(path.join(rootPath, "target-profile.json"));
  const evidence = readJson(path.join(rootPath, evidencePath));
  const selected = {
    ...inventory,
    branchObligations: (inventory.branchObligations ?? []).filter(
      (entry) => entry.driverPath === driverPath,
    ),
    sampledInputs: (inventory.sampledInputs ?? []).filter(
      (entry) => entry.driverPath === driverPath,
    ),
  };

  const result = validateTargetReplayEvidence(
    evidence,
    selected,
    sha256Text(stableStringify(inventory)),
    {
      expectedCleanroomManifestSourceCommitSha: manifestSourceCommit(rootPath),
      expectedTargetProfileSha256: sha256Text(stableStringify(profile)),
    },
  );

  if (result.issues.length > 0) {
    console.error("target replay evidence file check FAILED:");
    for (const issue of result.issues) {
      console.error(`  - ${issue}`);
    }
    process.exit(1);
  }

  console.log(
    `target replay evidence file check passed: ${result.covered.length} obligations covered`,
  );
  for (const obligationId of result.covered) {
    console.log(`  - ${obligationId}`);
  }
}

if (require.main === module) {
  main();
}
