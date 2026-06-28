# Fresh Cleanroom Dry Run Plan

Campaign: `level-1-2-runtime-reducer-route`

Status: accepted through the SDK tracer-bullet composition checkpoint, Pact
Slot handoff replay, FEXP-07 feature/species/metamagic substrate expansion, and current
fresh-target verifier.

## Purpose

Run a small fresh cleanroom campaign that consumes the cleanroom package and
campaign guidance without copying the dirty Rust cleanroom implementation. The
dry run should prove whether focused `.qnt` slices and reducer-route connectors
are sufficient to generate or constrain an independent reducer-shaped runtime,
and should record source-input blockers instead of inferring TypeScript or dirty
target behavior.

This is not another dirty-target coverage sweep. The dirty cleanroom campaign
closed the refreshed in-scope denominator at `659 / 659`; this plan tests
whether those lessons transfer to a fresh target.

## Real Goal

Prove and refine a QNT-driven architecture where focused `.qnt` slices and
curated cleanroom guidance define enough reducer-shaped semantics for an
independent character/battle runtime to be built without TypeScript
implementation knowledge or production authored-identity dispatch.

The fresh dry run is successful if it produces a small, honest target plus
blocker notes that improve the source-side QNT/guidance package. It is not
required to cover all `659` in-scope dirty-campaign obligations.

Current accepted target:
`/workspace/typescript/.codex-worktrees/dnd-fresh-cleanroom-dry-run-fc00`

Current accepted SDK tracer commit:
`893198ce66a35c8aad007ad8ac7a61c4631c64d9`

Current accepted fresh target head:
`05280a8e2d6e9705411c114c80ae2a4e4290de2c`

Current package gate:
`python3 tools/verify_current_fresh_target.py`

Current expansion state:
`FEXP-00-BASELINE-LOCK` is accepted at
`574c99d28ef2fe8779c500dad34879efa7aa4177` with no new runtime route coverage.
`FEXP-01-DIAGNOSTIC-BATTLE-ROUTE-PACK` is accepted at
`a78d1d6c4c5ec6eaad5ea99c9b6bfde296020639` with focused diagnostic battle
route coverage.
`FEXP-02-SPELL-ATTACK-SAVE-GATED-UNBLOCK` is accepted-with-blockers at
`773fe97d95e568c6acc99cc2bbe3ce6d57fc50bc` with generic spell-attack and
save-gated substrate route evidence; residual selected spell effects remain
explicit blockers outside the generic substrate.
`FEXP-03-CHAINED-AND-OBJECT-SPELL-ATTACKS` is accepted-with-blockers at
`9b2f81bfabb6f1afd7daede0455be054bb92d78c` with generic independent
spell-attack sequence, chained duplicate-damage leap, and object-target
spell-attack route evidence; isolated object stale replay remains explicitly
blocked.
`FEXP-04-ACTIVE-EFFECT-LIFECYCLE-AND-ROLL-MODIFIERS` is accepted
at `05280a8e2d6e9705411c114c80ae2a4e4290de2c` with generic roll-modifier,
scalar-buff, targeted-speed, turn-boundary lifecycle, Concentration cleanup,
and scalar profile projection/domain route evidence; exact
roll-choice payloads were resolved source-side by
source commit `0c2ba34c5a45f18b73dfe590e0e86419ba377375`, focused-replayed
in the dirty target by commit `801b05df55a1393d6acd5c3fa7b2624ed91f9494`, and
accepted in the fresh target by commit
`b43797af240c1486e5ad92c3698bf2cd2958a91e`; Ability Check/Search public
target-choice, Ability Check opening, rejection, failure, and success surfaces
were then accepted in the fresh target by commit
`cd4465556d18729121f56f5834ac00f8b0b3d15c`; failed-save, voluntary-end, and
replacement Concentration cleanup were accepted in the fresh target by commit
`bd6c6ba2407ac00a8295bbe1cd66a70e5ae8364c` after source feedback task
`FCSF-01` landed at source commit
`c62aa73be7f80e4d3a5b460aa2bef42cea0c0f7d`; scalar-buff profile projection
was accepted in the fresh target by commit
`05280a8e2d6e9705411c114c80ae2a4e4290de2c` after source feedback task
`FCSF-02` landed at source commit
`ee4894fa71e9307b9251639f0b54577ff764c63f`.
`FEXP-05-REACTION-INTERRUPT-AND-BOUNDARY` is accepted-with-blockers at
`eb05e8495eac993b69e17f68544edace6e56caee` with generic reaction casting-time
interrupt/resume, after-damage reaction, nested interrupt resume, active-effect
resume, and recorded procedure replay route evidence; selected reaction spell
projections and interrupt trigger taxonomy were explicitly blocked in that
historical snapshot. FCSF-05 now resolves those source-input blockers in the
current package; fresh target replay remains pending.
`FEXP-06-CHARACTER-CREATION-SHEET-HANDOFF-PACK` is accepted-with-blockers at
`0d5200e43fd7e9f094a93585f00eaf6bd2266c75` with character creation
finalization, sheet hit-point route projection, short-rest Pact-slot
completion, and happy-path battle settlement route evidence; partial,
rejection, resource, and conflict branches remain explicitly blocked.
`FEXP-07-FEATURE-SPECIES-METAMAGIC-SUBSTRATES` is accepted-with-blockers at
`a77a41dc752326eab69d8110de78928b9dcb9691` with feature bonus-action
Dash/temporary-hit-point, species creature-stat projection, metamagic resource
governor, and active-feature Spell Save DC benefit route evidence;
selected/grouped identity witnesses and residual species/metamagic/feature
branches remain explicitly blocked; the earlier exact metamagic driver note was
corrected as a campaign manifest naming error. The dirty current-package replay
later accepted SQNT-07B/C/D evidence for 15 species/passive-adjacent rows,
30 selected metamagic rows, and 3 active-feature spell-benefit rows, but this is
diagnostic dirty evidence only until a fresh target replays the same copied
route facts.

