# Fresh Cleanroom Expansion Plan

Campaign: `level-1-2-runtime-reducer-route`

Status: current-package FEXP-08 SQNT-07A selected-spatial replay accepted at fresh target head `64f7fad1902a7617264cbddf8d6eb236c786e8dd`

Baseline evidence:

- accepted fresh target: `/workspace/typescript/.codex-worktrees/dnd-fresh-cleanroom-dry-run-fc00`
- expansion baseline head: `a30e6729711ddc3f595cf008931ba5cd6265c58a`
- current accepted fresh target head: `64f7fad1902a7617264cbddf8d6eb236c786e8dd`
- current verifier: `python3 tools/verify_current_fresh_target.py`
- input source package: `545d7848692fcb18adf14e5c009d9e7f4d0cb1d5`

## Purpose

Run the next fresh-cleanroom campaign against the accepted fresh target, still
without TypeScript implementation knowledge and without dirty `src/**` input.

The aim is not to copy the dirty Rust target's `659 / 659` shape. The aim is to
prove that the current cleanroom package can continue expanding an independent
runtime through focused QNT route connectors, with blockers recorded when the
package does not state enough.

FEXP-00 through FEXP-07 are now complete. Remaining target blockers are grouped
as source-input feedback in `FEXP_BLOCKER_SOURCE_FEEDBACK.md`.

## Non-Negotiable Boundaries

Workers and reviewers for this campaign may read:

- the accepted fresh target worktree;
- `cleanroom-input/**` inside that target;
- fresh target `EVIDENCE/**`, `FRESH_RUN_STATE.json`, `FRESH_RUN_REPORT.md`,
  `BLOCKERS.json`, `STATE_OWNERS.md`, and `tools/**`;
- this campaign directory's planning and audit files.

Workers and reviewers must not read:

- `/workspace/typescript/dnd`;
- dirty cleanroom `src/**`;
- sibling source repositories.

Production runtime semantics must not dispatch on authored names, ids, slugs,
source headings, provenance sections, page references, official catalog labels,
or synthetic fixture labels. Selected identity remains limited to catalog,
adapter, fixture, test, and evidence boundaries.

## Acceptance Rule

An expansion row may be accepted only when all of these are true:

- observed route events are emitted by public reducer or handoff entrypoints;
- expected route records are independent literal QNT connector facts or direct
  projections from copied connector vocabulary, not built from observed code;
- every new durable field has a state-owner/derivability record tied to QNT,
  RAW/domain guidance, or an explicit blocker;
- production code is shape-based and identity-free;
- source input hashes are current for the accepted package, or the evidence is
  explicitly classified as a historical snapshot;
- the current verifier is updated or a new focused verifier is added so the
  accepted evidence remains mechanically checkable.

## Checkpoints

### FEXP-CP0: Baseline Lock

Purpose:

- confirm the accepted fresh target is clean at `a30e6729711ddc3f595cf008931ba5cd6265c58a`;
- run the current verifier;
- record a new expansion state file before any implementation lanes.

Exit evidence:

- `python3 tools/verify_current_fresh_target.py` passes;
- `FRESH_EXPANSION_STATE.json` records baseline commit and package facts.

### FEXP-CP1: Diagnostic Route Completion

Purpose:

Turn the reducer-spine substrate that FC-01 introduced into focused route
evidence for the remaining small diagnostic battle connectors.

Preferred lane:

- `FEXP-01-DIAGNOSTIC-BATTLE-ROUTE-PACK`

Status:

- accepted at fresh target head `a78d1d6c4c5ec6eaad5ea99c9b6bfde296020639`;
- verifier regenerates expected records from connector action bodies and
  observed records from the Rust public reducer-entrypoint example.

Why first:

- these drivers already shaped the reducer substrate;
- they exercise slots, HP lifecycle, zero-HP lifecycle, and Concentration
  teardown without selected identity;
- they provide a clean regression baseline before broader selected/grouped
  drivers.

### FEXP-CP2: Previously Blocked Spell Selected-Identity Groups

Purpose:

Revisit the two FC-03 blocked grouped drivers, but admit only the branches whose
generic connector substrates are now explicit in the current package.

Preferred lanes:

- `FEXP-02-SPELL-ATTACK-SAVE-GATED-UNBLOCK`
- `FEXP-03-CHAINED-AND-OBJECT-SPELL-ATTACKS`

