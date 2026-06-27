use crate::rules::battle_reducer_spine::{
    discover_battle_acts_observed, resolve_battle_subject_observed, start_battle_observed, Actor,
    BattleArmorClassBaseEffect, BattleArmorClassBaseProjectionFacts,
    BattleArmorClassEffectTargetFacts, BattleArmorClassSpellEffectFill,
    BattleArmorClassTargetAdmissionFacts, BattleEntrypointTrace, BattleResolutionInvalidReason,
    BattleResolutionRequest, BattleResolutionResult, BattleSetup, BattleState, BattleSubject,
    BattleSubjectKind,
};
use crate::rules::level_one_armor_spells::{
    discover_mage_armor_unarmored_self_target, expire_mage_armor_duration,
    mage_armor_initial_state, reject_mage_armor_armored_target,
    resolve_mage_armor_base_armor_class, MageArmorProtocol, MageArmorScenarioOutcome,
    MageArmorState, MAGE_ARMOR_BASE_ARMOR_CLASS, MAGE_ARMOR_DEXTERITY_MODIFIER,
    MAGE_ARMOR_DURATION_TICKS,
};

use super::battle_runtime_reducer_route::{
    observed_reducer_route, route_discover_battle_acts_from_route_holes,
    route_resolve_battle_subject_from_route_result,
    route_resolve_battle_subject_without_fill_from_route_result, route_start_battle,
    ReducerRouteEvent, ReducerRouteFillKind, ReducerRouteHoleKind, ReducerRouteOwnerGroup,
    ReducerRouteResolutionOutcome, ReducerRouteSubjectFamily,
};

pub const BRANCH_ACTIONS: [&str; 4] = [
    "doDiscoverMageArmorUnarmoredSelfTarget",
    "doExpireMageArmorDuration",
    "doRejectMageArmorArmoredTarget",
    "doResolveMageArmorBaseArmorClassProjection",
];

pub const ACCEPTED_ROUTE_BRANCH_ACTIONS: [&str; 3] = [
    "doDiscoverMageArmorUnarmoredSelfTarget",
    "doExpireMageArmorDuration",
    "doRejectMageArmorArmoredTarget",
];

