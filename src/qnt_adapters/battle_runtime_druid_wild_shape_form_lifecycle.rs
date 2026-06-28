use crate::rules::wild_shape::{
    assume_riding_horse_wild_shape, begin_wild_shape_next_turn, dismiss_wild_shape_form,
    reuse_wild_shape_as_cat, revert_wild_shape_due_to_death,
    revert_wild_shape_due_to_incapacitated, stutter_wild_shape_state, wild_shape_initial_state,
    WildShapeCreatureSize, WildShapeDruidStatus, WildShapeForm, WildShapeProtocol,
    WildShapeScenarioOutcome, WildShapeState,
};

use super::battle_runtime_reducer_route::{
    route_discover_battle_acts_from_route_holes, route_resolve_battle_subject_from_route_holes,
    route_resolve_battle_subject_without_fill_from_route_holes, route_start_battle,
    ReducerRouteEvent, ReducerRouteFillKind, ReducerRouteHoleKind, ReducerRouteOwnerGroup,
    ReducerRouteResolutionOutcome, ReducerRouteSubjectFamily,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DruidWildShapeFormLifecycleWitness {
    pub active_form: &'static str,
    pub bonus_action_available: bool,
    pub uses_remaining: i16,
    pub temporary_hit_points: i16,
    pub armor_class: i16,
    pub creature_size: &'static str,
    pub speed_feet: i16,
    pub shove_dc: i16,
    pub spell_available: bool,
    pub active_form_effect_count: i16,
    pub merged_equipment_count: i16,
    pub druid_status: &'static str,
    pub scenario_outcome: &'static str,
    pub protocol_result: &'static str,
    pub protocol_invalid_reason: &'static str,
    pub protocol_holes: Vec<&'static str>,
}

pub const BRANCH_ACTIONS: [&str; 7] = [
    "doAssumeRidingHorse",
    "doBeginNextTurn",
    "doDeathReversion",
    "doDismissForm",
    "doIncapacitatedReversion",
    "doReuseAsCat",
    "doStutter",
];

pub fn replay_observed_action(observed_action_taken: &str) -> DruidWildShapeFormLifecycleWitness {
    match observed_action_taken {
        "doAssumeRidingHorse" => witness_from_state(assumed_riding_horse()),
        "doBeginNextTurn" => witness_from_state(next_turn()),
        "doDeathReversion" => witness_from_state(death_reversion()),
        "doDismissForm" => witness_from_state(dismissed()),
        "doIncapacitatedReversion" => witness_from_state(incapacitated_reversion()),
        "doReuseAsCat" => witness_from_state(reused_cat()),
        "doStutter" => witness_from_state(stuttered_after_death()),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> DruidWildShapeFormLifecycleWitness {
    match observed_action_taken {
        "doAssumeRidingHorse" => expected_assumed_riding_horse(),
        "doBeginNextTurn" => expected_next_turn(),
        "doDeathReversion" => expected_death_reversion(),
        "doDismissForm" => expected_dismissed(),
        "doIncapacitatedReversion" => expected_incapacitated_reversion(),
        "doReuseAsCat" => expected_reused_cat(),
        "doStutter" => expected_death_reversion(),
        action => panic!("unsupported expected mbt::actionTaken {action}"),
    }
}

pub fn projection_payload(witness: &DruidWildShapeFormLifecycleWitness) -> String {
    [
        format!("qActiveForm={}", witness.active_form),
        format!("qBonusActionAvailable={}", witness.bonus_action_available),
        format!("qUsesRemaining={}", witness.uses_remaining),
        format!("qTempHp={}", witness.temporary_hit_points),
        format!("qArmorClass={}", witness.armor_class),
        format!("qCreatureSize={}", witness.creature_size),
        format!("qSpeedFeet={}", witness.speed_feet),
        format!("qShoveDc={}", witness.shove_dc),
        format!("qSpellAvailable={}", witness.spell_available),
        format!(
            "qActiveFormEffectCount={}",
            witness.active_form_effect_count
        ),
        format!("qMergedEquipmentCount={}", witness.merged_equipment_count),
        format!("qDruidStatus={}", witness.druid_status),
        format!("qScenarioOutcome={}", witness.scenario_outcome),
        format!("protocolResult={}", witness.protocol_result),
        format!("protocolInvalidReason={}", witness.protocol_invalid_reason),
        format!("protocolHoles={}", joined_or_none(&witness.protocol_holes)),
    ]
    .join("\n")
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doAssumeRidingHorse" => route_after_assume_form(),
        "doBeginNextTurn" => route_after_begin_next_turn(),
        "doDeathReversion" => route_after_death_reversion(),
        "doDismissForm" => route_after_dismiss_form(),
        "doIncapacitatedReversion" => route_after_incapacitated_reversion(),
        "doReuseAsCat" => route_after_reuse_form(),
        "doStutter" => route_after_death_reversion(),
        action => panic!("unsupported route mbt::actionTaken {action}"),
    }
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doAssumeRidingHorse" => expected_route_after_assume_form(),
        "doBeginNextTurn" => expected_route_after_begin_next_turn(),
        "doDeathReversion" => expected_route_after_death_reversion(),
        "doDismissForm" => expected_route_after_dismiss_form(),
        "doIncapacitatedReversion" => expected_route_after_incapacitated_reversion(),
        "doReuseAsCat" => expected_route_after_reuse_form(),
        "doStutter" => expected_route_after_death_reversion(),
        action => panic!("unsupported expected route mbt::actionTaken {action}"),
    }
}

fn assumed_riding_horse() -> WildShapeState {
    assume_riding_horse_wild_shape(wild_shape_initial_state())
}

fn next_turn() -> WildShapeState {
    begin_wild_shape_next_turn(assumed_riding_horse())
}

fn reused_cat() -> WildShapeState {
    reuse_wild_shape_as_cat(next_turn())
}

fn dismissed() -> WildShapeState {
    dismiss_wild_shape_form(next_turn())
}

fn incapacitated_reversion() -> WildShapeState {
    revert_wild_shape_due_to_incapacitated(assumed_riding_horse())
}

fn death_reversion() -> WildShapeState {
    revert_wild_shape_due_to_death(assumed_riding_horse())
}

fn stuttered_after_death() -> WildShapeState {
    stutter_wild_shape_state(death_reversion())
}

fn expected_assumed_riding_horse() -> DruidWildShapeFormLifecycleWitness {
    DruidWildShapeFormLifecycleWitness {
        active_form: "ridingHorse",
        bonus_action_available: false,
        uses_remaining: 1,
        temporary_hit_points: 2,
        armor_class: 11,
        creature_size: "large",
        speed_feet: 60,
        shove_dc: 13,
        spell_available: false,
        active_form_effect_count: 1,
        merged_equipment_count: 2,
        druid_status: "able",
        scenario_outcome: "AssumedRidingHorse",
        protocol_result: "resolved",
        protocol_invalid_reason: "",
        protocol_holes: vec![],
    }
}

fn expected_next_turn() -> DruidWildShapeFormLifecycleWitness {
    DruidWildShapeFormLifecycleWitness {
        bonus_action_available: true,
        scenario_outcome: "NextTurn",
        ..expected_assumed_riding_horse()
    }
}

fn expected_reused_cat() -> DruidWildShapeFormLifecycleWitness {
    DruidWildShapeFormLifecycleWitness {
        active_form: "cat",
        bonus_action_available: false,
        uses_remaining: 0,
        armor_class: 12,
        creature_size: "tiny",
        speed_feet: 40,
        shove_dc: 6,
        scenario_outcome: "ReusedCat",
        ..expected_assumed_riding_horse()
    }
}

fn expected_dismissed() -> DruidWildShapeFormLifecycleWitness {
    DruidWildShapeFormLifecycleWitness {
        active_form: "trueForm",
        bonus_action_available: false,
        armor_class: 16,
        creature_size: "medium",
        speed_feet: 30,
        shove_dc: 13,
        spell_available: true,
        active_form_effect_count: 0,
        merged_equipment_count: 0,
        scenario_outcome: "Dismissed",
        ..expected_next_turn()
    }
}

fn expected_incapacitated_reversion() -> DruidWildShapeFormLifecycleWitness {
    DruidWildShapeFormLifecycleWitness {
        active_form: "trueForm",
        armor_class: 16,
        creature_size: "medium",
        speed_feet: 30,
        shove_dc: 13,
        spell_available: false,
        active_form_effect_count: 0,
        merged_equipment_count: 0,
        druid_status: "incapacitated",
        scenario_outcome: "FormIncapacitated",
        ..expected_assumed_riding_horse()
    }
}

fn expected_death_reversion() -> DruidWildShapeFormLifecycleWitness {
    DruidWildShapeFormLifecycleWitness {
        active_form: "trueForm",
        armor_class: 16,
        creature_size: "medium",
        speed_feet: 30,
        shove_dc: 13,
        spell_available: false,
        active_form_effect_count: 0,
        merged_equipment_count: 0,
        druid_status: "dead",
        scenario_outcome: "FormDead",
        ..expected_assumed_riding_horse()
    }
}

fn route_after_assume_form() -> Vec<ReducerRouteEvent> {
    let mut route = initial_route();
    append_assume_or_reuse_form_route(&mut route);
    route
}

fn route_after_begin_next_turn() -> Vec<ReducerRouteEvent> {
    let mut route = route_after_assume_form();
    route.push(discover_active_form(
        Vec::new(),
        ReducerRouteOwnerGroup::TurnBoundary,
    ));
    route.push(resolve_active_form_without_fill(
        ReducerRouteOwnerGroup::TurnBoundary,
    ));
    route.push(resolve_active_form_without_fill(
        ReducerRouteOwnerGroup::ActionEconomy,
    ));
    route
}

fn route_after_reuse_form() -> Vec<ReducerRouteEvent> {
    let mut route = route_after_begin_next_turn();
    append_assume_or_reuse_form_route(&mut route);
    route
}

fn route_after_dismiss_form() -> Vec<ReducerRouteEvent> {
    let mut route = route_after_begin_next_turn();
    route.push(discover_active_form(
        Vec::new(),
        ReducerRouteOwnerGroup::ActionEconomy,
    ));
    route.push(resolve_active_form_without_fill(
        ReducerRouteOwnerGroup::ActionEconomy,
    ));
    route.push(resolve_active_form_without_fill(
        ReducerRouteOwnerGroup::ActiveEffect,
    ));
    route.push(resolve_active_form_without_fill(
        ReducerRouteOwnerGroup::CreatureState,
    ));
    route.push(resolve_active_form_without_fill(
        ReducerRouteOwnerGroup::MovementResource,
    ));
    route
}

fn route_after_incapacitated_reversion() -> Vec<ReducerRouteEvent> {
    let mut route = route_after_assume_form();
    route.push(discover_active_form(
        Vec::new(),
        ReducerRouteOwnerGroup::ConditionLifecycle,
    ));
    route.push(resolve_active_form_without_fill(
        ReducerRouteOwnerGroup::ConditionLifecycle,
    ));
    route.push(resolve_active_form_without_fill(
        ReducerRouteOwnerGroup::ActiveEffect,
    ));
    route.push(resolve_active_form_without_fill(
        ReducerRouteOwnerGroup::CreatureState,
    ));
    route.push(resolve_active_form_without_fill(
        ReducerRouteOwnerGroup::MovementResource,
    ));
    route
}

fn route_after_death_reversion() -> Vec<ReducerRouteEvent> {
    let mut route = route_after_assume_form();
    route.push(discover_active_form(
        Vec::new(),
        ReducerRouteOwnerGroup::HitPointAndZeroHpLifecycle,
    ));
    route.push(resolve_active_form_without_fill(
        ReducerRouteOwnerGroup::HitPointAndZeroHpLifecycle,
    ));
    route.push(resolve_active_form_without_fill(
        ReducerRouteOwnerGroup::ActiveEffect,
    ));
    route.push(resolve_active_form_without_fill(
        ReducerRouteOwnerGroup::CreatureState,
    ));
    route.push(resolve_active_form_without_fill(
        ReducerRouteOwnerGroup::MovementResource,
    ));
    route
}

fn initial_route() -> Vec<ReducerRouteEvent> {
    vec![route_start_battle(ReducerRouteOwnerGroup::ActionEconomy)]
}

fn append_assume_or_reuse_form_route(route: &mut Vec<ReducerRouteEvent>) {
    route.push(discover_active_form(
        vec![ReducerRouteHoleKind::WildShapeEquipmentDisposition],
        ReducerRouteOwnerGroup::ActionEconomy,
    ));
    route.push(route_resolve_battle_subject_from_route_holes(
        ReducerRouteSubjectFamily::ActiveFormLifecycle,
        ReducerRouteFillKind::WildShapeEquipmentDisposition,
        Vec::new(),
        ReducerRouteOwnerGroup::ActionEconomy,
    ));
    route.push(resolve_active_form_without_fill(
        ReducerRouteOwnerGroup::FeatureResource,
    ));
    route.push(resolve_active_form_without_fill(
        ReducerRouteOwnerGroup::TemporaryHitPoint,
    ));
    route.push(resolve_active_form_without_fill(
        ReducerRouteOwnerGroup::ActiveEffect,
    ));
    route.push(resolve_active_form_without_fill(
        ReducerRouteOwnerGroup::CreatureState,
    ));
    route.push(resolve_active_form_without_fill(
        ReducerRouteOwnerGroup::MovementResource,
    ));
}

fn discover_active_form(
    holes: Vec<ReducerRouteHoleKind>,
    owner: ReducerRouteOwnerGroup,
) -> ReducerRouteEvent {
    route_discover_battle_acts_from_route_holes(
        ReducerRouteSubjectFamily::ActiveFormLifecycle,
        holes,
        owner,
    )
}

fn resolve_active_form_without_fill(owner: ReducerRouteOwnerGroup) -> ReducerRouteEvent {
    route_resolve_battle_subject_without_fill_from_route_holes(
        ReducerRouteSubjectFamily::ActiveFormLifecycle,
        Vec::new(),
        owner,
    )
}

fn expected_route_after_assume_form() -> Vec<ReducerRouteEvent> {
    let mut route = expected_initial_route();
    append_expected_assume_or_reuse_form_route(&mut route);
    route
}

fn expected_route_after_begin_next_turn() -> Vec<ReducerRouteEvent> {
    let mut route = expected_route_after_assume_form();
    route.push(expected_discover_active_form(
        Vec::new(),
        ReducerRouteOwnerGroup::TurnBoundary,
    ));
    route.push(expected_resolve_active_form_without_fill(
        ReducerRouteOwnerGroup::TurnBoundary,
    ));
    route.push(expected_resolve_active_form_without_fill(
        ReducerRouteOwnerGroup::ActionEconomy,
    ));
    route
}

fn expected_route_after_reuse_form() -> Vec<ReducerRouteEvent> {
    let mut route = expected_route_after_begin_next_turn();
    append_expected_assume_or_reuse_form_route(&mut route);
    route
}

fn expected_route_after_dismiss_form() -> Vec<ReducerRouteEvent> {
    let mut route = expected_route_after_begin_next_turn();
    route.push(expected_discover_active_form(
        Vec::new(),
        ReducerRouteOwnerGroup::ActionEconomy,
    ));
    route.push(expected_resolve_active_form_without_fill(
        ReducerRouteOwnerGroup::ActionEconomy,
    ));
    route.push(expected_resolve_active_form_without_fill(
        ReducerRouteOwnerGroup::ActiveEffect,
    ));
    route.push(expected_resolve_active_form_without_fill(
        ReducerRouteOwnerGroup::CreatureState,
    ));
    route.push(expected_resolve_active_form_without_fill(
        ReducerRouteOwnerGroup::MovementResource,
    ));
    route
}

fn expected_route_after_incapacitated_reversion() -> Vec<ReducerRouteEvent> {
    let mut route = expected_route_after_assume_form();
    route.push(expected_discover_active_form(
        Vec::new(),
        ReducerRouteOwnerGroup::ConditionLifecycle,
    ));
    route.push(expected_resolve_active_form_without_fill(
        ReducerRouteOwnerGroup::ConditionLifecycle,
    ));
    route.push(expected_resolve_active_form_without_fill(
        ReducerRouteOwnerGroup::ActiveEffect,
    ));
    route.push(expected_resolve_active_form_without_fill(
        ReducerRouteOwnerGroup::CreatureState,
    ));
    route.push(expected_resolve_active_form_without_fill(
        ReducerRouteOwnerGroup::MovementResource,
    ));
    route
}

fn expected_route_after_death_reversion() -> Vec<ReducerRouteEvent> {
    let mut route = expected_route_after_assume_form();
    route.push(expected_discover_active_form(
        Vec::new(),
        ReducerRouteOwnerGroup::HitPointAndZeroHpLifecycle,
    ));
    route.push(expected_resolve_active_form_without_fill(
        ReducerRouteOwnerGroup::HitPointAndZeroHpLifecycle,
    ));
    route.push(expected_resolve_active_form_without_fill(
        ReducerRouteOwnerGroup::ActiveEffect,
    ));
    route.push(expected_resolve_active_form_without_fill(
        ReducerRouteOwnerGroup::CreatureState,
    ));
    route.push(expected_resolve_active_form_without_fill(
        ReducerRouteOwnerGroup::MovementResource,
    ));
    route
}

fn expected_initial_route() -> Vec<ReducerRouteEvent> {
    vec![ReducerRouteEvent::StartBattle {
        owner: ReducerRouteOwnerGroup::ActionEconomy,
    }]
}

fn append_expected_assume_or_reuse_form_route(route: &mut Vec<ReducerRouteEvent>) {
    route.push(expected_discover_active_form(
        vec![ReducerRouteHoleKind::WildShapeEquipmentDisposition],
        ReducerRouteOwnerGroup::ActionEconomy,
    ));
    route.push(ReducerRouteEvent::ResolveBattleSubject {
        subject: ReducerRouteSubjectFamily::ActiveFormLifecycle,
        fill: super::battle_runtime_reducer_route::ReducerRouteFillEvidence::FillKind(
            ReducerRouteFillKind::WildShapeEquipmentDisposition,
        ),
        outcome: ReducerRouteResolutionOutcome::Resolved,
        holes: Vec::new(),
        owner: ReducerRouteOwnerGroup::ActionEconomy,
    });
    route.push(expected_resolve_active_form_without_fill(
        ReducerRouteOwnerGroup::FeatureResource,
    ));
    route.push(expected_resolve_active_form_without_fill(
        ReducerRouteOwnerGroup::TemporaryHitPoint,
    ));
    route.push(expected_resolve_active_form_without_fill(
        ReducerRouteOwnerGroup::ActiveEffect,
    ));
    route.push(expected_resolve_active_form_without_fill(
        ReducerRouteOwnerGroup::CreatureState,
    ));
    route.push(expected_resolve_active_form_without_fill(
        ReducerRouteOwnerGroup::MovementResource,
    ));
}

fn expected_discover_active_form(
    holes: Vec<ReducerRouteHoleKind>,
    owner: ReducerRouteOwnerGroup,
) -> ReducerRouteEvent {
    ReducerRouteEvent::DiscoverBattleActs {
        subject: ReducerRouteSubjectFamily::ActiveFormLifecycle,
        holes,
        owner,
    }
}

fn expected_resolve_active_form_without_fill(owner: ReducerRouteOwnerGroup) -> ReducerRouteEvent {
    ReducerRouteEvent::ResolveBattleSubjectWithoutFill {
        subject: ReducerRouteSubjectFamily::ActiveFormLifecycle,
        outcome: ReducerRouteResolutionOutcome::Resolved,
        holes: Vec::new(),
        owner,
    }
}

fn witness_from_state(state: WildShapeState) -> DruidWildShapeFormLifecycleWitness {
    DruidWildShapeFormLifecycleWitness {
        active_form: active_form_ref(state.active_form),
        bonus_action_available: state.bonus_action_available,
        uses_remaining: state.uses_remaining,
        temporary_hit_points: state.temporary_hit_points,
        armor_class: state.armor_class,
        creature_size: creature_size_ref(state.creature_size),
        speed_feet: state.speed_feet,
        shove_dc: state.shove_dc,
        spell_available: state.spell_available,
        active_form_effect_count: state.active_form_effect_count,
        merged_equipment_count: state.merged_equipment_count,
        druid_status: druid_status_ref(state.druid_status),
        scenario_outcome: scenario_outcome_ref(state.scenario_outcome),
        protocol_result: protocol_result_ref(state.protocol),
        protocol_invalid_reason: "",
        protocol_holes: protocol_holes(state.protocol),
    }
}

fn active_form_ref(form: WildShapeForm) -> &'static str {
    match form {
        WildShapeForm::TrueForm => "trueForm",
        WildShapeForm::RidingHorse => "ridingHorse",
        WildShapeForm::Cat => "cat",
    }
}

