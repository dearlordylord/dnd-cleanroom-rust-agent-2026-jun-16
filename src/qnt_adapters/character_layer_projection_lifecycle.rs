use crate::rules::character_lifecycle::{
    create_sheet_from_build, finalize_draft_to_build, project_sheet_to_battle_init,
    resolve_skeleton_shortsword_attack, settle_battle_to_sheet, CharacterLifecycleFacts,
    CharacterLifecycleLayer, CharacterLifecycleProjection,
};

use crate::rules::character_battle_handoff::{
    route_enter_battle_runtime, route_project_character_sheet_to_battle,
    route_settle_battle_to_character_sheet, CharacterBattleRouteEvent,
    CharacterBattleRouteFillFamily, CharacterBattleRouteOwnerGroup,
    CharacterBattleRouteSubjectFamily,
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
    match observed_action_taken {
        "doFinalizeDraftToBuild" => expected_finalize_draft_to_build(),
        "doCreateSheetFromBuild" => expected_create_sheet_from_build(),
        "doProjectSheetToBattleInit" => expected_project_sheet_to_battle_init(),
        "doResolveSkeletonShortswordAttack" => expected_resolve_skeleton_shortsword_attack(),
        "doSettleBattleToSheet" => expected_settle_battle_to_sheet(),
        action => panic!("unsupported expected mbt::actionTaken {action}"),
    }
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<CharacterBattleRouteEvent> {
    match observed_action_taken {
        "doFinalizeDraftToBuild" => route_after_finalize_draft_to_build(),
        "doCreateSheetFromBuild" => route_after_create_sheet_from_build(),
        "doProjectSheetToBattleInit" => route_after_project_sheet_to_battle_init(),
        "doResolveSkeletonShortswordAttack" => route_after_resolve_skeleton_shortsword_attack(),
        "doSettleBattleToSheet" => route_after_settle_battle_to_sheet(),
        action => panic!("unsupported route mbt::actionTaken {action}"),
    }
}

pub fn expected_route(observed_action_taken: &str) -> Vec<CharacterBattleRouteEvent> {
    match observed_action_taken {
        "doFinalizeDraftToBuild" => expected_route_after_finalize_draft_to_build(),
        "doCreateSheetFromBuild" => expected_route_after_create_sheet_from_build(),
        "doProjectSheetToBattleInit" => expected_route_after_project_sheet_to_battle_init(),
        "doResolveSkeletonShortswordAttack" => {
            expected_route_after_resolve_skeleton_shortsword_attack()
        }
        "doSettleBattleToSheet" => expected_route_after_settle_battle_to_sheet(),
        action => panic!("unsupported expected route mbt::actionTaken {action}"),
    }
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

fn expected_finalize_draft_to_build() -> CharacterLifecycleProjection {
    expected_projection(
        CharacterLifecycleLayer::Build,
        CharacterLifecycleFacts {
            build_finalized: true,
            ..empty_expected_lifecycle_facts()
        },
        1,
    )
}

fn expected_create_sheet_from_build() -> CharacterLifecycleProjection {
    expected_projection(
        CharacterLifecycleLayer::Sheet,
        CharacterLifecycleFacts {
            build_finalized: true,
            sheet_owns_hit_points: true,
            sheet_current_hp: 12,
            sheet_max_hp: 12,
            ..empty_expected_lifecycle_facts()
        },
        2,
    )
}

fn expected_project_sheet_to_battle_init() -> CharacterLifecycleProjection {
    expected_projection(
        CharacterLifecycleLayer::BattleInitProjection,
        CharacterLifecycleFacts {
            build_finalized: true,
            sheet_owns_hit_points: true,
            sheet_current_hp: 12,
            sheet_max_hp: 12,
            battle_init_character_combatant: true,
            ..empty_expected_lifecycle_facts()
        },
        3,
    )
}

fn expected_resolve_skeleton_shortsword_attack() -> CharacterLifecycleProjection {
    expected_projection(
        CharacterLifecycleLayer::BattleRuntime,
        CharacterLifecycleFacts {
            build_finalized: true,
            sheet_owns_hit_points: true,
            sheet_current_hp: 12,
            sheet_max_hp: 12,
            battle_init_character_combatant: true,
            battle_runtime_character_hp: 8,
            battle_runtime_hp_changed: true,
            ..empty_expected_lifecycle_facts()
        },
        4,
    )
}

fn expected_settle_battle_to_sheet() -> CharacterLifecycleProjection {
    expected_projection(
        CharacterLifecycleLayer::Settlement,
        CharacterLifecycleFacts {
            build_finalized: true,
            sheet_owns_hit_points: true,
            sheet_current_hp: 12,
            sheet_max_hp: 12,
            battle_init_character_combatant: true,
            battle_runtime_character_hp: 8,
            battle_runtime_hp_changed: true,
            settlement_current_hp: 8,
            settlement_persisted_battle_hp: true,
            build_identity_unchanged: true,
            ..empty_expected_lifecycle_facts()
        },
        5,
    )
}

fn expected_projection(
    layer: CharacterLifecycleLayer,
    facts: CharacterLifecycleFacts,
    replay_index: i16,
) -> CharacterLifecycleProjection {
    CharacterLifecycleProjection {
        layer,
        facts,
        replay_index,
    }
}

fn empty_expected_lifecycle_facts() -> CharacterLifecycleFacts {
    CharacterLifecycleFacts {
        draft_has_open_holes: false,
        build_finalized: false,
        sheet_owns_hit_points: false,
        sheet_current_hp: 0,
        sheet_max_hp: 0,
        battle_init_character_combatant: false,
        battle_runtime_character_hp: 0,
        battle_runtime_hp_changed: false,
        settlement_current_hp: 0,
        settlement_persisted_battle_hp: false,
        build_identity_unchanged: false,
    }
}

fn route_after_finalize_draft_to_build() -> Vec<CharacterBattleRouteEvent> {
    let mut route = Vec::new();
    append_finalize_draft_to_build_route(&mut route);
    route
}

fn route_after_create_sheet_from_build() -> Vec<CharacterBattleRouteEvent> {
    let mut route = route_after_finalize_draft_to_build();
    append_create_sheet_from_build_route(&mut route);
    route
}

fn route_after_project_sheet_to_battle_init() -> Vec<CharacterBattleRouteEvent> {
    let mut route = route_after_create_sheet_from_build();
    append_project_sheet_to_battle_init_route(&mut route);
    route
}

fn route_after_resolve_skeleton_shortsword_attack() -> Vec<CharacterBattleRouteEvent> {
    let mut route = route_after_project_sheet_to_battle_init();
    append_resolve_battle_runtime_route(&mut route);
    route
}

fn route_after_settle_battle_to_sheet() -> Vec<CharacterBattleRouteEvent> {
    let mut route = route_after_resolve_skeleton_shortsword_attack();
    append_settle_battle_to_sheet_route(&mut route);
    route
}

fn expected_route_after_finalize_draft_to_build() -> Vec<CharacterBattleRouteEvent> {
    let mut route = Vec::new();
    append_finalize_draft_to_build_route(&mut route);
    route
}

fn expected_route_after_create_sheet_from_build() -> Vec<CharacterBattleRouteEvent> {
    let mut route = expected_route_after_finalize_draft_to_build();
    append_create_sheet_from_build_route(&mut route);
    route
}

fn expected_route_after_project_sheet_to_battle_init() -> Vec<CharacterBattleRouteEvent> {
    let mut route = expected_route_after_create_sheet_from_build();
    append_project_sheet_to_battle_init_route(&mut route);
    route
}

fn expected_route_after_resolve_skeleton_shortsword_attack() -> Vec<CharacterBattleRouteEvent> {
    let mut route = expected_route_after_project_sheet_to_battle_init();
    append_resolve_battle_runtime_route(&mut route);
    route
}

fn expected_route_after_settle_battle_to_sheet() -> Vec<CharacterBattleRouteEvent> {
    let mut route = expected_route_after_resolve_skeleton_shortsword_attack();
    append_settle_battle_to_sheet_route(&mut route);
    route
}

fn append_finalize_draft_to_build_route(route: &mut Vec<CharacterBattleRouteEvent>) {
    route.push(route_project_character_sheet_to_battle(
        CharacterBattleRouteSubjectFamily::SheetToBattleInitRouteSubject,
        CharacterBattleRouteOwnerGroup::CharacterBattleBuildProjectionOwner,
    ));
}

fn append_create_sheet_from_build_route(route: &mut Vec<CharacterBattleRouteEvent>) {
    route.push(route_project_character_sheet_to_battle(
        CharacterBattleRouteSubjectFamily::SheetToBattleInitRouteSubject,
        CharacterBattleRouteOwnerGroup::CharacterBattleSheetOwner,
    ));
}

fn append_project_sheet_to_battle_init_route(route: &mut Vec<CharacterBattleRouteEvent>) {
    route.push(route_enter_battle_runtime(
        CharacterBattleRouteSubjectFamily::SheetToBattleInitRouteSubject,
        CharacterBattleRouteOwnerGroup::CharacterBattleInitProjectionOwner,
    ));
}

fn append_resolve_battle_runtime_route(route: &mut Vec<CharacterBattleRouteEvent>) {
    route.push(route_enter_battle_runtime(
        CharacterBattleRouteSubjectFamily::HandoffBattleMutationRouteSubject,
        CharacterBattleRouteOwnerGroup::CharacterBattleRuntimeOwner,
    ));
}

fn append_settle_battle_to_sheet_route(route: &mut Vec<CharacterBattleRouteEvent>) {
    route.push(route_settle_battle_to_character_sheet(
        CharacterBattleRouteSubjectFamily::BattleToSheetSettlementRouteSubject,
        CharacterBattleRouteFillFamily::HandoffBattleDeltaFill,
        Vec::new(),
        CharacterBattleRouteOwnerGroup::CharacterBattleSettlementOwner,
    ));
}