Status:

- `FEXP-02-SPELL-ATTACK-SAVE-GATED-UNBLOCK` is accepted-with-blockers at fresh
  target head `773fe97d95e568c6acc99cc2bbe3ce6d57fc50bc`;
- generic spell-attack and save-gated substrates route through public reducer
  entrypoints;
- residual selected spell effects outside those generic substrates remain
  explicit blockers;
- `FEXP-03-CHAINED-AND-OBJECT-SPELL-ATTACKS` is accepted-with-blockers at fresh
  target head `9b2f81bfabb6f1afd7daede0455be054bb92d78c`;
- generic independent spell-attack sequence, chained duplicate-damage leap, and
  object-target spell-attack surfaces route through public reducer entrypoints;
- isolated object stale replay was blocked in that historical fresh snapshot,
  but FCSF-04 now resolves the source-input gap by packaging public object
  stale route-history evidence; fresh replay remains pending.

Why second:

- these are the freshest known blockers in `FRESH_RUN_STATE.json`;
- they directly test whether selected spell scenarios can be constrained by
  generic spell-attack, save-gated, chained-attack, and object-target shapes
  without identity dispatch.

### FEXP-CP3: Active Effect And Reaction Substrates

Purpose:

Add a broader battle lifecycle slice: active-effect projection/cleanup,
interrupt-stack/reaction routing, and roll-modifier effects.

Preferred lanes:

- `FEXP-04-ACTIVE-EFFECT-LIFECYCLE-AND-ROLL-MODIFIERS`
- `FEXP-05-REACTION-INTERRUPT-AND-BOUNDARY`

Status:

- `FEXP-04-ACTIVE-EFFECT-LIFECYCLE-AND-ROLL-MODIFIERS` is accepted
  at fresh target head `e8b0310e647ad471089fcd34737b8fd70211b373`;
- generic roll-modifier active-effect, scalar-buff active-effect,
  targeted-speed scalar buff, and turn-boundary cleanup surfaces route through
  public reducer entrypoints;
- post-refresh fresh target head
  `b43797af240c1486e5ad92c3698bf2cd2958a91e` accepts exact skill-choice,
  ability-choice, and two-target ability-choice route-fill payloads through
  public reducer route events after consuming source commit
  `0c2ba34c5a45f18b73dfe590e0e86419ba377375`;
- post-refresh fresh target head
  `cd4465556d18729121f56f5834ac00f8b0b3d15c` accepts public Ability
  Check/Search target-choice, Ability Check opening, rejection, failure, and
  success surfaces through public reducer route events;
- post-refresh fresh target head
  `bd6c6ba2407ac00a8295bbe1cd66a70e5ae8364c` accepts failed-save,
  voluntary-end, and replacement Concentration cleanup through public reducer
  route events after consuming source feedback task `FCSF-01` at source commit
  `c62aa73be7f80e4d3a5b460aa2bef42cea0c0f7d`;
- post-refresh fresh target head
  `05280a8e2d6e9705411c114c80ae2a4e4290de2c` accepts scalar-buff profile
  projection/domain sequencing through public reducer route observations after
  consuming source feedback task `FCSF-02` at source commit
  `ee4894fa71e9307b9251639f0b54577ff764c63f`;
- `FEXP-05-REACTION-INTERRUPT-AND-BOUNDARY` is accepted-with-blockers at fresh
  target head `eb05e8495eac993b69e17f68544edace6e56caee`;
- generic reaction casting-time interrupt/resume, after-damage reaction, nested
  interrupt resume, active-effect resume, and recorded procedure replay route
  surfaces route through public reducer entrypoints;
- selected reaction spell projections and interrupt trigger taxonomy were
  explicit source-input blockers in that historical fresh snapshot, but
  FCSF-05 now resolves them as source input by packaging generic
  reaction/interrupt payload taxonomy evidence; fresh replay remains pending.

Why third:

- many later selected-identity branches depend on these owners;
- this checkpoint should reduce pressure to smuggle identity into reducers for
  effect cleanup or reaction timing.

### FEXP-CP4: Character Runtime Expansion

Purpose:

Expand beyond the SDK tracer by adding more character creation, character
sheet, and handoff route connectors from the current package.

Preferred lanes:

- `FEXP-06-CHARACTER-CREATION-SHEET-HANDOFF-PACK`

Status:

- `FEXP-06-CHARACTER-CREATION-SHEET-HANDOFF-PACK` is accepted-with-blockers at
  fresh target head `0d5200e43fd7e9f094a93585f00eaf6bd2266c75`;
- character creation finalization, sheet hit-point route projection, short-rest
  Pact-slot completion, and happy-path battle settlement surfaces route through
  public character/sheet/handoff entrypoints;
- expected records are mechanically derived from copied QNT connector action
  bodies/helper vocabulary;
- partial/rejection/resource/conflict branches remain explicit source-input
  blockers.

Why fourth:

- the current SDK tracer proves one end-to-end surface;
- this checkpoint tests whether the same package can grow character-side state
  owners without duplicating battle runtime state.

### FEXP-CP5: Feature, Species, Metamagic, And Catalog-After-Substrate

Purpose:

Admit selected/catalog-heavy drivers only after generic substrates exist.

Preferred lanes:

- `FEXP-07-FEATURE-SPECIES-METAMAGIC-SUBSTRATES`
- later split lanes derived from `L15-RR07-FU01` and `L15-RR07-FU08` if the
  first feature/metamagic lane proves the substrate pattern.

Status:

- `FEXP-07-FEATURE-SPECIES-METAMAGIC-SUBSTRATES` is accepted-with-blockers at
  fresh target head `a77a41dc752326eab69d8110de78928b9dcb9691`;
- feature bonus-action Dash/temporary-hit-point, species creature-stat
  projection, metamagic resource governor, and active-feature Spell Save DC
  benefit route surfaces pass through public reducer entrypoints;
- expected records are mechanically derived from copied QNT connector action
  bodies/helper vocabulary;
- selected/grouped identity witnesses and residual species/metamagic/feature
  branches remain explicit source-input blockers; the earlier exact metamagic
  driver note was corrected as a campaign manifest naming error.
- `FEXP-08-SQNT07A-SELECTED-SPATIAL-CURRENT-PACKAGE-REPLAY` is accepted at
  fresh target head `64f7fad1902a7617264cbddf8d6eb236c786e8dd` against source
  package `545d7848692fcb18adf14e5c009d9e7f4d0cb1d5`; it accepts all 10
  selected spatial witness rows through the copied selected-spatial composition
  connector and public reducer entrypoints.
- The FEXP-08 review loop fixed two false-positive risks before acceptance:
  expected route records are parsed from copied QNT route bodies, and
  object-light target-choice rows use `SubjectProgress.selected_object` as the
  single runtime object owner instead of carrying duplicate object ids in route
  or no-fill state.

Why last:

- this is the highest risk area for authored identity dispatch;
- lanes should stay small and should stop on source-input blockers rather than
  infer identity behavior from the dirty campaign.

## Verification

Every implementation lane must run, at minimum:

- `cargo fmt --check`
- focused Rust tests for the lane
- `cargo test`
- `cargo clippy --all-targets -- -D warnings`
- the current verifier or a newly added focused verifier
- static forbidden-path search for `/workspace/typescript/dnd` and dirty
  cleanroom `src/**` references
- static production-source search for authored identity dispatch terms relevant
  to the selected lane
- `git diff --check <lane-base>...HEAD`

Every accepted checkpoint must update:

- `FRESH_EXPANSION_STATE.json`
- `FRESH_RUN_REPORT.md` or a checkpoint-specific expansion report
- `EVIDENCE/*.json`
- `STATE_OWNERS.md`
- verifier tooling

## Stop Conditions

Stop the expansion campaign and write source feedback instead of continuing
target implementation when:

- a route fact must be inferred from TypeScript, dirty `src/**`, or historical
  target behavior;
- a selected-identity branch has no generic connector substrate;
- a durable field would duplicate another state owner;
- expected and observed route records would share implementation helpers;
- production runtime behavior would need to branch on authored identity.

## Success Boundary

This expansion campaign is successful when it materially increases the accepted
fresh target surface beyond the FC-00 through FC-08 dry run while preserving the
cleanroom boundary and identity quarantine.

It still does not automatically complete the global goal. Completion requires a
separate audit proving that the broader independent runtime architecture is
covered by current fresh evidence, not just by this planned expansion.
