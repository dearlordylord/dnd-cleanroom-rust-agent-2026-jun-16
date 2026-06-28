use crate::rules::battle_reducer_spine::{
    discover_generic_route_subject_observed, generic_route_subject_for_current_actor,
    observe_battle_route_start, resolve_battle_subject_observed, start_standard_battle,
    BattleEntrypointTrace, BattleGenericRouteFill, BattleReducerRouteOwnerGroup,
    BattleResolutionRequest, BattleState, BattleSubjectKind,
};
use crate::rules::species_passive_traits::{
    creature_space_traversal_allowed, project_dragonborn_damage_resistance,
    project_dwarven_resilience, project_goliath_powerful_build, project_halfling_brave,
    CreatureSpaceTraversalFacts, CreatureSpaceTraversalPermission, SpeciesPassiveProtocol,
    SpeciesPassiveRollMode, SpeciesPassiveScenarioResult, SpeciesPassiveTraitState,
};

use super::battle_runtime_reducer_route::{
    reducer_route_event_from_observed, route_discover_battle_acts_from_route_holes,
    route_resolve_battle_subject_from_route_result,
    route_resolve_battle_subject_without_fill_from_route_result, route_start_battle,
    ReducerRouteEvent, ReducerRouteFillKind, ReducerRouteHoleKind, ReducerRouteOwnerGroup,
    ReducerRouteResolutionOutcome, ReducerRouteSubjectFamily,
};

pub const BRANCH_ACTIONS: [&str; 4] = [
    "doDragonbornDamageResistance",
    "doDwarvenResilience",
    "doHalflingBrave",
    "doGoliathPowerfulBuild",
];

