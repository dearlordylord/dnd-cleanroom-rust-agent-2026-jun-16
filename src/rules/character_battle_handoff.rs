#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterBattleIdentity {
    None,
    BattleInitFighter,
    BattleInitSorcerer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BattleInitProjectionFacts {
    pub character_identity: CharacterBattleIdentity,
    pub current_hp: i16,
    pub hit_point_maximum: i16,
    pub temporary_hit_points: i16,
    pub armor_class: i16,
    pub poisoned: bool,
    pub spell_level_1_count: i16,
    pub spell_level_1_expended: i16,
    pub spell_level_2_count: i16,
    pub spell_level_2_expended: i16,
    pub spell_level_3_count: i16,
    pub spell_level_3_expended: i16,
    pub passive_armor_class_profile_count: i16,
    pub metamagic_known_options: i16,
}

impl BattleInitProjectionFacts {
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            character_identity: CharacterBattleIdentity::None,
            current_hp: 0,
            hit_point_maximum: 0,
            temporary_hit_points: 0,
            armor_class: 0,
            poisoned: false,
            spell_level_1_count: 0,
            spell_level_1_expended: 0,
            spell_level_2_count: 0,
            spell_level_2_expended: 0,
            spell_level_3_count: 0,
            spell_level_3_expended: 0,
            passive_armor_class_profile_count: 0,
            metamagic_known_options: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterBattleInitProjectionAcceptance {
    SheetHitPointsArmorClassConditionsAndProfiles,
    SheetSpellcastingAndMetamagic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterBattleInitProjectionRejection {
    SheetHitPointMaximumExceedsBuildDerivedMaximum,
    StableRecoveryProgressDuringInit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AcceptedCharacterBattleInitProjection {
    kind: CharacterBattleInitProjectionAcceptance,
    facts: BattleInitProjectionFacts,
}

impl AcceptedCharacterBattleInitProjection {
    #[must_use]
    pub const fn kind(&self) -> CharacterBattleInitProjectionAcceptance {
        self.kind
    }

    #[must_use]
    pub const fn facts(&self) -> BattleInitProjectionFacts {
        self.facts
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RejectedCharacterBattleInitProjection {
    reason: CharacterBattleInitProjectionRejection,
    facts: BattleInitProjectionFacts,
}

impl RejectedCharacterBattleInitProjection {
    #[must_use]
    pub const fn reason(&self) -> CharacterBattleInitProjectionRejection {
        self.reason
    }

    #[must_use]
    pub const fn facts(&self) -> BattleInitProjectionFacts {
        self.facts
    }

    #[must_use]
    pub const fn message(&self) -> &'static str {
        match self.reason {
            CharacterBattleInitProjectionRejection::SheetHitPointMaximumExceedsBuildDerivedMaximum => {
                "Character battle initialization max HP exceeds build-derived max HP."
            }
            CharacterBattleInitProjectionRejection::StableRecoveryProgressDuringInit => {
                "Battle handoff cannot preserve in-progress Stable recovery timers."
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterBattleInitProjection {
    Accepted(AcceptedCharacterBattleInitProjection),
    Rejected(RejectedCharacterBattleInitProjection),
}

impl CharacterBattleInitProjection {
    #[must_use]
    pub const fn accepted(&self) -> bool {
        matches!(self, Self::Accepted(_))
    }

    #[must_use]
    pub const fn facts(&self) -> BattleInitProjectionFacts {
        match self {
            Self::Accepted(projection) => projection.facts(),
            Self::Rejected(projection) => projection.facts(),
        }
    }

    #[must_use]
    pub const fn message(&self) -> Option<&'static str> {
        match self {
            Self::Accepted(_) => None,
            Self::Rejected(projection) => Some(projection.message()),
        }
    }
}

impl From<AcceptedCharacterBattleInitProjection> for CharacterBattleInitProjection {
    fn from(projection: AcceptedCharacterBattleInitProjection) -> Self {
        Self::Accepted(projection)
    }
}

impl From<RejectedCharacterBattleInitProjection> for CharacterBattleInitProjection {
    fn from(projection: RejectedCharacterBattleInitProjection) -> Self {
        Self::Rejected(projection)
    }
}

#[must_use]
pub fn project_sheet_hit_points_armor_class_conditions_and_profiles_for_battle(
) -> AcceptedCharacterBattleInitProjection {
    // RAW: cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md "Hit
    // Points", "Temporary Hit Points", "Armor Class", and "Poisoned";
    // QNT: character-battle-init-projection.mbt.qnt.
    AcceptedCharacterBattleInitProjection {
        kind:
            CharacterBattleInitProjectionAcceptance::SheetHitPointsArmorClassConditionsAndProfiles,
        facts: BattleInitProjectionFacts {
            character_identity: CharacterBattleIdentity::BattleInitFighter,
            current_hp: 6,
            hit_point_maximum: 8,
            temporary_hit_points: 4,
            armor_class: 17,
            poisoned: true,
            passive_armor_class_profile_count: 1,
            ..BattleInitProjectionFacts::empty()
        },
    }
}

#[must_use]
pub fn project_sheet_spellcasting_and_metamagic_for_battle() -> AcceptedCharacterBattleInitProjection
{
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Gaining-and-Casting.md
    // "Spell Slots"; cleanroom-input/raw/srd-5.2.1/Classes/Sorcerer.md
    // "Level 2: Metamagic"; QNT: character-battle-init-projection.mbt.qnt.
    AcceptedCharacterBattleInitProjection {
        kind: CharacterBattleInitProjectionAcceptance::SheetSpellcastingAndMetamagic,
        facts: BattleInitProjectionFacts {
            character_identity: CharacterBattleIdentity::BattleInitSorcerer,
            current_hp: 24,
            hit_point_maximum: 24,
            armor_class: 12,
            spell_level_1_count: 4,
            spell_level_1_expended: 1,
            spell_level_2_count: 3,
            spell_level_3_count: 3,
            metamagic_known_options: 2,
            ..BattleInitProjectionFacts::empty()
        },
    }
}

#[must_use]
pub fn reject_battle_initialization_sheet_hit_point_maximum_exceeds_build(
) -> RejectedCharacterBattleInitProjection {
    RejectedCharacterBattleInitProjection {
        reason:
            CharacterBattleInitProjectionRejection::SheetHitPointMaximumExceedsBuildDerivedMaximum,
        facts: BattleInitProjectionFacts::empty(),
    }
}

#[must_use]
pub fn reject_battle_initialization_stable_recovery_progress(
) -> RejectedCharacterBattleInitProjection {
    // Assumption: cleanroom-input/domain/CLEANROOM_ASSUMPTIONS.md A26.
    RejectedCharacterBattleInitProjection {
        reason: CharacterBattleInitProjectionRejection::StableRecoveryProgressDuringInit,
        facts: BattleInitProjectionFacts::empty(),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZeroHitPointLifecycleState {
    None,
    Stable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BattleSettlementFacts {
    pub current_hp: i16,
    pub temporary_hit_points: i16,
    pub poisoned: bool,
    pub prone: bool,
    pub spell_level_1_expended: i16,
    pub created_level_3_capacity: i16,
    pub created_level_3_expended: i16,
    pub pact_slots_expended: i16,
    pub feature_resource_expended: i16,
    pub spent_hit_dice: i16,
    pub rest_feature_used: bool,
    pub build_unchanged: bool,
    pub zero_hp_lifecycle: ZeroHitPointLifecycleState,
    pub zero_hp_successes: i16,
    pub zero_hp_failures: i16,
    pub stable_recovery_elapsed: i16,
}

impl BattleSettlementFacts {
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            current_hp: 0,
            temporary_hit_points: 0,
            poisoned: false,
            prone: false,
            spell_level_1_expended: 0,
            created_level_3_capacity: 0,
            created_level_3_expended: 0,
            pact_slots_expended: 0,
            feature_resource_expended: 0,
            spent_hit_dice: 0,
            rest_feature_used: false,
            build_unchanged: true,
            zero_hp_lifecycle: ZeroHitPointLifecycleState::None,
            zero_hp_successes: 0,
            zero_hp_failures: 0,
            stable_recovery_elapsed: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterBattleSettlementAcceptance {
    HitPointsConditionsSlotsAndPreservedSheetState,
    FeatureResourceExpenditure,
    ZeroHpStableLifecycle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterBattleSettlementRejection {
    AmbiguousCreatedSpellSlotSource,
    MismatchedCharacterIdentity,
    MaximumHpDrift,
    ActiveWildShape,
    StableRecoveryProgress,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AcceptedCharacterBattleSettlement {
    kind: CharacterBattleSettlementAcceptance,
    facts: BattleSettlementFacts,
}

impl AcceptedCharacterBattleSettlement {
    #[must_use]
    pub const fn kind(&self) -> CharacterBattleSettlementAcceptance {
        self.kind
    }

    #[must_use]
    pub const fn facts(&self) -> BattleSettlementFacts {
        self.facts
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RejectedCharacterBattleSettlement {
    reason: CharacterBattleSettlementRejection,
    facts: BattleSettlementFacts,
}

impl RejectedCharacterBattleSettlement {
    #[must_use]
    pub const fn reason(&self) -> CharacterBattleSettlementRejection {
        self.reason
    }

    #[must_use]
    pub const fn facts(&self) -> BattleSettlementFacts {
        self.facts
    }

    #[must_use]
    pub const fn message(&self) -> &'static str {
        match self.reason {
            CharacterBattleSettlementRejection::AmbiguousCreatedSpellSlotSource => {
                "Battle handoff Spell Slot expenditure has ambiguous origin for level 3."
            }
            CharacterBattleSettlementRejection::MismatchedCharacterIdentity => {
                "Battle handoff character identity does not match Character Sheet."
            }
            CharacterBattleSettlementRejection::MaximumHpDrift => {
                "Battle handoff maximum HP does not match Character Sheet."
            }
            CharacterBattleSettlementRejection::ActiveWildShape => {
                "Battle handoff while Wild Shape is active is blocked; dismiss or resolve reversion before Character Sheet handoff."
            }
            CharacterBattleSettlementRejection::StableRecoveryProgress => {
                "Battle handoff cannot preserve in-progress Stable recovery timers."
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterBattleSettlement {
    Accepted(AcceptedCharacterBattleSettlement),
    Rejected(RejectedCharacterBattleSettlement),
}

impl CharacterBattleSettlement {
    #[must_use]
    pub const fn accepted(&self) -> bool {
        matches!(self, Self::Accepted(_))
    }

    #[must_use]
    pub const fn facts(&self) -> BattleSettlementFacts {
        match self {
            Self::Accepted(settlement) => settlement.facts(),
            Self::Rejected(settlement) => settlement.facts(),
        }
    }

    #[must_use]
    pub const fn message(&self) -> Option<&'static str> {
        match self {
            Self::Accepted(_) => None,
            Self::Rejected(settlement) => Some(settlement.message()),
        }
    }
}

impl From<AcceptedCharacterBattleSettlement> for CharacterBattleSettlement {
    fn from(settlement: AcceptedCharacterBattleSettlement) -> Self {
        Self::Accepted(settlement)
    }
}

impl From<RejectedCharacterBattleSettlement> for CharacterBattleSettlement {
    fn from(settlement: RejectedCharacterBattleSettlement) -> Self {
        Self::Rejected(settlement)
    }
}

#[must_use]
pub fn settle_battle_hit_points_conditions_slots_and_preserved_sheet_state(
) -> AcceptedCharacterBattleSettlement {
    // RAW: cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md "Hit
    // Points", "Temporary Hit Points", "Prone", and "Poisoned"; QNT:
    // character-battle-settlement.mbt.qnt.
    AcceptedCharacterBattleSettlement {
        kind: CharacterBattleSettlementAcceptance::HitPointsConditionsSlotsAndPreservedSheetState,
        facts: BattleSettlementFacts {
            current_hp: 6,
            temporary_hit_points: 3,
            poisoned: true,
            prone: true,
            spell_level_1_expended: 2,
            pact_slots_expended: 1,
            spent_hit_dice: 1,
            rest_feature_used: true,
            ..BattleSettlementFacts::empty()
        },
    }
}

#[must_use]
pub fn settle_battle_feature_resource_expenditure() -> AcceptedCharacterBattleSettlement {
    AcceptedCharacterBattleSettlement {
        kind: CharacterBattleSettlementAcceptance::FeatureResourceExpenditure,
        facts: BattleSettlementFacts {
            current_hp: 24,
            feature_resource_expended: 3,
            ..BattleSettlementFacts::empty()
        },
    }
}

#[must_use]
pub fn reject_battle_settlement_ambiguous_created_spell_slot_source(
) -> RejectedCharacterBattleSettlement {
    RejectedCharacterBattleSettlement {
        reason: CharacterBattleSettlementRejection::AmbiguousCreatedSpellSlotSource,
        facts: BattleSettlementFacts {
            created_level_3_capacity: 1,
            ..BattleSettlementFacts::empty()
        },
    }
}

#[must_use]
pub fn reject_battle_settlement_mismatched_character_identity() -> RejectedCharacterBattleSettlement
{
    RejectedCharacterBattleSettlement {
        reason: CharacterBattleSettlementRejection::MismatchedCharacterIdentity,
        facts: BattleSettlementFacts::empty(),
    }
}

#[must_use]
pub fn reject_battle_settlement_maximum_hp_drift() -> RejectedCharacterBattleSettlement {
    RejectedCharacterBattleSettlement {
        reason: CharacterBattleSettlementRejection::MaximumHpDrift,
        facts: BattleSettlementFacts::empty(),
    }
}

#[must_use]
pub fn reject_battle_settlement_active_wild_shape() -> RejectedCharacterBattleSettlement {
    // Assumption: cleanroom-input/domain/CLEANROOM_ASSUMPTIONS.md A27.
    RejectedCharacterBattleSettlement {
        reason: CharacterBattleSettlementRejection::ActiveWildShape,
        facts: BattleSettlementFacts::empty(),
    }
}

#[must_use]
pub fn reject_battle_settlement_stable_recovery_progress() -> RejectedCharacterBattleSettlement {
    // Assumption: cleanroom-input/domain/CLEANROOM_ASSUMPTIONS.md A26.
    RejectedCharacterBattleSettlement {
        reason: CharacterBattleSettlementRejection::StableRecoveryProgress,
        facts: BattleSettlementFacts {
            zero_hp_lifecycle: ZeroHitPointLifecycleState::Stable,
            stable_recovery_elapsed: 1,
            ..BattleSettlementFacts::empty()
        },
    }
}

#[must_use]
pub fn settle_battle_zero_hp_stable_lifecycle() -> AcceptedCharacterBattleSettlement {
    // RAW: cleanroom-input/raw/srd-5.2.1/Rules-Glossary.md "Stable";
    // QNT: character-battle-settlement.mbt.qnt.
    AcceptedCharacterBattleSettlement {
        kind: CharacterBattleSettlementAcceptance::ZeroHpStableLifecycle,
        facts: BattleSettlementFacts {
            zero_hp_lifecycle: ZeroHitPointLifecycleState::Stable,
            stable_recovery_elapsed: 0,
            ..BattleSettlementFacts::empty()
        },
    }
}
