# Next Ralph Campaign

Campaign: `level-1-2-runtime-reducer-route`

Prepared from dirty cleanroom branch `ralph/rrconv-19-cleanroom` and refreshed
after fresh baseline lane `FRESH-RR-BASELINE-21504EF` merged at
`c196b33c634169cfc991c3de101c23fde8f75bae`.

## Goal

Use Ralph lanes to keep proving that copied focused `.qnt` route facts can drive
independent reducer-shaped runtime behavior through public entrypoints. Dirty
target evidence remains rehearsal evidence. Fresh target replay is the stronger
proof path.

## Current Facts

- Current dirty package source: `21504ef764118f5fd13086aa6266f19280196664`.
- Dirty SQNT-07 replay merge: `4b2c415259ad5f3b10d281a536a5aa8499f926b7`.
- Current fresh package baseline:
  `c196b33c634169cfc991c3de101c23fde8f75bae`.
- Strongest pre-21504ef fresh runtime evidence:
  `05280a8e2d6e9705411c114c80ae2a4e4290de2c`.
- Dirty current-package replay already accepts:
  - FCSF-04 object stale public route history;
  - FCSF-05 reaction/interrupt payload taxonomy;
  - FCSF-06 character/sheet/handoff payloads with target blockers;
  - SQNT-03A through SQNT-03G;
  - SQNT-07B, SQNT-07C, and SQNT-07D.
- Fresh current-package runtime replay remains pending for those accepted dirty
  rows.
- SQNT-07A selected/grouped residual work remains source-input work, split by
  generic substrate family.

## Current Checkpoint Override

This document was originally prepared before the SQNT-07A Wave 2 source lanes
and fresh replay refresh completed. Current authoritative checkpoint:

- Source/cleanroom-input package is now
  `545d7848692fcb18adf14e5c009d9e7f4d0cb1d5`.
- Fresh target `master` is now
  `444295121a3cbe0631dc36ef2280c27d2ef5631c`.
- Source master contains the weapon-hosted selected-row public route witness at
  merge `06f8042513df94bc96e02ec9d213a0d52d942bac`.
- `FRESH-RR-BASELINE-E9F75E22`,
  `FRESH-RR-SQNT07A-ACTIVE-EFFECT-CONDITION`, and
  `FRESH-RR-SQNT07A-SPATIAL-MOVEMENT` are complete.
- Accepted fresh SQNT-07A coverage is generic-substrate coverage only:
  condition lifecycle plus marked damage/immunity active effects, and
  spatial-effect plus movement/presentation routes; the selected condition-save
  refresh now accepts 5 additional selected rows through copied generic route
  facts and carries forward the existing 4 selected rows. The level-1
  marked/immunity selected replay also accepts exactly 4 selected rows through
  copied generic marked-damage/immunity facts. The level-1 scalar-buff selected
  replay accepts exactly 2 selected rows through copied generic scalar-buff
  facts. The level-1 after-hit/timed rider selected replay accepts exactly 3
  selected rows through copied generic after-hit route/lifecycle/owner facts;
  exact damage details remain blocked. The level-1 weapon-hosted replay accepts
  generic route-surface groups through copied route connectors and public
  reducer observations; selected-row exactness remained blocked until the
  source witness above landed. Jump landing/prone generic route surfaces and
  concentration-backed area hazard generic route surfaces are now accepted in
  the fresh target.
- Remaining SQNT-07A blockers are exact and should drive future work:
  fresh target consumption of the weapon-hosted selected-row public route
  witness, Hex ability-check roll-mode, selected concentration hazard exactness,
  and exact after-hit and weapon-hosted damage/arithmetic details.
- Do not relaunch broad selected/grouped identity replay. Launch only lanes
  backed by copied generic route facts or by a source-QNT lane that creates
  those facts first.

## Execution Rules

- Each lane runs in its own worktree.
- Every worker logs the task base ref, `HEAD`, and
  `git merge-base --is-ancestor <Base SHA> HEAD` before editing.
