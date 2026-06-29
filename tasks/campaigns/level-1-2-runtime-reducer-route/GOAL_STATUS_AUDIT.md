# Goal Status Audit

Campaign: `level-1-2-runtime-reducer-route`

Audit date: 2026-06-29

Audit baseline:

- dirty campaign branch: `ralph/rrconv-19-cleanroom`
- dirty campaign head after current fresh-expansion bookkeeping:
  `abff50ff7ffc977ce503f4aa7ac2c20f599f6f17`
- accepted fresh target: `/workspace/typescript/.codex-worktrees/dnd-fresh-cleanroom-dry-run-fc00`
- current accepted fresh package/runtime head:
  `b0036b7a0d81ffcc54c5c2d828122bd1352dc31c`
- current fresh gate: `python3 tools/verify_current_fresh_target.py`
- current fresh package source:
  `545d7848692fcb18adf14e5c009d9e7f4d0cb1d5`
- current fresh package inventory:
  `98` drivers, `663` in-scope obligations, `45` out-of-scope obligations
- current accepted fresh runtime claims:
  FEXP-08 and FEXP-09A through FEXP-09G

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
| Fresh target can be built without reading TypeScript implementation or dirty `src/**` implementation files. | Fresh target state/evidence records `copiedDirtySrcImplementation: false`; `python3 tools/verify_current_fresh_target.py` passes at `b0036b7a0d81ffcc54c5c2d828122bd1352dc31c`; current package is `545d7848692fcb18adf14e5c009d9e7f4d0cb1d5`. | Proven for the FC/FEXP dry-run scope plus current-package FEXP-08 and FEXP-09A through FEXP-09G acceptance. | Older fresh evidence remains historical until renewed against the current package. |
| Fresh target exposes the reducer-shaped surface. | `FRESH_RUN_REPORT.md`, `STATE_OWNERS.md`, and tests in the fresh target cover `start_battle`, `discover_battle_acts`, `resolve_battle_subject`, and turn advancement. | Proven for the dry-run target. | The surface is intentionally minimal and tracer-driven. |
| Fresh target supports an SDK-style programmatic character plus simple battle scenario. | `FRESH_SDK_COMPOSITION_ACCEPTANCE.md`, `EVIDENCE/sdk-tracer-bullet-programmatic-surface.json`, and `python3 tools/verify_current_fresh_target.py` cover sheet creation, projection, encounter composition, battle entry, act resolution, HP mutation, action spend, and turn advancement. | Proven for the accepted tracer scenario. | This is a tracer-bullet scenario, not full character/battle runtime parity. |
| FEXP-01 diagnostic battle route pack is accepted in the fresh target. | Fresh target head `a78d1d6c4c5ec6eaad5ea99c9b6bfde296020639` includes `EVIDENCE/fexp01-diagnostic-battle-route-pack.json`, `examples/fexp01_route_observations.rs`, and `tools/verify_fexp01.py`; `python3 tools/verify_current_fresh_target.py` passes. | Proven for Magic Missile slot-spell route, HP restoration ordering, Death Saving Throw, and Concentration teardown. | The verifier parser is intentionally narrow for the accepted connector shapes; broader drivers still need their own lanes. |
| FEXP-02 spell-attack/save-gated unblock lane is accepted with blockers in the fresh target. | Fresh target head `773fe97d95e568c6acc99cc2bbe3ce6d57fc50bc` includes `EVIDENCE/fexp02-spell-attack-save-gated-unblock.json`, `examples/fexp02_route_observations.rs`, and `tools/verify_fexp02.py`; `python3 tools/verify_current_fresh_target.py` passes. | Proven for generic spell-attack and save-gated route substrates plus eight selected-identity branch subsets accepted only as substrate evidence. | Residual selected spell effects remain blocked outside the generic substrate; this is intentionally not full selected-spell coverage. |
| FEXP-03 chained/object spell attack lane is accepted with blockers in the fresh target. | Fresh target head `9b2f81bfabb6f1afd7daede0455be054bb92d78c` includes `EVIDENCE/fexp03-chained-and-object-spell-attacks.json`, `examples/fexp03_route_observations.rs`, and `tools/verify_fexp03.py`; `python3 tools/verify_current_fresh_target.py` passes. FCSF-04 at source commit `e8cb231ce1183ed96b2865000562f0395f2d712f` now packages public object stale route-history evidence. Dirty replay evidence `tasks/target-replay-evidence/FCSF-04-object-stale-dirty-replay.json` accepts 6 connector rows and covers all 7 Starry Wisp object obligations. | Proven for the historical fresh snapshot's generic independent spell-attack sequence, chained duplicate-damage leap, and object-target spell-attack route surfaces through public reducer entrypoints; source-resolved for the object stale blocker; dirty-replayed for all 7 current-package Starry Wisp object obligations. | FCSF-04 is not yet fresh-target replay acceptance against the current package. |
| FEXP-04 active-effect lifecycle lane is accepted in the fresh target. | Fresh target head `05280a8e2d6e9705411c114c80ae2a4e4290de2c` includes refreshed `EVIDENCE/fexp04-active-effect-lifecycle-and-roll-modifiers.json`, `examples/fexp04_route_observations.rs`, `tools/verify_fexp04.py`, and `tools/verify_current_fresh_target.py`; `python3 tools/verify_current_fresh_target.py` passes against source package `ee4894fa71e9307b9251639f0b54577ff764c63f`. | Proven for generic roll-modifier active-effect, scalar-buff active-effect, targeted-speed scalar buff, turn-boundary cleanup, exact skill/ability/two-target ability route-fill payloads, public Ability Check/Search target-choice/ability-check/rejection/result surfaces, failed-save/voluntary-end/replacement Concentration cleanup, and scalar-buff profile projection/domain sequencing through public reducer entrypoints. | FEXP-04 currently has no blockers. Older FEXP-01/FEXP-02/FEXP-03/FEXP-05/FEXP-07 evidence is historical after the full input refresh. |
| FEXP-05 reaction/interrupt lane is accepted with blockers in the fresh target. | Fresh target head `eb05e8495eac993b69e17f68544edace6e56caee` includes `EVIDENCE/fexp05-reaction-interrupt-and-boundary.json`, `examples/fexp05_route_observations.rs`, and `tools/verify_fexp05.py`; `python3 tools/verify_current_fresh_target.py` passes. FCSF-05 at source commit `cf60f7a5b822ee9d9458e98577d47026242fd16e` now packages generic reaction/interrupt payload taxonomy evidence. Dirty replay evidence `tasks/target-replay-evidence/FCSF-05-reaction-interrupt-dirty-replay.json` accepts all 5 connector rows and covers 3 selected-context obligations. | Proven for the historical fresh snapshot's generic reaction casting-time interrupt/resume, after-damage reaction, nested interrupt resume, active-effect resume, and recorded procedure replay route surfaces through public reducer entrypoints; source-resolved for selected reaction spell projection and interrupt-trigger taxonomy blockers; dirty-replayed for all 5 current-package taxonomy connector rows. | FCSF-05 is not yet fresh-target replay acceptance against the current package. |
| FEXP-06 character/sheet/handoff lane is accepted with blockers in the fresh target. | Fresh target head `0d5200e43fd7e9f094a93585f00eaf6bd2266c75` includes `EVIDENCE/fexp06-character-creation-sheet-handoff-pack.json`, `examples/fexp06_route_observations.rs`, and `tools/verify_fexp06.py`; `python3 tools/verify_current_fresh_target.py` passes. FCSF-06 at source commit `d63838e22137c4b329dc877ca0d963876f3459bf` now packages typed character/sheet/handoff rejection, resource, settlement, and zero-HP lifecycle evidence. Dirty replay evidence `tasks/target-replay-evidence/FCSF-06-character-sheet-handoff-dirty-replay.json` records 30 accepted reducer-route witness rows and 14 target blockers. | Proven for character creation finalization, sheet hit-point route projection, short-rest Pact-slot completion, and happy-path battle settlement route surfaces through public character/sheet/handoff entrypoints; source-resolved for partial/rejection/resource/conflict blockers; dirty-replayed for 30 current-package rows. | FCSF-06 is not yet fresh-target replay acceptance against the current package. The dirty target still lacks independent public sheet qRoute state for 11 spell-resource rows, and the copied settlement connector lacks 3 driver rows. |
| FEXP-07 feature/species/metamagic lane is accepted with blockers in the fresh target, and current-package SQNT-07B/C/D subsets have been renewed. | Historical FEXP-07 evidence remains at `a77a41dc752326eab69d8110de78928b9dcb9691`; current fresh verifier at `b0036b7a0d81ffcc54c5c2d828122bd1352dc31c` accepts FEXP-09A through FEXP-09G, covering active-feature spell benefit, species/passive rows, Quickened governor, metamagic save modes, spell shape, rerolls, and component/duration rows. | Proven for the renewed current-package FEXP-09A through FEXP-09G subsets. | Broad selected/grouped identity replay remains excluded; remaining true blockers stay source-input work. |
| Post-FC06 and post-FEXP04 source feedback was resolved and replayed. | `FC06_SOURCE_FEEDBACK.md` records encounter composition and Pact Slot route-surface resolution; fresh target head `05280a8e2d6e9705411c114c80ae2a4e4290de2c` includes the accepted Pact Slot replay, SDK tracer, FEXP-04 exact roll-choice replay, Ability Check/Search replay, Concentration cleanup replay, scalar profile replay, FEXP-06 current-package verifier, and historical classification for stale older FC/FEXP snapshots. Dirty package refreshes now package SQNT-03A/B/C source connectors plus Task 3, Task 8, FCSF-04, FCSF-05, FCSF-06, and SQNT-07B/C/D inputs through latest source package `21504ef764118f5fd13086aa6266f19280196664`. Dirty replay merges include FCSF-04 `9c9d728640e28a003a5a051f7ddc9be56bbc980e`, FCSF-05 `5d07ad816c08ab3335c7e95c87f26c2e73e80d9c`, FCSF-06 `4f4d8535b5dc1b35083dfda63bd8e5ed35c21f6d`, SQNT-03A `73627315f70528e73f5eb4ef781606e876e87367`, SQNT-03B `4437eacc311a8ea069bc1d7c9dd9d2b334a8fb4e`, SQNT-03C `7521115f61077326b67e933dc9663f19d7e41570`, and SQNT-07B/C/D replay merge `4b2c415259ad5f3b10d281a536a5aa8499f926b7`. | Proven for the two recorded FC-06 feedback items, exact roll-choice payload feedback, Ability Check/Search public observability feedback, Concentration cleanup source feedback `FCSF-01`, scalar profile source feedback `FCSF-02`, SQNT-03A/SQNT-03B/SQNT-03C source connector evidence and dirty target replay, object stale source feedback `FCSF-04`, reaction/interrupt taxonomy source feedback `FCSF-05`, character/sheet/handoff source feedback `FCSF-06`, all 6 dirty FCSF-04 connector rows / 7 Starry Wisp object obligations, all 5 dirty FCSF-05 connector rows, 30 dirty FCSF-06 replay rows, 3 dirty SQNT-03A connector transitions / 1 attack-shape obligation, 6 dirty SQNT-03B connector transitions / 2 selected-driver obligations, 4 dirty SQNT-03C connector transitions / 1 selected-driver obligation, and SQNT-07B/C/D dirty current-package rows. | Older FC/FEXP verifier artifacts that were not refreshed against source `ee4894fa71e9307b9251639f0b54577ff764c63f` are historical snapshots, not current-package gates. These dirty refreshes and replays do not claim fresh target acceptance or condition-immunity scalar-buff route replay acceptance. SQNT-03A/B/C and SQNT-07B/C/D still need fresh target replay before any count as fresh runtime acceptance. |
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

