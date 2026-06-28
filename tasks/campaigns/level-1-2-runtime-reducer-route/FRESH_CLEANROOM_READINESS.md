# Fresh Cleanroom Readiness

Campaign: `level-1-2-runtime-reducer-route`

Integration worktree: `/workspace/typescript/.codex-worktrees/dnd-cleanroom-rrconv-19`

Integration branch: `ralph/rrconv-19-cleanroom`

Integration head when this readiness report was refreshed: `5234a36b2831c5ab781dbc73e88e3d43e07a1154`

Verified reducer-route head: `73627315f70528e73f5eb4ef781606e876e87367`

## Goal Boundary

The real goal is to prove and refine a QNT-driven architecture where focused
`.qnt` slices and curated cleanroom guidance define enough reducer-shaped
semantics for an independent character/battle runtime to be built without
TypeScript implementation knowledge or production authored-identity dispatch.

This campaign is dirty cleanroom rehearsal evidence only. It does not prove
that a fresh cleanroom agent can already generate the complete runtime on the
first attempt. It does prove which QNT/guidance shapes were sufficient to drive
the existing dirty Rust target through shared reducer entrypoints, and which
review checks were needed to prevent false positives.

## Final Campaign State

Authoritative state:

- `STATE.json.status`: `sqnt03b-dirty-replay-merged-verified`
- refreshed in-scope obligations: `659`
- accepted obligations: `659`
- accepted driver coverage: `97 / 97`
- blockers: `0`
- out-of-scope obligations: `45`
- active lane worktrees: none
- active fresh dry-run worktrees: none
- strongest fresh target evidence: `05280a8e2d6e9705411c114c80ae2a4e4290de2c`
- latest dirty package refresh:
  `8d4ab3b2c567f3b292de6aebbc5b6fb49c3e00da`, from source package
  `d00c92a3d12531e50d95ead220303b66a5265e1e`
- latest source-QNT connector: SQNT-03B Next Attack Roll mode source input is
  packaged in
  `cleanroom-input/qnt/battle-runtime/battle-runtime-next-attack-roll-mode.route.mbt.qnt`
  and `cleanroom-input/qnt/battle-runtime/battle-runtime-next-attack-roll-mode-route-facts.qnt`;
  dirty target replay is accepted at merge
  `4437eacc311a8ea069bc1d7c9dd9d2b334a8fb4e`; fresh target replay remains
  pending.
- previous source-QNT connector: SQNT-03A Hit Point regain prevention source
  input is packaged in
  `cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-regain-prevention.route.mbt.qnt`;
  dirty target replay is accepted at merge
  `73627315f70528e73f5eb4ef781606e876e87367`; fresh target replay remains
  pending.
- latest dirty replay refreshes: FCSF-04 accepted 6 object stale route-history
  connector rows covering all 7 Starry Wisp object obligations in
  `tasks/target-replay-evidence/FCSF-04-object-stale-dirty-replay.json`;
  FCSF-05 accepted all 5 reaction/interrupt taxonomy connector rows in
  `tasks/target-replay-evidence/FCSF-05-reaction-interrupt-dirty-replay.json`;
  FCSF-06 accepted 30 reducer-route witness rows and recorded 14 target blockers
  in `tasks/target-replay-evidence/FCSF-06-character-sheet-handoff-dirty-replay.json`;
  SQNT-03A accepted 3 Hit Point regain prevention connector transitions and 1
  attack-shape obligation with 0 SQNT-03A target behavior blockers in
  `tasks/target-replay-evidence/FCSF-03A-hit-point-regain-prevention-dirty-replay.json`;
  SQNT-03B accepted 6 next-Attack-Roll mode connector transitions covering 2
  selected-driver obligations in
  `tasks/target-replay-evidence/FCSF-03B-next-attack-roll-mode-dirty-replay.json`.

The final CP8 lane accepted exactly these three newly accepted rows:

- `doDiscoverMageArmorUnarmoredSelfTarget`
- `doRejectMageArmorArmoredTarget`
- `doExpireMageArmorDuration`

The pre-existing base Armor Class projection row was refreshed to the same
generic connector route shape but was not counted as a new CP8 row.

