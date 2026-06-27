# Fresh Cleanroom Readiness

Campaign: `level-1-2-runtime-reducer-route`

Integration worktree: `/workspace/typescript/.codex-worktrees/dnd-cleanroom-rrconv-19`

Integration branch: `ralph/rrconv-19-cleanroom`

Integration head when this readiness report was written: `22d9257`

Verified reducer-route head: `5b1e976b6af7fadefa4ea5a065ae81de53b94d09`

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

- `STATE.json.status`: `post-source-refresh-complete`
- refreshed in-scope obligations: `659`
- accepted obligations: `659`
- accepted driver coverage: `97 / 97`
- blockers: `0`
- out-of-scope obligations: `45`
- active lane worktrees: none

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
- The FC-06 audit in `FC06_SOURCE_FEEDBACK.md` records two fresh-run source
  feedback items: Pact Slot branches in the character-battle init projection
  witness lack generic route connector surfaces, and the SDK tracer-bullet only
  proves sheet creation, handoff, and a simple battle turn separately because a
  single integrated handoff-to-simple-turn scenario lacks an encounter
  composition surface.
- It does not prove global harness cleanliness. `node
  scripts/check-cleanroom-harness.cjs` still fails on stale validator hashes in
  `cleanroom-input/MANIFEST.md` for:
  - `scripts/check-cleanroom-harness.cjs`
  - `scripts/cleanroom-branch-coverage-check.cjs`
- It does not remove old historical evidence debt or ambiguous legacy evidence
  references. Those are dirty-rehearsal cleanup tasks, not proof blockers for
  CP8-specific route acceptance.

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

The dirty Rust target campaign is closed for the refreshed in-scope denominator.
Next useful work is one of:

- launch the fresh cleanroom dry run described in
  `FRESH_CLEANROOM_DRY_RUN_PLAN.md`;
- address the FC-06 source feedback in `FC06_SOURCE_FEEDBACK.md` before
  requiring a future cleanroom run to prove a full integrated SDK tracer-bullet;
- clean the global stale validator-hash / historical evidence debt so the dirty
  harness can pass without exception;
- promote the strongest route-connector lessons back into source-side QNT and
  cleanroom guidance before a fresh run;
- audit whether any accepted rows still depend too much on dirty target
  scaffolding, then convert those lessons into source-side guidance rather than
  more dirty target code.
