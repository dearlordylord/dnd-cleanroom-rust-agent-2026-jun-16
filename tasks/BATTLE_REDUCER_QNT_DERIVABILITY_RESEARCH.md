# Battle Reducer QNT Derivability Research

## Question

Can the cleanroom implementation figure out the battle reducer from QNT alone?

This note does not audit every battle driver. It measures the current Rust
experiment against the copied QNT corpus and the TypeScript source exemplar to
separate three claims:

1. QNT can drive focused replay adapters.
2. QNT can drive local rule/projection helpers.
3. QNT can drive construction of a reusable battle reducer with start,
   discovery, subject resolution, turn resources, holes/fills, and durable
   combatant state.

The current experiment strongly supports the first two claims. Follow-up tracked
reducer-spine experiments now support the third claim for one narrow
weapon-attack path and one existing MBT adapter, but not for the full battle
reducer.

## Inputs

- Cleanroom Rust repo HEAD at first measurement:
  `b8da0923c1bea358f80f9024bfdaf73f348b00b1`
- Cleanroom manifest source commit: `829aee6441d76a921c9d9c14a0d0221062975334`
- Cleanroom branch inventory SHA:
  `0a5eaa1f6f79fddbe441dc94500a0dac5644ba7fc392fc6baa3d44da1f2e3248`
- TypeScript exemplar HEAD read for architecture comparison:
  `455c720040752aad8a8060acf47d64ea92666420`

The TypeScript source is comparison material only. It is not cleanroom input.

## Measurement Frame

The useful classification is not "active or inactive". For the main goal, the
useful question is what kind of implementation knowledge a slice provides.

| Class | Meaning | Why it matters |
| --- | --- | --- |
| QNT literal replay | The driver names fixed actions and fixed projection fields. A target can replay the trace without a reusable reducer. | Good branch evidence, weak reducer evidence. |
| QNT local rule helper | The driver or imported rule-core QNT gives enough state and transition logic to implement a reusable local rule helper. | Strong evidence for isolated mechanics. |
| QNT plus RAW fill-in | QNT gives examples and constraints, RAW supplies the missing general rule. | Acceptable cleanroom derivation if the missing step is traceable to RAW. |
| Reducer-spine required | The behavior depends on shared battle state, act discovery, subject dispatch, action resources, interrupt frames, or combatant ownership. | This is the core battle reducer question. |
| Unstated or TS-implicit | Required behavior cannot be traced to copied QNT, RAW, ubiquitous language, or assumptions. | This is a blocker; the source QNT/guidance needs improvement before a clean run. |

This classification is needed for the main goal because "replay evidence
passes" and "battle reducer can be reconstructed" are different claims. A full
driver-by-driver classification is not needed yet; the next useful step is a
smaller reducer-spine derivation experiment.

## Measurements

### Cleanroom Corpus

- `cleanroom-input/qnt/battle-runtime`: 257 QNT files.
- MBT drivers in that directory: 127.
- Non-MBT support/spec modules in that directory: 130.
- Active level-1/2 battle plus rule-core queue: 73 drivers.
- Active replay obligations in that subset: 502.
  - Battle and `creature-attack`: 378 obligations across 64 drivers.
  - Rule-core: 124 obligations across 9 drivers.
- Out-of-scope battle/rule-core obligations in the same inventory area: 33.

This is a broad corpus. The corpus contains reducer vocabulary, not only
examples: `battle-runtime-model.qnt` defines a `BattleState`, and support files
such as `battle-runtime-actor-combatants.qnt`,
`battle-runtime-combat-holes.qnt`, and `battle-runtime-turn-advancement.qnt`
define combatant addressing, open-hole projection, and turn advancement.

### Rust Experiment

- Rust rule modules in `src/rules`: 52.
- Rust battle/rule-core replay adapters in `src/qnt_adapters`: 61.
- Target replay evidence files: 78.
- Target replay runs recorded across those files: 480.
- Current-manifest evidence files: 5.
- Stale previous-manifest evidence files: 73.

The 5 current-manifest evidence files are the handoff lane. Their current
obligation count is 26, not 24.

