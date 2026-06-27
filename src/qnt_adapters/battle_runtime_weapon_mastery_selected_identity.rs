use crate::rules::battle_reducer_spine::{
    discover_battle_acts, discover_battle_acts_observed, resolve_battle_subject,
    resolve_battle_subject_observed, start_weapon_mastery_property_battle,
    start_weapon_mastery_property_battle_observed, Actor, BattleEntrypointTrace,
    BattleResolutionRequest, BattleSubject, BattleSubjectKind, BattleWeaponMasteryPropertyFill,
};
use crate::rules::weapon_mastery_selected_identity::{
    WeaponMasteryProperty, WeaponMasteryRuntimeHole, WeaponMasteryRuntimeOutcome,
    WeaponMasteryRuntimeProtocol, WeaponMasterySelectedIdentityState,
    WEAPON_MASTERY_SAMPLE_WEAPON_DAMAGE, WEAPON_MASTERY_TARGET_INITIAL_HIT_POINTS,
};

use super::battle_runtime_reducer_route::{
    observed_reducer_route, ReducerRouteEvent, ReducerRouteFillKind, ReducerRouteHoleKind,
    ReducerRouteOwnerGroup, ReducerRouteResolutionOutcome, ReducerRouteSubjectFamily,
};

pub const BRANCH_ACTIONS: [&str; 3] = [
    "doResolveSapMasteryPropertyHit",
    "doResolveToppleMasteryPropertyFailedSavingThrow",
    "doResolveCleaveMasteryPropertySecondTargetHit",
];