pub fn replay_observed_action(observed_action_taken: &str) -> SpeciesPassiveTraitState {
    match observed_action_taken {
        "doDragonbornDamageResistance" => project_dragonborn_damage_resistance(),
        "doDwarvenResilience" => project_dwarven_resilience(),
        "doHalflingBrave" => project_halfling_brave(),
        "doGoliathPowerfulBuild" => project_goliath_powerful_build(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> SpeciesPassiveTraitState {
    match observed_action_taken {
        "doDragonbornDamageResistance" => SpeciesPassiveTraitState {
            dragonborn_fire_damage_after: 4,
            dragonborn_cold_damage_after: 9,
            scenario_result: SpeciesPassiveScenarioResult::DragonbornDamageResistance,
            protocol: SpeciesPassiveProtocol::Resolved,
            ..initial_expected()
        },
        "doDwarvenResilience" => SpeciesPassiveTraitState {
            dwarf_poison_damage_after: 4,
            dwarf_fire_damage_after: 9,
            dwarf_poisoned_save_advantage: true,
            dwarf_charmed_save_advantage: false,
            scenario_result: SpeciesPassiveScenarioResult::DwarvenResilience,
            protocol: SpeciesPassiveProtocol::Resolved,
            ..initial_expected()
        },
        "doHalflingBrave" => SpeciesPassiveTraitState {
            halfling_frightened_avoid_save_advantage: true,
            halfling_frightened_end_save_advantage: true,
            halfling_poisoned_save_advantage: false,
            scenario_result: SpeciesPassiveScenarioResult::HalflingBrave,
            protocol: SpeciesPassiveProtocol::Resolved,
            ..initial_expected()
        },
        "doGoliathPowerfulBuild" => SpeciesPassiveTraitState {
            goliath_escape_roll_mode: SpeciesPassiveRollMode::Advantage,
            goliath_poisoned_escape_roll_mode: SpeciesPassiveRollMode::Normal,
            scenario_result: SpeciesPassiveScenarioResult::GoliathPowerfulBuild,
            protocol: SpeciesPassiveProtocol::Resolved,
            ..initial_expected()
        },
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doDragonbornDamageResistance" => observed_passive_route_after_damage_adjustment(),
        "doDwarvenResilience" | "doHalflingBrave" => {
            observed_passive_route_after_saving_throw_roll_mode()
        }
        "doGoliathPowerfulBuild" => observed_passive_route_after_ability_check_roll_mode(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doDragonbornDamageResistance" => passive_route_after_damage_adjustment(),
        "doDwarvenResilience" | "doHalflingBrave" => passive_route_after_saving_throw_roll_mode(),
        "doGoliathPowerfulBuild" => passive_route_after_ability_check_roll_mode(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn projection_payload(state: &SpeciesPassiveTraitState) -> String {
    [
        format!(
            "dragonbornFireDamageAfter={}",
            state.dragonborn_fire_damage_after
        ),
        format!(
            "dragonbornColdDamageAfter={}",
            state.dragonborn_cold_damage_after
        ),
        format!("dwarfPoisonDamageAfter={}", state.dwarf_poison_damage_after),
        format!("dwarfFireDamageAfter={}", state.dwarf_fire_damage_after),
        format!(
            "dwarfPoisonedSaveAdvantage={}",
            state.dwarf_poisoned_save_advantage
        ),
        format!(
            "dwarfCharmedSaveAdvantage={}",
            state.dwarf_charmed_save_advantage
        ),
        format!(
            "halflingFrightenedAvoidSaveAdvantage={}",
            state.halfling_frightened_avoid_save_advantage
        ),
        format!(
            "halflingFrightenedEndSaveAdvantage={}",
            state.halfling_frightened_end_save_advantage
        ),
        format!(
            "halflingPoisonedSaveAdvantage={}",
            state.halfling_poisoned_save_advantage
        ),
        format!(
            "goliathEscapeRollMode={}",
            roll_mode_ref(state.goliath_escape_roll_mode)
        ),
        format!(
            "goliathPoisonedEscapeRollMode={}",
            roll_mode_ref(state.goliath_poisoned_escape_roll_mode)
        ),
        format!("scenarioResult={}", scenario_ref(state.scenario_result)),
        format!("protocolResult={}", protocol_ref(state.protocol)),
        "protocolHoles=none".to_string(),
    ]
    .join("\n")
}

pub fn observed_passive_route_after_accepted_creature_space_movement() -> Vec<ReducerRouteEvent> {
    let mut trace = observed_passive_trace_after_ability_check_roll_mode();
    let state = start_standard_battle();
    let state = resolve_creature_space_movement(
        state,
        creature_space_traversal_allowed(accepted_creature_space_traversal()),
        &mut trace,
    );
    let _state = resolve_generic_route_only(
        state,
        BattleSubjectKind::CreatureSpaceMovementResourceSpend,
        BattleGenericRouteFill::WithoutFill,
        &mut trace,
    );
    route_events_from_trace(&trace)
}

pub fn observed_passive_route_after_rejected_creature_space_movement(
    rejected_count_after_acceptance: usize,
) -> Vec<ReducerRouteEvent> {
    let mut trace = observed_passive_trace_after_ability_check_roll_mode();
    let mut state = start_standard_battle();
    state = resolve_creature_space_movement(
        state,
        creature_space_traversal_allowed(accepted_creature_space_traversal()),
        &mut trace,
    );
    state = resolve_generic_route_only(
        state,
        BattleSubjectKind::CreatureSpaceMovementResourceSpend,
        BattleGenericRouteFill::WithoutFill,
        &mut trace,
    );
    for _ in 0..rejected_count_after_acceptance {
        state = resolve_creature_space_movement(
            state,
            creature_space_traversal_allowed(rejected_creature_space_traversal()),
            &mut trace,
        );
    }
    route_events_from_trace(&trace)
}

fn observed_passive_route_after_damage_adjustment() -> Vec<ReducerRouteEvent> {
    let mut trace = observed_passive_trace_after_creature_stat_projection();
    let state = start_standard_battle();
    let _state = discover_and_resolve_generic_route(
        state,
        BattleSubjectKind::PassiveDamageAdjustment,
        BattleGenericRouteFill::WithoutFill,
        &mut trace,
    );
    route_events_from_trace(&trace)
}

fn observed_passive_route_after_saving_throw_roll_mode() -> Vec<ReducerRouteEvent> {
    let mut trace = observed_passive_trace_after_damage_adjustment();
    let state = start_standard_battle();
    let _state = discover_and_resolve_generic_route(
        state,
        BattleSubjectKind::PassiveSavingThrowRollMode,
        BattleGenericRouteFill::SavingThrowOutcome,
        &mut trace,
    );
    route_events_from_trace(&trace)
}

fn observed_passive_route_after_ability_check_roll_mode() -> Vec<ReducerRouteEvent> {
    let trace = observed_passive_trace_after_ability_check_roll_mode();
    route_events_from_trace(&trace)
}

fn observed_passive_trace_after_ability_check_roll_mode() -> BattleEntrypointTrace {
    let mut trace = observed_passive_trace_after_saving_throw_roll_mode();
    let state = start_standard_battle();
    let _state = discover_and_resolve_generic_route(
        state,
        BattleSubjectKind::PassiveAbilityCheckRollMode,
        BattleGenericRouteFill::GrappleOutcome,
        &mut trace,
    );
    trace
}

fn observed_passive_trace_after_saving_throw_roll_mode() -> BattleEntrypointTrace {
    let mut trace = observed_passive_trace_after_damage_adjustment();
    let state = start_standard_battle();
    let _state = discover_and_resolve_generic_route(
        state,
        BattleSubjectKind::PassiveSavingThrowRollMode,
        BattleGenericRouteFill::SavingThrowOutcome,
        &mut trace,
    );
    trace
}

fn observed_passive_trace_after_damage_adjustment() -> BattleEntrypointTrace {
    let mut trace = observed_passive_trace_after_creature_stat_projection();
    let state = start_standard_battle();
    let _state = discover_and_resolve_generic_route(
        state,
        BattleSubjectKind::PassiveDamageAdjustment,
        BattleGenericRouteFill::WithoutFill,
        &mut trace,
    );
    trace
}

fn observed_passive_trace_after_creature_stat_projection() -> BattleEntrypointTrace {
    let mut trace = BattleEntrypointTrace::default();
    observe_battle_route_start(BattleReducerRouteOwnerGroup::CreatureState, &mut trace);
    let state = start_standard_battle();
    let state = discover_and_resolve_generic_route(
        state,
        BattleSubjectKind::CreatureStatProjection,
        BattleGenericRouteFill::WithoutFill,
        &mut trace,
    );
    let _state = resolve_generic_route_only(
        state,
        BattleSubjectKind::CreatureStatMovementResourceProjection,
        BattleGenericRouteFill::WithoutFill,
        &mut trace,
    );
    trace
}

fn resolve_creature_space_movement(
    state: BattleState,
    accepted: bool,
    trace: &mut BattleEntrypointTrace,
) -> BattleState {
    discover_and_resolve_generic_route(
        state,
        BattleSubjectKind::CreatureSpaceMovementPermission,
        BattleGenericRouteFill::Movement { accepted },
        trace,
    )
}

fn discover_and_resolve_generic_route(
    state: BattleState,
    kind: BattleSubjectKind,
    fill: BattleGenericRouteFill,
    trace: &mut BattleEntrypointTrace,
) -> BattleState {
    let (state, subject) = discover_generic_route_subject_observed(state, kind, trace);
    let request = BattleResolutionRequest::generic_route(subject, fill)
        .expect("generic route subject kind must match fill request");
    resolve_battle_subject_observed(state, request, trace).into_state()
}

fn resolve_generic_route_only(
    state: BattleState,
    kind: BattleSubjectKind,
    fill: BattleGenericRouteFill,
    trace: &mut BattleEntrypointTrace,
) -> BattleState {
    let subject = generic_route_subject_for_current_actor(&state, kind);
    let request = BattleResolutionRequest::generic_route(subject, fill)
        .expect("generic route subject kind must match fill request");
    resolve_battle_subject_observed(state, request, trace).into_state()
}

fn route_events_from_trace(trace: &BattleEntrypointTrace) -> Vec<ReducerRouteEvent> {
    trace
        .route_events()
        .iter()
        .map(reducer_route_event_from_observed)
        .collect()
}

pub fn passive_route_after_ability_check_roll_mode() -> Vec<ReducerRouteEvent> {
    let mut route = passive_route_after_saving_throw_roll_mode();
    append_ability_check_roll_mode(&mut route);
    route
}

pub fn passive_route_after_accepted_creature_space_movement() -> Vec<ReducerRouteEvent> {
    let mut route = passive_route_after_ability_check_roll_mode();
    append_accepted_creature_space_movement(&mut route);
    route
}

pub fn passive_route_after_rejected_creature_space_movement(
    rejected_count_after_acceptance: usize,
) -> Vec<ReducerRouteEvent> {
    let mut route = passive_route_after_accepted_creature_space_movement();
    for _ in 0..rejected_count_after_acceptance {
        append_rejected_creature_space_movement(&mut route);
    }
    route
}

fn passive_route_after_damage_adjustment() -> Vec<ReducerRouteEvent> {
    let mut route = passive_route_after_creature_stat_projection();
    append_damage_adjustment(&mut route);
    route
}

fn passive_route_after_saving_throw_roll_mode() -> Vec<ReducerRouteEvent> {
    let mut route = passive_route_after_damage_adjustment();
    append_saving_throw_roll_mode(&mut route);
    route
}

fn passive_route_after_creature_stat_projection() -> Vec<ReducerRouteEvent> {
    let mut route = vec![route_start_battle(ReducerRouteOwnerGroup::CreatureState)];
    route.extend([
        route_discover_battle_acts_from_route_holes(
            ReducerRouteSubjectFamily::CreatureStatProjection,
            Vec::new(),
            ReducerRouteOwnerGroup::CreatureState,
        ),
        route_resolve_battle_subject_without_fill_from_route_result(
            ReducerRouteSubjectFamily::CreatureStatProjection,
            ReducerRouteResolutionOutcome::Resolved,
            Vec::new(),
            ReducerRouteOwnerGroup::CreatureState,
        ),
        route_resolve_battle_subject_without_fill_from_route_result(
            ReducerRouteSubjectFamily::CreatureStatProjection,
            ReducerRouteResolutionOutcome::Resolved,
            Vec::new(),
            ReducerRouteOwnerGroup::MovementResource,
        ),
    ]);
    route
}

fn append_damage_adjustment(route: &mut Vec<ReducerRouteEvent>) {
    route.extend([
        route_discover_battle_acts_from_route_holes(
            ReducerRouteSubjectFamily::PassiveDamageAdjustment,
            Vec::new(),
            ReducerRouteOwnerGroup::DamageAdjustment,
        ),
        route_resolve_battle_subject_without_fill_from_route_result(
            ReducerRouteSubjectFamily::PassiveDamageAdjustment,
            ReducerRouteResolutionOutcome::Resolved,
            Vec::new(),
            ReducerRouteOwnerGroup::DamageAdjustment,
        ),
    ]);
}

fn append_saving_throw_roll_mode(route: &mut Vec<ReducerRouteEvent>) {
    route.extend([
        route_discover_battle_acts_from_route_holes(
            ReducerRouteSubjectFamily::PassiveSavingThrowRollMode,
            vec![ReducerRouteHoleKind::SavingThrowOutcome],
            ReducerRouteOwnerGroup::SavingThrowRollMode,
        ),
        route_resolve_battle_subject_from_route_result(
            ReducerRouteSubjectFamily::PassiveSavingThrowRollMode,
            ReducerRouteFillKind::SavingThrowOutcome,
            ReducerRouteResolutionOutcome::Resolved,
            Vec::new(),
            ReducerRouteOwnerGroup::SavingThrowRollMode,
        ),
    ]);
}

fn append_ability_check_roll_mode(route: &mut Vec<ReducerRouteEvent>) {
    route.extend([
        route_discover_battle_acts_from_route_holes(
            ReducerRouteSubjectFamily::PassiveAbilityCheckRollMode,
            vec![ReducerRouteHoleKind::GrappleOutcome],
            ReducerRouteOwnerGroup::AbilityCheckRollMode,
        ),
        route_resolve_battle_subject_from_route_result(
            ReducerRouteSubjectFamily::PassiveAbilityCheckRollMode,
            ReducerRouteFillKind::GrappleOutcome,
            ReducerRouteResolutionOutcome::Resolved,
            Vec::new(),
            ReducerRouteOwnerGroup::AbilityCheckRollMode,
        ),
    ]);
}

fn append_accepted_creature_space_movement(route: &mut Vec<ReducerRouteEvent>) {
    route.extend([
        route_discover_battle_acts_from_route_holes(
            ReducerRouteSubjectFamily::CreatureSpaceMovementPermission,
            vec![ReducerRouteHoleKind::Movement],
            ReducerRouteOwnerGroup::CreatureSpaceMovement,
        ),
        route_resolve_battle_subject_from_route_result(
            ReducerRouteSubjectFamily::CreatureSpaceMovementPermission,
            ReducerRouteFillKind::Movement,
            ReducerRouteResolutionOutcome::Resolved,
            Vec::new(),
            ReducerRouteOwnerGroup::CreatureSpaceMovement,
        ),
        route_resolve_battle_subject_without_fill_from_route_result(
            ReducerRouteSubjectFamily::CreatureSpaceMovementPermission,
            ReducerRouteResolutionOutcome::Resolved,
            Vec::new(),
            ReducerRouteOwnerGroup::MovementResource,
        ),
    ]);
}

fn append_rejected_creature_space_movement(route: &mut Vec<ReducerRouteEvent>) {
    route.extend([
        route_discover_battle_acts_from_route_holes(
            ReducerRouteSubjectFamily::CreatureSpaceMovementPermission,
            vec![ReducerRouteHoleKind::Movement],
            ReducerRouteOwnerGroup::CreatureSpaceMovement,
        ),
        route_resolve_battle_subject_from_route_result(
            ReducerRouteSubjectFamily::CreatureSpaceMovementPermission,
            ReducerRouteFillKind::Movement,
            ReducerRouteResolutionOutcome::NeedsHoles,
            vec![ReducerRouteHoleKind::Movement],
            ReducerRouteOwnerGroup::CreatureSpaceMovement,
        ),
    ]);
}

fn roll_mode_ref(mode: SpeciesPassiveRollMode) -> &'static str {
    match mode {
        SpeciesPassiveRollMode::Normal => "normal",
        SpeciesPassiveRollMode::Advantage => "advantage",
    }
}

fn scenario_ref(result: SpeciesPassiveScenarioResult) -> &'static str {
    match result {
        SpeciesPassiveScenarioResult::Init => "init",
        SpeciesPassiveScenarioResult::DragonbornDamageResistance => "dragonbornDamageResistance",
        SpeciesPassiveScenarioResult::DwarvenResilience => "dwarvenResilience",
        SpeciesPassiveScenarioResult::HalflingBrave => "halflingBrave",
        SpeciesPassiveScenarioResult::GoliathPowerfulBuild => "goliathPowerfulBuild",
    }
}

fn protocol_ref(protocol: SpeciesPassiveProtocol) -> &'static str {
    match protocol {
        SpeciesPassiveProtocol::Init => "init",
        SpeciesPassiveProtocol::Resolved => "resolved",
    }
}

fn accepted_creature_space_traversal() -> CreatureSpaceTraversalFacts {
    CreatureSpaceTraversalFacts {
        permission: Some(CreatureSpaceTraversalPermission::LargerCreatureSpaceNoStop),
        mover_size_rank: 1,
        occupied_creature_size_rank: 2,
        stops_in_occupied_space: false,
    }
}

fn rejected_creature_space_traversal() -> CreatureSpaceTraversalFacts {
    CreatureSpaceTraversalFacts {
        permission: Some(CreatureSpaceTraversalPermission::LargerCreatureSpaceNoStop),
        mover_size_rank: 1,
        occupied_creature_size_rank: 1,
        stops_in_occupied_space: false,
    }
}

fn initial_expected() -> SpeciesPassiveTraitState {
    SpeciesPassiveTraitState {
        dragonborn_fire_damage_after: 9,
        dragonborn_cold_damage_after: 9,
        dwarf_poison_damage_after: 9,
        dwarf_fire_damage_after: 9,
        dwarf_poisoned_save_advantage: false,
        dwarf_charmed_save_advantage: false,
        halfling_frightened_avoid_save_advantage: false,
        halfling_frightened_end_save_advantage: false,
        halfling_poisoned_save_advantage: false,
        goliath_escape_roll_mode: SpeciesPassiveRollMode::Normal,
        goliath_poisoned_escape_roll_mode: SpeciesPassiveRollMode::Normal,
        scenario_result: SpeciesPassiveScenarioResult::Init,
        protocol: SpeciesPassiveProtocol::Init,
    }
}