Search result at first measurement: the Rust target had no `BattleState`,
`start_battle`, `discover_battle_acts`, or `resolve_battle_subject` equivalent.
Its battle work was represented as per-slice states such as
`WeaponAttackSkeletonState`, `MovementState`, and `MagicMissileState`.

After the follow-up experiment, the Rust target has an explicitly experimental
`src/rules/battle_reducer_spine.rs` module with:

- `BattleState`
- `start_battle`
- `discover_battle_acts`
- `resolve_battle_subject`

That module is deliberately narrow: it derives a Fighter/Goblin initial state
from `battle-runtime-model.qnt`, discovers one Fighter weapon attack from
`battle-runtime-combat-holes.qnt`, orders target/attack-roll/damage fills using
`battle-runtime-weapon-attack-ordering.qnt`, applies attack-roll hit logic from
`shared-algebras/proofs/rule-core/attack-damage-composition.qnt`, and applies
damage through the existing Rust HP helper derived from rule-core hit point QNT.

The `battle_runtime_weapon_attack_ordering` adapter now uses that reducer spine
for observed replay and keeps the old focused helper as the expected witness.
That is the first existing MBT adapter test routed through
`start_battle`/`discover_battle_acts`/`resolve_battle_subject`.

`tasks/target-replay-evidence/T063-battle-runtime-weapon-attack-ordering.json`
now also references the current cleanroom manifest and source branch inventory,
and its `generatedBy.name` records the spine replay function. The per-file
evidence validates cleanly against the selected T063 inventory slice and covers
all seven T063 branch obligations. This is still not repo-wide cleanroom
acceptance: the rolling task artifacts and ledger do not select T063 as the
active accepted work-loop task, and the evidence schema records the harness
function by name rather than independently proving the Rust call graph.

### TypeScript Exemplar

The TypeScript runtime exposes the reducer boundary directly:

- `BattleState` in `packages/battle-runtime/src/battle-reducer.ts`.
- `startBattle` in
  `packages/battle-runtime/src/battle-reducer/api-lifecycle.ts`.
- `discoverBattleActs` in
  `packages/battle-runtime/src/battle-reducer/battle-discovery.ts`.
- `resolveBattleSubject` in
  `packages/battle-runtime/src/battle-reducer/dispatcher.ts`.

The TypeScript reducer sub-tree contains 175 `.ts` files under
`packages/battle-runtime/src/battle-reducer`.

This is the architecture the cleanroom target ultimately needs to recover or
replace with an equivalent reducer contract.

## Findings

### 1. Focused Replay Is Working

The Rust adapters mirror QNT branch actions and projection payloads. Example:
`battle_runtime_weapon_attack_skeleton.rs` maps the QNT actions
`doDiscoverAttack`, `doFillTarget`, `doFillAttackRollHit`, and related branches
to Rust functions, then emits the same projected state fields:

- `qSkeletonHp`
- `qSkeletonDead`
- `qActionAvailable`
- `qMultiattackDispatchesAvailable`
- `qSneakAttackUsedThisTurn`
- protocol result, invalid reason, and holes

That is useful, but it mostly proves replayability of the driver-shaped witness.

### 2. Local Rule Helpers Are Often QNT-Derivable

Some Rust rule modules are more than literal replay. `rule_core_movement.rs`
captures local movement, dash, disengage, grapple, escape, and opportunity
attack projection concepts that are visible in `rule-core-movement.mbt.qnt` and
its imported rule-core QNT modules.

This is strong evidence that focused rule-core and projection slices can be
implemented from copied QNT plus RAW.

### 3. The Reducer Spine Is Partially Measured

The first measurement found no generic battle state or public reducer
entrypoints. The follow-up code experiment adds the smallest useful version and
measures one vertical path:

- start battle from the QNT `initialState`;
- discover current actor weapon attack;
- fill target choice;
- fill attack roll;
- fill damage roll;
- mutate durable target HP;
- spend action on resolution;
- reject an attack roll before target choice.

This proves that copied QNT plus existing QNT-derived helpers are sufficient for
one reducer-shaped path. The adapter-routing experiment then proves that at
least one existing MBT adapter can compare observed replay from that spine
against the prior focused expected witness. It does not prove the full
reducer-wide contract:

- start/init boundary;
- actor and combatant ownership model;
- act discovery over the current state;
- subject identity and dispatcher routing;
- hole/fill protocol over an in-progress subject;
- action, bonus action, reaction, movement, and spell-cast resource ownership;
- interrupt stack and continuation behavior;
- turn advancement and start/end-turn effects.

Copied QNT contains many of these concepts. The experiment shows they can be
assembled without TypeScript for a small slice. The remaining question is
whether each broader reducer subsystem can be made to replay existing MBT
drivers through the shared spine rather than through slice-specific adapter
functions.

### 4. No Current Evidence Shows TS-Only Behavior Was Needed

For the implemented slices, I did not find a hard example where the Rust code
clearly had to rely on TypeScript-only behavior. The local states usually mirror
QNT variable names, action names, constants, and protocol shapes.

The risk is different: when moving from local slices to a reducer, the missing
contract may be architectural rather than a single rule. That is where
TS-implicit knowledge is most likely to leak in unless the experiment forbids
source consultation and records every missing fact as a source-QNT blocker.

## Measurement Answer

Current Rust evidence can be recreated from QNT alone for focused replay and
many local helper slices with high confidence.

The new reducer-spine experiment can also be recreated from copied QNT alone
for one Fighter weapon-attack path. Measured generic reducer coverage is no
longer zero, but it is still only a seed. One existing battle MBT adapter test
now replays through the generic reducer entrypoints instead of a slice-specific
observed helper, and the corresponding per-file T063 target replay evidence is
current and locally validator-clean. The full cleanroom claim remains unproven
until that route is recorded as a complete work-loop task and then expanded
across the level-1/2 battle queue.

## Completed Follow-Up Experiment

The recommended current-snapshot reducer-spine evidence experiment has now been
run for T063 at the per-file evidence level.

Result:

- The existing Rust test path can replay observed T063 actions through
  `battle_reducer_spine.rs`.
- The T063 replay evidence has current manifest and inventory hashes.
- A selected-driver validation of the T063 evidence reports zero issues and
  seven covered obligations.
- The existing evidence schema cannot enforce "observed replay used reducer
  spine" beyond the generated harness name and target source path. That fact is
  therefore code-reviewed evidence, not a machine-checked harness property.
- `node scripts/check-cleanroom-harness.cjs` still fails at repo scope. The
  remaining failures are structural and not specific to T063: missing
  `tasks/RUN_LEDGER.json`, undeclared historical evidence files including T063,
  stale selected T074 evidence, missing T074 validation-report rows, and the
  existing adapter-quarantine/authored-identity scan findings.
- `scripts/check-target-replay-evidence-file.cjs` now captures the per-driver
  evidence check as a repeatable diagnostic command. It is intentionally not a
  work-loop acceptance gate.

Diagnostic command:

```bash
node scripts/check-target-replay-evidence-file.cjs \
  --driver cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-ordering.mbt.qnt \
  --evidence tasks/target-replay-evidence/T063-battle-runtime-weapon-attack-ordering.json
```

## Second Reducer-Spine Experiment

`battle-runtime-weapon-attack-skeleton.mbt.qnt` is now also routed through the
experimental reducer spine on the observed side.

This is materially stronger than T063 because it checks durable battle state,
not only ordering protocol shape:

- target HP changes;
- death projection from zero HP;
- action availability is spent on miss or completed damage;
- sneak attack use is marked on the attacking actor;
- Skeleton multiattack creates and spends a dispatch;
- stale resolved subjects are rejected with no open holes.

Result:

- `battle_runtime_weapon_attack_skeleton` observed replay uses
  `start_skeleton_battle`, `start_skeleton_actor_turn`,
  `discover_battle_acts`, and `resolve_battle_subject`.
- The old focused helper remains the expected witness.
- `tasks/target-replay-evidence/T064-battle-runtime-weapon-attack-skeleton.json`
  has current manifest and inventory hashes and records the spine replay
  function.
- `scripts/check-target-replay-evidence-file.cjs` validates all 14 T064 branch
  obligations.

This expands measured reducer-spine coverage from 7 T063 ordering obligations
to 21 obligations across two battle drivers. It still does not solve global
work-loop acceptance because the dirty evidence/adapter denominator remains
global.

