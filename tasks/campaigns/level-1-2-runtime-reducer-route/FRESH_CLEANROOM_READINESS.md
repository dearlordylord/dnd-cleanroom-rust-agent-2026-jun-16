# Fresh Cleanroom Readiness

Campaign: `level-1-2-runtime-reducer-route`

Integration worktree: `/workspace/typescript/.codex-worktrees/dnd-cleanroom-rrconv-19`

Integration branch: `ralph/rrconv-19-cleanroom`

Integration head when this readiness report was refreshed: `ebd3699fb03c99d0ec674361ebf69835e64bfd0c`

Verified reducer-route replay merge head: `4b2c415259ad5f3b10d281a536a5aa8499f926b7`

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

- `STATE.json.status`: `sqnt07-current-package-replay-merged-verified`
- refreshed in-scope obligations: `659`
- accepted obligations: `659`
- accepted driver coverage: `97 / 97`
- blockers: `0`
- out-of-scope obligations: `45`
- active lane worktrees: none
- active fresh dry-run worktrees: none
- current fresh package baseline:
  `c196b33c634169cfc991c3de101c23fde8f75bae`
- strongest pre-21504ef fresh runtime evidence:
  `05280a8e2d6e9705411c114c80ae2a4e4290de2c`
- latest dirty package source used by current replay evidence:
  `545d7848692fcb18adf14e5c009d9e7f4d0cb1d5`
- latest accepted fresh target head:
  `eaaefdbb1172b37b75f9a29ca3132f8e547cd2f1`
- latest current-package fresh runtime evidence:
  `FEXP-08-SQNT07A-SELECTED-SPATIAL-CURRENT-PACKAGE-REPLAY` accepts all 10
  SQNT-07A selected spatial witness rows through copied selected-spatial route
  connector facts and public battle reducer entrypoints, with 0 blocked rows.
  Its review loop fixed expected-route derivation to parse copied QNT route
  bodies and removed duplicate object ids from object-light route/no-fill
  state.
  `FEXP-09A-SQNT07D-ACTIVE-FEATURE-CURRENT-PACKAGE-REPLAY` accepts all 3
  SQNT-07D active-feature spell-benefit rows through copied active-feature route
  connector facts and public reducer entrypoints, with 0 blocked rows.
- latest source-QNT connector replay batch: SQNT-07B species/passive-adjacent,
  SQNT-07C metamagic, and SQNT-07D active-feature spell-benefit dirty replay is
  accepted at merge head `4b2c415259ad5f3b10d281a536a5aa8499f926b7`; fresh
  target replay remains pending.
- latest fresh baseline refresh: `FRESH-RR-BASELINE-21504EF` is accepted at
  `c196b33c634169cfc991c3de101c23fde8f75bae`; it refreshes the copied package
  to `21504ef764118f5fd13086aa6266f19280196664` and classifies older fresh
  runtime evidence as historical snapshot evidence without claiming new
  current-package runtime coverage.
- previous source-QNT connectors: SQNT-03A Hit Point regain prevention,
  SQNT-03B Next Attack Roll mode, and SQNT-03C Opportunity Attack Denial are
  packaged and dirty-replayed; fresh target replay remains pending.
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
  SQNT-03B accepted 6 next-Attack-Roll connector transitions and 2 selected
  driver obligations in
  `tasks/target-replay-evidence/FCSF-03B-next-attack-roll-mode-dirty-replay.json`;
  SQNT-03C accepted 4 Opportunity Attack denial connector transitions and 1
  selected-driver obligation in
  `tasks/target-replay-evidence/FCSF-03C-opportunity-attack-denial-dirty-replay.json`;
  SQNT-07B accepted 15 species/passive-adjacent rows with 2 inherited Adrenaline
  Rush rows out of scope for that lane; SQNT-07C accepted 30 selected metamagic
  rows; SQNT-07D accepted 3 active-feature spell-benefit rows.

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
- It does not prove global harness cleanliness. After the SQNT-07 bookkeeping
  pass, `node scripts/check-cleanroom-harness.cjs` progresses past guidance and
  validator provenance and fails on older historical replay/ledger debt still
  pinned to source package `564376fd95218a209bb9eae5c9ccb54ca3e04a52`, plus
  pre-existing heuristic findings outside the SQNT-07B/C/D replay batch.
- The `73fb3ce66c3fb107a1e53f6a9a5d516c78880391` dirty package refresh
  packages source-feedback Task 3 residual selected-spell route tasks, SQNT-03A
  hit-point-regain-prevention connector evidence, SQNT-03B next-Attack-Roll mode
  connector evidence, SQNT-03C Opportunity Attack denial connector evidence, the
  Task 8 six-driver active reducer diagnostic seed, and FCSF-04 public object
  stale route-history evidence, FCSF-05 reaction/interrupt payload taxonomy
  evidence, and FCSF-06 character/sheet/handoff rejection and resource payload
  evidence. The later SQNT-07B/C/D replay batch uses source package
  `21504ef764118f5fd13086aa6266f19280196664` for species/passive-adjacent,
  metamagic, and active-feature spell-benefit route evidence. The dirty replays
  add diagnostic Rust coverage for FCSF-04, FCSF-05, FCSF-06, SQNT-03A,
  SQNT-03B, SQNT-03C, SQNT-07B, SQNT-07C, and SQNT-07D rows through public
  reducer route events, but do not add fresh target acceptance.
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
- use fresh target commit `eaaefdbb1172b37b75f9a29ca3132f8e547cd2f1` as the
  current `545d7848692fcb18adf14e5c009d9e7f4d0cb1d5` package baseline for
  FEXP-08 selected-spatial and FEXP-09A active-feature runtime evidence. Use
  `05280a8e2d6e9705411c114c80ae2a4e4290de2c` and
  `FRESH_SDK_COMPOSITION_ACCEPTANCE.md` as historical fresh runtime evidence
  for the integrated SDK tracer-bullet, post-FC06 Pact Slot handoff replay, and
  FEXP-04/FEXP-06 runtime surfaces until current-package replay lanes renew
  those claims. The dirty campaign records FCSF-04, FCSF-05, SQNT-03A, SQNT-03B,
  SQNT-03C, SQNT-07B, SQNT-07C, SQNT-07D, and SQNT-07A selected-spatial
  current-package dirty replays as accepted and FCSF-06 current-package dirty
  replay as accepted-with-target blockers;
- clean the old historical evidence debt so the dirty
  harness can pass without exception;
- promote the strongest route-connector lessons back into source-side QNT and
  cleanroom guidance before a fresh run;
- audit whether any accepted rows still depend too much on dirty target
  scaffolding, then convert those lessons into source-side guidance rather than
  more dirty target code.

Do not launch more dirty target lanes for Task 3 residual selected-spell
branches beyond SQNT-03C, or FEXP-07 selected/grouped residual branches, until
the generic route facts in `SOURCE_QNT_NEXT_TASKS.md` exist in the source
package. SQNT-03A, SQNT-03B, and SQNT-03C now have both packaged source input
and accepted dirty target replay; they still need fresh target replay before any
counts as fresh target acceptance.
