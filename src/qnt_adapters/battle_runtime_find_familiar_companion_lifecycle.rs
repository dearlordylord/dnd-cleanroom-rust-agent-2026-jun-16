use crate::rules::battle_reducer_spine::{
    discover_generic_route_subject_observed, generic_route_subject_for_current_actor,
    resolve_battle_subject_observed, start_battle_observed, BattleEntrypointTrace,
    BattleGenericRouteFill, BattleResolutionRequest, BattleSetup, BattleSubjectKind,
};
use crate::rules::find_familiar::{
    create_find_familiar_companion, deliver_find_familiar_touch_spell,
    find_familiar_companion_initial_state, replace_find_familiar_companion_form,
    resolve_pact_find_familiar_reaction_attack, share_find_familiar_senses,
    FamiliarCreatureTypeOverride, FamiliarForm, FamiliarFormFacts, FamiliarSlot, FamiliarStatus,
    FindFamiliarCompanionProtocol, FindFamiliarCompanionScenarioOutcome,
    FindFamiliarCompanionState,
};

use super::battle_runtime_reducer_route::{
    observed_reducer_route, route_discover_battle_acts_from_route_holes,
    route_resolve_battle_subject_from_route_result,
    route_resolve_battle_subject_without_fill_from_route_result, route_start_battle,
    ReducerRouteEvent, ReducerRouteFillKind, ReducerRouteHoleKind, ReducerRouteOwnerGroup,
    ReducerRouteResolutionOutcome, ReducerRouteSubjectFamily,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FindFamiliarCompanionLifecycleWitness {
    pub familiar_status: &'static str,
    pub familiar_id: &'static str,
    pub familiar_form: &'static str,
    pub creature_type_override: &'static str,
    pub companion_count: i16,
    pub telepathy_available: bool,
    pub shared_senses_active: bool,
    pub bonus_action_available: bool,
    pub owner_attack_available: bool,
    pub familiar_reaction_available: bool,
    pub touch_delivery_reaction_spent: bool,
    pub pact_reaction_attack_resolved: bool,
    pub spell_slot_committed: bool,
    pub target_hp: i16,
    pub scenario_outcome: &'static str,
    pub protocol_result: &'static str,
    pub protocol_holes: Vec<&'static str>,
}

pub const BRANCH_ACTIONS: [&str; 5] = [
    "doCreateCatFamiliar",
    "doDeliverTouchSpell",
    "doPactFamiliarAttack",
    "doReplaceWithRatFamiliar",
    "doShareSenses",
];

pub fn replay_observed_action(
    observed_action_taken: &str,
) -> FindFamiliarCompanionLifecycleWitness {
    match observed_action_taken {
        "doCreateCatFamiliar" => witness_from_state(created_cat()),
        "doDeliverTouchSpell" => witness_from_state(touch_spell_delivered()),
        "doPactFamiliarAttack" => witness_from_state(pact_familiar_attack()),
        "doReplaceWithRatFamiliar" => witness_from_state(replaced_rat()),
        "doShareSenses" => witness_from_state(shared_senses()),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> FindFamiliarCompanionLifecycleWitness {
    replay_observed_action(observed_action_taken)
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doCreateCatFamiliar" | "doReplaceWithRatFamiliar" => replay_generic_route(
            BattleSubjectKind::CompanionLifecycle,
            &[BattleGenericRouteFill::WithoutFill],
            &[ReducerRouteSubjectFamily::CompanionLifecycle],
        ),
        "doShareSenses" => replay_shared_senses_route(),
        "doDeliverTouchSpell" => replay_generic_route(
            BattleSubjectKind::CompanionTouchDelivery,
            &[
                BattleGenericRouteFill::TargetChoice,
                BattleGenericRouteFill::RolledDice,
                BattleGenericRouteFill::WithoutFill,
            ],
            &[ReducerRouteSubjectFamily::CompanionTouchDelivery],
        ),
        "doPactFamiliarAttack" => replay_pact_reaction_attack_route(),
        action => panic!("unsupported route mbt::actionTaken {action}"),
    }
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doCreateCatFamiliar" | "doReplaceWithRatFamiliar" => vec![
            route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
            route_discover_battle_acts_from_route_holes(
                ReducerRouteSubjectFamily::CompanionLifecycle,
                Vec::new(),
                ReducerRouteOwnerGroup::Companion,
            ),
            route_resolve_battle_subject_without_fill_from_route_result(
                ReducerRouteSubjectFamily::CompanionLifecycle,
                ReducerRouteResolutionOutcome::Resolved,
                Vec::new(),
                ReducerRouteOwnerGroup::Companion,
            ),
        ],
        "doShareSenses" => vec![
            route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
            route_discover_battle_acts_from_route_holes(
                ReducerRouteSubjectFamily::CompanionSharedSenses,
                Vec::new(),
                ReducerRouteOwnerGroup::Companion,
            ),
            route_resolve_battle_subject_without_fill_from_route_result(
                ReducerRouteSubjectFamily::CompanionSharedSenses,
                ReducerRouteResolutionOutcome::Resolved,
                Vec::new(),
                ReducerRouteOwnerGroup::ActionEconomy,
            ),
            route_resolve_battle_subject_without_fill_from_route_result(
                ReducerRouteSubjectFamily::CompanionSharedSenses,
                ReducerRouteResolutionOutcome::Resolved,
                Vec::new(),
                ReducerRouteOwnerGroup::ActiveEffect,
            ),
        ],
        "doDeliverTouchSpell" => vec![
            route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
            route_discover_battle_acts_from_route_holes(
                ReducerRouteSubjectFamily::CompanionTouchDelivery,
                vec![ReducerRouteHoleKind::TargetChoice],
                ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
            ),
            route_resolve_battle_subject_from_route_result(
                ReducerRouteSubjectFamily::CompanionTouchDelivery,
                ReducerRouteFillKind::TargetChoice,
                ReducerRouteResolutionOutcome::NeedsHoles,
                vec![ReducerRouteHoleKind::RolledDice],
                ReducerRouteOwnerGroup::Companion,
            ),
            route_resolve_battle_subject_from_route_result(
                ReducerRouteSubjectFamily::CompanionTouchDelivery,
                ReducerRouteFillKind::RolledDice,
                ReducerRouteResolutionOutcome::Resolved,
                Vec::new(),
                ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
            ),
            route_resolve_battle_subject_without_fill_from_route_result(
                ReducerRouteSubjectFamily::CompanionTouchDelivery,
                ReducerRouteResolutionOutcome::Resolved,
                Vec::new(),
                ReducerRouteOwnerGroup::ActionEconomy,
            ),
        ],
        "doPactFamiliarAttack" => vec![
            route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
            route_discover_battle_acts_from_route_holes(
                ReducerRouteSubjectFamily::CompanionReactionAttack,
                vec![ReducerRouteHoleKind::TargetChoice],
                ReducerRouteOwnerGroup::Companion,
            ),
            route_resolve_battle_subject_without_fill_from_route_result(
                ReducerRouteSubjectFamily::CompanionReactionAttack,
                ReducerRouteResolutionOutcome::NeedsHoles,
                vec![ReducerRouteHoleKind::TargetChoice],
                ReducerRouteOwnerGroup::StatBlockAction,
            ),
            route_resolve_battle_subject_from_route_result(
                ReducerRouteSubjectFamily::CompanionReactionAttack,
                ReducerRouteFillKind::TargetChoice,
                ReducerRouteResolutionOutcome::NeedsHoles,
                vec![ReducerRouteHoleKind::AttackRoll],
                ReducerRouteOwnerGroup::TargetSelection,
            ),
            route_resolve_battle_subject_from_route_result(
                ReducerRouteSubjectFamily::CompanionReactionAttack,
                ReducerRouteFillKind::AttackRoll,
                ReducerRouteResolutionOutcome::NeedsHoles,
                vec![ReducerRouteHoleKind::RolledDice],
                ReducerRouteOwnerGroup::AttackRoll,
            ),
            route_resolve_battle_subject_from_route_result(
                ReducerRouteSubjectFamily::CompanionReactionAttack,
                ReducerRouteFillKind::RolledDice,
                ReducerRouteResolutionOutcome::Resolved,
                Vec::new(),
                ReducerRouteOwnerGroup::HitPoint,
            ),
            route_resolve_battle_subject_without_fill_from_route_result(
                ReducerRouteSubjectFamily::CompanionReactionAttack,
                ReducerRouteResolutionOutcome::Resolved,
                Vec::new(),
                ReducerRouteOwnerGroup::ActionEconomy,
            ),
        ],
        action => panic!("unsupported expected route mbt::actionTaken {action}"),
    }
}

