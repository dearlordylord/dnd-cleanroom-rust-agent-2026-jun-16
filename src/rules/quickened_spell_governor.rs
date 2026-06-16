pub const QUICKENED_INITIAL_SORCERY_POINTS: i16 = 4;
pub const QUICKENED_HIGH_SORCERY_POINTS: i16 = 5;
pub const QUICKENED_UNAFFORDABLE_SORCERY_POINTS: i16 = 1;
pub const QUICKENED_INITIAL_TARGET_HIT_POINTS: i16 = 4;
pub const QUICKENED_HEALING_RESULT_HIT_POINTS: i16 = 14;
pub const QUICKENED_SORCERY_POINT_COST: i16 = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuickenedSpellScenarioOutcome {
    Init,
    ResolvedQuickenedRestoration,
    ResolvedQuickenedSaveGatedCondition,
    ResolvedQuickenedSaveGatedConditionImmunity,
    ResolvedQuickenedDirectCondition,
    ResolvedQuickenedRollModifier,
    ResolvedAfterMagicActionSpent,
    RejectedUnaffordable,
    RejectedUnknownOption,
    RejectedUnsupportedSecondOption,
    RejectedOnePerSpell,
    RejectedPriorLevelOnePlusSpell,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuickenedSpellInvalidKind {
    None,
    Unaffordable,
    UnknownOption,
    UnsupportedSecondOption,
    OnePerSpell,
    SameTurnLevelOnePlus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuickenedSpellProtocol {
    Init,
    Resolved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuickenedSpellGovernorState {
    pub quickened_cure_wounds_offered: bool,
    pub color_spray_blinded: bool,
    pub calm_emotions_immunity: bool,
    pub invisibility_active: bool,
    pub bless_active: bool,
    pub magic_action_available: bool,
    pub bonus_action_available: bool,
    pub sorcery_points_remaining: i16,
    pub target_hit_points: i16,
    pub spell_slot_committed: bool,
    pub level_one_plus_cast_this_turn: bool,
    pub quickened_level_one_plus_cast_this_turn: bool,
    pub spell_slot_acts_available: bool,
    pub invalid_kind: QuickenedSpellInvalidKind,
    pub scenario_outcome: QuickenedSpellScenarioOutcome,
    pub protocol: QuickenedSpellProtocol,
}

#[must_use]
pub fn quickened_spell_governor_initial_state() -> QuickenedSpellGovernorState {
    QuickenedSpellGovernorState {
        quickened_cure_wounds_offered: true,
        color_spray_blinded: false,
        calm_emotions_immunity: false,
        invisibility_active: false,
        bless_active: false,
        magic_action_available: true,
        bonus_action_available: true,
        sorcery_points_remaining: QUICKENED_INITIAL_SORCERY_POINTS,
        target_hit_points: QUICKENED_INITIAL_TARGET_HIT_POINTS,
        spell_slot_committed: false,
        level_one_plus_cast_this_turn: false,
        quickened_level_one_plus_cast_this_turn: false,
        spell_slot_acts_available: true,
        invalid_kind: QuickenedSpellInvalidKind::None,
        scenario_outcome: QuickenedSpellScenarioOutcome::Init,
        protocol: QuickenedSpellProtocol::Init,
    }
}

#[must_use]
pub fn resolve_quickened_restoration() -> QuickenedSpellGovernorState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Classes/Sorcerer.md
    // "Quickened Spell"; QNT: battle-runtime-quickened-spell-governor.mbt.qnt.
    resolved_state(
        QUICKENED_HEALING_RESULT_HIT_POINTS,
        QuickenedSpellScenarioOutcome::ResolvedQuickenedRestoration,
        ResolvedFlags::default(),
    )
}

#[must_use]
pub fn resolve_quickened_save_gated_condition() -> QuickenedSpellGovernorState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Classes/Sorcerer.md
    // "Quickened Spell"; QNT: battle-runtime-quickened-spell-governor.mbt.qnt.
    resolved_state(
        QUICKENED_INITIAL_TARGET_HIT_POINTS,
        QuickenedSpellScenarioOutcome::ResolvedQuickenedSaveGatedCondition,
        ResolvedFlags {
            color_spray_blinded: true,
            ..ResolvedFlags::default()
        },
    )
}

#[must_use]
pub fn resolve_quickened_save_gated_condition_immunity() -> QuickenedSpellGovernorState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Classes/Sorcerer.md
    // "Quickened Spell"; QNT: battle-runtime-quickened-spell-governor.mbt.qnt.
    resolved_state(
        QUICKENED_INITIAL_TARGET_HIT_POINTS,
        QuickenedSpellScenarioOutcome::ResolvedQuickenedSaveGatedConditionImmunity,
        ResolvedFlags {
            calm_emotions_immunity: true,
            ..ResolvedFlags::default()
        },
    )
}

#[must_use]
pub fn resolve_quickened_direct_condition() -> QuickenedSpellGovernorState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Classes/Sorcerer.md
    // "Quickened Spell"; QNT: battle-runtime-quickened-spell-governor.mbt.qnt.
    resolved_state(
        QUICKENED_INITIAL_TARGET_HIT_POINTS,
        QuickenedSpellScenarioOutcome::ResolvedQuickenedDirectCondition,
        ResolvedFlags {
            invisibility_active: true,
            ..ResolvedFlags::default()
        },
    )
}