## Hard Boundaries

Fresh-run workers must not read:

- `/workspace/typescript/dnd`
- any sibling source repository
- dirty cleanroom implementation files from `src/**` as implementation source

Allowed inputs:

- `cleanroom-input/qnt/**`
- `cleanroom-input/raw/**`
- `cleanroom-input/domain/**`
- `cleanroom-input/guidance/**`
- `cleanroom-input/branch-coverage/reducer-route-inventory.json`
- `cleanroom-input/branch-coverage/source-branch-inventory.json`
- `tasks/campaigns/level-1-2-runtime-reducer-route/PLAN.md`
- `tasks/campaigns/level-1-2-runtime-reducer-route/PROMPTS.md`
- `tasks/campaigns/level-1-2-runtime-reducer-route/FRESH_CLEANROOM_READINESS.md`
- this file

Dirty cleanroom files under `tasks/**` may be read as campaign evidence and
review lessons, but not as production implementation templates. If a worker
needs a dirty `src/**` fact to implement behavior, that is a blocker and must be
converted into source-side QNT/guidance before the fresh result is meaningful.

Production runtime semantics must not dispatch on authored names, ids, slugs,
source headings, provenance sections, page references, official catalog labels,
or synthetic fixture labels. Selected identity is allowed only at catalog,
adapter, test, fixture, or evidence boundaries.

## Required Fresh Target Shape

The fresh target may be a minimal Rust crate or another explicitly chosen
runtime target, but it must expose a shared reducer surface for battle routes:

- `start_battle`
- `discover_battle_acts`
- `resolve_battle_subject`
- turn advancement

Accepted route evidence must come from observed reducer entrypoints, not from
adapter-local route synthesis. Tests may translate observed reducer events into
comparison records, but expected route records must be independent QNT-derived
facts.