- current `545d7848692fcb18adf14e5c009d9e7f4d0cb1d5` input package inventory;
- `98` drivers, `663` in-scope obligations, and `45` out-of-scope obligations;
- current-package runtime acceptance claims for FEXP-08 and FEXP-09A through
  FEXP-09G;
- historical snapshot classification for older runtime evidence that has not
  yet been renewed against the current package.

The next fresh proof-renewal batch should target the historical evidence still
worth renewing: SQNT-07A active-effect/condition, battle active-effects
SQNT-03A/B/C/D, spatial/damage, reaction/interrupt taxonomy, and
character/sheet/handoff.

Dirty campaign state:

```sh
cd /workspace/typescript/.codex-worktrees/dnd-cleanroom-rrconv-19
jq '.status, .currentCheckpoint, .coverageCurrent' \
  tasks/campaigns/level-1-2-runtime-reducer-route/STATE.json
```

Expected state:

- `dirty-rehearsal-complete-current-fresh-package-replay-batch-accepted`
- CP8 dirty rehearsal coverage remains `659 / 659` refreshed in-scope
  obligations accepted
- current fresh package checkpoint records target head
  `b0036b7a0d81ffcc54c5c2d828122bd1352dc31c`, source package
  `545d7848692fcb18adf14e5c009d9e7f4d0cb1d5`, and FEXP-08 plus FEXP-09A
  through FEXP-09G accepted runtime claims

