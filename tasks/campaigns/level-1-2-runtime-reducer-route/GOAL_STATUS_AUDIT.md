# Goal Status Audit

Campaign: `level-1-2-runtime-reducer-route`

Audit date: 2026-06-27

Audit baseline:

- dirty campaign branch: `ralph/rrconv-19-cleanroom`
- dirty campaign head before this audit file: `15d158b`
- accepted fresh target: `/workspace/typescript/.codex-worktrees/dnd-fresh-cleanroom-dry-run-fc00`
- accepted fresh target head: `0d5200e43fd7e9f094a93585f00eaf6bd2266c75`
- current fresh gate: `python3 tools/verify_current_fresh_target.py`

## Objective

Prove and refine a QNT-driven architecture where focused `.qnt` slices can
generate or constrain an independent character/battle runtime through shared
reducer-shaped semantics, without TypeScript implementation knowledge or
authored identity dispatch.

The dirty Rust cleanroom is only a rehearsal and evidence target. It is useful
only insofar as it teaches which QNT/guidance facts are sufficient for a later
fresh cleanroom and which review checks prevent false positives.

## Requirement Audit

| Requirement | Current evidence | Status | Notes |
| --- | --- | --- | --- |
| Dirty rehearsal routes the refreshed in-scope denominator through reducer-shaped evidence. | `STATE.json.coverageCurrent` records `97 / 97` accepted drivers and `659 / 659` accepted refreshed in-scope obligations; `CHECKPOINT_REPORT.md` records CP8 and Pact Slot replay evidence. | Proven for dirty rehearsal. | This is diagnostic evidence only. It must not be treated as final architecture proof. |
| Accepted dirty evidence uses shared reducer entrypoints rather than local replay islands. | `FRESH_CLEANROOM_READINESS.md` summarizes the review loop that rejected adapter-local synthesis and required observed route events from reducer entrypoints; `CHECKPOINT_REPORT.md` records per-lane verification. | Proven for accepted dirty rows by campaign review artifacts. | The proof strength is bounded by the dirty target and its historical scaffolding. |
| Fresh target can be built without reading TypeScript implementation or dirty `src/**` implementation files. | Fresh target state/evidence records `copiedDirtySrcImplementation: false`; `python3 tools/verify_current_fresh_target.py` passes; the accepted fresh target is at `0d5200e43fd7e9f094a93585f00eaf6bd2266c75`. | Proven for the FC-00 through FC-08 dry-run scope plus FEXP-00/FEXP-01/FEXP-02/FEXP-03/FEXP-04/FEXP-05/FEXP-06 expansion lanes. | This does not prove a complete runtime can be generated from scratch in one larger fresh run. |
| Fresh target exposes the reducer-shaped surface. | `FRESH_RUN_REPORT.md`, `STATE_OWNERS.md`, and tests in the fresh target cover `start_battle`, `discover_battle_acts`, `resolve_battle_subject`, and turn advancement. | Proven for the dry-run target. | The surface is intentionally minimal and tracer-driven. |
| Fresh target supports an SDK-style programmatic character plus simple battle scenario. | `FRESH_SDK_COMPOSITION_ACCEPTANCE.md`, `EVIDENCE/sdk-tracer-bullet-programmatic-surface.json`, and `python3 tools/verify_current_fresh_target.py` cover sheet creation, projection, encounter composition, battle entry, act resolution, HP mutation, action spend, and turn advancement. | Proven for the accepted tracer scenario. | This is a tracer-bullet scenario, not full character/battle runtime parity. |
| FEXP-01 diagnostic battle route pack is accepted in the fresh target. | Fresh target head `a78d1d6c4c5ec6eaad5ea99c9b6bfde296020639` includes `EVIDENCE/fexp01-diagnostic-battle-route-pack.json`, `examples/fexp01_route_observations.rs`, and `tools/verify_fexp01.py`; `python3 tools/verify_current_fresh_target.py` passes. | Proven for Magic Missile slot-spell route, HP restoration ordering, Death Saving Throw, and Concentration teardown. | The verifier parser is intentionally narrow for the accepted connector shapes; broader drivers still need their own lanes. |
| FEXP-02 spell-attack/save-gated unblock lane is accepted with blockers in the fresh target. | Fresh target head `773fe97d95e568c6acc99cc2bbe3ce6d57fc50bc` includes `EVIDENCE/fexp02-spell-attack-save-gated-unblock.json`, `examples/fexp02_route_observations.rs`, and `tools/verify_fexp02.py`; `python3 tools/verify_current_fresh_target.py` passes. | Proven for generic spell-attack and save-gated route substrates plus eight selected-identity branch subsets accepted only as substrate evidence. | Residual selected spell effects remain blocked outside the generic substrate; this is intentionally not full selected-spell coverage. |
| FEXP-03 chained/object spell attack lane is accepted with blockers in the fresh target. | Fresh target head `9b2f81bfabb6f1afd7daede0455be054bb92d78c` includes `EVIDENCE/fexp03-chained-and-object-spell-attacks.json`, `examples/fexp03_route_observations.rs`, and `tools/verify_fexp03.py`; `python3 tools/verify_current_fresh_target.py` passes. | Proven for generic independent spell-attack sequence, chained duplicate-damage leap, and object-target spell-attack route surfaces through public reducer entrypoints. | Isolated object stale replay remains blocked because the route is not publicly observable without fabricating a same-battle stale object subject token. |
| FEXP-04 active-effect lifecycle lane is accepted with blockers in the fresh target. | Fresh target head `e8b0310e647ad471089fcd34737b8fd70211b373` includes `EVIDENCE/fexp04-active-effect-lifecycle-and-roll-modifiers.json`, `examples/fexp04_route_observations.rs`, and `tools/verify_fexp04.py`; `python3 tools/verify_current_fresh_target.py` passes. | Proven for generic roll-modifier active-effect, scalar-buff active-effect, targeted-speed scalar buff, and turn-boundary cleanup route surfaces through public reducer entrypoints. | Residual concentration-break cleanup, cumulative scalar sequencing, and exact roll-choice payloads remain source-input blockers. |
| FEXP-05 reaction/interrupt lane is accepted with blockers in the fresh target. | Fresh target head `eb05e8495eac993b69e17f68544edace6e56caee` includes `EVIDENCE/fexp05-reaction-interrupt-and-boundary.json`, `examples/fexp05_route_observations.rs`, and `tools/verify_fexp05.py`; `python3 tools/verify_current_fresh_target.py` passes. | Proven for generic reaction casting-time interrupt/resume, after-damage reaction, nested interrupt resume, active-effect resume, and recorded procedure replay route surfaces through public reducer entrypoints. | Selected reaction spell projections and interrupt trigger taxonomy remain source-input blockers. |
| FEXP-06 character/sheet/handoff lane is accepted with blockers in the fresh target. | Fresh target head `0d5200e43fd7e9f094a93585f00eaf6bd2266c75` includes `EVIDENCE/fexp06-character-creation-sheet-handoff-pack.json`, `examples/fexp06_route_observations.rs`, and `tools/verify_fexp06.py`; `python3 tools/verify_current_fresh_target.py` passes. | Proven for character creation finalization, sheet hit-point route projection, short-rest Pact-slot completion, and happy-path battle settlement route surfaces through public character/sheet/handoff entrypoints. | Partial/rejection/resource/conflict branches remain source-input blockers. |
| Post-FC06 source feedback was resolved and replayed. | `FC06_SOURCE_FEEDBACK.md` records encounter composition and Pact Slot route-surface resolution; fresh target head `0d5200e43fd7e9f094a93585f00eaf6bd2266c75` includes the accepted Pact Slot replay, current verifier, FEXP-00 baseline lock, FEXP-01 route pack, FEXP-02 spell substrate lane, FEXP-03 chained/object spell attack lane, FEXP-04 active-effect lifecycle lane, FEXP-05 reaction/interrupt lane, and FEXP-06 character/sheet/handoff lane. | Proven for the two recorded FC-06 feedback items. | Older FC-03/FC-04/FC-05 verifier artifacts are now historical snapshots, not current gates. |
| Production runtime semantics avoid authored identity dispatch. | Dirty campaign review artifacts and fresh verifier check production source for selected identity terms; accepted docs state identity remains in adapter/test/evidence boundaries. | Proven for sampled/accepted target scopes. | A future full fresh run still needs the same static and review gates across its whole production surface. |
| Focused QNT slices are sufficient to guide a full independent runtime. | Dirty campaign plus fresh dry run prove multiple focused slices, route connectors, source feedback loops, and SDK tracer surfaces. | Not fully proven. | Current evidence is strong but partial: it proves a successful dry run and diagnostic dirty coverage, not a complete fresh runtime campaign from only the package. |
| The final architecture is complete and ready to mark the thread goal achieved. | No artifact currently proves full fresh-cleanroom generation or complete runtime coverage independent of dirty scaffolding. | Not achieved. | Keep the goal active. The next proof step should be a larger fresh-cleanroom campaign or source-side guidance/QNT promotion based on this audit. |