fn creature_size_ref(size: WildShapeCreatureSize) -> &'static str {
    match size {
        WildShapeCreatureSize::Tiny => "tiny",
        WildShapeCreatureSize::Small => "small",
        WildShapeCreatureSize::Medium => "medium",
        WildShapeCreatureSize::Large => "large",
        WildShapeCreatureSize::Huge => "huge",
        WildShapeCreatureSize::Gargantuan => "gargantuan",
    }
}

fn druid_status_ref(status: WildShapeDruidStatus) -> &'static str {
    match status {
        WildShapeDruidStatus::Able => "able",
        WildShapeDruidStatus::Incapacitated => "incapacitated",
        WildShapeDruidStatus::Dead => "dead",
    }
}

fn scenario_outcome_ref(outcome: WildShapeScenarioOutcome) -> &'static str {
    match outcome {
        WildShapeScenarioOutcome::Init => "Init",
        WildShapeScenarioOutcome::AssumedRidingHorse => "AssumedRidingHorse",
        WildShapeScenarioOutcome::NextTurn => "NextTurn",
        WildShapeScenarioOutcome::ReusedCat => "ReusedCat",
        WildShapeScenarioOutcome::Dismissed => "Dismissed",
        WildShapeScenarioOutcome::FormIncapacitated => "FormIncapacitated",
        WildShapeScenarioOutcome::FormDead => "FormDead",
    }
}

fn protocol_result_ref(protocol: WildShapeProtocol) -> &'static str {
    match protocol {
        WildShapeProtocol::Init => "init",
        WildShapeProtocol::Resolved => "resolved",
    }
}

fn protocol_holes(protocol: WildShapeProtocol) -> Vec<&'static str> {
    match protocol {
        WildShapeProtocol::Init => vec!["WitnessProtocolHole"],
        WildShapeProtocol::Resolved => vec![],
    }
}

fn joined_or_none(values: &[&'static str]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}