Production state may grow only with durable owners justified by a
state-owner/derivability record tied to QNT, RAW/domain guidance, or an explicit
blocker. Do not add driver-local duplicate state when `BattleState`, character
state, component state, or a shared substrate can own the fact.

## Output Artifacts

The fresh run must write these artifacts inside the fresh target or its campaign
directory:

- `FRESH_RUN_STATE.json`: current phase, target commit, accepted rows, blocked
  rows, and verification status.
- `FRESH_RUN_REPORT.md`: human-readable summary of what was generated,
  accepted, blocked, and learned.
- `BLOCKERS.json`: source-input blockers with the exact missing QNT, RAW,
  domain, or guidance fact.
- `EVIDENCE/*.json`: route evidence records that include QNT input paths,
  expected route facts, observed reducer events, comparison result, and source
  hashes.
- `STATE_OWNERS.md` or equivalent: durable state owners and derivability
  records.

The SDK tracer-bullet acceptance is summarized in
`FRESH_SDK_COMPOSITION_ACCEPTANCE.md`. The current accepted target state after
the `ee4894fa71e9307b9251639f0b54577ff764c63f` source package refresh is
verified by `python3 tools/verify_current_fresh_target.py`; older unrefreshed
FC/FEXP evidence is explicitly historical snapshot evidence.

## Tracer Sequence

### FC-00: Package Admission And Inventory Parser

Input:

- `cleanroom-input/branch-coverage/reducer-route-inventory.json`
- `cleanroom-input/branch-coverage/source-branch-inventory.json`
- `cleanroom-input/MANIFEST.md`

Task:

- create the fresh target skeleton;
- parse the branch inventories;
- summarize denominator counts and route connector paths;
- record package hashes and known stale dirty-harness validator-hash debt as a
  dirty-target limitation, not a fresh-run blocker.

Pass criteria:

- fresh target has no copied dirty `src/**` implementation;
- inventory parser can list driver files, in-scope obligations, out-of-scope
  obligations, route task ids, and route connector paths;
- `FRESH_RUN_STATE.json` records source package inputs.

Blocker criteria:

- inventory schema cannot be read without dirty target code;
- package lacks enough metadata to identify QNT driver and connector paths.

### FC-01: Reducer Spine Substrate

Input:

- `cleanroom-input/guidance/**`
- reducer-route connector patterns in `cleanroom-input/qnt/**`
- `FRESH_CLEANROOM_READINESS.md` review lessons

Task:

- implement the minimal `BattleState` and reducer entrypoints;
- define typed subject, fill, hole, outcome, and route-event vocabulary only for
  later tracer tasks;
- provide a route observer emitted from reducer entrypoints.

Pass criteria:

- route observations cannot be produced by adapter-local helpers;
- every durable field has a state-owner/derivability record;
- no selected authored identity appears in production reducer dispatch.

Blocker criteria:

- reducer surface, subject/fill lifecycle, or state ownership must be inferred
  from dirty implementation instead of QNT/guidance.

### FC-02: Minimal Battle Action Route

Preferred drivers:

- `battle-runtime-weapon-attack-skeleton.mbt.qnt`
- another small non-spell battle action driver if inventory scope changed

Task:

- route a tiny attack/action sequence through the shared reducer surface;
- compare observed reducer route events to independent QNT-derived expected
  records.

Pass criteria:

- no local replay island is counted as reducer evidence;
- attack/action durable facts are battle-owned or explicitly component-owned;
- evidence records cite the focused QNT driver and any connector facts used.

Blocker criteria:

- target choice, attack roll, damage, or action-economy facts require unstated
  source behavior.

### FC-03: Spell Attack Or Save-Gated Connector

Preferred drivers:

- `battle-runtime-attack-spell-shape-selected-identity.mbt.qnt`
- `battle-runtime-level1-damage-spell-selected-identity.mbt.qnt`