fn replay_generic_route(
    subject_kind: BattleSubjectKind,
    fills: &[BattleGenericRouteFill],
    route_subjects: &[ReducerRouteSubjectFamily],
) -> Vec<ReducerRouteEvent> {
    let mut trace = BattleEntrypointTrace::default();
    let state = start_battle_observed(BattleSetup::standard(), &mut trace).state;
    let (mut state, subject) =
        discover_generic_route_subject_observed(state, subject_kind, &mut trace);
    for fill in fills {
        state = resolve_battle_subject_observed(
            state,
            BattleResolutionRequest::generic_route(subject, *fill)
                .expect("generic companion route should accept fill"),
            &mut trace,
        )
        .into_state();
    }
    observed_reducer_route(&trace, route_subjects)
}

fn replay_shared_senses_route() -> Vec<ReducerRouteEvent> {
    let mut trace = BattleEntrypointTrace::default();
    let state = start_battle_observed(BattleSetup::standard(), &mut trace).state;
    let (state, action_subject) = discover_generic_route_subject_observed(
        state,
        BattleSubjectKind::CompanionSharedSensesActionEconomy,
        &mut trace,
    );
    let state = resolve_battle_subject_observed(
        state,
        BattleResolutionRequest::generic_route(action_subject, BattleGenericRouteFill::WithoutFill)
            .expect("shared-senses action-economy route should accept without-fill"),
        &mut trace,
    )
    .into_state();
    let active_effect_subject = generic_route_subject_for_current_actor(
        &state,
        BattleSubjectKind::CompanionSharedSensesActiveEffect,
    );
    let _ = resolve_battle_subject_observed(
        state,
        BattleResolutionRequest::generic_route(
            active_effect_subject,
            BattleGenericRouteFill::WithoutFill,
        )
        .expect("shared-senses active-effect route should accept without-fill"),
        &mut trace,
    );
    observed_reducer_route(&trace, &[ReducerRouteSubjectFamily::CompanionSharedSenses])
}

