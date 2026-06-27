# Source Corpus Handoff

Campaign: `level-1-2-runtime-reducer-route`

Integration worktree: `/workspace/typescript/.codex-worktrees/dnd-cleanroom-rrconv-19`

Integration branch: `ralph/rrconv-19-cleanroom`

Last dirty-cleanroom accounting head: `673df2329de89144270ab0d977f5cce1bb670ade`

## Status

CP7 exhausted the target-side reducer-route work that could be accepted from the previous copied cleanroom input package.

Source refresh `d5a70b23ad05abd4188b1f0d37d9c6aba600cce5` satisfied this handoff:

- the nine fixture scenario transition rows are now out-of-scope transit-only rows;
- the three Mage Armor admission/lifecycle rows are now reducer-routed through `cleanroom-input/qnt/battle-runtime/battle-runtime-spell-base-armor-class-effect.route.mbt.qnt`.

Current target evidence coverage remains `656` accepted obligations, but the refreshed denominator is now `659` in-scope obligations. The next dirty cleanroom lane is target-side CP8 for the remaining three rows.

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

Current evidence reason: resolved by refreshed generic spell base Armor Class route connector.

Source-side work completed:

- Added a generic spell base Armor Class route connector.
- Covered unarmored target admission, armored-target rejection, base Armor Class active-effect projection, and duration expiry.
- Kept selected spell identity at catalog, selection, admission, or fixture boundaries only.
- Regenerated the cleanroom input package for the dirty target.

## Next Campaign Boundary

The next useful dirty-cleanroom phase is CP8. It should consume the regenerated package and launch only `L15-RRCP8-A-MAGE-ARMOR-GENERIC-AC-ROUTES`.