#[must_use]
pub fn resolve_quickened_roll_modifier() -> QuickenedSpellGovernorState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Classes/Sorcerer.md
    // "Quickened Spell"; QNT: battle-runtime-quickened-spell-governor.mbt.qnt.
    resolved_state(
        QUICKENED_INITIAL_TARGET_HIT_POINTS,
        QuickenedSpellScenarioOutcome::ResolvedQuickenedRollModifier,
        ResolvedFlags {
            bless_active: true,
            ..ResolvedFlags::default()
        },
    )
}

#[must_use]
pub fn resolve_quickened_after_magic_action_spent() -> QuickenedSpellGovernorState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Classes/Sorcerer.md
    // "Quickened Spell"; QNT: battle-runtime-quickened-spell-governor.mbt.qnt.
    QuickenedSpellGovernorState {
        magic_action_available: false,
        ..resolved_state(
            QUICKENED_HEALING_RESULT_HIT_POINTS,
            QuickenedSpellScenarioOutcome::ResolvedAfterMagicActionSpent,
            ResolvedFlags::default(),
        )
    }
}

#[must_use]
pub fn reject_quickened_unaffordable() -> QuickenedSpellGovernorState {
    rejected_state(
        false,
        QUICKENED_UNAFFORDABLE_SORCERY_POINTS,
        false,
        QuickenedSpellInvalidKind::Unaffordable,
        QuickenedSpellScenarioOutcome::RejectedUnaffordable,
    )
}

#[must_use]
pub fn reject_quickened_unknown_option() -> QuickenedSpellGovernorState {
    rejected_state(
        true,
        QUICKENED_INITIAL_SORCERY_POINTS,
        false,
        QuickenedSpellInvalidKind::UnknownOption,
        QuickenedSpellScenarioOutcome::RejectedUnknownOption,
    )
}

#[must_use]
pub fn reject_quickened_unsupported_second_option() -> QuickenedSpellGovernorState {
    rejected_state(
        true,
        QUICKENED_HIGH_SORCERY_POINTS,
        false,
        QuickenedSpellInvalidKind::UnsupportedSecondOption,
        QuickenedSpellScenarioOutcome::RejectedUnsupportedSecondOption,
    )
}

#[must_use]
pub fn reject_quickened_one_per_spell() -> QuickenedSpellGovernorState {
    rejected_state(
        true,
        QUICKENED_HIGH_SORCERY_POINTS,
        false,
        QuickenedSpellInvalidKind::OnePerSpell,
        QuickenedSpellScenarioOutcome::RejectedOnePerSpell,
    )
}

#[must_use]
pub fn reject_quickened_prior_level_one_plus_spell() -> QuickenedSpellGovernorState {
    rejected_state(
        false,
        QUICKENED_INITIAL_SORCERY_POINTS,
        true,
        QuickenedSpellInvalidKind::SameTurnLevelOnePlus,
        QuickenedSpellScenarioOutcome::RejectedPriorLevelOnePlusSpell,
    )
}

#[derive(Debug, Clone, Copy, Default)]
struct ResolvedFlags {
    color_spray_blinded: bool,
    calm_emotions_immunity: bool,
    invisibility_active: bool,
    bless_active: bool,
}

fn resolved_state(
    target_hit_points: i16,
    scenario_outcome: QuickenedSpellScenarioOutcome,
    flags: ResolvedFlags,
) -> QuickenedSpellGovernorState {
    QuickenedSpellGovernorState {
        quickened_cure_wounds_offered: false,
        color_spray_blinded: flags.color_spray_blinded,
        calm_emotions_immunity: flags.calm_emotions_immunity,
        invisibility_active: flags.invisibility_active,
        bless_active: flags.bless_active,
        magic_action_available: true,
        bonus_action_available: false,
        sorcery_points_remaining: QUICKENED_INITIAL_SORCERY_POINTS - QUICKENED_SORCERY_POINT_COST,
        target_hit_points,
        spell_slot_committed: true,
        level_one_plus_cast_this_turn: true,
        quickened_level_one_plus_cast_this_turn: true,
        spell_slot_acts_available: false,
        invalid_kind: QuickenedSpellInvalidKind::None,
        scenario_outcome,
        protocol: QuickenedSpellProtocol::Resolved,
    }
}

fn rejected_state(
    quickened_cure_wounds_offered: bool,
    sorcery_points_remaining: i16,
    level_one_plus_cast_this_turn: bool,
    invalid_kind: QuickenedSpellInvalidKind,
    scenario_outcome: QuickenedSpellScenarioOutcome,
) -> QuickenedSpellGovernorState {
    QuickenedSpellGovernorState {
        quickened_cure_wounds_offered,
        sorcery_points_remaining,
        level_one_plus_cast_this_turn,
        invalid_kind,
        scenario_outcome,
        protocol: QuickenedSpellProtocol::Resolved,
        ..quickened_spell_governor_initial_state()
    }
}