## Work-Loop Promotion Findings

T063/T064 cannot be promoted to repo-wide harness acceptance by adding only
driver-local ledger/history records in the current dirty repo.

Measured denominator:

- Active replayable obligations: 631 across 96 drivers.
- Battle/rule-core replayable obligations: 502 across 73 drivers.
- T063 plus T064 reducer-spine obligations: 21.
- Target replay evidence files under `tasks/target-replay-evidence`: 78.
- Current-snapshot evidence files: 7.
- Stale previous-snapshot evidence files: 71.
- Rust adapter files under `src/qnt_adapters`: 78.
- `tasks/history` is absent, and `tasks/RUN_LEDGER.json` is absent.

Harness implication:

- Without `tasks/RUN_LEDGER.json`, `check-cleanroom-harness.cjs` validates every
  evidence file under `tasks/target-replay-evidence` against the current rolling
  task's declared evidence. A T063/T064 rolling task would therefore still fail
  on the other evidence files.
- With `tasks/RUN_LEDGER.json`, the harness requires every evidence file under
  `tasks/target-replay-evidence` to be accounted for by ledger entries. A
  T063/T064-only ledger would still fail on the other evidence files.
- The production source scan treats undeclared adapter files as production
  source. A T063/T064-only engine-depth manifest would therefore still scan the
  other adapter modules and report witness-protocol/authored-identity findings.

This is not a QNT insufficiency. It is a dirty-repo acceptance-denominator
problem.

Options from here:

1. **Source-side reducer-spine witness first.** Add a source QNT/MBT witness
   that explicitly names the reducer contract (`start_battle`,
   `discover_battle_acts`, `resolve_battle_subject`) so future cleanroom runs
   do not rely on generated-by strings as call-graph evidence.
2. **Dirty-repo cleanup first.** Move stale historical evidence/adapters out of
   the harness denominator or create proper ledger/history entries for them.
   This makes the cleanroom harness useful again but is broader than the reducer
   question.
3. **Continue per-file reducer-spine diagnostics.** Use
   `check-target-replay-evidence-file.cjs` while expanding the spine to another
   driver, then promote only after the dirty denominator is fixed.

The best next reducer-specific experiment is Option 1 or Option 3, not a
T063/T064-only work-loop promotion inside the current dirty repo.

## Recommended Next Experiment

Run a complete work-loop reducer-spine task for T063 or add a source-side
reducer-spine witness before broadening to the next battle driver. Do not start
with a full driver audit.

Method:

1. Use only `cleanroom-input/**` as implementation input.
2. Promote `battle-runtime-weapon-attack-ordering.mbt.qnt` into a complete
   work-loop task whose rolling artifacts and ledger declare the T063 evidence.
3. If the current harness must express "observed via reducer spine, expected
   via focused QNT witness", add a small source-side reducer-spine witness
   before attempting a broad driver such as
   `battle-runtime-weapon-attack-skeleton.mbt.qnt`.
4. Record every fact that cannot be derived from copied QNT, RAW, domain docs,
   or assumptions as a blocker.
5. Compare the resulting public shape to TypeScript only after the experiment,
   as an evaluation step.

Expected outcomes:

- If current target replay evidence can run through the generic spine without
  blockers, QNT is likely sufficient for the first level-1/2 battle reducer
  architecture slice, and the next artifact should be a PRD/task list for
  expanding driver coverage.
- If blockers appear, update source QNT/guidance to make the missing reducer
  contract explicit before another cleanroom run.

Measurements for that experiment:

- Number of reducer-spine decisions derived directly from QNT.
- Number derived from RAW or ubiquitous language.
- Number blocked as unstated.
- Number where the post-hoc TypeScript comparison reveals a behavior mismatch.
- Whether the implementation can produce accepted current evidence for at least
  one existing battle MBT driver through the generic reducer entrypoints rather
  than a slice-specific observed helper.

## Current Research Decision

Do not produce a full PRD yet. The next artifact should follow current-snapshot
reducer-spine evidence, because that is the first step that measures whether
the cleanroom target can use a QNT-derived reducer entrypoint as the accepted
parity surface.
