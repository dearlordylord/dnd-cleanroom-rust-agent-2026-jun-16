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

The current experiment strongly supports the first two claims. It does not yet
prove the third, because the Rust target does not currently contain a generic
battle reducer.

## Inputs

- Cleanroom Rust repo HEAD: `b8da0923c1bea358f80f9024bfdaf73f348b00b1`
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

Search result: the Rust target currently has no `BattleState`, `start_battle`,
`discover_battle_acts`, or `resolve_battle_subject` equivalent. Its battle work
is represented as per-slice states such as `WeaponAttackSkeletonState`,
`MovementState`, and `MagicMissileState`.

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

### 3. The Reducer Spine Is Not Yet Measured

The current Rust target does not have a generic battle state or public reducer
entrypoints. Therefore it has not measured whether QNT alone is enough to derive
the reducer-wide contract:

- start/init boundary;
- actor and combatant ownership model;
- act discovery over the current state;
- subject identity and dispatcher routing;
- hole/fill protocol over an in-progress subject;
- action, bonus action, reaction, movement, and spell-cast resource ownership;
- interrupt stack and continuation behavior;
- turn advancement and start/end-turn effects.

Copied QNT contains many of these concepts, but the Rust implementation has not
consumed them into one reducer. The current evidence cannot distinguish "QNT was
enough" from "an implementer used prior TypeScript knowledge to know what shape
to build," because that shape has not been built.

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

The existing Rust battle implementation cannot yet be counted as a QNT-only
battle reducer reconstruction. Measured generic reducer coverage is effectively
zero because no generic reducer exists in the Rust target. The QNT corpus may be
close enough to derive one, but that remains unmeasured.

## Recommended Next Experiment

Run a narrow reducer-spine derivation experiment, not a full driver audit.

Method:

1. Use only `cleanroom-input/**` as implementation input.
2. Build the smallest Rust reducer skeleton that has:
   - `BattleState` or equivalent durable state;
   - a start/init function;
   - current-actor act discovery;
   - subject resolution with holes/fills;
   - one weapon/stat-block attack path.
3. Target one vertical scenario first:
   Skeleton `Shortsword` or the existing weapon-attack-skeleton witness against
   a character or fighter combatant.
4. Record every fact that cannot be derived from copied QNT, RAW, domain docs,
   or assumptions as a blocker.
5. Compare the resulting public shape to TypeScript only after the experiment,
   as an evaluation step.

Expected outcomes:

- If the reducer spine can be built without blockers, QNT is likely sufficient
  for the level-1/2 battle reducer architecture, and the next artifact should be
  a PRD/task list for expanding driver coverage.
- If blockers appear, update source QNT/guidance to make the missing reducer
  contract explicit before another cleanroom run.

Measurements for that experiment:

- Number of reducer-spine decisions derived directly from QNT.
- Number derived from RAW or ubiquitous language.
- Number blocked as unstated.
- Number where the post-hoc TypeScript comparison reveals a behavior mismatch.
- Whether the implementation can replay at least one existing battle MBT driver
  through the generic reducer entrypoints rather than a slice-specific adapter.

## Current Research Decision

Do not produce a full PRD yet. The next artifact should follow the reducer-spine
experiment, because that is the first step that directly measures the real goal:
whether the cleanroom target can infer the battle reducer from QNT.
