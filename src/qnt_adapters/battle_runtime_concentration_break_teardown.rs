use crate::rules::concentration::{
    cast_concentration_spell, cast_replacement_concentration_spell, concentration_damage_total,
    concentration_initial_state, fail_concentration_saving_throw,
    request_concentration_save_after_damage, voluntarily_end_concentration,
    ConcentrationBreakScenario, ConcentrationDamageFacts, ConcentrationProtocol,
    ConcentrationSavingThrowFacts, ConcentrationState,
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

pub fn expected_witness(observed_action_taken: &str) -> ConcentrationBreakTeardownWitness {
    replay_observed_action(observed_action_taken)
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