Preferred connectors:

- `battle-runtime-spell-attack-ordering.route.mbt.qnt`
- `battle-runtime-save-gated-spell-ordering.route.mbt.qnt`

Task:

- admit selected spell scenarios through generic spell attack/save-gated route
  shapes;
- quarantine selected identity to adapter, fixture, and evidence boundaries.

Pass criteria:

- production code dispatches on typed spell shape/effect facts, not spell names;
- expected route records are copied from QNT connector facts or directly derived
  from connector state vocabulary;
- observed events come from reducer entrypoints.

Blocker criteria:

- connector QNT does not state enough target/fill/outcome shape to avoid
  TypeScript or dirty-target inference.

### FC-04: Generic Base Armor Class Regression

Preferred driver:

- `battle-runtime-mage-armor-selected-identity.mbt.qnt`

Required connector:

- `battle-runtime-spell-base-armor-class-effect.route.mbt.qnt`

Task:

- prove the fresh target can accept the Mage Armor selected-identity scenario
  only through a generic base Armor Class effect substrate.

Pass criteria:

- production implementation contains a generic base Armor Class effect owner;
- no production branch mentions the selected spell identity;
- target-choice discovery, target-choice resolution, active-effect no-fill, and
  Armor Class no-fill projection events are observed through reducer entrypoints.

Blocker criteria:

- the fresh target can only implement the behavior by naming the selected spell,
  or by copying dirty target state shape.

### FC-05: Character, Sheet, Or Handoff Seed

Preferred route family:

- one character creation, character sheet, or character-to-battle handoff driver
  from the route inventory with a small obligation count.

Task:

- prove the same QNT-driven pattern can establish non-battle state ownership and
  handoff facts without duplicating battle-owned runtime state.

Pass criteria:

- durable character facts have character-side owners;
- battle handoff consumes projections from those owners rather than creating a
  parallel local copy;
- evidence records cite QNT and domain guidance paths.

Blocker criteria:

- the package does not state enough character-owner or handoff semantics to
  continue without source implementation knowledge.

### FC-06: Audit And Source Feedback

Task:

- review every accepted fresh-run row for reducer-route honesty;
- classify every blocker as QNT missing fact, RAW/domain missing fact, guidance
  missing fact, or target implementation gap;
- propose source-side QNT/guidance changes before running a larger fresh
  cleanroom campaign.

Pass criteria:

- `FRESH_RUN_REPORT.md` states exactly what was proven and what was not;
- all accepted rows have route evidence from reducer entrypoints;
- all blockers are actionable against source-side QNT/guidance or target code.

## Acceptance Gates

A fresh-run row may be accepted only when all of these are true:

- observed route events are emitted by shared reducer entrypoints;
- expected route facts are independent from the observed implementation path;
- selected identity is absent from production runtime dispatch;
- every durable production field has an owner/derivability record;
- QNT driver and connector paths are cited;
- RAW/domain guidance is cited when a rule fact is modeled;
- blocker records exist for every unstated fact that would otherwise be inferred
  from TypeScript or the dirty target.

Reject the row if evidence is adapter-synthesized, self-confirming, copied from
dirty implementation code, or based on authored identity dispatch.

## Review Loop

Each fresh-run phase needs a read-only review before it can unlock the next
phase. Review findings must lead with file/line references and focus on:

- cleanroom boundary violations;
- reducer-route evidence honesty;
- state-owner/derivability records;
- authored identity quarantine;
- QNT/RAW/domain traceability;
- stale hashes or self-confirming evidence;
- local replay islands counted as reducer evidence.

Run fixer passes until no reasonable findings remain. Rejected findings must be
recorded with concrete reasons in `FRESH_RUN_REPORT.md`.

## Verification

Minimum verification after each implementation phase:

- language-native formatter check;
- language-native unit/integration tests;
- route evidence JSON schema check;
- branch inventory parser check;
- static search proving no source repo paths or dirty `src/**` implementation
  paths were used as production inputs;
