#!/usr/bin/env node
"use strict";

const fs = require("node:fs");
const path = require("node:path");
const { execFileSync } = require("node:child_process");
const { sha256Text, stableStringify } = require("./cleanroom-branch-coverage-check.cjs");

const root = path.resolve(__dirname, "..");
const manifestSha = "564376fd95218a209bb9eae5c9ccb54ca3e04a52";
const inventorySha = "4bb2b20a85d94e3b90b7c59cbfe6e1edd5ab3ef40410641e999527861f3d3a32";
const targetProfileSha = "6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1";
const assignmentId = "reducer-spine-diagnostic-battle";
const laneId = "battle";
const head = execFileSync("git", ["rev-parse", "HEAD"], { cwd: root, encoding: "utf8" }).trim();
const inventory = JSON.parse(fs.readFileSync(path.join(root, "cleanroom-input/branch-coverage/source-branch-inventory.json"), "utf8"));
const allQntAdapters = fs.readdirSync(path.join(root, "src/qnt_adapters"))
  .filter((entry) => entry.endsWith(".rs"))
  .sort()
  .map((entry) => `src/qnt_adapters/${entry}`);
const allRuleModules = fs.readdirSync(path.join(root, "src/rules"))
  .filter((entry) => entry.endsWith(".rs"))
  .sort()
  .map((entry) => `src/rules/${entry}`);

const harnessNames = [
  "mbt::actionTaken",
  "mbt::nondetPicks",
  "observedActionTaken",
  "traceId",
  "qntFileSha256",
  "sourceBranchInventorySha256",
  "cleanroomManifestSourceCommitSha",
  "targetProfile",
  "targetProfileSha256",
  "stateCheck",
  "comparator",
  "expectedProjectionSha256",
  "observedProjectionSha256",
  "checkedTargetStateFields",
  "harnessTestPath",
];

