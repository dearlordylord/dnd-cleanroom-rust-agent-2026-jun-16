use crate::rules::character_lifecycle::{
    create_sheet_from_build, finalize_draft_to_build, project_sheet_to_battle_init,
    resolve_skeleton_shortsword_attack, settle_battle_to_sheet, CharacterLifecycleLayer,
    CharacterLifecycleProjection,
};

pub const BRANCH_ACTIONS: [&str; 5] = [
    "doFinalizeDraftToBuild",
    "doCreateSheetFromBuild",
    "doProjectSheetToBattleInit",
    "doResolveSkeletonShortswordAttack",
    "doSettleBattleToSheet",
];

pub fn replay_observed_action(observed_action_taken: &str) -> CharacterLifecycleProjection {
    match observed_action_taken {
        "doFinalizeDraftToBuild" => finalize_draft_to_build(),
        "doCreateSheetFromBuild" => create_sheet_from_build(),
        "doProjectSheetToBattleInit" => project_sheet_to_battle_init(),
        "doResolveSkeletonShortswordAttack" => resolve_skeleton_shortsword_attack(),
        "doSettleBattleToSheet" => settle_battle_to_sheet(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> CharacterLifecycleProjection {
    replay_observed_action(observed_action_taken)
}

pub fn projection_payload(projection: &CharacterLifecycleProjection) -> String {
    let facts = projection.facts();
    [
        format!("qLayer={}", layer_ref(projection.layer())),
        format!("qDraftHasOpenHoles={}", facts.draft_has_open_holes()),
        format!("qBuildFinalized={}", facts.build_finalized()),
        format!("qSheetOwnsHitPoints={}", facts.sheet_owns_hit_points()),
        format!("qSheetCurrentHp={}", facts.sheet_current_hp()),
        format!("qSheetMaxHp={}", facts.sheet_max_hp()),
        format!(
            "qBattleInitCharacterCombatant={}",
            facts.battle_init_character_combatant()
        ),
        format!(
            "qBattleRuntimeCharacterHp={}",
            facts.battle_runtime_character_hp()
        ),
        format!(
            "qBattleRuntimeHpChanged={}",
            facts.battle_runtime_hp_changed()
        ),
        format!("qSettlementCurrentHp={}", facts.settlement_current_hp()),
        format!(
            "qSettlementPersistedBattleHp={}",
            facts.settlement_persisted_battle_hp()
        ),
        format!(
            "qBuildIdentityUnchanged={}",
            facts.build_identity_unchanged()
        ),
        format!("qReplayIndex={}", projection.replay_index()),
    ]
    .join("\n")
}

fn layer_ref(layer: CharacterLifecycleLayer) -> &'static str {
    match layer {
        CharacterLifecycleLayer::Draft => "Draft",
        CharacterLifecycleLayer::Build => "Build",
        CharacterLifecycleLayer::Sheet => "Sheet",
        CharacterLifecycleLayer::BattleInitProjection => "BattleInitProjection",
        CharacterLifecycleLayer::BattleRuntime => "BattleRuntime",
        CharacterLifecycleLayer::Settlement => "Settlement",
    }
}