## Current Accepted Gates

Fresh target gate:

```sh
cd /workspace/typescript/.codex-worktrees/dnd-fresh-cleanroom-dry-run-fc00
python3 tools/verify_current_fresh_target.py
```

This gate validates:

- current `b57772b459f1b75592fd45b9196fd60965b534d3` input package inventory;
- strict FC-07 Pact Slot source-input hashes;
- FC-07 route comparisons;
- SDK tracer-bullet public surface;
- FC-03/FC-04/FC-05 historical snapshot classification.
- FEXP-01 diagnostic battle route evidence, including connector-derived
  expected records and executable reducer-entrypoint observed records.
- FEXP-02 spell-attack/save-gated substrate evidence, including connector-derived
  expected records, executable reducer-entrypoint observed records, and explicit
  blockers for residual selected spell effects outside the generic substrate.
- FEXP-03 chained/object spell-attack substrate evidence, including
  connector-derived expected records, executable reducer-entrypoint observed
  records, and an explicit blocker for isolated object stale replay.
- FEXP-04 active-effect lifecycle substrate evidence, including expected records
  mechanically derived from copied QNT connector action bodies/helper
  vocabulary, executable reducer-entrypoint observed records, and explicit
  blockers for residual active-effect source-input gaps.
- FEXP-05 reaction/interrupt substrate evidence, including expected records
  mechanically derived from copied QNT connector action bodies/helper
  vocabulary, executable reducer-entrypoint observed records, and explicit
  blockers for selected reaction spell projections and interrupt trigger
  taxonomy.