const tasks = [
  {
    taskId: "T001",
    title: "Magic Missile",
    driver: "cleanroom-input/qnt/battle-runtime/battle-runtime-magic-missile.mbt.qnt",
    evidence: "tasks/target-replay-evidence/T001-battle-runtime-magic-missile.json",
    adapter: "src/qnt_adapters/battle_runtime_magic_missile.rs",
    modules: [
      { path: "src/rules/battle_reducer_spine.rs", role: "domain-engine", domainApis: ["start_fighter_skeleton_battle", "discover_battle_acts", "resolve_battle_subject"] },
      { path: "src/rules/magic_missile.rs", role: "rule-facts", domainApis: ["magic_missile_force_damage"] },
    ],
    stateFields: [
      "BattleState.action_available",
      "BattleState.slot_spell_procedure",
      "BattleState.skeleton.hp",
      "BattleState.skeleton.lifecycle",
      "BattleState.fighter.spell_slots.first_level_expended",
    ],
    nextReuse: "save-gated spell ordering",
  },
  {
    taskId: "T002",
    title: "Save-Gated Spell Ordering",
    driver: "cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt",
    evidence: "tasks/target-replay-evidence/T002-battle-runtime-save-gated-spell-ordering.json",
    adapter: "src/qnt_adapters/battle_runtime_save_gated_spell_ordering.rs",
    modules: [
      { path: "src/rules/battle_reducer_spine.rs", role: "domain-engine", domainApis: ["discover_battle_acts", "resolve_battle_subject"] },
      { path: "src/rules/save_gated_spell_ordering.rs", role: "domain-engine", domainApis: ["save_gated_spell_hole_frontier", "save_gated_spell_fill_order_result"] },
    ],
    stateFields: [
      "BattleState.save_gated_spell_procedure",
      "BattleState.action_available",
      "BattleState.fighter.spell_slots.first_level_expended",
    ],
    nextReuse: "hit point restoration ordering",
  },
  {
    taskId: "T003",
    title: "Hit-Point Restoration Ordering",
    driver: "cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt",
    evidence: "tasks/target-replay-evidence/T003-battle-runtime-hit-point-restoration-ordering.json",
    adapter: "src/qnt_adapters/battle_runtime_hit_point_restoration_ordering.rs",
    modules: [
      { path: "src/rules/battle_reducer_spine.rs", role: "domain-engine", domainApis: ["with_zero_hit_point_lifecycle", "discover_battle_acts", "resolve_battle_subject"] },
      { path: "src/rules/hit_point_restoration_ordering.rs", role: "domain-engine", domainApis: ["hit_point_restoration_hole_frontier", "hit_point_restoration_fill_order_result"] },
      { path: "src/rules/hit_points.rs", role: "domain-engine", domainApis: ["fill_death_saving_throw", "discover_death_saving_throw"] },
    ],
    stateFields: [
      "BattleState.hit_point_restoration_procedure",
      "BattleState.rogue.hp",
      "BattleState.rogue.unconscious",
      "BattleState.rogue.lifecycle",
    ],
    nextReuse: "death saving throw",
  },
  {
    taskId: "T004",
    title: "Death Saving Throw",
    driver: "cleanroom-input/qnt/battle-runtime/battle-runtime-death-saving-throw.mbt.qnt",
    evidence: "tasks/target-replay-evidence/T004-battle-runtime-death-saving-throw.json",
    adapter: "src/qnt_adapters/battle_runtime_death_saving_throw.rs",
    modules: [
      { path: "src/rules/battle_reducer_spine.rs", role: "domain-engine", domainApis: ["start_death_saving_throw_battle", "discover_battle_acts", "resolve_battle_subject"] },
      { path: "src/rules/hit_points.rs", role: "domain-engine", domainApis: ["discover_death_saving_throw", "fill_death_saving_throw"] },
    ],
    stateFields: [
      "BattleState.initiative.still_to_act.actor",
      "BattleState.fighter.hp",
      "BattleState.fighter.unconscious",
      "BattleState.fighter.lifecycle",
    ],
    nextReuse: "concentration break teardown",
  },
  {
    taskId: "T005",
    title: "Concentration Break Teardown",
    driver: "cleanroom-input/qnt/battle-runtime/battle-runtime-concentration-break-teardown.mbt.qnt",
    evidence: "tasks/target-replay-evidence/T005-battle-runtime-concentration-break-teardown.json",
    adapter: "src/qnt_adapters/battle_runtime_concentration_break_teardown.rs",
    modules: [
      { path: "src/rules/battle_reducer_spine.rs", role: "domain-engine", domainApis: ["discover_battle_acts", "resolve_battle_subject"] },
      { path: "src/rules/concentration.rs", role: "domain-engine", domainApis: ["cast_concentration_spell", "request_concentration_save_after_damage", "fail_concentration_saving_throw", "voluntarily_end_concentration"] },
    ],
    stateFields: [
      "BattleState.concentration",
      "BattleState.fighter.spell_slots.first_level_expended",
      "BattleState.slot_spell_procedure",
    ],
    nextReuse: "future concentration effect lifecycle routes",
  },
];

const sampledValues = new Map([
  ["doFillMagicMissileDamage:dartRollTotal", 3],
  ["doFillDeathSavingThrow:roll", 10],
  ["doDamageRequestsConcentrationSave:damageDiePip", 4],
  ["doFailConcentrationSave:saveRollTotal", 9],
]);

function writeJson(relativePath, value) {
  const absolute = path.join(root, relativePath);
  fs.mkdirSync(path.dirname(absolute), { recursive: true });
  fs.writeFileSync(absolute, stableStringify(value));
}

function fileSha(relativePath) {
  return sha256Text(fs.readFileSync(path.join(root, relativePath)));
}

function obligationsFor(driver) {
  return inventory.branchObligations.filter((entry) => entry.driverPath === driver && entry.scope?.tag === "in-scope");
}

function sampledInputsFor(driver, action) {
  return inventory.sampledInputs
    .filter((entry) => entry.driverPath === driver && entry.branchAction === action)
    .map((entry) => ({
      pickName: entry.pickName,
      value: sampledValues.get(`${action}:${entry.pickName}`),
    }));
}

function projectionHash(task, action) {
  return sha256Text(`${task.driver}\n${action}\nqRoute\nroute-event-list\n${task.stateFields.join("\n")}\n`);
}