- Workers consume copied `.qnt`, RAW/domain files, guidance, inventory, and
  campaign files. They keep TypeScript implementation and dirty `src/**`
  implementation history out of their evidence source.
- Expected records come from copied `.qnt` connector bodies or helper
  vocabulary.
- Observed records come from public reducer, character, sheet, or handoff
  entrypoints.
- Durable runtime fields need state-owner derivability records.
- Production route decisions are shape/fact based.

## Wave 1: Fresh Replay Refresh

One preparation lane first, then four implementation lanes in parallel.

### FRESH-RR-BASELINE-21504EF

Status: accepted and merged into fresh target `master` at
`c196b33c634169cfc991c3de101c23fde8f75bae`.

Purpose: refresh the fresh cleanroom package baseline to source
`21504ef764118f5fd13086aa6266f19280196664` and classify existing fresh evidence
as current or historical.

Inputs:

- `cleanroom-input/MANIFEST.md`
- `cleanroom-input/branch-coverage/reducer-route-inventory.json`
- `tasks/campaigns/level-1-2-runtime-reducer-route/FRESH_EXPANSION_LANES.json`
- accepted fresh target `05280a8e2d6e9705411c114c80ae2a4e4290de2c`

Outputs:

- refreshed fresh target package metadata;
- verifier update that recognizes current package `21504ef`;
- no new runtime acceptance claims.

Checks:

- `python3 tools/verify_current_fresh_target.py`
- `cargo fmt --check`
- `cargo test`
- `cargo clippy --all-targets -- -D warnings`

### FRESH-RR-BATTLE-ACTIVE-EFFECTS

Depends on: `FRESH-RR-BASELINE-21504EF`.

Base commit: `c196b33c634169cfc991c3de101c23fde8f75bae`.

Purpose: replay the battle active-effect residual routes that already have dirty
acceptance.

Inputs:

- SQNT-03A Hit Point regain prevention;
- SQNT-03B next-Attack-Roll mode;
- SQNT-03C Opportunity Attack denial;
- SQNT-03D condition and poison riders;
- FCSF-05 reaction/interrupt taxonomy.

Expected output:

- fresh evidence generated from copied route connectors;
- observed route records from public battle reducer entrypoints;
- explicit fresh blockers only where copied QNT facts remain insufficient.

Parallel notes:

- Runs independently from character/sheet/handoff and feature/species/metamagic.
- Shares battle active-effect code with spatial/damage lanes, so merge after a
  reducer-surface review checkpoint.

### FRESH-RR-SPATIAL-DAMAGE

Depends on: `FRESH-RR-BASELINE-21504EF`.

Base commit: `c196b33c634169cfc991c3de101c23fde8f75bae`.

Purpose: replay object/light, mixed-target, exact-damage, and public stale route
history facts against the fresh target.

Inputs:

- SQNT-03E object and light riders;
- SQNT-03F mixed target outcomes;
- SQNT-03G exact damage projection facts;
- FCSF-04 object stale public route history.

Expected output:

- fresh route evidence for copied object/light, mixed-target, and exact-damage
  connector rows;
- clear classification for residual spatial blockers named in
  `SOURCE_QNT_NEXT_TASKS.md`.

Parallel notes:

- Runs independently from character/sheet/handoff.
- May touch battle damage/object route surfaces; merge after
  `FRESH-RR-BATTLE-ACTIVE-EFFECTS` review if both edit the same reducer module.

### FRESH-RR-CHARACTER-SHEET-HANDOFF

Depends on: `FRESH-RR-BASELINE-21504EF`.

Base commit: `c196b33c634169cfc991c3de101c23fde8f75bae`.

Purpose: replay FCSF-06 character creation, sheet, resource, handoff,
settlement, and zero-HP lifecycle facts against the fresh target.

Inputs:

- character creation route connectors;
- character sheet route connectors;
- character-battle handoff route connectors;
- `tasks/target-replay-evidence/FCSF-06-character-sheet-handoff-dirty-replay.json`
  as diagnostic contrast only.

Expected output:

