use crate::rules::battle_reducer_spine::{
    discover_battle_acts_observed, resolve_battle_subject_observed, start_battle_observed, Actor,
    BattleArmorClassBaseProjectionFacts, BattleArmorClassSpellEffectFill, BattleEntrypointTrace,
    BattleResolutionRequest, BattleResolutionResult, BattleSetup, BattleState, BattleSubject,
    BattleSubjectKind,
};
use crate::rules::level_one_armor_spells::{
    discover_mage_armor_unarmored_self_target, expire_mage_armor_duration,
    reject_mage_armor_armored_target, resolve_mage_armor_base_armor_class, MageArmorProtocol,
    MageArmorScenarioOutcome, MageArmorState, MAGE_ARMOR_BASE_ARMOR_CLASS,
    MAGE_ARMOR_DEXTERITY_MODIFIER, MAGE_ARMOR_DURATION_TICKS,
};

use super::battle_runtime_reducer_route::{
    observed_reducer_route, route_discover_battle_acts_from_route_holes,
    route_resolve_battle_subject_from_route_result, route_start_battle, ReducerRouteEvent,
    ReducerRouteFillKind, ReducerRouteOwnerGroup, ReducerRouteResolutionOutcome,
    ReducerRouteSubjectFamily,
};

pub const BRANCH_ACTIONS: [&str; 4] = [
    "doDiscoverMageArmorUnarmoredSelfTarget",
    "doExpireMageArmorDuration",
    "doRejectMageArmorArmoredTarget",
    "doResolveMageArmorBaseArmorClassProjection",
];

pub const ACCEPTED_ROUTE_BRANCH_ACTIONS: [&str; 1] = ["doResolveMageArmorBaseArmorClassProjection"];

pub fn replay_observed_action(observed_action_taken: &str) -> MageArmorState {
    match observed_action_taken {
        "doDiscoverMageArmorUnarmoredSelfTarget" => discover_mage_armor_unarmored_self_target(),
        "doExpireMageArmorDuration" => expire_mage_armor_duration(),
        "doRejectMageArmorArmoredTarget" => reject_mage_armor_armored_target(),
        "doResolveMageArmorBaseArmorClassProjection" => {
            mage_armor_state_from_battle(&replay_observed_state_and_route(observed_action_taken).0)
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> MageArmorState {
    replay_observed_action(observed_action_taken)
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    replay_observed_state_and_route(observed_action_taken).1
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doResolveMageArmorBaseArmorClassProjection" => vec![
            route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
            route_discover_battle_acts_from_route_holes(
                ReducerRouteSubjectFamily::ArmorClassSpellEffect,
                Vec::new(),
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
                observed_reducer_route(
                    &observer,
                    &[ReducerRouteSubjectFamily::ArmorClassSpellEffect],
                ),
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

fn resolved_state(result: BattleResolutionResult, context: &str) -> BattleState {
    let BattleResolutionResult::Resolved { state } = result else {
        panic!("{context} should resolve")
    };
    state
}

fn mage_armor_state_from_battle(state: &BattleState) -> MageArmorState {
    let (effect_present, projected_base_is_mage_armor, armor_class, duration_ticks) =
        match state.fighter.spell_active_effects.armor_class_base_effect {
            crate::rules::battle_reducer_spine::BattleArmorClassBaseEffect::None => {
                (false, false, state.fighter.armor_class, 0)
            }
            crate::rules::battle_reducer_spine::BattleArmorClassBaseEffect::Active {
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
        scenario_outcome: MageArmorScenarioOutcome::Resolved,
        protocol: MageArmorProtocol::Resolved,
        ..resolve_mage_armor_base_armor_class()
    }
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