## Evidence That Moved The Architecture Forward

- Focused QNT route connectors were strong enough to force target evidence onto
  shared reducer-shaped semantics: subject family, fill kind, hole kind, owner,
  outcome, and no-fill projection steps.
- The dirty Rust target now routes accepted evidence through observed reducer
  entrypoints such as battle start, act discovery, subject resolution, and
  route-event observation rather than only adapter-local replay helpers.
- The source refresh at `d5a70b23ad05abd4188b1f0d37d9c6aba600cce5`
  successfully changed the target decision:
  - nine fixture scenario transition rows left the reducer-route denominator;
  - the generic spell base Armor Class route connector supplied enough shape
    to accept the three remaining Mage Armor admission/lifecycle rows.
- The CP8 review loop caught a false-positive risk: the base Armor Class
  projection row still used an old route shape after the connector refresh.
  The follow-up fixed it to target-choice discovery, target-choice resolution,
  active-effect no-fill, and Armor Class no-fill.
- Production reducer-spine code stayed shape-based for CP8. `MageArmor` naming
  is absent from `src/rules/battle_reducer_spine.rs`; selected names remain in
  witness/adapter-oriented files and evidence.

## What This Does Not Prove

- It does not prove a fresh cleanroom can produce the same implementation
  without the old dirty Rust scaffolding.
- It does not prove the Rust target is the desired final architecture. The Rust
  implementation is an evidence target, not the final product.
- The FC-06 audit in `FC06_SOURCE_FEEDBACK.md` originally recorded two
  fresh-run source feedback items. Encounter composition is resolved by source
  commit `0387d29f9282037637b4256c3c7f292bab7ef85c` and fresh target commit
  `893198ce66a35c8aad007ad8ac7a61c4631c64d9`. Pact Slot route surfaces are
  resolved by source commit `b57772b459f1b75592fd45b9196fd60965b534d3`.
  Dirty target replay evidence consumes those Pact Slot connector facts in
  `tasks/target-replay-evidence/pact-slot-handoff-init-projection-route.json`;
  fresh target commit `a30e6729711ddc3f595cf008931ba5cd6265c58a` now also
  accepts them through public handoff entrypoints and verifies the current
  package through `python3 tools/verify_current_fresh_target.py`.
- It does not prove global harness cleanliness. `node
  scripts/check-cleanroom-harness.cjs` still fails on stale validator hashes in
  `cleanroom-input/MANIFEST.md` for:
  - `scripts/check-cleanroom-harness.cjs`
  - `scripts/cleanroom-branch-coverage-check.cjs`
- The `8d4ab3b2c567f3b292de6aebbc5b6fb49c3e00da` dirty package refresh
  packages source-feedback Task 3 residual selected-spell route tasks, SQNT-03A
  hit-point-regain-prevention connector evidence, SQNT-03B next-Attack-Roll mode
  connector evidence, the Task 8 six-driver active reducer diagnostic seed, and
  FCSF-04 public object stale route-history evidence, FCSF-05
  reaction/interrupt payload taxonomy evidence, and FCSF-06
  character/sheet/handoff rejection and resource payload evidence. The earlier
  FCSF-04 dirty replay adds diagnostic Rust coverage for all
  7 Starry Wisp object obligations including public stale route history, the
  FCSF-05 dirty replay adds diagnostic Rust coverage for all 5 reaction/interrupt
  taxonomy connector rows, and the FCSF-06 dirty replay adds diagnostic Rust
  coverage for 30 rows while recording 14 target blockers. The SQNT-03A dirty
  replay adds diagnostic Rust coverage for the copied Hit Point regain
  prevention route connector through public reducer route events. The SQNT-03B
  dirty replay adds diagnostic Rust coverage for the copied next-Attack-Roll
  mode route connector through public reducer route events. These dirty replays
  do not add fresh target acceptance, and
  condition-immunity is still not accepted as scalar-buff route replay
  evidence.
- It does not remove every old historical evidence artifact. The fresh current
  gate classifies FC-03/FC-04/FC-05 as historical snapshots after the input
  refresh; broader dirty-rehearsal cleanup remains optional unless the dirty
  harness must pass without exception.

## Fresh Cleanroom Inputs That Worked

