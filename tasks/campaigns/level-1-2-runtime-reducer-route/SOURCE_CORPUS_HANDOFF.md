# Source Corpus Handoff

Campaign: `level-1-2-runtime-reducer-route`

Integration worktree: `/workspace/typescript/.codex-worktrees/dnd-cleanroom-rrconv-19`

Integration branch: `ralph/rrconv-19-cleanroom`

Last dirty-cleanroom accounting head: `673df2329de89144270ab0d977f5cce1bb670ade`

## Status

CP7 exhausted the target-side reducer-route work that can be accepted from the copied cleanroom input package.

Current coverage is `656 / 668` accepted in-scope obligations. The remaining `12` obligations are not ready for another dirty Rust cleanroom implementation lane. Accepting them in target code now would infer semantics that are not present in the cleanroom package.

No Ralph worker should be launched for these rows until the source QNT/branch inventory package is refreshed or the denominator is intentionally changed.

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

Source-side decision required:

- If these actions are fixture scenario sequencing, update source branch-scope or branch-inventory generation so they are excluded from reducer-route coverage.
- If source decides any of them are real reducer transitions, add explicit QNT route connectors for that transition surface and regenerate the cleanroom package before target implementation.

The dirty cleanroom must not synthesize a route for these rows from adapter names or prior implementation history.

### Armor-class battle route owner rows

Count: `3`

Rows:

- `cleanroom-input/qnt/battle-runtime/battle-runtime-mage-armor-selected-identity.mbt.qnt#step:doDiscoverMageArmorUnarmoredSelfTarget`
- `cleanroom-input/qnt/battle-runtime/battle-runtime-mage-armor-selected-identity.mbt.qnt#step:doExpireMageArmorDuration`
- `cleanroom-input/qnt/battle-runtime/battle-runtime-mage-armor-selected-identity.mbt.qnt#step:doRejectMageArmorArmoredTarget`

Current evidence reason: copied reducer-route inventory has no Mage Armor qRoute connector or generic armor-class target-admission / active-effect lifecycle route owner.

Source-side work required:

- Add a generic armor-class base active-effect battle route connector.
- Cover unarmored target admission, armored-target rejection, and duration expiry.
- Keep selected spell identity at catalog, selection, admission, or fixture boundaries only.
- Regenerate the cleanroom input package before any target lane attempts acceptance.

## Next Campaign Boundary

The next useful dirty-cleanroom phase is a refresh phase after source/corpus work lands. It should consume a regenerated cleanroom package, recompute the branch denominator, and then launch only the rows that the new package makes derivable.

Until that refresh exists, the campaign is blocked on source/corpus or denominator decisions, not on Rust implementation.