fn replay_pact_reaction_attack_route() -> Vec<ReducerRouteEvent> {
    let mut trace = BattleEntrypointTrace::default();
    let state = start_battle_observed(BattleSetup::standard(), &mut trace).state;
    let (mut state, subject) = discover_generic_route_subject_observed(
        state,
        BattleSubjectKind::CompanionReactionAttackStatBlockGate,
        &mut trace,
    );
    for fill in [
        BattleGenericRouteFill::WithoutFill,
        BattleGenericRouteFill::TargetChoice,
        BattleGenericRouteFill::AttackRoll,
        BattleGenericRouteFill::RolledDice,
    ] {
        state = resolve_battle_subject_observed(
            state,
            BattleResolutionRequest::generic_route(subject, fill)
                .expect("companion reaction attack route should accept fill"),
            &mut trace,
        )
        .into_state();
    }
    let cleanup_subject = generic_route_subject_for_current_actor(
        &state,
        BattleSubjectKind::CompanionReactionAttackActionEconomyCleanup,
    );
    let _ = resolve_battle_subject_observed(
        state,
        BattleResolutionRequest::generic_route(
            cleanup_subject,
            BattleGenericRouteFill::WithoutFill,
        )
        .expect("companion reaction attack cleanup route should accept without-fill"),
        &mut trace,
    );
    observed_reducer_route(
        &trace,
        &[ReducerRouteSubjectFamily::CompanionReactionAttack],
    )
}

pub fn projection_payload(witness: &FindFamiliarCompanionLifecycleWitness) -> String {
    [
        format!("qFamiliarStatus={}", witness.familiar_status),
        format!("qFamiliarId={}", witness.familiar_id),
        format!("qFamiliarForm={}", witness.familiar_form),
        format!("qCreatureTypeOverride={}", witness.creature_type_override),
        format!("qCompanionCount={}", witness.companion_count),
        format!("qTelepathyAvailable={}", witness.telepathy_available),
        format!("qSharedSensesActive={}", witness.shared_senses_active),
        format!("qBonusActionAvailable={}", witness.bonus_action_available),
        format!("qOwnerAttackAvailable={}", witness.owner_attack_available),
        format!(
            "qFamiliarReactionAvailable={}",
            witness.familiar_reaction_available
        ),
        format!(
            "qTouchDeliveryReactionSpent={}",
            witness.touch_delivery_reaction_spent
        ),
        format!(
            "qPactReactionAttackResolved={}",
            witness.pact_reaction_attack_resolved
        ),
        format!("qSpellSlotCommitted={}", witness.spell_slot_committed),
        format!("qTargetHp={}", witness.target_hp),
        format!("qScenarioOutcome={}", witness.scenario_outcome),
        format!("protocolResult={}", witness.protocol_result),
        format!("protocolHoles={}", joined_or_none(&witness.protocol_holes)),
    ]
    .join("\n")
}

