const INITIAL_WARDED_HP: i16 = 12;
const DAMAGE_DEALT_BY_WARDED_CREATURE: i16 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SanctuarySelectedIdentityScenarioOutcome {
    Init,
    WardCreated,
    AttackLost,
    SpellSaveSucceeded,
    ReplacementAdmitted,
    ReplacementRejected,
    AreaEffectExcluded,
    AttackRollEndedWard,
    SpellCastEndedWard,
    DamageEndedWard,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SanctuarySelectedIdentityProtocol {
    Init,
    Resolved,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SanctuarySelectedIdentityState {
    pub ward_present: bool,
    pub ward_source_is_sanctuary: bool,
    pub wisdom_save_requested: bool,
    pub attack_or_spell_lost: bool,
    pub successful_save_pass_through: bool,
    pub legal_replacement_pass_through: bool,
    pub illegal_replacement_rejected: bool,
    pub area_effect_bypassed_interdiction: bool,
    pub warded_hp: i16,
    pub scenario_outcome: SanctuarySelectedIdentityScenarioOutcome,
    pub protocol: SanctuarySelectedIdentityProtocol,
}

#[must_use]
pub fn sanctuary_selected_identity_initial_state() -> SanctuarySelectedIdentityState {
    base_projection(
        false,
        false,
        INITIAL_WARDED_HP,
        SanctuarySelectedIdentityScenarioOutcome::Init,
        SanctuarySelectedIdentityProtocol::Init,
    )
}

#[must_use]
pub fn cast_sanctuary_ward_creation() -> SanctuarySelectedIdentityState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-S-Z.md
    // "Sanctuary"; QNT: battle-runtime-sanctuary-selected-identity.mbt.qnt.
    warded_projection(SanctuarySelectedIdentityScenarioOutcome::WardCreated)
}

#[must_use]
pub fn interdict_direct_attack_failed_save_loss() -> SanctuarySelectedIdentityState {
    let mut state = warded_projection(SanctuarySelectedIdentityScenarioOutcome::AttackLost);
    state.wisdom_save_requested = true;
    state.attack_or_spell_lost = true;
    state
}

#[must_use]
pub fn interdict_direct_spell_successful_save_pass_through() -> SanctuarySelectedIdentityState {
    let mut state = warded_projection(SanctuarySelectedIdentityScenarioOutcome::SpellSaveSucceeded);
    state.wisdom_save_requested = true;
    state.successful_save_pass_through = true;
    state
}

#[must_use]
pub fn retarget_direct_attack_to_legal_replacement() -> SanctuarySelectedIdentityState {
    let mut state =
        warded_projection(SanctuarySelectedIdentityScenarioOutcome::ReplacementAdmitted);
    state.wisdom_save_requested = true;
    state.legal_replacement_pass_through = true;
    state
}

#[must_use]
pub fn reject_illegal_replacement_target() -> SanctuarySelectedIdentityState {
    let mut state =
        warded_projection(SanctuarySelectedIdentityScenarioOutcome::ReplacementRejected);
    state.wisdom_save_requested = true;
    state.illegal_replacement_rejected = true;
    state
}

#[must_use]
pub fn exclude_area_effect_from_interdiction() -> SanctuarySelectedIdentityState {
    let mut state = warded_projection(SanctuarySelectedIdentityScenarioOutcome::AreaEffectExcluded);
    state.area_effect_bypassed_interdiction = true;
    state
}

#[must_use]
pub fn end_ward_on_warded_attack_roll() -> SanctuarySelectedIdentityState {
    ended_projection(
        INITIAL_WARDED_HP,
        SanctuarySelectedIdentityScenarioOutcome::AttackRollEndedWard,
    )
}

#[must_use]
pub fn end_ward_on_warded_spell_cast() -> SanctuarySelectedIdentityState {
    ended_projection(
        INITIAL_WARDED_HP,
        SanctuarySelectedIdentityScenarioOutcome::SpellCastEndedWard,
    )
}

#[must_use]
pub fn end_ward_on_warded_damage_dealt() -> SanctuarySelectedIdentityState {
    ended_projection(
        INITIAL_WARDED_HP - DAMAGE_DEALT_BY_WARDED_CREATURE,
        SanctuarySelectedIdentityScenarioOutcome::DamageEndedWard,
    )
}

fn warded_projection(
    scenario_outcome: SanctuarySelectedIdentityScenarioOutcome,
) -> SanctuarySelectedIdentityState {
    base_projection(
        true,
        true,
        INITIAL_WARDED_HP,
        scenario_outcome,
        SanctuarySelectedIdentityProtocol::Resolved,
    )
}

fn ended_projection(
    warded_hp: i16,
    scenario_outcome: SanctuarySelectedIdentityScenarioOutcome,
) -> SanctuarySelectedIdentityState {
    base_projection(
        false,
        false,
        warded_hp,
        scenario_outcome,
        SanctuarySelectedIdentityProtocol::Resolved,
    )
}

fn base_projection(
    ward_present: bool,
    ward_source_is_sanctuary: bool,
    warded_hp: i16,
    scenario_outcome: SanctuarySelectedIdentityScenarioOutcome,
    protocol: SanctuarySelectedIdentityProtocol,
) -> SanctuarySelectedIdentityState {
    SanctuarySelectedIdentityState {
        ward_present,
        ward_source_is_sanctuary,
        wisdom_save_requested: false,
        attack_or_spell_lost: false,
        successful_save_pass_through: false,
        legal_replacement_pass_through: false,
        illegal_replacement_rejected: false,
        area_effect_bypassed_interdiction: false,
        warded_hp,
        scenario_outcome,
        protocol,
    }
}
