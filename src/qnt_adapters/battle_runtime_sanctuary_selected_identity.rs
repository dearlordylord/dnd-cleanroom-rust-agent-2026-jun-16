use crate::rules::battle_reducer_spine::{
    discover_battle_acts_observed, protection_charm_ward_from_battle,
    resolve_battle_subject_observed, start_protection_charm_ward_battle_observed,
    BattleEntrypointTrace, BattleHoleKind, BattleProtectionCharmWardFill,
    BattleProtectionCharmWardProjection, BattleProtectionCharmWardSubject, BattleResolutionRequest,
    BattleResolutionResult, BattleSubjectKind,
};
use crate::rules::sanctuary_selected_identity::{
    SanctuarySelectedIdentityProtocol, SanctuarySelectedIdentityScenarioOutcome,
    SanctuarySelectedIdentityState,
};

use super::battle_runtime_reducer_route::{
    observed_reducer_route, route_discover_battle_acts, route_resolve_battle_subject,
    route_start_battle, ReducerRouteEvent, ReducerRouteFillKind, ReducerRouteOwnerGroup,
    ReducerRouteSubjectFamily,
};

pub const BRANCH_ACTIONS: [&str; 9] = [
    "doCastSanctuaryWardCreation",
    "doInterdictDirectAttackFailedSaveLoss",
    "doInterdictDirectSpellSuccessfulSavePassThrough",
    "doRetargetDirectAttackToLegalReplacement",
    "doRejectIllegalReplacementTarget",
    "doExcludeAreaEffectFromInterdiction",
    "doEndWardOnWardedAttackRoll",
    "doEndWardOnWardedSpellCast",
    "doEndWardOnWardedDamageDealt",
];

pub fn replay_observed_action(observed_action_taken: &str) -> SanctuarySelectedIdentityState {
    replay_observed_state_and_route(observed_action_taken).0
}

pub fn expected_witness(observed_action_taken: &str) -> SanctuarySelectedIdentityState {
    replay_observed_action(observed_action_taken)
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    replay_observed_state_and_route(observed_action_taken).1
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    let (subject, holes, owner) = expected_route_witness(observed_action_taken);
    vec![
        route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
        route_discover_battle_acts(subject, holes, owner),
        route_resolve_battle_subject(
            subject,
            ReducerRouteFillKind::UnitFeatureDecision,
            Vec::new(),
            owner,
        ),
    ]
}

pub fn projection_payload(state: &SanctuarySelectedIdentityState) -> String {
    [
        format!("qWardPresent={}", state.ward_present),
        format!("qWardSourceIsSanctuary={}", state.ward_source_is_sanctuary),
        format!("qWisdomSaveRequested={}", state.wisdom_save_requested),
        format!("qAttackOrSpellLost={}", state.attack_or_spell_lost),
        format!(
            "qSuccessfulSavePassThrough={}",
            state.successful_save_pass_through
        ),
        format!(
            "qLegalReplacementPassThrough={}",
            state.legal_replacement_pass_through
        ),
        format!(
            "qIllegalReplacementRejected={}",
            state.illegal_replacement_rejected
        ),
        format!(
            "qAreaEffectBypassedInterdiction={}",
            state.area_effect_bypassed_interdiction
        ),
        format!("qWardedHp={}", state.warded_hp),
        format!(
            "qScenarioOutcome={}",
            scenario_outcome_ref(state.scenario_outcome)
        ),
        format!("protocolResult={}", protocol_ref(state.protocol)),
        "protocolHoles=none".to_string(),
    ]
    .join("\n")
}

fn scenario_outcome_ref(outcome: SanctuarySelectedIdentityScenarioOutcome) -> &'static str {
    match outcome {
        SanctuarySelectedIdentityScenarioOutcome::Init => "Init",
        SanctuarySelectedIdentityScenarioOutcome::WardCreated => "WardCreated",
        SanctuarySelectedIdentityScenarioOutcome::AttackLost => "AttackLost",
        SanctuarySelectedIdentityScenarioOutcome::SpellSaveSucceeded => "SpellSaveSucceeded",
        SanctuarySelectedIdentityScenarioOutcome::ReplacementAdmitted => "ReplacementAdmitted",
        SanctuarySelectedIdentityScenarioOutcome::ReplacementRejected => "ReplacementRejected",
        SanctuarySelectedIdentityScenarioOutcome::AreaEffectExcluded => "AreaEffectExcluded",
        SanctuarySelectedIdentityScenarioOutcome::AttackRollEndedWard => "AttackRollEndedWard",
        SanctuarySelectedIdentityScenarioOutcome::SpellCastEndedWard => "SpellCastEndedWard",
        SanctuarySelectedIdentityScenarioOutcome::DamageEndedWard => "DamageEndedWard",
    }
}

fn protocol_ref(protocol: SanctuarySelectedIdentityProtocol) -> &'static str {
    match protocol {
        SanctuarySelectedIdentityProtocol::Init => "init",
        SanctuarySelectedIdentityProtocol::Resolved => "resolved",
    }
}