## Remaining Work To Prove The Full Objective

1. Run the next fresh current-package proof-renewal batch from accepted target
   head `b0036b7a0d81ffcc54c5c2d828122bd1352dc31c`, using Ralph workers in
   parallel where write scopes are independent.
2. Prioritize renewal lanes that still have historical fresh evidence:
   SQNT-07A active-effect/condition, battle active-effects SQNT-03A/B/C/D,
   spatial/damage, reaction/interrupt taxonomy, and character/sheet/handoff.
3. Keep remaining true blockers as source-input work only: marked-immunity
   residual selected rows, Jump landing legality plus failed-landing Prone, and
   concentration-backed area hazards.
4. Preserve the review gates from this campaign: observed reducer events,
   independent expected route records, state-owner derivability records, and no
   production authored identity dispatch.
5. Treat old dirty-harness historical replay/ledger cleanup as optional unless
   the dirty harness must pass without exception; it is not the main proof path.

## Completion Decision

Do not mark the global objective complete from the current evidence alone.

The current evidence proves:

- the dirty rehearsal reached full refreshed in-scope route coverage;
- the fresh dry run independently accepted the reducer-spine tracer sequence,
  SDK composition tracer, post-feedback Pact Slot replay, and the post-refresh
  FEXP-04 exact roll-choice payload replay;
