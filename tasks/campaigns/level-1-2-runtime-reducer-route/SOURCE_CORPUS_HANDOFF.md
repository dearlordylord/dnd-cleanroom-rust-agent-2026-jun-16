# Source Corpus Handoff

Campaign: `level-1-2-runtime-reducer-route`

Integration worktree: `/workspace/typescript/.codex-worktrees/dnd-cleanroom-rrconv-19`

Integration branch: `ralph/rrconv-19-cleanroom`

Last dirty-cleanroom integration head: `ebd3699fb03c99d0ec674361ebf69835e64bfd0c`

## Status

CP7 exhausted the target-side reducer-route work that could be accepted from the previous copied cleanroom input package. Source refresh `d5a70b23ad05abd4188b1f0d37d9c6aba600cce5` satisfied this handoff, and CP8 consumed the refreshed package in the dirty Rust target.

The source refresh:

- the nine fixture scenario transition rows are now out-of-scope transit-only rows;
- the three Mage Armor admission/lifecycle rows are now reducer-routed through `cleanroom-input/qnt/battle-runtime/battle-runtime-spell-base-armor-class-effect.route.mbt.qnt`.

CP8 accepted the remaining three Mage Armor rows. Current target evidence coverage is `659 / 659` refreshed in-scope obligations, with `45` out-of-scope obligations.

## Remaining Blocker Groups

### Scenario transition rows

Count: `9`

Rows:

- `cleanroom-input/qnt/battle-runtime/battle-runtime-after-hit-damage-riders.mbt.qnt#step:doFinish`
- `cleanroom-input/qnt/battle-runtime/battle-runtime-after-hit-damage-riders.mbt.qnt#step:doStartDivineSmiteFreeCast`
- `cleanroom-input/qnt/battle-runtime/battle-runtime-after-hit-damage-riders.mbt.qnt#step:doStartEnsnaringFailedSave`
- `cleanroom-input/qnt/battle-runtime/battle-runtime-after-hit-damage-riders.mbt.qnt#step:doStartEnsnaringSuccessfulSave`
- `cleanroom-input/qnt/battle-runtime/battle-runtime-after-hit-damage-riders.mbt.qnt#step:doStartSearingSmite`
- `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-hosted-attack-and-riders.mbt.qnt#step:doFinish`
- `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-hosted-attack-and-riders.mbt.qnt#step:doStartDivineFavor`
- `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-hosted-attack-and-riders.mbt.qnt#step:doStartMagicWeapon`
- `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-hosted-attack-and-riders.mbt.qnt#step:doStartShillelagh`

Current evidence reason: `scenario transition has no reducer route surface`.

Source-side decision result:

- These actions were classified as fixture scenario sequencing and excluded from reducer-route coverage as out-of-scope transit-only rows.

The dirty cleanroom must not synthesize a route for these rows from adapter names or prior implementation history.

### Armor-class battle route owner rows

Count: `3`

Rows:

- `cleanroom-input/qnt/battle-runtime/battle-runtime-mage-armor-selected-identity.mbt.qnt#step:doDiscoverMageArmorUnarmoredSelfTarget`
- `cleanroom-input/qnt/battle-runtime/battle-runtime-mage-armor-selected-identity.mbt.qnt#step:doExpireMageArmorDuration`
- `cleanroom-input/qnt/battle-runtime/battle-runtime-mage-armor-selected-identity.mbt.qnt#step:doRejectMageArmorArmoredTarget`

Current evidence reason: resolved by refreshed generic spell base Armor Class route connector and accepted by CP8 target replay.

Source-side work completed:

- Added a generic spell base Armor Class route connector.
- Covered unarmored target admission, armored-target rejection, base Armor Class active-effect projection, and duration expiry.
- Kept selected spell identity at catalog, selection, admission, or fixture boundaries only.
- Regenerated the cleanroom input package for the dirty target.

## Next Campaign Boundary

This handoff is closed. The next useful campaign boundary is not another target lane for these rows; it is deciding whether to clean up old historical evidence debt, or to start a new source-QNT architecture campaign beyond this dirty cleanroom rehearsal.