- FEXP-06 character/sheet/handoff substrate evidence, including expected records
  mechanically derived from copied QNT connector action bodies/helper
  vocabulary, executable character/sheet/handoff observed records, and explicit
  blockers for partial/rejection/resource/conflict source-input gaps.

Dirty campaign state:

```sh
cd /workspace/typescript/.codex-worktrees/dnd-cleanroom-rrconv-19
jq '.status, .coverageCurrent, .freshCleanroomDryRun.status' \
  tasks/campaigns/level-1-2-runtime-reducer-route/STATE.json
```

Expected state:

- `fresh-current-verifier-accepted-source-feedback-resolved`
- `659 / 659` refreshed in-scope dirty obligations accepted
- `accepted-through-current-verifier`

## Remaining Work To Prove The Full Objective

1. Run a larger fresh cleanroom campaign from the accepted input package and
   campaign guidance, not from dirty `src/**`. The proposed campaign boundary
   is `FRESH_CLEANROOM_EXPANSION_PLAN.md` with lane definitions in
   `FRESH_EXPANSION_LANES.json`.
2. Keep the accepted fresh target at `0d5200e43fd7e9f094a93585f00eaf6bd2266c75`
   as the baseline evidence target unless a newer package refresh supersedes it.
3. Promote any additional route-connector lessons back into source-side QNT or
   curated cleanroom guidance before treating a fresh-run blocker as a target
   implementation problem.
4. Preserve the review gates from this campaign: observed reducer events,
   independent expected route records, state-owner derivability records, and no
   production authored identity dispatch.
5. Treat global dirty-harness validator-hash cleanup as optional unless the
   dirty harness must pass without exception; it is not the main proof path.

## Completion Decision

Do not mark the global objective complete from the current evidence alone.

The current evidence proves:

- the dirty rehearsal reached full refreshed in-scope route coverage;
- the fresh dry run independently accepted the reducer-spine tracer sequence,
  SDK composition tracer, post-feedback Pact Slot replay, and FEXP-01
  diagnostic battle route pack;
- FEXP-02 independently accepted generic spell-attack/save-gated substrate
  evidence with explicit blockers for remaining selected spell effects;
- FEXP-03 independently accepted chained/object spell-attack substrate evidence
  with an explicit blocker for isolated object stale replay;
- FEXP-04 independently accepted active-effect lifecycle substrate evidence with
  explicit blockers for residual source-input gaps;
- FEXP-05 independently accepted reaction/interrupt substrate evidence with
  explicit blockers for residual selected reaction spell and interrupt-trigger
  source-input gaps;
- FEXP-06 independently accepted character/sheet/handoff substrate evidence with
  explicit blockers for partial/rejection/resource/conflict source-input gaps;
- the current verifier makes that fresh evidence mechanically checkable.

The current evidence does not yet prove:

- a complete independent character/battle runtime can be generated or
  constrained from focused `.qnt` slices alone;
- the architecture holds across a broader fresh-cleanroom implementation
  campaign without dirty historical scaffolding.
