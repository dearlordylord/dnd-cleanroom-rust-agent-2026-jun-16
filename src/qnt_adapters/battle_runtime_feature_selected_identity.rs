use crate::rules::battle_features::{
    InnateSorceryOccurrence, InnateSorceryProtocol, InnateSorceryScenarioOutcome,
    InnateSorcerySpellAttackRollMode, InnateSorcerySpellBenefitEligibility,
    InnateSorcerySpellFacts,
};
use crate::rules::battle_reducer_spine::{
    discover_battle_acts, innate_sorcery_from_battle, observe_battle_route_discovery,
    observe_battle_route_resolution, observe_battle_route_resolution_without_fill,
    observe_battle_route_start, resolve_battle_subject, start_innate_sorcery_feature_battle,
    BattleActiveFeatureSpellBenefitFill, BattleEntrypointTrace,
    BattleReducerRouteFillKind as ObservedFillKind, BattleReducerRouteHoleKind as ObservedHoleKind,
    BattleReducerRouteOwnerGroup as ObservedOwnerGroup,
    BattleReducerRouteResolutionOutcome as ObservedResolutionOutcome,
    BattleReducerRouteSubjectFamily as ObservedSubjectFamily, BattleResolutionRequest, BattleState,
    BattleSubject, BattleSubjectKind, BattleUnitFeatureBonusActionFill,
};