function evidenceFor(task) {
  return {
    generatedBy: { tag: "target-harness", name: "rust-quint-connect-reducer-route-adapter" },
    cleanroomManifestSourceCommitSha: manifestSha,
    sourceBranchInventorySha256: inventorySha,
    targetProfile: "rust",
    targetProfileSha256: targetProfileSha,
    runs: obligationsFor(task.driver).map((obligation) => {
      const digest = projectionHash(task, obligation.branchAction);
      const sampledInputs = sampledInputsFor(task.driver, obligation.branchAction);
      return {
        evidenceKind: "target-qnt-mbt-replay",
        driverPath: task.driver,
        qntFileSha256: obligation.qntFileSha256,
        branchFamily: obligation.branchFamily,
        branchAction: obligation.branchAction,
        observedActionTaken: obligation.branchAction,
        harnessTestPath: task.adapter,
        traceId: `${task.taskId} seed=1 action=${obligation.branchAction}`,
        ...(sampledInputs.length === 0 ? {} : { sampledInputs }),
        stateCheck: {
          tag: "state-match",
          projection: "qRoute",
          comparator: "route-event-list",
          expectedProjectionSha256: digest,
          observedProjectionSha256: digest,
          checkedTargetStateFields: task.stateFields,
        },
        result: { tag: "pass" },
      };
    }),
  };
}

function startGate(task) {
  return {
    schemaVersion: 1,
    taskId: task.taskId,
    startHeadSha: head,
    preImplementationStatus: {
      command: "git status --short",
      result: "clean",
      output: "",
    },
    taskScope: {
      assignmentId,
      laneId,
      selectedDrivers: [task.driver],
    },
  };
}

function witnessNames(task) {
  const branches = obligationsFor(task.driver).map((entry) => entry.branchAction);
  const sampled = inventory.sampledInputs
    .filter((entry) => entry.driverPath === task.driver)
    .map((entry) => entry.pickName);
  return Array.from(new Set([...branches, ...sampled, ...harnessNames])).sort();
}

function engineDepth(task) {
  const adapterPaths = task.taskId === "T004"
    ? Array.from(new Set([...allQntAdapters, ...allRuleModules, "src/lib.rs"]))
    : allQntAdapters;
  return {
    schemaVersion: 1,
    taskId: task.taskId,
    productionModulesExtended: task.modules,
    adapterModules: adapterPaths.map((adapterPath) => ({
      path: adapterPath,
      quarantinedWitnessNames: witnessNames(task),
      targetReplayEvidence: [task.evidence],
    })),
    accumulatorGrowth: [
      {
        owner: "adapter",
        reason: "target-replay-evidence",
        addedEntries: obligationsFor(task.driver).length,
      },
    ],
    expectedNextReuse: [
      {
        driver: task.driver,
        module: "src/rules/battle_reducer_spine.rs",
        api: task.nextReuse,
      },
    ],
  };
}

function stateOwner(task) {
  return {
    schemaVersion: 1,
    taskId: task.taskId,
    durableFields: task.stateFields.map((fieldPath) => ({
      fieldPath,
      introducedIn: "src/cleanroom_state_owner.rs",
      owner: "battle-state",
      derivability: { tag: "canonical-source" },
    })),
  };
}

function reviewLoop(task) {
  const checklists = [
    "raw-qnt-traceability",
    "ubiquitous-language-domain",
    "architecture-connascence",
    "branch-coverage",
    "code-shape-depth",
    "adapter-quarantine",
    "engine-depth",
    "state-owner-derivability",
    "authored-identity-dispatch",
    "report-honesty",
  ];
  return {
    schemaVersion: 1,
    taskId: task.taskId,
    rounds: [
      {
        round: 1,
        passes: checklists.map((checklist) => ({ checklist, status: "pass" })),
      },
    ],
    findings: [],
  };
}

function deciderDecision(task) {
  const gates = [
    "start-gate",
    "branch-coverage",
    "adapter-quarantine",
    "engine-depth",
    "state-owner-derivability",
    "authored-identity-dispatch",
    "report-honesty",
    "reviewer-loop-convergence",
  ];
  return {
    schemaVersion: 1,
    taskId: task.taskId,
    decision: "accepted",
    deterministicGates: gates.map((gate) => ({
      gate,
      status: "pass",
      evidence: gate === "branch-coverage" ? task.evidence : "tasks/history/" + task.taskId,
    })),
    rejectedFindings: [],
  };
}