- FEXP-02 independently accepted generic spell-attack/save-gated substrate
  evidence with explicit blockers for remaining selected spell effects;
- FEXP-03 independently accepted chained/object spell-attack substrate evidence
  with an explicit historical blocker for isolated object stale replay; FCSF-04
  resolves that blocker as source input at
  `e8cb231ce1183ed96b2865000562f0395f2d712f`, but fresh-target replay remains
  pending;
- FEXP-04 independently accepted active-effect lifecycle substrate evidence.
  Exact roll-choice payload derivability is resolved source-side by commit
  `0c2ba34c5a45f18b73dfe590e0e86419ba377375`, focused-replayed in the dirty
  target by commit `801b05df55a1393d6acd5c3fa7b2624ed91f9494`, and accepted in
  the fresh target by commit `b43797af240c1486e5ad92c3698bf2cd2958a91e`;
  Ability Check/Search public observability is accepted in the fresh target by
  commit `cd4465556d18729121f56f5834ac00f8b0b3d15c`; Concentration cleanup
  source feedback `FCSF-01` landed at source commit
  `c62aa73be7f80e4d3a5b460aa2bef42cea0c0f7d` and is accepted in the fresh
  target by commit `bd6c6ba2407ac00a8295bbe1cd66a70e5ae8364c`; scalar profile
  source feedback `FCSF-02` landed at source commit
  `ee4894fa71e9307b9251639f0b54577ff764c63f` and is accepted in the fresh
  target by commit `05280a8e2d6e9705411c114c80ae2a4e4290de2c`;
- source-feedback Task 3, SQNT-03A/B/C, Task 8, FCSF-04/05/06, and later
  SQNT-07B/C/D are packaged in the dirty cleanroom input. The SQNT-03/FCSF
  package refresh at source commit `ebc37e935fdd45ac07198bbec6b3bcc23be2270e`
  carried residual selected-spell route tasks, hit-point-regain-prevention,
  next-Attack-Roll mode, Opportunity Attack denial, object stale route-history,
  reaction/interrupt taxonomy, and character/sheet/handoff payload evidence.
  The later current-package replay batch uses source commit
  `21504ef764118f5fd13086aa6266f19280196664` for SQNT-07B species/passive
  adjacent evidence, SQNT-07C metamagic evidence, and SQNT-07D active-feature
  spell-benefit evidence. These dirty replays add target-side rehearsal evidence
  but do not add fresh target acceptance;
- FEXP-05 independently accepted reaction/interrupt substrate evidence with
  explicit historical blockers for residual selected reaction spell and
  interrupt-trigger source-input gaps; FCSF-05 resolves those blockers as source
  input at `cf60f7a5b822ee9d9458e98577d47026242fd16e`, but fresh-target replay
  remains pending;
- FEXP-06 independently accepted character/sheet/handoff substrate evidence with
  explicit blockers for partial/rejection/resource/conflict source-input gaps;
  FCSF-06 resolves those blockers as source input at
  `d63838e22137c4b329dc877ca0d963876f3459bf`, but fresh-target replay remains
  pending;
- FEXP-07 independently accepted feature/species/metamagic substrate evidence
  with explicit blockers for selected/grouped identity witnesses and residual
  substrate branches; the earlier exact metamagic driver note was corrected as
  a campaign manifest naming error. The later SQNT-07B/C/D dirty replay batch
  adds current-package diagnostic evidence for species/passive-adjacent,
  metamagic, and active-feature spell-benefit rows, but not fresh target
  acceptance;
- the current verifier makes that fresh evidence mechanically checkable.

The current evidence does not yet prove:

- a complete independent character/battle runtime can be generated or
  constrained from focused `.qnt` slices alone;
- the architecture holds across a broader fresh-cleanroom implementation
  campaign without dirty historical scaffolding.
