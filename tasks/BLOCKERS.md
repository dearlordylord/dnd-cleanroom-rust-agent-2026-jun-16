# Blockers

- FCSF-03A-HIT-POINT-REGAIN-PREVENTION: no SQNT-03A target behavior blockers. The dirty target now observes `SpellAttackRouteSubject` damage, `HitPointRegainPreventionRouteSubject` active-effect admission, later `HitPointHealingDistributionFillKind` interdiction, `BattleTurnBoundaryOwner` expiry, and `BattleActiveEffectOwner` cleanup through public reducer route events in `tasks/target-replay-evidence/FCSF-03A-hit-point-regain-prevention-dirty-replay.json`.
  - The evidence file records five connector-scope blockers for the other attack-spell selected-identity branches only because the per-driver evidence verifier requires every branch in `battle-runtime-attack-spell-shape-selected-identity.mbt.qnt` to be accounted for. Those rows are not SQNT-03A target behavior blockers; the copied hit-point-regain-prevention route connector defines no `HitPointRegainPreventionRouteSubject` action sequence for those non-regain branches.
- FCSF-04-OBJECT-STALE-DIRTY-REPLAY: no target blockers. All 6 copied Starry Wisp object route connector rows are observed through public battle reducer route entrypoints in `tasks/target-replay-evidence/FCSF-04-object-stale-dirty-replay.json`; the 7 copied source-driver obligations are covered, with low/high damage sharing `doRouteObjectDamageAndLight`.
- FCSF-05-REACTION-INTERRUPT-PAYLOAD-TAXONOMY: no target blockers. All 5 copied connector route rows are observed through public battle reducer route entrypoints in `tasks/target-replay-evidence/FCSF-05-reaction-interrupt-dirty-replay.json`.
- FCSF-06-CHARACTER-SHEET-HANDOFF-DIRTY-REPLAY: 14 target blockers are recorded in `tasks/target-replay-evidence/FCSF-06-character-sheet-handoff-dirty-replay.json`.
  - Character sheet Spell Slot/Pact Slot route rows: all 11 copied route connector actions are target-blocked because the dirty target exposes public `SheetSlotFacts` projection replay but no independent public `CharacterSheet` qRoute state for spell-resource rows.
  - Character battle settlement route rows: `doRejectActiveBattleStateHandoff`, `doRejectMixedSpellAndPactSlotSettlement`, and `doSettlePurePactMagicSlotExpenditure` are target-blocked because `cleanroom-input/qnt/character-battle-runtime/character-battle-settlement.route.mbt.qnt` does not define those actions, and no alternate copied connector in this lane defines them.
- L15-RR16-CHAINED-ATTACK-PROCEDURE-ROUTES: no blockers. All 10 copied branch obligations are covered by reducer-routed target replay evidence.
- L15-RR22-BATTLE-INDEPENDENT-SPELL-ATTACK-SEQUENCE-ROUTES: no blockers. All 8 copied branch obligations are covered by reducer-routed target replay evidence.

---

L15-RRCP5-B-ACTIVE-EFFECT-LIFECYCLE-ROUTES

- No target blockers. All selected CP5 active-effect lifecycle route rows are covered by tasks/target-replay-evidence/L15-RRCP5-B-active-effect-lifecycle-routes.json.

## L15-RR19-BATTLE-REACTION-INTERRUPT-ROUTES

- doCounterspellAllowsSpellCastResume: Counterspell reaction branches are listed as current out-of-scope obligations for the reaction-casting-time route assignment.
- doCounterspellEndsSpellCast: Counterspell reaction branches are listed as current out-of-scope obligations for the reaction-casting-time route assignment.