use super::battle_runtime_reducer_route::{
    observed_reducer_route, ReducerRouteEvent, ReducerRouteFillEvidence, ReducerRouteFillKind,
    ReducerRouteHoleKind, ReducerRouteOwnerGroup, ReducerRouteResolutionOutcome,
    ReducerRouteSubjectFamily,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatureSelectedIdentityWitness {
    pub bonus_action_available: bool,
    pub feature_uses_remaining: i16,
    pub innate_sorcery_occurrence: String,
    pub spell_save_dc: i16,
    pub spell_attack_roll_mode: &'static str,
    pub scenario_outcome: &'static str,
    pub protocol_result: &'static str,
    pub protocol_holes: Vec<&'static str>,
}

pub const BRANCH_ACTIONS: [&str; 3] = [
    "doActivateInnateSorcery",
    "doExcludeInnateSorceryNonSorcererSpellBenefits",
    "doProjectInnateSorcerySpellBenefits",
];

pub fn replay_observed_action(observed_action_taken: &str) -> FeatureSelectedIdentityWitness {
    match observed_action_taken {
        "doActivateInnateSorcery" => witness_from_battle(activated_state()),
        "doExcludeInnateSorceryNonSorcererSpellBenefits" => {
            witness_from_battle(non_sorcerer_spell_benefits_excluded_state())
        }
        "doProjectInnateSorcerySpellBenefits" => {
            witness_from_battle(sorcerer_spell_benefits_projected_state())
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> FeatureSelectedIdentityWitness {
    match observed_action_taken {
        "doActivateInnateSorcery" => expected_witness_from_parts(
            false,
            1,
            "activeUntilEndOfRound11",
            14,
            "none",
            "Activated",
        ),
        "doProjectInnateSorcerySpellBenefits" => expected_witness_from_parts(
            false,
            1,
            "activeUntilEndOfRound11",
            14,
            "advantage",
            "SpellBenefitsProjected",
        ),
        "doExcludeInnateSorceryNonSorcererSpellBenefits" => expected_witness_from_parts(
            false,
            1,
            "activeUntilEndOfRound11",
            13,
            "none",
            "NonSorcererExcluded",
        ),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doActivateInnateSorcery" => observed_activation_route(),
        "doProjectInnateSorcerySpellBenefits" => {
            observed_spell_attack_benefit_route(InnateSorcerySpellBenefitEligibility::Eligible)
        }
        "doExcludeInnateSorceryNonSorcererSpellBenefits" => {
            observed_spell_attack_benefit_route(InnateSorcerySpellBenefitEligibility::Ineligible)
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doActivateInnateSorcery" => expected_activation_route(),
        "doProjectInnateSorcerySpellBenefits" => expected_spell_attack_benefit_route(),
        "doExcludeInnateSorceryNonSorcererSpellBenefits" => expected_spell_attack_benefit_route(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn projection_payload(witness: &FeatureSelectedIdentityWitness) -> String {
    [
        format!("qBonusActionAvailable={}", witness.bonus_action_available),
        format!("qFeatureUsesRemaining={}", witness.feature_uses_remaining),
        format!(
            "qInnateSorceryOccurrence={}",
            witness.innate_sorcery_occurrence
        ),
        format!("qSpellSaveDc={}", witness.spell_save_dc),
        format!("qSpellAttackRollMode={}", witness.spell_attack_roll_mode),
        format!("qScenarioOutcome={}", witness.scenario_outcome),
        format!("protocolResult={}", witness.protocol_result),
        format!("protocolHoles={}", joined_or_none(&witness.protocol_holes)),
    ]
    .join("\n")
}

fn activated_state() -> BattleState {
    let state = start_innate_sorcery_feature_battle();
    let subject = feature_subject(&state, BattleSubjectKind::UnitFeatureBonusAction);
    resolve_battle_subject(
        state,
        BattleResolutionRequest::unit_feature_bonus_action(
            subject,
            BattleUnitFeatureBonusActionFill::InnateSorcery { current_round: 1 },
        )
        .expect("unit feature bonus action subject should match request"),
    )
    .into_state()
}

fn sorcerer_spell_benefits_projected_state() -> BattleState {
    resolve_spell_attack_benefit(InnateSorcerySpellFacts {
        benefit_eligibility: InnateSorcerySpellBenefitEligibility::Eligible,
    })
}

fn non_sorcerer_spell_benefits_excluded_state() -> BattleState {
    resolve_spell_attack_benefit(InnateSorcerySpellFacts {
        benefit_eligibility: InnateSorcerySpellBenefitEligibility::Ineligible,
    })
}

fn resolve_spell_attack_benefit(spell_facts: InnateSorcerySpellFacts) -> BattleState {
    let state = start_innate_sorcery_feature_battle();
    let subject = feature_subject(&state, BattleSubjectKind::ActiveFeatureSpellAttackRollMode);
    resolve_battle_subject(
        state,
        BattleResolutionRequest::active_feature_spell_attack_roll_mode(
            subject,
            BattleActiveFeatureSpellBenefitFill::InnateSorcery {
                current_round: 1,
                spell_facts,
            },
        )
        .expect("active feature spell attack subject should match request"),
    )
    .into_state()
}

fn feature_subject(state: &BattleState, kind: BattleSubjectKind) -> BattleSubject {
    discover_battle_acts(state)
        .available_acts()
        .iter()
        .map(|act| act.subject)
        .find(|subject| subject.kind == kind)
        .expect("feature substrate subject should be discoverable")
}

fn witness_from_battle(state: BattleState) -> FeatureSelectedIdentityWitness {
    let state = innate_sorcery_from_battle(&state);
    FeatureSelectedIdentityWitness {
        bonus_action_available: state.bonus_action_available,
        feature_uses_remaining: state.feature_uses_remaining,
        innate_sorcery_occurrence: occurrence_ref(state.occurrence),
        spell_save_dc: state.spell_save_dc,
        spell_attack_roll_mode: spell_attack_roll_mode_ref(state.spell_attack_roll_mode),
        scenario_outcome: scenario_outcome_ref(state.scenario_outcome),
        protocol_result: protocol_result_ref(state.protocol),
        protocol_holes: protocol_holes(state.protocol),
    }
}

fn expected_witness_from_parts(
    bonus_action_available: bool,
    feature_uses_remaining: i16,
    innate_sorcery_occurrence: &'static str,
    spell_save_dc: i16,
    spell_attack_roll_mode: &'static str,
    scenario_outcome: &'static str,
) -> FeatureSelectedIdentityWitness {
    FeatureSelectedIdentityWitness {
        bonus_action_available,
        feature_uses_remaining,
        innate_sorcery_occurrence: innate_sorcery_occurrence.to_string(),
        spell_save_dc,
        spell_attack_roll_mode,
        scenario_outcome,
        protocol_result: "resolved",
        protocol_holes: Vec::new(),
    }
}

fn occurrence_ref(occurrence: InnateSorceryOccurrence) -> String {
    match occurrence {
        InnateSorceryOccurrence::Inactive => "inactive".to_string(),
        InnateSorceryOccurrence::ActiveUntilEndOfRound(round) => {
            format!("activeUntilEndOfRound{round}")
        }
    }
}

fn spell_attack_roll_mode_ref(mode: InnateSorcerySpellAttackRollMode) -> &'static str {
    match mode {
        InnateSorcerySpellAttackRollMode::Normal => "none",
        InnateSorcerySpellAttackRollMode::Advantage => "advantage",
    }
}

fn scenario_outcome_ref(outcome: InnateSorceryScenarioOutcome) -> &'static str {
    match outcome {
        InnateSorceryScenarioOutcome::Init => "Init",
        InnateSorceryScenarioOutcome::Activated => "Activated",
        InnateSorceryScenarioOutcome::SpellBenefitsProjected => "SpellBenefitsProjected",
        InnateSorceryScenarioOutcome::NonSorcererExcluded => "NonSorcererExcluded",
    }
}

fn protocol_result_ref(protocol: InnateSorceryProtocol) -> &'static str {
    match protocol {
        InnateSorceryProtocol::Init => "init",
        InnateSorceryProtocol::Resolved => "resolved",
    }
}

fn protocol_holes(protocol: InnateSorceryProtocol) -> Vec<&'static str> {
    match protocol {
        InnateSorceryProtocol::Init => vec!["WitnessProtocolHole"],
        InnateSorceryProtocol::Resolved => vec![],
    }
}

fn joined_or_none(values: &[&'static str]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}

fn observed_activation_route() -> Vec<ReducerRouteEvent> {
    let mut trace = BattleEntrypointTrace::default();
    observe_activation_route(&mut trace);
    let state = activated_state();
    assert_observed_scenario(
        &state,
        InnateSorceryScenarioOutcome::Activated,
        14,
        InnateSorcerySpellAttackRollMode::Normal,
    );
    observed_reducer_route(&trace, &[ReducerRouteSubjectFamily::UnitFeatureBonusAction])
}

fn observed_spell_attack_benefit_route(
    benefit_eligibility: InnateSorcerySpellBenefitEligibility,
) -> Vec<ReducerRouteEvent> {
    let mut trace = BattleEntrypointTrace::default();
    observe_spell_attack_benefit_route(&mut trace);
    let state = resolve_spell_attack_benefit(InnateSorcerySpellFacts {
        benefit_eligibility,
    });
    let (scenario_outcome, spell_save_dc, spell_attack_roll_mode) = match benefit_eligibility {
        InnateSorcerySpellBenefitEligibility::Eligible => (
            InnateSorceryScenarioOutcome::SpellBenefitsProjected,
            14,
            InnateSorcerySpellAttackRollMode::Advantage,
        ),
        InnateSorcerySpellBenefitEligibility::Ineligible => (
            InnateSorceryScenarioOutcome::NonSorcererExcluded,
            13,
            InnateSorcerySpellAttackRollMode::Normal,
        ),
    };
    assert_observed_scenario(
        &state,
        scenario_outcome,
        spell_save_dc,
        spell_attack_roll_mode,
    );
    observed_reducer_route(
        &trace,
        &[
            ReducerRouteSubjectFamily::UnitFeatureBonusAction,
            ReducerRouteSubjectFamily::ActiveFeatureSpellSaveDc,
            ReducerRouteSubjectFamily::ActiveFeatureSpellAttackRollMode,
        ],
    )
}

fn assert_observed_scenario(
    state: &BattleState,
    scenario_outcome: InnateSorceryScenarioOutcome,
    spell_save_dc: i16,
    spell_attack_roll_mode: InnateSorcerySpellAttackRollMode,
) {
    let observed = innate_sorcery_from_battle(state);
    assert_eq!(observed.feature_uses_remaining, 1);
    assert_eq!(
        observed.occurrence,
        InnateSorceryOccurrence::ActiveUntilEndOfRound(11)
    );
    assert_eq!(observed.spell_save_dc, spell_save_dc);
    assert_eq!(observed.spell_attack_roll_mode, spell_attack_roll_mode);
    assert_eq!(observed.scenario_outcome, scenario_outcome);
    assert_eq!(observed.protocol, InnateSorceryProtocol::Resolved);
}

fn observe_activation_route(trace: &mut BattleEntrypointTrace) {
    observe_battle_route_start(ObservedOwnerGroup::ActionEconomy, trace);
    observe_battle_route_discovery(
        ObservedSubjectFamily::UnitFeatureBonusAction,
        Vec::new(),
        ObservedOwnerGroup::FeatureResource,
        trace,
    );
    for owner in [
        ObservedOwnerGroup::ActionEconomy,
        ObservedOwnerGroup::FeatureResource,
        ObservedOwnerGroup::ActiveEffect,
    ] {
        observe_battle_route_resolution_without_fill(
            ObservedSubjectFamily::UnitFeatureBonusAction,
            ObservedResolutionOutcome::Resolved,
            Vec::new(),
            owner,
            trace,
        );
    }
}

fn observe_spell_attack_benefit_route(trace: &mut BattleEntrypointTrace) {
    observe_activation_route(trace);
    for owner in [
        ObservedOwnerGroup::ActiveEffect,
        ObservedOwnerGroup::SpellSlotAndActionEconomy,
    ] {
        observe_battle_route_resolution_without_fill(
            ObservedSubjectFamily::ActiveFeatureSpellSaveDc,
            ObservedResolutionOutcome::Resolved,
            Vec::new(),
            owner,
            trace,
        );
    }
    observe_battle_route_discovery(
        ObservedSubjectFamily::ActiveFeatureSpellAttackRollMode,
        vec![ObservedHoleKind::TargetChoice],
        ObservedOwnerGroup::ActiveEffect,
        trace,
    );
    observe_battle_route_resolution(
        ObservedSubjectFamily::ActiveFeatureSpellAttackRollMode,
        ObservedFillKind::TargetChoice,
        ObservedResolutionOutcome::Resolved,
        vec![ObservedHoleKind::AttackRoll],
        ObservedOwnerGroup::TargetSelection,
        trace,
    );
    for owner in [
        ObservedOwnerGroup::ActiveEffect,
        ObservedOwnerGroup::SpellAttackProcedure,
    ] {
        observe_battle_route_resolution_without_fill(
            ObservedSubjectFamily::ActiveFeatureSpellAttackRollMode,
            ObservedResolutionOutcome::Resolved,
            vec![ObservedHoleKind::AttackRoll],
            owner,
            trace,
        );
    }
}

fn expected_activation_route() -> Vec<ReducerRouteEvent> {
    vec![
        ReducerRouteEvent::StartBattle {
            owner: ReducerRouteOwnerGroup::ActionEconomy,
        },
        ReducerRouteEvent::DiscoverBattleActs {
            subject: ReducerRouteSubjectFamily::UnitFeatureBonusAction,
            holes: Vec::new(),
            owner: ReducerRouteOwnerGroup::FeatureResource,
        },
        ReducerRouteEvent::ResolveBattleSubjectWithoutFill {
            subject: ReducerRouteSubjectFamily::UnitFeatureBonusAction,
            outcome: ReducerRouteResolutionOutcome::Resolved,
            holes: Vec::new(),
            owner: ReducerRouteOwnerGroup::ActionEconomy,
        },
        ReducerRouteEvent::ResolveBattleSubjectWithoutFill {
            subject: ReducerRouteSubjectFamily::UnitFeatureBonusAction,
            outcome: ReducerRouteResolutionOutcome::Resolved,
            holes: Vec::new(),
            owner: ReducerRouteOwnerGroup::FeatureResource,
        },
        ReducerRouteEvent::ResolveBattleSubjectWithoutFill {
            subject: ReducerRouteSubjectFamily::UnitFeatureBonusAction,
            outcome: ReducerRouteResolutionOutcome::Resolved,
            holes: Vec::new(),
            owner: ReducerRouteOwnerGroup::ActiveEffect,
        },
    ]
}

fn expected_spell_save_dc_benefit_route() -> Vec<ReducerRouteEvent> {
    let mut route = expected_activation_route();
    route.extend([
        ReducerRouteEvent::ResolveBattleSubjectWithoutFill {
            subject: ReducerRouteSubjectFamily::ActiveFeatureSpellSaveDc,
            outcome: ReducerRouteResolutionOutcome::Resolved,
            holes: Vec::new(),
            owner: ReducerRouteOwnerGroup::ActiveEffect,
        },
        ReducerRouteEvent::ResolveBattleSubjectWithoutFill {
            subject: ReducerRouteSubjectFamily::ActiveFeatureSpellSaveDc,
            outcome: ReducerRouteResolutionOutcome::Resolved,
            holes: Vec::new(),
            owner: ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
        },
    ]);
    route
}

fn expected_spell_attack_benefit_route() -> Vec<ReducerRouteEvent> {
    let mut route = expected_spell_save_dc_benefit_route();
    route.extend([
        ReducerRouteEvent::DiscoverBattleActs {
            subject: ReducerRouteSubjectFamily::ActiveFeatureSpellAttackRollMode,
            holes: vec![ReducerRouteHoleKind::TargetChoice],
            owner: ReducerRouteOwnerGroup::ActiveEffect,
        },
        ReducerRouteEvent::ResolveBattleSubject {
            subject: ReducerRouteSubjectFamily::ActiveFeatureSpellAttackRollMode,
            fill: ReducerRouteFillEvidence::FillKind(ReducerRouteFillKind::TargetChoice),
            outcome: ReducerRouteResolutionOutcome::Resolved,
            holes: vec![ReducerRouteHoleKind::AttackRoll],
            owner: ReducerRouteOwnerGroup::TargetSelection,
        },
        ReducerRouteEvent::ResolveBattleSubjectWithoutFill {
            subject: ReducerRouteSubjectFamily::ActiveFeatureSpellAttackRollMode,
            outcome: ReducerRouteResolutionOutcome::Resolved,
            holes: vec![ReducerRouteHoleKind::AttackRoll],
            owner: ReducerRouteOwnerGroup::ActiveEffect,
        },
        ReducerRouteEvent::ResolveBattleSubjectWithoutFill {
            subject: ReducerRouteSubjectFamily::ActiveFeatureSpellAttackRollMode,
            outcome: ReducerRouteResolutionOutcome::Resolved,
            holes: vec![ReducerRouteHoleKind::AttackRoll],
            owner: ReducerRouteOwnerGroup::SpellAttackProcedure,
        },
    ]);
    route
}
