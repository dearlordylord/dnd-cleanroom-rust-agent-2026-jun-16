#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConcentrationBreakScenario {
    Init,
    ConcentrationSpellCast,
    DamageSaveNeeded,
    DamageFailedTeardownBeforeNextCommand,
    VoluntaryEndTeardown,
    ReplacementTeardownBeforeNewEffect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConcentrationProtocol {
    Init,
    Resolved,
    NeedsSavingThrow,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConcentrationState {
    pub scenario: ConcentrationBreakScenario,
    pub protocol: ConcentrationProtocol,
    pub damage_taken: i16,
    pub save_dc: i16,
    pub saving_throw_total: i16,
    pub concentration_save_offered: bool,
    pub caster_concentrating: bool,
    pub active_concentration_effect_count: i16,
    pub spell_slot_expended: i16,
    pub teardown_before_next_command: bool,
    pub replacement_started_after_teardown: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConcentrationDamageFacts {
    pub damage_roll_result: i16,
    pub damage_bonus: i16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConcentrationSavingThrowFacts {
    pub saving_throw_total: i16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConcentrationSavingThrowError {
    SaveDidNotFail,
}

#[must_use]
pub fn concentration_initial_state() -> ConcentrationState {
    ConcentrationState {
        scenario: ConcentrationBreakScenario::Init,
        protocol: ConcentrationProtocol::Init,
        damage_taken: 0,
        save_dc: 0,
        saving_throw_total: 0,
        concentration_save_offered: false,
        caster_concentrating: false,
        active_concentration_effect_count: 0,
        spell_slot_expended: 0,
        teardown_before_next_command: false,
        replacement_started_after_teardown: false,
    }
}

#[must_use]
pub fn concentration_damage_total(facts: ConcentrationDamageFacts) -> i16 {
    facts.damage_roll_result + facts.damage_bonus
}

#[must_use]
pub fn concentration_save_dc_for_damage(damage_taken: i16) -> i16 {
    // RAW: cleanroom-input/raw/srd-5.2.1/Rules-Glossary.md
    // "Concentration"; QNT: battle-runtime-concentration-break-teardown.mbt.qnt
    // concentrationSaveDcForDamage.
    let half_damage = damage_taken / 2;
    half_damage.clamp(10, 30)
}

#[must_use]
pub fn cast_concentration_spell(_state: ConcentrationState) -> ConcentrationState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Rules-Glossary.md
    // "Concentration"; QNT: battle-runtime-concentration-break-teardown.mbt.qnt.
    ConcentrationState {
        scenario: ConcentrationBreakScenario::ConcentrationSpellCast,
        protocol: ConcentrationProtocol::Resolved,
        damage_taken: 0,
        save_dc: 0,
        saving_throw_total: 0,
        concentration_save_offered: false,
        caster_concentrating: true,
        active_concentration_effect_count: 1,
        spell_slot_expended: 1,
        teardown_before_next_command: false,
        replacement_started_after_teardown: false,
    }
}

#[must_use]
pub fn request_concentration_save_after_damage(
    _state: ConcentrationState,
    facts: ConcentrationDamageFacts,
) -> ConcentrationState {
    let damage_taken = concentration_damage_total(facts);
    ConcentrationState {
        scenario: ConcentrationBreakScenario::DamageSaveNeeded,
        protocol: ConcentrationProtocol::NeedsSavingThrow,
        damage_taken,
        save_dc: concentration_save_dc_for_damage(damage_taken),
        saving_throw_total: 0,
        concentration_save_offered: true,
        caster_concentrating: true,
        active_concentration_effect_count: 1,
        spell_slot_expended: 1,
        teardown_before_next_command: false,
        replacement_started_after_teardown: false,
    }
}

pub fn fail_concentration_saving_throw(
    state: ConcentrationState,
    facts: ConcentrationSavingThrowFacts,
) -> Result<ConcentrationState, ConcentrationSavingThrowError> {
    if facts.saving_throw_total >= state.save_dc {
        return Err(ConcentrationSavingThrowError::SaveDidNotFail);
    }

    Ok(ConcentrationState {
        scenario: ConcentrationBreakScenario::DamageFailedTeardownBeforeNextCommand,
        protocol: ConcentrationProtocol::Resolved,
        damage_taken: state.damage_taken,
        save_dc: state.save_dc,
        saving_throw_total: facts.saving_throw_total,
        concentration_save_offered: false,
        caster_concentrating: false,
        active_concentration_effect_count: 0,
        spell_slot_expended: 1,
        teardown_before_next_command: true,
        replacement_started_after_teardown: false,
    })
}

#[must_use]
pub fn voluntarily_end_concentration(_state: ConcentrationState) -> ConcentrationState {
    ConcentrationState {
        scenario: ConcentrationBreakScenario::VoluntaryEndTeardown,
        protocol: ConcentrationProtocol::Resolved,
        damage_taken: 0,
        save_dc: 0,
        saving_throw_total: 0,
        concentration_save_offered: false,
        caster_concentrating: false,
        active_concentration_effect_count: 0,
        spell_slot_expended: 1,
        teardown_before_next_command: true,
        replacement_started_after_teardown: false,
    }
}

#[must_use]
pub fn cast_replacement_concentration_spell(_state: ConcentrationState) -> ConcentrationState {
    ConcentrationState {
        scenario: ConcentrationBreakScenario::ReplacementTeardownBeforeNewEffect,
        protocol: ConcentrationProtocol::Resolved,
        damage_taken: 0,
        save_dc: 0,
        saving_throw_total: 0,
        concentration_save_offered: false,
        caster_concentrating: true,
        active_concentration_effect_count: 1,
        spell_slot_expended: 1,
        teardown_before_next_command: false,
        replacement_started_after_teardown: true,
    }
}