- static search for selected authored identity dispatch in production files;
- reviewer-loop convergence.

The dirty cleanroom commands remain useful as reference checks only. Passing the
dirty target harness does not prove fresh-run acceptance.

Before the broader reducer-spine goal is considered done, run an SDK-style
programmatic tracer-bullet scenario against the freshest usable target surface:

- prefer the fresh cleanroom target if it can express the scenario;
- otherwise run it against the dirty Rust cleanroom and record that limitation;
- create a character programmatically;
- project or hand off that character into a simple battle setup;
- drive at least one complete simple battle turn through public reducer/runtime
  APIs rather than test-only replay internals;
- assert that the public programmatic surface is usable end to end, or record a
  concrete blocker explaining which missing surface prevents it.

FC-06 source feedback is recorded in `FC06_SOURCE_FEEDBACK.md`. The first fresh
SDK tracer-bullet exposed a missing encounter-composition surface; source commit
`0387d29f9282037637b4256c3c7f292bab7ef85c` added that route contract, and fresh
target commit `893198ce66a35c8aad007ad8ac7a61c4631c64d9` accepted the full
integrated sheet-to-composed-encounter-to-simple-turn scenario.

The same feedback report also recorded a missing Pact Slot handoff route
surface. Source commit `b57772b459f1b75592fd45b9196fd60965b534d3` supplied the
generic route facts for pure Pact Slot projection and mixed ordinary/Pact Slot
handoff rejection. Fresh target commit
`a30e6729711ddc3f595cf008931ba5cd6265c58a` accepts that replay through public
handoff entrypoints and adds the current verifier gate that validates strict
FC-07 source hashes, route comparisons, SDK tracer evidence, and FC-03/FC-04/FC-05
historical snapshot classification.

## Parallelism

Use a simple checkpoint sequence until the reducer surface is stable:

1. `FC-00` must finish first.
2. `FC-01` must finish second.
3. `FC-02`, `FC-03`, and `FC-04` may run in parallel only if they write
   disjoint adapter/test/evidence files and treat the reducer core as frozen.
4. `FC-05` may run in parallel with one battle tracer after `FC-01`, but should
   not define battle-owned fields.
5. `FC-06` is sequential source-feedback audit work.
6. `FC-07` and `FC-08` are post-feedback acceptance gates for refreshed source
   package facts and the current verifier.

Recommended initial execution is one worker lane at a time through `FC-04`.
Parallelize only after the first fresh reducer API has survived review.

## Launch Checklist

Before launch:

1. choose a fresh target directory outside the dirty cleanroom worktree;
2. copy or mount only the allowed input package and campaign files;
3. record the dirty campaign commit used as input context;
4. create `FRESH_RUN_STATE.json` with phase `FC-00`;
5. launch a worker with the cleanroom boundary above;
6. launch a reviewer after every phase;
7. update this campaign's `STATE.json` only with pointers to the fresh target
   and final fresh-run report, not with dirty coverage claims.

## Completion Criteria

The dry run is complete when:

- `FC-00` through `FC-06` are either accepted or blocked with concrete blocker
  records;
- post-feedback source gaps are either accepted in the fresh target or remain
  concrete blockers;
- the current fresh target verifier passes for the accepted source package;
- the fresh target has at least one battle route accepted through reducer
  entrypoints;
- the generic base Armor Class tracer is accepted or blocked with a source-input
  reason;
- an SDK-style programmatic tracer-bullet scenario for character creation plus a
  simple battle is run against the freshest usable target surface, with any
  inability to run it recorded as a concrete blocker;
- `FRESH_RUN_REPORT.md` clearly separates proven fresh-cleanroom behavior from
  dirty rehearsal evidence;
- source-side QNT/guidance follow-up tasks are listed for every cleanroom-input
  insufficiency.
