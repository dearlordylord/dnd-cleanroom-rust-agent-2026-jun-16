use crate::rules::sanctuary_selected_identity::{
    cast_sanctuary_ward_creation, end_ward_on_warded_attack_roll, end_ward_on_warded_damage_dealt,
    end_ward_on_warded_spell_cast, exclude_area_effect_from_interdiction,
    interdict_direct_attack_failed_save_loss, interdict_direct_spell_successful_save_pass_through,
    reject_illegal_replacement_target, retarget_direct_attack_to_legal_replacement,
    SanctuarySelectedIdentityProtocol, SanctuarySelectedIdentityScenarioOutcome,
    SanctuarySelectedIdentityState,
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
    match observed_action_taken {
        "doCastSanctuaryWardCreation" => cast_sanctuary_ward_creation(),
        "doInterdictDirectAttackFailedSaveLoss" => interdict_direct_attack_failed_save_loss(),
        "doInterdictDirectSpellSuccessfulSavePassThrough" => {
            interdict_direct_spell_successful_save_pass_through()
        }
        "doRetargetDirectAttackToLegalReplacement" => retarget_direct_attack_to_legal_replacement(),
        "doRejectIllegalReplacementTarget" => reject_illegal_replacement_target(),
        "doExcludeAreaEffectFromInterdiction" => exclude_area_effect_from_interdiction(),
        "doEndWardOnWardedAttackRoll" => end_ward_on_warded_attack_roll(),
        "doEndWardOnWardedSpellCast" => end_ward_on_warded_spell_cast(),
        "doEndWardOnWardedDamageDealt" => end_ward_on_warded_damage_dealt(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> SanctuarySelectedIdentityState {
    replay_observed_action(observed_action_taken)
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
