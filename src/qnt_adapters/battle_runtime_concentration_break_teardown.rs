use crate::rules::battle_reducer_spine::{
    discover_battle_acts, resolve_battle_subject, start_fighter_skeleton_battle,
    BattleConcentrationFill, BattleFill, BattleHoleKind, BattleResolutionResult, BattleState,
    BattleSubject, BattleSubjectKind,
};
use crate::rules::concentration::{
    cast_concentration_spell, cast_replacement_concentration_spell, concentration_damage_total,
    concentration_initial_state, fail_concentration_saving_throw,
    request_concentration_save_after_damage, voluntarily_end_concentration,
    ConcentrationBreakScenario, ConcentrationDamageFacts, ConcentrationProtocol,
    ConcentrationSavingThrowFacts, ConcentrationState,
};

use super::battle_runtime_reducer_route::{
    route_discover_battle_acts, route_resolve_battle_subject,
    route_resolve_battle_subject_without_fill, route_start_battle, ReducerRouteEvent,
    ReducerRouteFillKind, ReducerRouteOwnerGroup, ReducerRouteSubjectFamily,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConcentrationBreakTeardownWitness {
    pub scenario: &'static str,
    pub damage_taken: i16,
    pub save_dc: i16,
    pub save_roll_total: i16,
    pub concentration_save_offered: bool,
    pub caster_concentrating: bool,
    pub blurred_effect_count: i16,
    pub spell_slot_expended: i16,
    pub teardown_before_next_command: bool,
    pub replacement_started_after_teardown: bool,
    pub protocol_result: &'static str,
    pub protocol_holes: Vec<&'static str>,
}

pub const BRANCH_ACTIONS: [&str; 5] = [
    "doCastConcentrationSpell",
    "doCastReplacementConcentrationSpell",
    "doDamageRequestsConcentrationSave",
    "doFailConcentrationSave",
    "doVoluntaryEndConcentration",
];

pub fn replay_observed_action(observed_action_taken: &str) -> ConcentrationBreakTeardownWitness {
    match observed_action_taken {
        "doCastConcentrationSpell" => {
            witness_from_battle_state(&cast_concentration_spell_route().0)
        }
        "doCastReplacementConcentrationSpell" => {
            witness_from_battle_state(&cast_replacement_concentration_spell_route().0)
        }
        "doDamageRequestsConcentrationSave" => {
            witness_from_battle_state(&damage_requests_concentration_save_route().0)
        }
        "doFailConcentrationSave" => witness_from_battle_state(&fail_concentration_save_route().0),
        "doVoluntaryEndConcentration" => {
            witness_from_battle_state(&voluntary_end_concentration_route().0)
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> ConcentrationBreakTeardownWitness {
    match observed_action_taken {
        "doCastConcentrationSpell" => witness_from_state(cast_from_initial()),
        "doCastReplacementConcentrationSpell" => {
            witness_from_state(cast_replacement_from_initial())
        }
        "doDamageRequestsConcentrationSave" => witness_from_state(request_save_from_damage()),
        "doFailConcentrationSave" => witness_from_state(fail_requested_save()),
        "doVoluntaryEndConcentration" => witness_from_state(end_voluntarily_from_initial()),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doCastConcentrationSpell" => cast_concentration_spell_route().1,
        "doCastReplacementConcentrationSpell" => cast_replacement_concentration_spell_route().1,
        "doDamageRequestsConcentrationSave" => damage_requests_concentration_save_route().1,
        "doFailConcentrationSave" => fail_concentration_save_route().1,
        "doVoluntaryEndConcentration" => voluntary_end_concentration_route().1,
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doCastConcentrationSpell" | "doCastReplacementConcentrationSpell" => {
            concentration_cast_route()
        }
        "doDamageRequestsConcentrationSave" => {
            let mut route = concentration_cast_route();
            route.push(route_resolve_battle_subject(
                ReducerRouteSubjectFamily::ConcentrationTeardown,
                ReducerRouteFillKind::RolledDice,
                vec![BattleHoleKind::ConcentrationSavingThrow],
                ReducerRouteOwnerGroup::Concentration,
            ));
            route
        }
        "doFailConcentrationSave" => {
            let mut route = expected_route("doDamageRequestsConcentrationSave");
            route.push(route_resolve_battle_subject(
                ReducerRouteSubjectFamily::ConcentrationTeardown,
                ReducerRouteFillKind::ConcentrationSavingThrow,
                Vec::new(),
                ReducerRouteOwnerGroup::Concentration,
            ));
            route
        }
        "doVoluntaryEndConcentration" => {
            let mut route = concentration_cast_route();
            route.push(route_resolve_battle_subject_without_fill(
                ReducerRouteSubjectFamily::ConcentrationTeardown,
                Vec::new(),
                ReducerRouteOwnerGroup::Concentration,
            ));
            route
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn projection_payload(witness: &ConcentrationBreakTeardownWitness) -> String {
    [
        format!("scenario={}", witness.scenario),
        format!("damageTaken={}", witness.damage_taken),
        format!("saveDc={}", witness.save_dc),
        format!("saveRollTotal={}", witness.save_roll_total),
        format!(
            "concentrationSaveOffered={}",
            witness.concentration_save_offered
        ),
        format!("casterConcentrating={}", witness.caster_concentrating),
        format!("blurredEffectCount={}", witness.blurred_effect_count),
        format!("spellSlotExpended={}", witness.spell_slot_expended),
        format!(
            "teardownBeforeNextCommand={}",
            witness.teardown_before_next_command
        ),
        format!(
            "replacementStartedAfterTeardown={}",
            witness.replacement_started_after_teardown
        ),
        format!("protocolResult={}", witness.protocol_result),
        format!("protocolHoles={}", joined_or_none(&witness.protocol_holes)),
    ]
    .join("\n")
}

fn cast_from_initial() -> ConcentrationState {
    cast_concentration_spell(concentration_initial_state())
}

fn request_save_from_damage() -> ConcentrationState {
    request_concentration_save_after_damage(cast_from_initial(), sampled_damage_facts())
}

fn fail_requested_save() -> ConcentrationState {
    fail_concentration_saving_throw(
        request_save_from_damage(),
        ConcentrationSavingThrowFacts {
            saving_throw_total: 9,
        },
    )
    .expect("QNT sampled saving throw total fails the concentration DC")
}

fn end_voluntarily_from_initial() -> ConcentrationState {
    voluntarily_end_concentration(concentration_initial_state())
}

fn cast_replacement_from_initial() -> ConcentrationState {
    cast_replacement_concentration_spell(concentration_initial_state())
}

fn cast_concentration_spell_route() -> (BattleState, Vec<ReducerRouteEvent>) {
    resolve_concentration_without_fill(BattleConcentrationFill::CastSpell)
}

fn cast_replacement_concentration_spell_route() -> (BattleState, Vec<ReducerRouteEvent>) {
    resolve_concentration_without_fill(BattleConcentrationFill::CastReplacementSpell)
}

fn voluntary_end_concentration_route() -> (BattleState, Vec<ReducerRouteEvent>) {
    let (state, mut route) = cast_concentration_spell_route();
    let subject = concentration_teardown_subject();
    let result = resolve_battle_subject(
        state,
        subject,
        BattleFill::Concentration(BattleConcentrationFill::VoluntaryEnd),
    );
    let state = resolved_state(result);
    route.push(route_resolve_battle_subject_without_fill(
        ReducerRouteSubjectFamily::ConcentrationTeardown,
        Vec::new(),
        ReducerRouteOwnerGroup::Concentration,
    ));
    (state, route)
}

fn damage_requests_concentration_save_route() -> (BattleState, Vec<ReducerRouteEvent>) {
    let (state, mut route) = cast_concentration_spell_route();
    let subject = concentration_teardown_subject();
    let result = resolve_battle_subject(
        state,
        subject,
        BattleFill::Concentration(BattleConcentrationFill::DamageTaken(sampled_damage_facts())),
    );
    let holes = result_holes(&result);
    let state = needs_holes_or_resolved_state(result);
    route.push(route_resolve_battle_subject(
        ReducerRouteSubjectFamily::ConcentrationTeardown,
        ReducerRouteFillKind::RolledDice,
        holes,
        ReducerRouteOwnerGroup::Concentration,
    ));
    (state, route)
}

fn fail_concentration_save_route() -> (BattleState, Vec<ReducerRouteEvent>) {
    let (state, mut route) = damage_requests_concentration_save_route();
    let subject = concentration_teardown_subject();
    let result = resolve_battle_subject(
        state,
        subject,
        BattleFill::Concentration(BattleConcentrationFill::SavingThrow(
            ConcentrationSavingThrowFacts {
                saving_throw_total: 9,
            },
        )),
    );
    let holes = result_holes(&result);
    let state = resolved_state(result);
    route.push(route_resolve_battle_subject(
        ReducerRouteSubjectFamily::ConcentrationTeardown,
        ReducerRouteFillKind::ConcentrationSavingThrow,
        holes,
        ReducerRouteOwnerGroup::Concentration,
    ));
    (state, route)
}

fn resolve_concentration_without_fill(
    fill: BattleConcentrationFill,
) -> (BattleState, Vec<ReducerRouteEvent>) {
    let state = start_fighter_skeleton_battle();
    let act = discovered_concentration_teardown_act(&state);
    let mut route = vec![route_start_battle(ReducerRouteOwnerGroup::ActionEconomy)];
    route.push(route_discover_battle_acts(
        ReducerRouteSubjectFamily::ConcentrationTeardown,
        act.holes,
        ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
    ));
    let result = resolve_battle_subject(state, act.subject, BattleFill::Concentration(fill));
    let state = resolved_state(result);
    route.push(route_resolve_battle_subject_without_fill(
        ReducerRouteSubjectFamily::ConcentrationTeardown,
        Vec::new(),
        ReducerRouteOwnerGroup::Concentration,
    ));
    (state, route)
}

fn concentration_cast_route() -> Vec<ReducerRouteEvent> {
    vec![
        route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
        route_discover_battle_acts(
            ReducerRouteSubjectFamily::ConcentrationTeardown,
            Vec::new(),
            ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
        ),
        route_resolve_battle_subject_without_fill(
            ReducerRouteSubjectFamily::ConcentrationTeardown,
            Vec::new(),
            ReducerRouteOwnerGroup::Concentration,
        ),
    ]
}

fn discovered_concentration_teardown_act(
    state: &BattleState,
) -> crate::rules::battle_reducer_spine::AvailableBattleAct {
    discover_battle_acts(state)
        .into_iter()
        .find(|act| act.subject.kind == BattleSubjectKind::ConcentrationTeardown)
        .expect("concentration route should discover one teardown act")
}

fn concentration_teardown_subject() -> BattleSubject {
    BattleSubject {
        kind: BattleSubjectKind::ConcentrationTeardown,
        actor: crate::rules::battle_reducer_spine::Actor::Fighter,
        target: None,
        stage: crate::rules::weapon_attack_ordering::WeaponAttackFrontierStage::Resolved,
        damage_modifier: 0,
    }
}

fn result_holes(result: &BattleResolutionResult) -> Vec<BattleHoleKind> {
    match result {
        BattleResolutionResult::NeedsHoles { holes, .. }
        | BattleResolutionResult::Invalid { holes, .. } => holes.clone(),
        BattleResolutionResult::Resolved { .. } => Vec::new(),
    }
}

fn resolved_state(result: BattleResolutionResult) -> BattleState {
    match result {
        BattleResolutionResult::Resolved { state } => state,
        other => panic!("concentration route should resolve, got {other:?}"),
    }
}

fn needs_holes_or_resolved_state(result: BattleResolutionResult) -> BattleState {
    match result {
        BattleResolutionResult::NeedsHoles { state, .. }
        | BattleResolutionResult::Resolved { state } => state,
        other => panic!("concentration route should not reject, got {other:?}"),
    }
}

fn witness_from_battle_state(state: &BattleState) -> ConcentrationBreakTeardownWitness {
    witness_from_state(state.concentration.clone())
}

fn sampled_damage_facts() -> ConcentrationDamageFacts {
    ConcentrationDamageFacts {
        damage_roll_result: 4,
        damage_bonus: 2,
    }
}

fn witness_from_state(state: ConcentrationState) -> ConcentrationBreakTeardownWitness {
    ConcentrationBreakTeardownWitness {
        scenario: scenario_ref(state.scenario),
        damage_taken: state.damage_taken,
        save_dc: state.save_dc,
        save_roll_total: state.saving_throw_total,
        concentration_save_offered: state.concentration_save_offered,
        caster_concentrating: state.caster_concentrating,
        blurred_effect_count: state.active_concentration_effect_count,
        spell_slot_expended: state.spell_slot_expended,
        teardown_before_next_command: state.teardown_before_next_command,
        replacement_started_after_teardown: state.replacement_started_after_teardown,
        protocol_result: protocol_result_ref(state.protocol),
        protocol_holes: protocol_holes(state.protocol),
    }
}

pub fn sampled_damage_total() -> i16 {
    concentration_damage_total(sampled_damage_facts())
}

fn scenario_ref(scenario: ConcentrationBreakScenario) -> &'static str {
    match scenario {
        ConcentrationBreakScenario::Init => "Init",
        ConcentrationBreakScenario::ConcentrationSpellCast => "ConcentrationSpellCast",
        ConcentrationBreakScenario::DamageSaveNeeded => "DamageSaveNeeded",
        ConcentrationBreakScenario::DamageFailedTeardownBeforeNextCommand => {
            "DamageFailedTeardownBeforeNextCommand"
        }
        ConcentrationBreakScenario::VoluntaryEndTeardown => "VoluntaryEndTeardown",
        ConcentrationBreakScenario::ReplacementTeardownBeforeNewEffect => {
            "ReplacementTeardownBeforeNewEffect"
        }
    }
}

fn protocol_result_ref(protocol: ConcentrationProtocol) -> &'static str {
    match protocol {
        ConcentrationProtocol::Init => "init",
        ConcentrationProtocol::Resolved => "resolved",
        ConcentrationProtocol::NeedsSavingThrow => "needsHoles",
    }
}

fn protocol_holes(protocol: ConcentrationProtocol) -> Vec<&'static str> {
    match protocol {
        ConcentrationProtocol::NeedsSavingThrow => vec!["ConcentrationSavingThrow"],
        ConcentrationProtocol::Init | ConcentrationProtocol::Resolved => vec![],
    }
}

fn joined_or_none(values: &[&'static str]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}