fn replay_observed_state_and_route(
    observed_action_taken: &str,
) -> (SanctuarySelectedIdentityState, Vec<ReducerRouteEvent>) {
    let fill = fill_for_action(observed_action_taken);
    let mut trace = BattleEntrypointTrace::default();
    let state = start_protection_charm_ward_battle_observed(fill, &mut trace);
    let discovery = discover_battle_acts_observed(&state, &mut trace);
    let subject = discovery
        .available_acts()
        .iter()
        .map(|act| act.subject)
        .find(|subject| subject.kind == route_subject_kind(fill.subject()))
        .expect("ward route subject should be discoverable");
    let request = BattleResolutionRequest::protection_charm_ward(subject, fill)
        .expect("ward fill should match route subject");
    let result = resolve_battle_subject_observed(state, request, &mut trace);
    let BattleResolutionResult::Resolved { state } = result else {
        panic!("{observed_action_taken} should resolve through ward route")
    };
    let projection = protection_charm_ward_from_battle(&state, fill);
    let BattleProtectionCharmWardProjection::WardingInterdiction(ward) = projection else {
        panic!("{observed_action_taken} should project ward/interdiction facts")
    };
    (
        ward,
        observed_reducer_route(&trace, &[route_subject_family(fill.subject())]),
    )
}

fn fill_for_action(observed_action_taken: &str) -> BattleProtectionCharmWardFill {
    match observed_action_taken {
        "doCastSanctuaryWardCreation" => BattleProtectionCharmWardFill::WardingInterdictionCreation,
        "doInterdictDirectAttackFailedSaveLoss" => {
            BattleProtectionCharmWardFill::WardingInterdictionFailedSaveLoss
        }
        "doInterdictDirectSpellSuccessfulSavePassThrough" => {
            BattleProtectionCharmWardFill::WardingInterdictionSuccessfulSavePassThrough
        }
        "doRetargetDirectAttackToLegalReplacement" => {
            BattleProtectionCharmWardFill::WardingInterdictionLegalReplacement
        }
        "doRejectIllegalReplacementTarget" => {
            BattleProtectionCharmWardFill::WardingInterdictionIllegalReplacement
        }
        "doExcludeAreaEffectFromInterdiction" => {
            BattleProtectionCharmWardFill::WardingInterdictionAreaEffectExcluded
        }
        "doEndWardOnWardedAttackRoll" => {
            BattleProtectionCharmWardFill::WardingActiveEffectEndedByAttackRoll
        }
        "doEndWardOnWardedSpellCast" => {
            BattleProtectionCharmWardFill::WardingActiveEffectEndedBySpellCast
        }
        "doEndWardOnWardedDamageDealt" => {
            BattleProtectionCharmWardFill::WardingActiveEffectEndedByDamageDealt
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

fn route_subject_family(subject: BattleProtectionCharmWardSubject) -> ReducerRouteSubjectFamily {
    match subject {
        BattleProtectionCharmWardSubject::TargetAdmission => {
            ReducerRouteSubjectFamily::TargetAdmission
        }
        BattleProtectionCharmWardSubject::ActiveEffect => ReducerRouteSubjectFamily::ActiveEffect,
        BattleProtectionCharmWardSubject::ConditionLifecycle => {
            ReducerRouteSubjectFamily::ConditionLifecycle
        }
        BattleProtectionCharmWardSubject::SavingThrowRollMode => {
            ReducerRouteSubjectFamily::SavingThrowRollMode
        }
        BattleProtectionCharmWardSubject::TargetInterdiction => {
            ReducerRouteSubjectFamily::TargetInterdiction
        }
    }
}

fn expected_route_witness(
    observed_action_taken: &str,
) -> (
    ReducerRouteSubjectFamily,
    Vec<BattleHoleKind>,
    ReducerRouteOwnerGroup,
) {
    match observed_action_taken {
        "doCastSanctuaryWardCreation"
        | "doEndWardOnWardedAttackRoll"
        | "doEndWardOnWardedSpellCast"
        | "doEndWardOnWardedDamageDealt" => (
            ReducerRouteSubjectFamily::ActiveEffect,
            Vec::new(),
            ReducerRouteOwnerGroup::ActiveEffect,
        ),
        "doInterdictDirectAttackFailedSaveLoss"
        | "doInterdictDirectSpellSuccessfulSavePassThrough"
        | "doRetargetDirectAttackToLegalReplacement"
        | "doRejectIllegalReplacementTarget"
        | "doExcludeAreaEffectFromInterdiction" => (
            ReducerRouteSubjectFamily::TargetInterdiction,
            vec![BattleHoleKind::TargetChoice],
            ReducerRouteOwnerGroup::TargetInterdiction,
        ),
        action => panic!("unsupported expected route action {action}"),
    }
}

fn route_subject_kind(subject: BattleProtectionCharmWardSubject) -> BattleSubjectKind {
    match subject {
        BattleProtectionCharmWardSubject::TargetAdmission => BattleSubjectKind::TargetAdmission,
        BattleProtectionCharmWardSubject::ActiveEffect => BattleSubjectKind::ActiveEffect,
        BattleProtectionCharmWardSubject::ConditionLifecycle => {
            BattleSubjectKind::ConditionLifecycle
        }
        BattleProtectionCharmWardSubject::SavingThrowRollMode => {
            BattleSubjectKind::SavingThrowRollMode
        }
        BattleProtectionCharmWardSubject::TargetInterdiction => {
            BattleSubjectKind::TargetInterdiction
        }
    }
}
