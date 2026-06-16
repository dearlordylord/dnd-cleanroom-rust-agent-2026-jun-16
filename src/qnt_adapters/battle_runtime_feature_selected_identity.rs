use crate::rules::battle_features::{
    activate_innate_sorcery, innate_sorcery_initial_state, project_innate_sorcery_spell_benefits,
    sorcerer_spell_save_dc, InnateSorceryOccurrence, InnateSorceryProtocol,
    InnateSorceryScenarioOutcome, InnateSorcerySpellAttackRollMode,
    InnateSorcerySpellBenefitEligibility, InnateSorcerySpellFacts, InnateSorceryState,
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
        "doActivateInnateSorcery" => witness_from_state(activated()),
        "doExcludeInnateSorceryNonSorcererSpellBenefits" => {
            witness_from_state(non_sorcerer_spell_benefits_excluded())
        }
        "doProjectInnateSorcerySpellBenefits" => {
            witness_from_state(sorcerer_spell_benefits_projected())
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> FeatureSelectedIdentityWitness {
    replay_observed_action(observed_action_taken)
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

fn activated() -> InnateSorceryState {
    activate_innate_sorcery(initial_state(), 1)
}

fn sorcerer_spell_benefits_projected() -> InnateSorceryState {
    project_innate_sorcery_spell_benefits(
        initial_state(),
        1,
        InnateSorcerySpellFacts {
            benefit_eligibility: InnateSorcerySpellBenefitEligibility::Eligible,
        },
    )
}

fn non_sorcerer_spell_benefits_excluded() -> InnateSorceryState {
    project_innate_sorcery_spell_benefits(
        initial_state(),
        1,
        InnateSorcerySpellFacts {
            benefit_eligibility: InnateSorcerySpellBenefitEligibility::Ineligible,
        },
    )
}

fn initial_state() -> InnateSorceryState {
    innate_sorcery_initial_state(sorcerer_spell_save_dc(3, 2))
}

fn witness_from_state(state: InnateSorceryState) -> FeatureSelectedIdentityWitness {
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
