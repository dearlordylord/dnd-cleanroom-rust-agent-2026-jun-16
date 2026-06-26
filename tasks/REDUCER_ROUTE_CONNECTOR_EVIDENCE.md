# Reducer Route Connector Evidence

Dirty rehearsal target for source route connector package:

- Source commit SHA: `0e024f089687526c6bc4b9e6f9c7e640414f6486`
- Source branch inventory SHA: `47b0589f442c0aaff2a814c19384fcaed7a6dbe3e7a78b5d0df8b011f56e7eae`
- Reducer-route inventory SHA: `f19710f4cbc4d09208d38568c04375f0df4e2a738bbadc76c19f0f7161ffd733`
- Cleanroom input copy date: `2026-06-22T19:55:27.102Z`

## QNT Route Connectors

| Driver | Connector SHA256 |
| --- | --- |
| `battle-runtime-magic-missile.route.mbt.qnt` | `f91f7c392b838d06ea22e610499613ab634e036ea7a1a80dfe4f0b72f7cc36af` |
| `battle-runtime-save-gated-spell-ordering.route.mbt.qnt` | `53158acbee365d467cab341ed75609aab7fc2d3092f8d5904a717298e21b25e3` |
| `battle-runtime-hit-point-restoration-ordering.route.mbt.qnt` | `6cbfc11389bbf392a9ab9d7de1e2d221c5a5b89a09915f3649edc1db532cf90a` |
| `battle-runtime-death-saving-throw.route.mbt.qnt` | `e6aa6b9f5b07c8836ef80d3f3d21ef9a28be505ff69e6ce3e7c2fbc465b374ce` |
| `battle-runtime-concentration-break-teardown.route.mbt.qnt` | `a5c1fde8643dca5851c4777746e8fd3553af21cb27ac02e0933e763236892553` |

## Dirty Target Route Tests

The following adapter tests now compare focused projections and reducer-route
events:

- `magic_missile_adapter_replays_all_branches`
- `save_gated_spell_ordering_adapter_replays_all_branches`
- `hit_point_restoration_ordering_adapter_replays_all_branches`
- `death_saving_throw_adapter_replays_all_branches`
- `concentration_break_teardown_adapter_replays_all_branches`

Observed route events are produced by the shared reducer calls:

- `start_fighter_skeleton_battle`
- `start_death_saving_throw_battle`
- `discover_battle_acts`
- `resolve_battle_subject`

The expected events mirror copied `qRoute` connector shape through the
quarantined adapter vocabulary in
`src/qnt_adapters/battle_runtime_reducer_route.rs`.

## Verification

- `cargo test adapter_replays_all_branches` passed: 70 filtered adapter tests.
- `cargo fmt --check` passed.
- `cargo test` passed: 173 tests.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` failed on stale dirty-ledger
  artifacts rather than route behavior.

This is dirty-rehearsal evidence only. The historical replay evidence JSON and
run ledger remain stale and are not repaired by this note.