pub fn replay_observed_action(observed_action_taken: &str) -> MageArmorState {
    match observed_action_taken {
        "doDiscoverMageArmorUnarmoredSelfTarget"
        | "doExpireMageArmorDuration"
        | "doRejectMageArmorArmoredTarget"
        | "doResolveMageArmorBaseArmorClassProjection" => mage_armor_state_from_battle(
            &replay_observed_state_and_route(observed_action_taken).0,
            scenario_outcome(observed_action_taken),
        ),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> MageArmorState {
    match observed_action_taken {
        "doDiscoverMageArmorUnarmoredSelfTarget" => discover_mage_armor_unarmored_self_target(),
        "doExpireMageArmorDuration" => expire_mage_armor_duration(),
        "doRejectMageArmorArmoredTarget" => reject_mage_armor_armored_target(),
        "doResolveMageArmorBaseArmorClassProjection" => resolve_mage_armor_base_armor_class(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    replay_observed_state_and_route(observed_action_taken).1
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doDiscoverMageArmorUnarmoredSelfTarget" => vec![
            route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
            discover_target_admission(),
            resolve_target_admission(ReducerRouteResolutionOutcome::Resolved, Vec::new()),
        ],
        "doRejectMageArmorArmoredTarget" => vec![
            route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
            discover_target_admission(),
            resolve_target_admission(
                ReducerRouteResolutionOutcome::Invalid(BattleResolutionInvalidReason::WrongTarget),
                vec![ReducerRouteHoleKind::TargetChoice],
            ),
        ],
        "doExpireMageArmorDuration" => vec![
            route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
            route_discover_battle_acts_from_route_holes(
                ReducerRouteSubjectFamily::ArmorClassSpellEffect,
                Vec::new(),
                ReducerRouteOwnerGroup::ActiveEffect,
            ),
            resolve_without_fill(ReducerRouteOwnerGroup::TurnBoundary),
            resolve_without_fill(ReducerRouteOwnerGroup::ActiveEffect),
            resolve_without_fill(ReducerRouteOwnerGroup::ArmorClass),
        ],
        "doResolveMageArmorBaseArmorClassProjection" => vec![
            route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
            route_discover_battle_acts_from_route_holes(
                ReducerRouteSubjectFamily::ArmorClassSpellEffect,
                vec![ReducerRouteHoleKind::TargetChoice],
                ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
            ),
            route_resolve_battle_subject_from_route_result(
                ReducerRouteSubjectFamily::ArmorClassSpellEffect,
                ReducerRouteFillKind::UnitFeatureDecision,
                ReducerRouteResolutionOutcome::Resolved,
                Vec::new(),
                ReducerRouteOwnerGroup::ActiveEffect,
            ),
        ],
        action => panic!("unsupported expected route mbt::actionTaken {action}"),
    }
}

pub fn projection_payload(state: &MageArmorState) -> String {
    [
        format!("qSelfTargetAdmitted={}", state.self_target_admitted),
        format!("qArmoredTargetRejected={}", state.armored_target_rejected),
        format!(
            "qMageArmorEffectPresent={}",
            state.mage_armor_effect_present
        ),
        format!(
            "qStoredArmorBaseStillUnarmored={}",
            state.stored_armor_base_still_unarmored
        ),
        format!(
            "qProjectedBaseIsMageArmor={}",
            state.projected_base_is_mage_armor
        ),
        format!("qArmorClass={}", state.armor_class),
        format!(
            "qMageArmorDurationTicks={}",
            state.mage_armor_duration_ticks
        ),
        format!("qLevel1SlotsExpended={}", state.level_one_slots_expended),
        format!("qActionAvailable={}", state.action_available),
        format!(
            "qScenarioOutcome={}",
            scenario_outcome_ref(state.scenario_outcome)
        ),
        format!("protocolResult={}", protocol_ref(state.protocol)),
        "protocolHoles=none".to_string(),
    ]
    .join("\n")
}

fn replay_observed_state_and_route(
    observed_action_taken: &str,
) -> (BattleState, Vec<ReducerRouteEvent>) {
    match observed_action_taken {
        "doDiscoverMageArmorUnarmoredSelfTarget" => {
            let mut observer = BattleEntrypointTrace::default();
            let state = start_battle_observed(mage_armor_setup(), &mut observer).state;
            let subject = discover_armor_class_spell_effect_subject(&state, &mut observer);
            let result = resolve_battle_subject_observed(
                state,
                BattleResolutionRequest::armor_class_spell_effect(
                    subject,
                    BattleArmorClassSpellEffectFill::TargetAdmission(
                        BattleArmorClassTargetAdmissionFacts {
                            target: Actor::Fighter,
                            target_wears_armor: false,
                        },
                    ),
                )
                .expect("armor-class spell effect subject should accept target-admission facts"),
                &mut observer,
            );
            (
                resolved_state(result, observed_action_taken),
                observed_armor_class_spell_effect_route(&observer),
            )
        }
        "doRejectMageArmorArmoredTarget" => {
            let mut observer = BattleEntrypointTrace::default();
            let state = start_battle_observed(mage_armor_setup(), &mut observer).state;
            let subject = discover_armor_class_spell_effect_subject(&state, &mut observer);
            let result = resolve_battle_subject_observed(
                state,
                BattleResolutionRequest::armor_class_spell_effect(
                    subject,
                    BattleArmorClassSpellEffectFill::TargetAdmission(
                        BattleArmorClassTargetAdmissionFacts {
                            target: Actor::Fighter,
                            target_wears_armor: true,
                        },
                    ),
                )
                .expect("armor-class spell effect subject should accept target-admission facts"),
                &mut observer,
            );
            (
                invalid_state(result, observed_action_taken),
                observed_armor_class_spell_effect_route(&observer),
            )
        }
        "doExpireMageArmorDuration" => {
            let mut observer = BattleEntrypointTrace::default();
            let mut state = start_battle_observed(active_mage_armor_setup(), &mut observer).state;
            let subject = discover_armor_class_spell_effect_subject(&state, &mut observer);

            state = resolved_state(
                resolve_battle_subject_observed(
                    state,
                    armor_class_spell_effect_request(
                        subject,
                        BattleArmorClassSpellEffectFill::DurationBoundary(
                            BattleArmorClassEffectTargetFacts {
                                target: Actor::Fighter,
                            },
                        ),
                    ),
                    &mut observer,
                ),
                observed_action_taken,
            );
            state = resolved_state(
                resolve_battle_subject_observed(
                    state,
                    armor_class_spell_effect_request(
                        subject,
                        BattleArmorClassSpellEffectFill::ActiveEffectEnd(
                            BattleArmorClassEffectTargetFacts {
                                target: Actor::Fighter,
                            },
                        ),
                    ),
                    &mut observer,
                ),
                observed_action_taken,
            );
            state = resolved_state(
                resolve_battle_subject_observed(
                    state,
                    armor_class_spell_effect_request(
                        subject,
                        BattleArmorClassSpellEffectFill::ArmorClassProjection(
                            BattleArmorClassEffectTargetFacts {
                                target: Actor::Fighter,
                            },
                        ),
                    ),
                    &mut observer,
                ),
                observed_action_taken,
            );

            (state, observed_armor_class_spell_effect_route(&observer))
        }
        "doResolveMageArmorBaseArmorClassProjection" => {
            let mut observer = BattleEntrypointTrace::default();
            let state = start_battle_observed(mage_armor_setup(), &mut observer).state;
            let subject = discover_armor_class_spell_effect_subject(&state, &mut observer);
            let result = resolve_battle_subject_observed(
                state,
                BattleResolutionRequest::armor_class_spell_effect(
                    subject,
                    BattleArmorClassSpellEffectFill::BaseArmorClassProjection(
                        BattleArmorClassBaseProjectionFacts {
                            target: Actor::Fighter,
                            base_armor_class: MAGE_ARMOR_BASE_ARMOR_CLASS,
                            dexterity_modifier: MAGE_ARMOR_DEXTERITY_MODIFIER,
                            duration_ticks: MAGE_ARMOR_DURATION_TICKS,
                        },
                    ),
                )
                .expect("armor-class spell effect subject should accept base projection facts"),
                &mut observer,
            );
            (
                resolved_state(result, observed_action_taken),
                observed_armor_class_spell_effect_route(&observer),
            )
        }
        action => panic!("unsupported routed mbt::actionTaken {action}"),
    }
}

fn mage_armor_setup() -> BattleSetup {
    let mut setup = BattleSetup::standard();
    setup.fighter.armor_class = 12;
    setup
}

fn active_mage_armor_setup() -> BattleSetup {
    let mut setup = mage_armor_setup();
    setup.fighter.spell_slots.first_level_expended = 1;
    setup.fighter.spell_active_effects.armor_class_base_effect =
        BattleArmorClassBaseEffect::Active {
            base_armor_class: MAGE_ARMOR_BASE_ARMOR_CLASS,
            dexterity_modifier: MAGE_ARMOR_DEXTERITY_MODIFIER,
            duration_ticks: MAGE_ARMOR_DURATION_TICKS,
        };
    setup.action_available = false;
    setup
}

fn armor_class_spell_effect_request(
    subject: BattleSubject,
    fill: BattleArmorClassSpellEffectFill,
) -> BattleResolutionRequest {
    BattleResolutionRequest::armor_class_spell_effect(subject, fill)
        .expect("armor-class spell effect subject should accept armor-class fills")
}

fn discover_armor_class_spell_effect_subject(
    state: &BattleState,
    observer: &mut BattleEntrypointTrace,
) -> BattleSubject {
    discover_battle_acts_observed(state, observer)
        .into_available_acts()
        .into_iter()
        .find(|act| act.subject.kind == BattleSubjectKind::ArmorClassSpellEffect)
        .map(|act| act.subject)
        .expect("armor-class spell effect should be discoverable")
}

fn observed_armor_class_spell_effect_route(
    observer: &BattleEntrypointTrace,
) -> Vec<ReducerRouteEvent> {
    observed_reducer_route(
        observer,
        &[ReducerRouteSubjectFamily::ArmorClassSpellEffect],
    )
}

fn resolved_state(result: BattleResolutionResult, context: &str) -> BattleState {
    let BattleResolutionResult::Resolved { state } = result else {
        panic!("{context} should resolve")
    };
    state
}

fn invalid_state(result: BattleResolutionResult, context: &str) -> BattleState {
    let BattleResolutionResult::Invalid { state, .. } = result else {
        panic!("{context} should be invalid")
    };
    state
}

fn mage_armor_state_from_battle(
    state: &BattleState,
    scenario_outcome: MageArmorScenarioOutcome,
) -> MageArmorState {
    let (effect_present, projected_base_is_mage_armor, armor_class, duration_ticks) =
        match state.fighter.spell_active_effects.armor_class_base_effect {
            BattleArmorClassBaseEffect::None => (false, false, state.fighter.armor_class, 0),
            BattleArmorClassBaseEffect::Active {
                base_armor_class,
                dexterity_modifier,
                duration_ticks,
            } => (
                true,
                base_armor_class == MAGE_ARMOR_BASE_ARMOR_CLASS,
                i16::from(base_armor_class + dexterity_modifier),
                duration_ticks,
            ),
        };

    MageArmorState {
        mage_armor_effect_present: effect_present,
        projected_base_is_mage_armor,
        armor_class: i8::try_from(armor_class).expect("mage armor projection should fit i8"),
        mage_armor_duration_ticks: duration_ticks,
        level_one_slots_expended: u8::try_from(state.fighter.spell_slots.first_level_expended)
            .expect("level-one slot expenditure should fit u8"),
        action_available: state.action_available,
        scenario_outcome,
        protocol: MageArmorProtocol::Resolved,
        ..expected_witness_for_scenario(scenario_outcome)
    }
}

fn expected_witness_for_scenario(scenario_outcome: MageArmorScenarioOutcome) -> MageArmorState {
    match scenario_outcome {
        MageArmorScenarioOutcome::Init => mage_armor_initial_state(),
        MageArmorScenarioOutcome::Discovered => discover_mage_armor_unarmored_self_target(),
        MageArmorScenarioOutcome::ArmoredRejected => reject_mage_armor_armored_target(),
        MageArmorScenarioOutcome::Resolved => resolve_mage_armor_base_armor_class(),
        MageArmorScenarioOutcome::DurationExpired => expire_mage_armor_duration(),
    }
}

fn scenario_outcome(observed_action_taken: &str) -> MageArmorScenarioOutcome {
    match observed_action_taken {
        "doDiscoverMageArmorUnarmoredSelfTarget" => MageArmorScenarioOutcome::Discovered,
        "doExpireMageArmorDuration" => MageArmorScenarioOutcome::DurationExpired,
        "doRejectMageArmorArmoredTarget" => MageArmorScenarioOutcome::ArmoredRejected,
        "doResolveMageArmorBaseArmorClassProjection" => MageArmorScenarioOutcome::Resolved,
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

fn discover_target_admission() -> ReducerRouteEvent {
    route_discover_battle_acts_from_route_holes(
        ReducerRouteSubjectFamily::ArmorClassSpellEffect,
        vec![ReducerRouteHoleKind::TargetChoice],
        ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
    )
}

fn resolve_target_admission(
    outcome: ReducerRouteResolutionOutcome,
    holes: Vec<ReducerRouteHoleKind>,
) -> ReducerRouteEvent {
    route_resolve_battle_subject_from_route_result(
        ReducerRouteSubjectFamily::ArmorClassSpellEffect,
        ReducerRouteFillKind::TargetChoice,
        outcome,
        holes,
        ReducerRouteOwnerGroup::TargetSelection,
    )
}

fn resolve_without_fill(owner: ReducerRouteOwnerGroup) -> ReducerRouteEvent {
    route_resolve_battle_subject_without_fill_from_route_result(
        ReducerRouteSubjectFamily::ArmorClassSpellEffect,
        ReducerRouteResolutionOutcome::Resolved,
        Vec::new(),
        owner,
    )
}

fn scenario_outcome_ref(outcome: MageArmorScenarioOutcome) -> &'static str {
    match outcome {
        MageArmorScenarioOutcome::Init => "Init",
        MageArmorScenarioOutcome::Discovered => "Discovered",
        MageArmorScenarioOutcome::ArmoredRejected => "ArmoredRejected",
        MageArmorScenarioOutcome::Resolved => "Resolved",
        MageArmorScenarioOutcome::DurationExpired => "DurationExpired",
    }
}

fn protocol_ref(protocol: MageArmorProtocol) -> &'static str {
    match protocol {
        MageArmorProtocol::Init => "init",
        MageArmorProtocol::Resolved => "resolved",
    }
}
