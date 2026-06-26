use crate::rules::battle_reducer_spine::{
    discover_battle_acts, hit_point_restoration_from_battle, resolve_battle_subject_test_fill,
    start_fighter_skeleton_battle, with_zero_hit_point_lifecycle, Actor, BattleFill,
    BattleHitPointRestorationFill, BattleHoleKind, BattleResolutionResult, BattleState,
    BattleSubject, BattleSubjectKind,
};
use crate::rules::hit_point_restoration_ordering::{
    hit_point_restoration_fill_order_result, hit_point_restoration_projection,
    hit_point_restoration_projection_from_result, restored_hit_points_from_zero,
    HitPointRestorationFillKind, HitPointRestorationFillOrderingError,
    HitPointRestorationFrontierStage, HitPointRestorationHoleKind,
    HitPointRestorationProjectionFacts, HitPointRestorationProtocol,
    HitPointRestorationRuntimeResult, HitPointRestorationState,
};

use super::battle_runtime_reducer_route::{
    battle_resolution_continuation, route_discover_battle_acts, route_resolve_battle_subject,
    route_resolve_battle_subject_from_result, route_start_battle, ReducerRouteEvent,
    ReducerRouteFillKind, ReducerRouteOwnerGroup, ReducerRouteResolveConnector,
    ReducerRouteResolveFill, ReducerRouteSubjectFamily,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HitPointRestorationOrderingWitness {
    pub stage: &'static str,
    pub protocol_result: &'static str,
    pub protocol_holes: Vec<&'static str>,
    pub last_ordering_error: &'static str,
    pub spell_target_hp: i16,
    pub spell_target_zero_hp_lifecycle_cleared: bool,
    pub feature_target_hp: i16,
    pub feature_target_zero_hp_lifecycle_cleared: bool,
}

pub const BRANCH_ACTIONS: [&str; 9] = [
    "doDiscoverSingleTargetSpellHealing",
    "doSubmitHealingRollBeforeTargetChoice",
    "doFillSpellHealingTargetChoice",
    "doDiscoverTargetListSpellHealing",
    "doSubmitHealingRollBeforeTargetList",
    "doFillSpellHealingTargetList",
    "doFillSpellHealingRoll",
    "doDiscoverFeatureHealingPool",
    "doFillFeatureHealingDistribution",
];

pub fn replay_observed_action(observed_action_taken: &str) -> HitPointRestorationOrderingWitness {
    match observed_action_taken {
        "doDiscoverSingleTargetSpellHealing" => witness_from_state(discover_single_target_spell()),
        "doSubmitHealingRollBeforeTargetChoice" => witness_from_state(state_after_spell_fill(
            BattleSubjectKind::HitPointRestorationSingleTargetSpell,
            BattleHitPointRestorationFill::HealingRoll(5),
        )),
        "doFillSpellHealingTargetChoice" => witness_from_state(state_after_spell_fill(
            BattleSubjectKind::HitPointRestorationSingleTargetSpell,
            BattleHitPointRestorationFill::TargetChoice(Actor::Rogue),
        )),
        "doDiscoverTargetListSpellHealing" => witness_from_state(discover_target_list_spell()),
        "doSubmitHealingRollBeforeTargetList" => witness_from_state(state_after_spell_fill(
            BattleSubjectKind::HitPointRestorationTargetListSpell,
            BattleHitPointRestorationFill::HealingRoll(5),
        )),
        "doFillSpellHealingTargetList" => witness_from_state(state_after_spell_fill(
            BattleSubjectKind::HitPointRestorationTargetListSpell,
            BattleHitPointRestorationFill::SpellTargetList(Actor::Rogue),
        )),
        "doFillSpellHealingRoll" => witness_from_state(state_after_spell_healing_roll()),
        "doDiscoverFeatureHealingPool" => witness_from_state(discover_feature_healing_pool()),
        "doFillFeatureHealingDistribution" => {
            witness_from_state(state_after_feature_distribution())
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> HitPointRestorationOrderingWitness {
    match observed_action_taken {
        "doDiscoverSingleTargetSpellHealing" => {
            witness_from_state(discover_single_target_spell_witness())
        }
        "doSubmitHealingRollBeforeTargetChoice" => {
            witness_from_state(submit_roll_before_target_choice())
        }
        "doFillSpellHealingTargetChoice" => witness_from_state(fill_spell_target_choice()),
        "doDiscoverTargetListSpellHealing" => {
            witness_from_state(discover_target_list_spell_witness())
        }
        "doSubmitHealingRollBeforeTargetList" => {
            witness_from_state(submit_roll_before_target_list())
        }
        "doFillSpellHealingTargetList" => witness_from_state(fill_spell_target_list()),
        "doFillSpellHealingRoll" => witness_from_state(fill_spell_healing_roll()),
        "doDiscoverFeatureHealingPool" => {
            witness_from_state(discover_feature_healing_pool_witness())
        }
        "doFillFeatureHealingDistribution" => witness_from_state(fill_feature_distribution()),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doDiscoverSingleTargetSpellHealing" => {
            restoration_discovery_route(BattleSubjectKind::HitPointRestorationSingleTargetSpell).2
        }
        "doSubmitHealingRollBeforeTargetChoice" => {
            restoration_route_after_spell_fill(
                BattleSubjectKind::HitPointRestorationSingleTargetSpell,
                BattleHitPointRestorationFill::HealingRoll(5),
            )
            .1
        }
        "doFillSpellHealingTargetChoice" => {
            restoration_route_after_spell_fill(
                BattleSubjectKind::HitPointRestorationSingleTargetSpell,
                BattleHitPointRestorationFill::TargetChoice(Actor::Rogue),
            )
            .1
        }
        "doDiscoverTargetListSpellHealing" => {
            restoration_discovery_route(BattleSubjectKind::HitPointRestorationTargetListSpell).2
        }
        "doSubmitHealingRollBeforeTargetList" => {
            restoration_route_after_spell_fill(
                BattleSubjectKind::HitPointRestorationTargetListSpell,
                BattleHitPointRestorationFill::HealingRoll(5),
            )
            .1
        }
        "doFillSpellHealingTargetList" => {
            restoration_route_after_spell_fill(
                BattleSubjectKind::HitPointRestorationTargetListSpell,
                BattleHitPointRestorationFill::SpellTargetList(Actor::Rogue),
            )
            .1
        }
        "doFillSpellHealingRoll" => spell_healing_roll_route(),
        "doDiscoverFeatureHealingPool" => feature_restoration_discovery_route().2,
        "doFillFeatureHealingDistribution" => feature_distribution_route(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doDiscoverSingleTargetSpellHealing" => expected_single_target_route(),
        "doSubmitHealingRollBeforeTargetChoice" => {
            let mut route = expected_single_target_route();
            route.push(route_resolve_battle_subject(
                ReducerRouteSubjectFamily::HitPointRestoration,
                ReducerRouteFillKind::RolledDice,
                vec![BattleHoleKind::TargetChoice],
                ReducerRouteOwnerGroup::HoleFrontier,
            ));
            route
        }
        "doFillSpellHealingTargetChoice" => {
            let mut route = expected_single_target_route();
            route.push(route_resolve_battle_subject(
                ReducerRouteSubjectFamily::HitPointRestoration,
                ReducerRouteFillKind::TargetChoice,
                vec![BattleHoleKind::RolledDice],
                ReducerRouteOwnerGroup::HoleFrontier,
            ));
            route
        }
        "doDiscoverTargetListSpellHealing" => expected_target_list_route(),
        "doSubmitHealingRollBeforeTargetList" => {
            let mut route = expected_target_list_route();
            route.push(route_resolve_battle_subject(
                ReducerRouteSubjectFamily::HitPointRestoration,
                ReducerRouteFillKind::RolledDice,
                vec![BattleHoleKind::SpellTargetList],
                ReducerRouteOwnerGroup::HoleFrontier,
            ));
            route
        }
        "doFillSpellHealingTargetList" => {
            let mut route = expected_target_list_route();
            route.push(route_resolve_battle_subject(
                ReducerRouteSubjectFamily::HitPointRestoration,
                ReducerRouteFillKind::SpellTargetList,
                vec![BattleHoleKind::RolledDice],
                ReducerRouteOwnerGroup::HoleFrontier,
            ));
            route
        }
        "doFillSpellHealingRoll" => {
            let mut route = expected_route("doFillSpellHealingTargetChoice");
            route.push(route_resolve_battle_subject(
                ReducerRouteSubjectFamily::HitPointRestoration,
                ReducerRouteFillKind::RolledDice,
                Vec::new(),
                ReducerRouteOwnerGroup::HitPointAndZeroHpLifecycle,
            ));
            route
        }
        "doDiscoverFeatureHealingPool" => expected_feature_route(),
        "doFillFeatureHealingDistribution" => {
            let mut route = expected_feature_route();
            route.push(route_resolve_battle_subject(
                ReducerRouteSubjectFamily::HitPointRestoration,
                ReducerRouteFillKind::HitPointHealingDistribution,
                Vec::new(),
                ReducerRouteOwnerGroup::HitPointAndZeroHpLifecycle,
            ));
            route
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn projection_payload(witness: &HitPointRestorationOrderingWitness) -> String {
    [
        format!("qStage={}", witness.stage),
        format!("protocolResult={}", witness.protocol_result),
        format!("protocolHoles={}", joined_or_none(&witness.protocol_holes)),
        format!("qLastOrderingError={}", witness.last_ordering_error),
        format!("qSpellTargetHp={}", witness.spell_target_hp),
        format!(
            "qSpellTargetZeroHpLifecycleCleared={}",
            witness.spell_target_zero_hp_lifecycle_cleared
        ),
        format!("qFeatureTargetHp={}", witness.feature_target_hp),
        format!(
            "qFeatureTargetZeroHpLifecycleCleared={}",
            witness.feature_target_zero_hp_lifecycle_cleared
        ),
    ]
    .join("\n")
}

fn discover_single_target_spell() -> HitPointRestorationState {
    let state = restoration_fixture();
    let act = discovered_restoration_act(
        &state,
        BattleSubjectKind::HitPointRestorationSingleTargetSpell,
    );
    assert_eq!(
        act.subject.kind,
        BattleSubjectKind::HitPointRestorationSingleTargetSpell,
        "single-target healing must be discovered through reducer"
    );
    discover_single_target_spell_witness()
}

fn discover_single_target_spell_witness() -> HitPointRestorationState {
    projection(
        HitPointRestorationFrontierStage::SpellHealingTargetChoice,
        HitPointRestorationRuntimeResult::NeedsHoles,
        None,
        0,
        false,
        0,
        false,
    )
}

fn submit_roll_before_target_choice() -> HitPointRestorationState {
    let stage = HitPointRestorationFrontierStage::SpellHealingTargetChoice;
    let result =
        hit_point_restoration_fill_order_result(stage, HitPointRestorationFillKind::RolledDice);
    hit_point_restoration_projection_from_result(result, 0, false, 0, false)
}

fn fill_spell_target_choice() -> HitPointRestorationState {
    let stage = HitPointRestorationFrontierStage::SpellHealingTargetChoice;
    let result =
        hit_point_restoration_fill_order_result(stage, HitPointRestorationFillKind::TargetChoice);
    hit_point_restoration_projection_from_result(result, 0, false, 0, false)
}

fn discover_target_list_spell() -> HitPointRestorationState {
    let state = restoration_fixture();
    let act = discovered_restoration_act(
        &state,
        BattleSubjectKind::HitPointRestorationTargetListSpell,
    );
    assert_eq!(
        act.subject.kind,
        BattleSubjectKind::HitPointRestorationTargetListSpell,
        "target-list healing must be discovered through reducer"
    );
    discover_target_list_spell_witness()
}

fn discover_target_list_spell_witness() -> HitPointRestorationState {
    projection(
        HitPointRestorationFrontierStage::SpellHealingTargetList,
        HitPointRestorationRuntimeResult::NeedsHoles,
        None,
        0,
        false,
        0,
        false,
    )
}

fn submit_roll_before_target_list() -> HitPointRestorationState {
    let stage = HitPointRestorationFrontierStage::SpellHealingTargetList;
    let result =
        hit_point_restoration_fill_order_result(stage, HitPointRestorationFillKind::RolledDice);
    hit_point_restoration_projection_from_result(result, 0, false, 0, false)
}

fn fill_spell_target_list() -> HitPointRestorationState {
    let stage = HitPointRestorationFrontierStage::SpellHealingTargetList;
    let result = hit_point_restoration_fill_order_result(
        stage,
        HitPointRestorationFillKind::SpellTargetList,
    );
    hit_point_restoration_projection_from_result(result, 0, false, 0, false)
}

fn fill_spell_healing_roll() -> HitPointRestorationState {
    let stage = HitPointRestorationFrontierStage::SpellHealingRoll;
    let result =
        hit_point_restoration_fill_order_result(stage, HitPointRestorationFillKind::RolledDice);
    let (spell_hp, spell_cleared) = restored_hit_points_from_zero(5);
    hit_point_restoration_projection_from_result(result, spell_hp, spell_cleared, 0, false)
}

fn discover_feature_healing_pool() -> HitPointRestorationState {
    let state = feature_restoration_fixture();
    let act = discovered_restoration_act(
        &state,
        BattleSubjectKind::HitPointRestorationFeatureHealingPool,
    );
    assert_eq!(
        act.subject.kind,
        BattleSubjectKind::HitPointRestorationFeatureHealingPool,
        "feature healing pool must be discovered through reducer"
    );
    discover_feature_healing_pool_witness()
}

fn discover_feature_healing_pool_witness() -> HitPointRestorationState {
    projection(
        HitPointRestorationFrontierStage::FeatureHealingPoolDistribution,
        HitPointRestorationRuntimeResult::NeedsHoles,
        None,
        0,
        false,
        0,
        false,
    )
}

fn fill_feature_distribution() -> HitPointRestorationState {
    let stage = HitPointRestorationFrontierStage::FeatureHealingPoolDistribution;
    let result = hit_point_restoration_fill_order_result(
        stage,
        HitPointRestorationFillKind::HitPointHealingDistribution,
    );
    let (feature_hp, feature_cleared) = restored_hit_points_from_zero(8);
    hit_point_restoration_projection_from_result(result, 0, false, feature_hp, feature_cleared)
}

fn projection(
    stage: HitPointRestorationFrontierStage,
    runtime_result: HitPointRestorationRuntimeResult,
    last_ordering_error: Option<HitPointRestorationFillOrderingError>,
    spell_target_hit_points: i16,
    spell_target_zero_hit_point_lifecycle_cleared: bool,
    feature_target_hit_points: i16,
    feature_target_zero_hit_point_lifecycle_cleared: bool,
) -> HitPointRestorationState {
    hit_point_restoration_projection(HitPointRestorationProjectionFacts {
        stage,
        runtime_result,
        last_ordering_error,
        spell_target_hit_points,
        spell_target_zero_hit_point_lifecycle_cleared,
        feature_target_hit_points,
        feature_target_zero_hit_point_lifecycle_cleared,
    })
}

fn witness_from_state(state: HitPointRestorationState) -> HitPointRestorationOrderingWitness {
    HitPointRestorationOrderingWitness {
        stage: stage_ref(state.stage),
        protocol_result: protocol_result_ref(&state.protocol),
        protocol_holes: protocol_holes(&state.protocol),
        last_ordering_error: ordering_error_ref(state.last_ordering_error),
        spell_target_hp: state.spell_target_hit_points,
        spell_target_zero_hp_lifecycle_cleared: state.spell_target_zero_hit_point_lifecycle_cleared,
        feature_target_hp: state.feature_target_hit_points,
        feature_target_zero_hp_lifecycle_cleared: state
            .feature_target_zero_hit_point_lifecycle_cleared,
    }
}

fn stage_ref(stage: HitPointRestorationFrontierStage) -> &'static str {
    match stage {
        HitPointRestorationFrontierStage::ActSelection => "HitPointRestorationActSelectionStage",
        HitPointRestorationFrontierStage::SpellHealingTargetChoice => {
            "SpellHealingTargetChoiceStage"
        }
        HitPointRestorationFrontierStage::SpellHealingTargetList => "SpellHealingTargetListStage",
        HitPointRestorationFrontierStage::SpellHealingRoll => "SpellHealingRollStage",
        HitPointRestorationFrontierStage::FeatureHealingPoolDistribution => {
            "FeatureHealingPoolDistributionStage"
        }
        HitPointRestorationFrontierStage::Resolved => "HitPointRestorationResolvedStage",
    }
}

fn protocol_result_ref(protocol: &HitPointRestorationProtocol) -> &'static str {
    match protocol {
        HitPointRestorationProtocol::Init => "init",
        HitPointRestorationProtocol::NeedsHoles(_) => "needsHoles",
        HitPointRestorationProtocol::Resolved => "resolved",
    }
}

fn protocol_holes(protocol: &HitPointRestorationProtocol) -> Vec<&'static str> {
    match protocol {
        HitPointRestorationProtocol::NeedsHoles(holes) => {
            holes.iter().copied().map(hole_ref).collect()
        }
        HitPointRestorationProtocol::Init | HitPointRestorationProtocol::Resolved => vec![],
    }
}

fn hole_ref(hole: HitPointRestorationHoleKind) -> &'static str {
    match hole {
        HitPointRestorationHoleKind::TargetChoice => "TargetChoiceHoleKind",
        HitPointRestorationHoleKind::SpellTargetList => "SpellTargetListHoleKind",
        HitPointRestorationHoleKind::RolledDice => "RolledDiceHoleKind",
        HitPointRestorationHoleKind::HitPointHealingDistribution => {
            "HitPointHealingDistributionHoleKind"
        }
    }
}

fn ordering_error_ref(error: Option<HitPointRestorationFillOrderingError>) -> &'static str {
    match error {
        Some(HitPointRestorationFillOrderingError::HealingTargetRequired) => {
            "healingTargetRequired"
        }
        Some(HitPointRestorationFillOrderingError::HealingAmountRequired) => {
            "healingAmountRequired"
        }
        Some(HitPointRestorationFillOrderingError::HealingDistributionRequired) => {
            "healingDistributionRequired"
        }
        None => "",
    }
}

fn joined_or_none(values: &[&'static str]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}

fn expected_single_target_route() -> Vec<ReducerRouteEvent> {
    vec![
        route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
        route_discover_battle_acts(
            ReducerRouteSubjectFamily::HitPointRestoration,
            vec![BattleHoleKind::TargetChoice],
            ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
        ),
    ]
}

fn expected_target_list_route() -> Vec<ReducerRouteEvent> {
    vec![
        route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
        route_discover_battle_acts(
            ReducerRouteSubjectFamily::HitPointRestoration,
            vec![BattleHoleKind::SpellTargetList],
            ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
        ),
    ]
}

fn expected_feature_route() -> Vec<ReducerRouteEvent> {
    vec![
        route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
        route_discover_battle_acts(
            ReducerRouteSubjectFamily::HitPointRestoration,
            vec![BattleHoleKind::HitPointHealingDistribution],
            ReducerRouteOwnerGroup::ActionEconomy,
        ),
    ]
}

fn restoration_fixture() -> BattleState {
    with_zero_hit_point_lifecycle(start_fighter_skeleton_battle(), Actor::Rogue)
        .expect("restoration fixture target uses death saving throws")
}

fn feature_restoration_fixture() -> BattleState {
    with_zero_hit_point_lifecycle(start_fighter_skeleton_battle(), Actor::Rogue)
        .expect("feature restoration fixture target uses death saving throws")
}

fn state_after_spell_fill(
    kind: BattleSubjectKind,
    fill: BattleHitPointRestorationFill,
) -> HitPointRestorationState {
    let state = restoration_fixture();
    let subject = discovered_restoration_act(&state, kind).subject;
    project_restoration_result(resolve_battle_subject_test_fill(
        state,
        subject,
        BattleFill::HitPointRestoration(fill),
    ))
}

fn restoration_discovery_route(
    kind: BattleSubjectKind,
) -> (BattleState, BattleSubject, Vec<ReducerRouteEvent>) {
    let state = restoration_fixture();
    let act = discovered_restoration_act(&state, kind);
    let route = vec![
        route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
        route_discover_battle_acts(
            ReducerRouteSubjectFamily::HitPointRestoration,
            act.holes,
            ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
        ),
    ];
    (state, act.subject, route)
}

fn feature_restoration_discovery_route() -> (BattleState, BattleSubject, Vec<ReducerRouteEvent>) {
    let state = feature_restoration_fixture();
    let act = discovered_restoration_act(
        &state,
        BattleSubjectKind::HitPointRestorationFeatureHealingPool,
    );
    let route = vec![
        route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
        route_discover_battle_acts(
            ReducerRouteSubjectFamily::HitPointRestoration,
            act.holes,
            ReducerRouteOwnerGroup::ActionEconomy,
        ),
    ];
    (state, act.subject, route)
}

fn restoration_route_after_spell_fill(
    kind: BattleSubjectKind,
    fill: BattleHitPointRestorationFill,
) -> (BattleResolutionResult, Vec<ReducerRouteEvent>) {
    let (state, subject, route) = restoration_discovery_route(kind);
    resolve_restoration_with_route(
        state,
        subject,
        fill,
        hit_point_restoration_route_fill(fill),
        ReducerRouteOwnerGroup::HoleFrontier,
        route,
    )
}

fn spell_healing_roll_route() -> Vec<ReducerRouteEvent> {
    let (result, route) = restoration_route_after_spell_fill(
        BattleSubjectKind::HitPointRestorationSingleTargetSpell,
        BattleHitPointRestorationFill::TargetChoice(Actor::Rogue),
    );
    let (state, subject) =
        battle_resolution_continuation(result, "hit point restoration spell healing roll route");
    resolve_restoration_with_route(
        state,
        subject,
        BattleHitPointRestorationFill::HealingRoll(5),
        ReducerRouteFillKind::RolledDice,
        ReducerRouteOwnerGroup::HitPointAndZeroHpLifecycle,
        route,
    )
    .1
}

fn feature_distribution_route() -> Vec<ReducerRouteEvent> {
    let (state, subject, route) = feature_restoration_discovery_route();
    resolve_restoration_with_route(
        state,
        subject,
        BattleHitPointRestorationFill::HitPointHealingDistribution {
            target: Actor::Rogue,
            amount: 8,
        },
        ReducerRouteFillKind::HitPointHealingDistribution,
        ReducerRouteOwnerGroup::HitPointAndZeroHpLifecycle,
        route,
    )
    .1
}

fn resolve_restoration_with_route(
    state: BattleState,
    subject: BattleSubject,
    fill: BattleHitPointRestorationFill,
    route_fill: ReducerRouteFillKind,
    owner: ReducerRouteOwnerGroup,
    mut route: Vec<ReducerRouteEvent>,
) -> (BattleResolutionResult, Vec<ReducerRouteEvent>) {
    let result =
        resolve_battle_subject_test_fill(state, subject, BattleFill::HitPointRestoration(fill));
    route.push(route_resolve_battle_subject_from_result(
        ReducerRouteResolveConnector {
            subject: ReducerRouteSubjectFamily::HitPointRestoration,
            fill: ReducerRouteResolveFill::Fill(route_fill),
            owner,
        },
        &result,
    ));
    (result, route)
}

fn hit_point_restoration_route_fill(fill: BattleHitPointRestorationFill) -> ReducerRouteFillKind {
    match fill {
        BattleHitPointRestorationFill::TargetChoice(_) => ReducerRouteFillKind::TargetChoice,
        BattleHitPointRestorationFill::SpellTargetList(_) => ReducerRouteFillKind::SpellTargetList,
        BattleHitPointRestorationFill::HealingRoll(_) => ReducerRouteFillKind::RolledDice,
        BattleHitPointRestorationFill::HitPointHealingDistribution { .. } => {
            ReducerRouteFillKind::HitPointHealingDistribution
        }
    }
}

fn state_after_spell_healing_roll() -> HitPointRestorationState {
    let (state, subject) = reducer_state_after_restoration_fill(
        restoration_fixture(),
        BattleSubjectKind::HitPointRestorationSingleTargetSpell,
        BattleHitPointRestorationFill::TargetChoice(Actor::Rogue),
    );
    project_restoration_result(resolve_battle_subject_test_fill(
        state,
        subject,
        BattleFill::HitPointRestoration(BattleHitPointRestorationFill::HealingRoll(5)),
    ))
}

fn state_after_feature_distribution() -> HitPointRestorationState {
    let state = feature_restoration_fixture();
    let subject = discovered_restoration_act(
        &state,
        BattleSubjectKind::HitPointRestorationFeatureHealingPool,
    )
    .subject;
    project_restoration_result(resolve_battle_subject_test_fill(
        state,
        subject,
        BattleFill::HitPointRestoration(
            BattleHitPointRestorationFill::HitPointHealingDistribution {
                target: Actor::Rogue,
                amount: 8,
            },
        ),
    ))
}

fn reducer_state_after_restoration_fill(
    state: BattleState,
    kind: BattleSubjectKind,
    fill: BattleHitPointRestorationFill,
) -> (BattleState, BattleSubject) {
    let subject = discovered_restoration_act(&state, kind).subject;
    let result =
        resolve_battle_subject_test_fill(state, subject, BattleFill::HitPointRestoration(fill));
    battle_resolution_continuation(result, "hit point restoration state after fill")
}

fn discovered_restoration_act(
    state: &BattleState,
    kind: BattleSubjectKind,
) -> crate::rules::battle_reducer_spine::AvailableBattleAct {
    discover_battle_acts(state)
        .into_available_acts()
        .into_iter()
        .find(|act| act.subject.kind == kind)
        .expect("restoration diagnostic act should be discovered through reducer")
}

fn project_restoration_result(result: BattleResolutionResult) -> HitPointRestorationState {
    hit_point_restoration_from_battle(result.state())
}