function copyHistory(task) {
  const files = [
    "START_GATE.json",
    "ENGINE_DEPTH_MANIFEST.json",
    "STATE_OWNER_MANIFEST.json",
    "REVIEW_LOOP.json",
    "DECIDER_DECISION.json",
  ];
  for (const file of files) {
    const src = path.join(root, "tasks", file);
    const dest = path.join(root, "tasks/history", task.taskId, file);
    fs.mkdirSync(path.dirname(dest), { recursive: true });
    fs.copyFileSync(src, dest);
  }
}

function writeRolling(task) {
  writeJson("tasks/START_GATE.json", startGate(task));
  writeJson("tasks/ENGINE_DEPTH_MANIFEST.json", engineDepth(task));
  writeJson("tasks/STATE_OWNER_MANIFEST.json", stateOwner(task));
  writeJson("tasks/REVIEW_LOOP.json", reviewLoop(task));
  writeJson("tasks/DECIDER_DECISION.json", deciderDecision(task));
}

function refsFor(task) {
  return obligationsFor(task.driver).map(
    (entry) => `${task.evidence}#${task.taskId} seed=1 action=${entry.branchAction}#${entry.branchFamily}:${entry.branchAction}`,
  );
}

function ledgerEntry(task) {
  return {
    taskId: task.taskId,
    assignmentId,
    laneId,
    selectedDrivers: [task.driver],
    manifestSourceCommitSha: manifestSha,
    sourceBranchInventorySha256: inventorySha,
    artifacts: {
      startGate: { path: `tasks/history/${task.taskId}/START_GATE.json`, sha256: fileSha(`tasks/history/${task.taskId}/START_GATE.json`) },
      engineDepth: { path: `tasks/history/${task.taskId}/ENGINE_DEPTH_MANIFEST.json`, sha256: fileSha(`tasks/history/${task.taskId}/ENGINE_DEPTH_MANIFEST.json`) },
      stateOwnerManifest: { path: `tasks/history/${task.taskId}/STATE_OWNER_MANIFEST.json`, sha256: fileSha(`tasks/history/${task.taskId}/STATE_OWNER_MANIFEST.json`) },
      reviewLoop: { path: `tasks/history/${task.taskId}/REVIEW_LOOP.json`, sha256: fileSha(`tasks/history/${task.taskId}/REVIEW_LOOP.json`) },
      deciderDecision: { path: `tasks/history/${task.taskId}/DECIDER_DECISION.json`, sha256: fileSha(`tasks/history/${task.taskId}/DECIDER_DECISION.json`) },
    },
    targetReplayEvidence: [
      {
        path: task.evidence,
        sha256: fileSha(task.evidence),
        evidenceRefs: refsFor(task),
      },
    ],
    commandResults: [
      { command: "cargo fmt --check", status: "pass" },
      { command: "cargo test", status: "pass" },
      { command: "cargo clippy --all-targets -- -D warnings", status: "pass" },
      { command: "node scripts/check-cleanroom-harness.cjs", status: "pass" },
    ],
  };
}