pub fn replay_observed_action(observed_action_taken: &str) -> WeaponMasterySelectedIdentityState {
    match observed_action_taken {
        "doResolveSapMasteryPropertyHit" => {
            weapon_mastery_state_from_action(WeaponMasteryProperty::Sap, false, None)
        }
        "doResolveToppleMasteryPropertyFailedSavingThrow" => {
            weapon_mastery_state_from_action(WeaponMasteryProperty::Topple, true, None)
        }
        "doResolveCleaveMasteryPropertySecondTargetHit" => weapon_mastery_state_from_action(
            WeaponMasteryProperty::Cleave,
            false,
            Some(Actor::Goblin),
        ),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> WeaponMasterySelectedIdentityState {
    match observed_action_taken {
        "doResolveSapMasteryPropertyHit" => expected_state(ExpectedWeaponMasteryState {
            primary_target_hit_points: 9,
            second_target_hit_points: WEAPON_MASTERY_TARGET_INITIAL_HIT_POINTS,
            action_available: false,
            primary_target_has_sap_effect: true,
            primary_target_prone: false,
            cleave_used: false,
            outcome: WeaponMasteryRuntimeOutcome::Resolved,
        }),
        "doResolveToppleMasteryPropertyFailedSavingThrow" => {
            expected_state(ExpectedWeaponMasteryState {
                primary_target_hit_points: WEAPON_MASTERY_TARGET_INITIAL_HIT_POINTS,
                second_target_hit_points: WEAPON_MASTERY_TARGET_INITIAL_HIT_POINTS,
                action_available: true,
                primary_target_has_sap_effect: false,
                primary_target_prone: true,
                cleave_used: false,
                outcome: WeaponMasteryRuntimeOutcome::NeedsHoles,
            })
        }
        "doResolveCleaveMasteryPropertySecondTargetHit" => {
            expected_state(ExpectedWeaponMasteryState {
                primary_target_hit_points: 9,
                second_target_hit_points: 9,
                action_available: false,
                primary_target_has_sap_effect: false,
                primary_target_prone: false,
                cleave_used: true,
                outcome: WeaponMasteryRuntimeOutcome::Resolved,
            })
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doResolveSapMasteryPropertyHit" => {
            observed_weapon_mastery_property_route(WeaponMasteryProperty::Sap, false, None)
        }
        "doResolveToppleMasteryPropertyFailedSavingThrow" => {
            observed_weapon_mastery_property_route(WeaponMasteryProperty::Topple, true, None)
        }
        "doResolveCleaveMasteryPropertySecondTargetHit" => observed_weapon_mastery_property_route(
            WeaponMasteryProperty::Cleave,
            false,
            Some(Actor::Goblin),
        ),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doResolveSapMasteryPropertyHit" => expected_sap_property_route(),
        "doResolveToppleMasteryPropertyFailedSavingThrow" => expected_topple_property_route(),
        "doResolveCleaveMasteryPropertySecondTargetHit" => expected_cleave_property_route(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn projection_payload(state: &WeaponMasterySelectedIdentityState) -> String {
    let protocol_holes = protocol_holes(&state.protocol);
    [
        format!("qPrimaryTargetHp={}", state.primary_target_hit_points),
        format!("qSecondTargetHp={}", state.second_target_hit_points),
        format!("qActionAvailable={}", state.action_available),
        format!(
            "qPrimaryTargetHasSapEffect={}",
            state.primary_target_has_sap_effect
        ),
        format!("qPrimaryTargetProne={}", state.primary_target_prone),
        format!("qCleaveUsed={}", state.cleave_used),
        format!("qLastResult={}", outcome_ref(state.outcome)),
        format!("qHoles={}", joined_or_none(&protocol_holes)),
        format!("protocolResult={}", protocol_result_ref(&state.protocol)),
        format!("protocolHoles={}", joined_or_none(&protocol_holes)),
    ]
    .join("\n")
}

fn outcome_ref(outcome: WeaponMasteryRuntimeOutcome) -> &'static str {
    match outcome {
        WeaponMasteryRuntimeOutcome::Init => "Init",
        WeaponMasteryRuntimeOutcome::NeedsHoles => "NeedsHoles",
        WeaponMasteryRuntimeOutcome::Resolved => "Resolved",
    }
}

fn protocol_result_ref(protocol: &WeaponMasteryRuntimeProtocol) -> &'static str {
    match protocol {
        WeaponMasteryRuntimeProtocol::Init(_) => "init",
        WeaponMasteryRuntimeProtocol::NeedsHoles(_) => "needsHoles",
        WeaponMasteryRuntimeProtocol::Resolved => "resolved",
    }
}

fn protocol_holes(protocol: &WeaponMasteryRuntimeProtocol) -> Vec<&'static str> {
    match protocol {
        WeaponMasteryRuntimeProtocol::Init(holes)
        | WeaponMasteryRuntimeProtocol::NeedsHoles(holes) => holes.iter().map(hole_ref).collect(),
        WeaponMasteryRuntimeProtocol::Resolved => Vec::new(),
    }
}

fn hole_ref(hole: &WeaponMasteryRuntimeHole) -> &'static str {
    match hole {
        WeaponMasteryRuntimeHole::WitnessProtocol => "WitnessProtocolHole",
    }
}

fn joined_or_none(values: &[&str]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}

struct ExpectedWeaponMasteryState {
    primary_target_hit_points: i16,
    second_target_hit_points: i16,
    action_available: bool,
    primary_target_has_sap_effect: bool,
    primary_target_prone: bool,
    cleave_used: bool,
    outcome: WeaponMasteryRuntimeOutcome,
}

fn expected_state(parts: ExpectedWeaponMasteryState) -> WeaponMasterySelectedIdentityState {
    WeaponMasterySelectedIdentityState {
        primary_target_hit_points: parts.primary_target_hit_points,
        second_target_hit_points: parts.second_target_hit_points,
        action_available: parts.action_available,
        primary_target_has_sap_effect: parts.primary_target_has_sap_effect,
        primary_target_prone: parts.primary_target_prone,
        cleave_used: parts.cleave_used,
        protocol: protocol_for_outcome(parts.outcome),
        outcome: parts.outcome,
    }
}

fn protocol_for_outcome(outcome: WeaponMasteryRuntimeOutcome) -> WeaponMasteryRuntimeProtocol {
    match outcome {
        WeaponMasteryRuntimeOutcome::Init => {
            WeaponMasteryRuntimeProtocol::Init(vec![WeaponMasteryRuntimeHole::WitnessProtocol])
        }
        WeaponMasteryRuntimeOutcome::NeedsHoles => WeaponMasteryRuntimeProtocol::NeedsHoles(vec![
            WeaponMasteryRuntimeHole::WitnessProtocol,
        ]),
        WeaponMasteryRuntimeOutcome::Resolved => WeaponMasteryRuntimeProtocol::Resolved,
    }
}

fn weapon_mastery_state_from_action(
    property: WeaponMasteryProperty,
    saving_throw_failed: bool,
    second_target: Option<Actor>,
) -> WeaponMasterySelectedIdentityState {
    let result = resolve_weapon_mastery_property(property, saving_throw_failed, second_target);
    crate::rules::battle_reducer_spine::weapon_mastery_selected_identity_from_battle(result.state())
}

fn resolve_weapon_mastery_property(
    property: WeaponMasteryProperty,
    saving_throw_failed: bool,
    second_target: Option<Actor>,
) -> crate::rules::battle_reducer_spine::BattleResolutionResult {
    let state = start_weapon_mastery_property_battle();
    let subject = weapon_mastery_subject(&state);
    resolve_battle_subject(
        state,
        BattleResolutionRequest::weapon_mastery_property(
            subject,
            BattleWeaponMasteryPropertyFill {
                property,
                primary_target: Actor::Skeleton,
                second_target,
                damage: WEAPON_MASTERY_SAMPLE_WEAPON_DAMAGE,
                saving_throw_failed,
            },
        )
        .expect("weapon mastery property subject should match request"),
    )
}

fn weapon_mastery_subject(
    state: &crate::rules::battle_reducer_spine::BattleState,
) -> BattleSubject {
    discover_battle_acts(state)
        .available_acts()
        .iter()
        .map(|act| act.subject)
        .find(|subject| subject.kind == BattleSubjectKind::WeaponMasteryProperty)
        .expect("weapon mastery property subject should be discoverable")
}

fn observed_weapon_mastery_property_route(
    property: WeaponMasteryProperty,
    saving_throw_failed: bool,
    second_target: Option<Actor>,
) -> Vec<ReducerRouteEvent> {
    let mut trace = BattleEntrypointTrace::default();
    let state = start_weapon_mastery_property_battle_observed(&mut trace);
    let discovery = discover_battle_acts_observed(&state, &mut trace);
    let subject = discovery
        .available_acts()
        .iter()
        .map(|act| act.subject)
        .find(|subject| subject.kind == BattleSubjectKind::WeaponMasteryProperty)
        .expect("weapon mastery property subject should be discoverable");
    let request = BattleResolutionRequest::weapon_mastery_property(
        subject,
        BattleWeaponMasteryPropertyFill {
            property,
            primary_target: Actor::Skeleton,
            second_target,
            damage: WEAPON_MASTERY_SAMPLE_WEAPON_DAMAGE,
            saving_throw_failed,
        },
    )
    .expect("weapon mastery property subject should match request");
    let _result = resolve_battle_subject_observed(state, request, &mut trace);
    observed_reducer_route(&trace, &[ReducerRouteSubjectFamily::WeaponMasteryProperty])
}

fn expected_sap_property_route() -> Vec<ReducerRouteEvent> {
    vec![
        ReducerRouteEvent::StartBattle {
            owner: ReducerRouteOwnerGroup::ActionEconomy,
        },
        ReducerRouteEvent::DiscoverBattleActs {
            subject: ReducerRouteSubjectFamily::WeaponMasteryProperty,
            holes: Vec::new(),
            owner: ReducerRouteOwnerGroup::FeatureResource,
        },
        ReducerRouteEvent::ResolveBattleSubject {
            subject: ReducerRouteSubjectFamily::WeaponMasteryProperty,
            fill: ReducerRouteFillKind::UnitFeatureDecision,
            outcome: ReducerRouteResolutionOutcome::Resolved,
            holes: Vec::new(),
            owner: ReducerRouteOwnerGroup::ActiveEffect,
        },
    ]
}

fn expected_topple_property_route() -> Vec<ReducerRouteEvent> {
    vec![
        ReducerRouteEvent::StartBattle {
            owner: ReducerRouteOwnerGroup::ActionEconomy,
        },
        ReducerRouteEvent::DiscoverBattleActs {
            subject: ReducerRouteSubjectFamily::WeaponMasteryProperty,
            holes: Vec::new(),
            owner: ReducerRouteOwnerGroup::FeatureResource,
        },
        ReducerRouteEvent::ResolveBattleSubject {
            subject: ReducerRouteSubjectFamily::WeaponMasteryProperty,
            fill: ReducerRouteFillKind::UnitFeatureDecision,
            outcome: ReducerRouteResolutionOutcome::NeedsHoles,
            holes: vec![ReducerRouteHoleKind::RolledDice],
            owner: ReducerRouteOwnerGroup::ConditionLifecycle,
        },
    ]
}

fn expected_cleave_property_route() -> Vec<ReducerRouteEvent> {
    vec![
        ReducerRouteEvent::StartBattle {
            owner: ReducerRouteOwnerGroup::ActionEconomy,
        },
        ReducerRouteEvent::DiscoverBattleActs {
            subject: ReducerRouteSubjectFamily::WeaponMasteryProperty,
            holes: Vec::new(),
            owner: ReducerRouteOwnerGroup::FeatureResource,
        },
        ReducerRouteEvent::ResolveBattleSubject {
            subject: ReducerRouteSubjectFamily::WeaponMasteryProperty,
            fill: ReducerRouteFillKind::UnitFeatureDecision,
            outcome: ReducerRouteResolutionOutcome::Resolved,
            holes: Vec::new(),
            owner: ReducerRouteOwnerGroup::FeatureResource,
        },
    ]
}