- fresh public-entrypoint evidence for accepted FCSF-06 rows;
- blocker list for rows requiring more copied source input.

Parallel notes:

- Highest parallel value because it writes outside most battle reducer surfaces.

### FRESH-RR-FEATURE-SPECIES-METAMAGIC

Depends on: `FRESH-RR-BASELINE-21504EF`.

Base commit: `c196b33c634169cfc991c3de101c23fde8f75bae`.

Purpose: replay SQNT-07B/C/D source facts against the fresh target.

Inputs:

- `battle-runtime-species-passive-trait-substrates.route.mbt.qnt`;
- `battle-runtime-sorcerer-metamagic.route.mbt.qnt`;
- `battle-runtime-feature-selected-identity.route.mbt.qnt`.

Expected output:

- fresh public-entrypoint evidence for 15 species/passive-adjacent rows, 30
  selected metamagic rows, and 3 active-feature spell-benefit rows where copied
  QNT facts are sufficient;
- explicit carry-forward of the two inherited Adrenaline Rush rows and SQNT-07A
  selected/grouped residual blockers.

Parallel notes:

- Runs independently from character/sheet/handoff.
- May share battle spell procedure/resource code with active-effect lanes; merge
  after route-owner review if code surfaces overlap.

## Wave 2: Source-QNT Residual Substrates

These lanes belong in source-side worktrees, then a package refresh, then target
replay. They are parallelizable because each owns a distinct substrate family.

### SQNT-07A-CONDITION-LIFECYCLE

Purpose: add generic route facts for Paralyzed/Incapacitated until-spell-end
repeat-save lifecycle and laughter-style Incapacitated plus Prone lifecycle.

Blocks:

- Hold Person selected branches;
- Hideous Laughter selected branches.

Output:

- focused route connector or route-facts file;
- reducer-route inventory records;
- TS MBT parity test for any new source witness.

### SQNT-07A-MARKED-DAMAGE-AND-IMMUNITY

Purpose: add generic route facts for marked-target damage rider/transfer and
condition-immunity plus turn-start Temporary Hit Point active-effect substrate.

Blocks:

- residual selected damage-rider branches;
- condition-immunity scalar-buff route replay.

Output:

- focused route connector or component witness;
- state-owner derivability record for damage transfer or turn-start projection.

### SQNT-07A-SPATIAL-EFFECTS

Purpose: add generic route facts for movable multi-emitter light,
outline/invisible-revealing/attack-advantage effects, and area
hazard/obscurement lifecycle.

Blocks:

- Dancing Lights movable Dim Light;
- Faerie Fire outline/invisible/advantage;
- Fog Cloud and Grease area effects.

Output:

- focused route connectors that separate battle-owned effect facts from
  table-owned geometry, presentation, sight, and object facts.

### SQNT-07A-MOVEMENT-PRESENTATION

Purpose: add generic route facts for movement replacement, forced movement, and
object-push/presentation effects.

Blocks:

- residual movement replacement branches;
- forced-movement plus object-push presentation branches.

Output:

- focused movement/presentation route connector;
- explicit boundary between movement-resource facts, table presentation facts,
  and object-boundary facts.

## Merge Checkpoints

1. Baseline checkpoint after `FRESH-RR-BASELINE-21504EF`.
2. Battle reducer checkpoint after `FRESH-RR-BATTLE-ACTIVE-EFFECTS`,
   `FRESH-RR-SPATIAL-DAMAGE`, and `FRESH-RR-FEATURE-SPECIES-METAMAGIC` are
   reviewed for overlapping reducer owners.
3. Character/sheet checkpoint can merge independently after verifier review.
4. Source-QNT substrate checkpoint merges only after TS MBT parity, closure
   checks, and cleanroom package sync evidence.

## Reviewer Loop

Each lane needs:

- RAW/domain traceability for newly modeled rule facts;
- ubiquitous-language/domain naming review;
- architecture and connascence review;
- static check for production authored-identity dispatch;
- evidence review proving expected records derive from copied QNT and observed
  records derive from public entrypoints.