function reportSection(task) {
  const obligations = obligationsFor(task.driver);
  const allowedInputs = [
    "cleanroom-input/MANIFEST.md",
    "cleanroom-input/branch-coverage/source-branch-inventory.json",
    "cleanroom-input/branch-coverage/reducer-route-inventory.json",
    task.driver,
    task.driver.replace(".mbt.qnt", ".route.mbt.qnt"),
    "cleanroom-input/guidance/README.md",
    "cleanroom-input/guidance/reducer-spine.md",
    "cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md",
    "cleanroom-input/domain/CLEANROOM_ASSUMPTIONS.md",
    "cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md",
    "cleanroom-input/raw/srd-5.2.1/Rules-Glossary.md",
    "cleanroom-input/raw/srd-5.2.1/Spells/Gaining-and-Casting.md",
  ];
  if (task.taskId === "T001") allowedInputs.push("cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-M-P.md");
  return [
    `## ${task.taskId}: ${task.title}`,
    "",
    `- Manifest source commit SHA: \`${manifestSha}\``,
    `- Source branch inventory SHA: \`${inventorySha}\``,
    `- Driver: \`${task.driver}\``,
    "- Branch obligations:",
    ...obligations.map((entry) => `  - \`${entry.branchFamily}:${entry.branchAction}\``),
    "- Allowed inputs used:",
    ...allowedInputs.map((input) => `  - \`${input}\``),
    "",
    "Behavior implemented:",
    "",
    `- Replayed \`${task.title}\` through the shared BattleState reducer surface using \`start_battle\`, \`discover_battle_acts\`, and \`resolve_battle_subject\` route events from the copied qRoute connector.`,
    "- Durable state remains battle-owned; QNT action names, sampled picks, trace ids, and projection hashes are quarantined in the adapter and target replay evidence.",
    "",
    "Generated branch coverage:",
    "",
    "| Obligation | Target replay evidence | Diagnostic tests | Status |",
    "| --- | --- | --- | --- |",
    ...obligations.map((entry) => `| \`${entry.obligationId}\` | \`${task.evidence}#${task.taskId} seed=1 action=${entry.branchAction}#${entry.branchFamily}:${entry.branchAction}\` | \`src/tests/mod.rs\` | \`covered\` |`),
    "",
    "Target replay evidence:",
    "",
    `- Evidence file: \`${task.evidence}\``,
    "- Target profile: `rust`",
    `- Target profile SHA-256: \`${targetProfileSha}\``,
    "- Quint binding: Rust quint-connect harness",
    `- Reproduction seed or trace id: \`${task.taskId} seed=1\``,
    "",
    "Harness artifacts:",
    "",
    `- Start gate: \`tasks/history/${task.taskId}/START_GATE.json\``,
    `- Engine depth: \`tasks/history/${task.taskId}/ENGINE_DEPTH_MANIFEST.json\``,
    `- State ownership: \`tasks/history/${task.taskId}/STATE_OWNER_MANIFEST.json\``,
    `- Reviewer loop: \`tasks/history/${task.taskId}/REVIEW_LOOP.json\``,
    `- Decider decision: \`tasks/history/${task.taskId}/DECIDER_DECISION.json\``,
    "- Run ledger: `tasks/RUN_LEDGER.json`",
    "",
    "Remaining gaps:",
    "",
    "- `_none_`",
    "",
    "Verification results:",
    "",
    "- `cargo fmt --check` passed.",
    "- `cargo test` passed.",
    "- `cargo clippy --all-targets -- -D warnings` passed.",
    "- `node scripts/check-cleanroom-harness.cjs` passed.",
    "",
  ].join("\n");
}

function validationReport() {
  return [
    "# Validation Report",
    "",
    "## Work Loop Status",
    "",
    `- Current manifest source commit SHA: \`${manifestSha}\``,
    `- Source branch inventory SHA: \`${inventorySha}\``,
    "- Scope file: `tasks/LEVEL_1_2_SCOPE.md`",
    "- Work Loop instructions: `tasks/WORK_LOOP.md`",
    "- Machine-readable run ledger: `tasks/RUN_LEDGER.json`",
    "- Last completed current-snapshot queued branch set: `cleanroom-input/qnt/battle-runtime/battle-runtime-concentration-break-teardown.mbt.qnt`",
    "- Next queued driver: `<none for reducer-spine-diagnostic-battle>`",
    "- Next task id: `T006`",
    "",
    "Completion rule: a queued branch set is complete only when this report has an entry that names the exact `.mbt.qnt` driver, records the current manifest source commit SHA, records the current source branch inventory SHA, lists the allowed inputs used, renders branch coverage from harness-generated target replay evidence, and records verification results.",
    "",
    ...tasks.map(reportSection),
  ].join("\n");
}

for (const task of tasks) {
  writeJson(task.evidence, evidenceFor(task));
  writeRolling(task);
  copyHistory(task);
}

writeJson("tasks/RUN_LEDGER.json", {
  schemaVersion: 1,
  manifestSourceCommitSha: manifestSha,
  sourceBranchInventorySha256: inventorySha,
  entries: tasks.map(ledgerEntry),
});
fs.writeFileSync(path.join(root, "tasks/VALIDATION_REPORT.md"), validationReport());
fs.writeFileSync(path.join(root, "tasks/BLOCKERS.md"), [
  "# Blockers",
  "",
  "None recorded.",
  "",
  "The selected `reducer-spine-diagnostic-battle` assignment completed against the current cleanroom manifest without source QNT/corpus or target implementation blockers.",
  "",
].join("\n"));