fn created_cat() -> FindFamiliarCompanionState {
    create_find_familiar_companion(
        find_familiar_companion_initial_state(),
        FamiliarFormFacts {
            form: FamiliarForm::Cat,
            creature_type_override: FamiliarCreatureTypeOverride::Fey,
        },
    )
}

fn replaced_rat() -> FindFamiliarCompanionState {
    replace_find_familiar_companion_form(created_cat(), FamiliarForm::Rat)
}

fn shared_senses() -> FindFamiliarCompanionState {
    share_find_familiar_senses(created_cat())
}

fn touch_spell_delivered() -> FindFamiliarCompanionState {
    deliver_find_familiar_touch_spell(created_cat())
}

fn pact_familiar_attack() -> FindFamiliarCompanionState {
    resolve_pact_find_familiar_reaction_attack(created_cat(), 1)
}

fn witness_from_state(state: FindFamiliarCompanionState) -> FindFamiliarCompanionLifecycleWitness {
    FindFamiliarCompanionLifecycleWitness {
        familiar_status: status_ref(state.status),
        familiar_id: slot_ref(state.slot),
        familiar_form: form_ref(state.form),
        creature_type_override: creature_type_ref(state.creature_type_override),
        companion_count: state.companion_count,
        telepathy_available: state.telepathy_available,
        shared_senses_active: state.shared_senses_active,
        bonus_action_available: state.bonus_action_available,
        owner_attack_available: state.owner_attack_available,
        familiar_reaction_available: state.familiar_reaction_available,
        touch_delivery_reaction_spent: state.touch_delivery_reaction_spent,
        pact_reaction_attack_resolved: state.pact_reaction_attack_resolved,
        spell_slot_committed: state.spell_slot_committed,
        target_hp: state.target_hit_points,
        scenario_outcome: scenario_outcome_ref(state.scenario_outcome),
        protocol_result: protocol_result_ref(state.protocol),
        protocol_holes: protocol_holes(state.protocol),
    }
}

fn status_ref(status: FamiliarStatus) -> &'static str {
    match status {
        FamiliarStatus::None => "none",
        FamiliarStatus::Present => "present",
    }
}

fn slot_ref(slot: FamiliarSlot) -> &'static str {
    match slot {
        FamiliarSlot::None => "none",
        FamiliarSlot::Primary => "primary",
    }
}

fn form_ref(form: FamiliarForm) -> &'static str {
    match form {
        FamiliarForm::None => "none",
        FamiliarForm::Cat => "cat",
        FamiliarForm::Rat => "rat",
    }
}

fn creature_type_ref(creature_type: FamiliarCreatureTypeOverride) -> &'static str {
    match creature_type {
        FamiliarCreatureTypeOverride::None => "none",
        FamiliarCreatureTypeOverride::Fey => "fey",
    }
}

fn scenario_outcome_ref(outcome: FindFamiliarCompanionScenarioOutcome) -> &'static str {
    match outcome {
        FindFamiliarCompanionScenarioOutcome::Init => "Init",
        FindFamiliarCompanionScenarioOutcome::Created => "CreatedCat",
        FindFamiliarCompanionScenarioOutcome::Replaced => "ReplacedRat",
        FindFamiliarCompanionScenarioOutcome::DismissedAndReappeared => "DismissedAndReappeared",
        FindFamiliarCompanionScenarioOutcome::SharedSenses => "SharedSenses",
        FindFamiliarCompanionScenarioOutcome::TouchDelivered => "TouchDelivered",
        FindFamiliarCompanionScenarioOutcome::PactAttack => "PactAttack",
    }
}

fn protocol_result_ref(protocol: FindFamiliarCompanionProtocol) -> &'static str {
    match protocol {
        FindFamiliarCompanionProtocol::Init => "init",
        FindFamiliarCompanionProtocol::Resolved => "resolved",
    }
}

fn protocol_holes(protocol: FindFamiliarCompanionProtocol) -> Vec<&'static str> {
    match protocol {
        FindFamiliarCompanionProtocol::Init => vec!["WitnessProtocolHole"],
        FindFamiliarCompanionProtocol::Resolved => vec![],
    }
}

fn joined_or_none(values: &[&'static str]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}