A future fresh cleanroom agent should be able to start from:

- `cleanroom-input/qnt/**`
- `cleanroom-input/raw/**`
- `cleanroom-input/domain/**`
- `cleanroom-input/guidance/**`
- `cleanroom-input/branch-coverage/reducer-route-inventory.json`
- `cleanroom-input/branch-coverage/source-branch-inventory.json`
- this campaign's `PLAN.md`, `PROMPTS.md`, and review lessons

The strongest reusable pattern is:

1. Use focused `.mbt.qnt` witnesses for selected scenario facts.
2. Use `.route.mbt.qnt` connectors for the generic reducer route shape.
3. Admit production behavior only through typed subject/fill variants and
   battle/component/character-owned state.
4. Compare observed target routes emitted by reducer entrypoints to independent
   expected route records.
5. Keep selected identity in adapters, tests, fixtures, catalogs, or evidence,
   never in production reducer dispatch.

## Review Lessons To Carry Forward

Future reviewers should reject:

- adapter-local route synthesis claimed as observed reducer evidence;
- expected route witnesses built from the same helper path as observed routes;
- target evidence accepted without copied QNT/guidance connector facts;
- durable production fields with no state-owner or derivability record;
- route evidence whose artifact hashes or manifest source commit are stale;
- selected authored identity dispatch in production reducers, even for SRD or
  synthetic fixtures.

The campaign repeatedly improved after reviewers forced workers to replace
projection-only or adapter-synthesized evidence with production-observed route
events. That review loop is part of the architecture evidence, not incidental
bookkeeping.

## Recommended Next Work

The dirty Rust target campaign is closed for the refreshed in-scope denominator,
the fresh SDK composition tracer is accepted, the FC-06 source feedback is
resolved in the copied source package, and the current fresh verifier is green.
Next useful work is one of:

- use `SOURCE_QNT_NEXT_TASKS.md` and `FEXP_BLOCKER_SOURCE_FEEDBACK.md` to
  promote the remaining Task 3 selected-spell blockers after SQNT-03B and the
  FEXP-07 selected/grouped residual blockers into source-side QNT/guidance
  updates before launching more target implementation;
- use `FRESH_SDK_COMPOSITION_ACCEPTANCE.md` and fresh target commit
  `05280a8e2d6e9705411c114c80ae2a4e4290de2c` as the accepted evidence for the
  integrated SDK tracer-bullet plus post-FC06 Pact Slot handoff replay plus
  FEXP-00 expansion baseline lock, FEXP-01 diagnostic battle route pack, and
  FEXP-02 generic spell-attack/save-gated substrate lane plus FEXP-03
  chained/object spell-attack substrate lane plus FEXP-04 active-effect
  lifecycle substrate lane, exact roll-choice replay, Ability Check/Search
  public reducer replay, Concentration cleanup replay, and scalar profile
  replay plus FEXP-05
  reaction/interrupt substrate lane plus
  FEXP-06 character/sheet/handoff substrate lane plus FEXP-07
  feature/species/metamagic substrate lane. The current gate verifies the
  refreshed `ee4894fa` package plus current FEXP-04/FEXP-06 evidence, and the
  dirty campaign now also records FCSF-04, FCSF-05, SQNT-03A, and SQNT-03B
  current-package dirty replays as accepted and FCSF-06 current-package dirty replay as
  accepted-with-target-blockers. Older stale snapshots remain explicitly
  classified;
- clean the global stale validator-hash / historical evidence debt so the dirty
  harness can pass without exception;
- promote the strongest route-connector lessons back into source-side QNT and
  cleanroom guidance before a fresh run;
- audit whether any accepted rows still depend too much on dirty target
  scaffolding, then convert those lessons into source-side guidance rather than
  more dirty target code.

Do not launch more dirty target lanes for Task 3 residual selected-spell
branches beyond SQNT-03B, or FEXP-07 selected/grouped residual branches, until
the generic route facts in `SOURCE_QNT_NEXT_TASKS.md` exist in the source
package. SQNT-03A and SQNT-03B now have both packaged source input and accepted
dirty target replay; they still need fresh target replay before either counts as
fresh target acceptance.
