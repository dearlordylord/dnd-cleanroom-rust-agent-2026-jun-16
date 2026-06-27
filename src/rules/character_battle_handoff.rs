#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterBattleIdentity {
    None,
    BattleInitFighter,
    BattleInitSorcerer,
    BattleInitWarlock,
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
    PurePactMagicSlot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterBattleInitProjectionRejection {
    SheetHitPointMaximumExceedsBuildDerivedMaximum,
    MixedSpellAndPactSlot,
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
            CharacterBattleInitProjectionRejection::MixedSpellAndPactSlot => {
                "Battle handoff cannot project mixed Spell Slot and Pact Slot state without origin-distinct battle slots."
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
            hit_point_maximum: 9,
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
            hit_point_maximum: 27,
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
pub fn project_pure_pact_magic_slot_for_battle() -> AcceptedCharacterBattleInitProjection {
    // RAW: cleanroom-input/raw/srd-5.2.1/Classes/Warlock.md "Pact Magic";
    // QNT: character-battle-init-projection.mbt.qnt.
    AcceptedCharacterBattleInitProjection {
        kind: CharacterBattleInitProjectionAcceptance::PurePactMagicSlot,
        facts: BattleInitProjectionFacts {
            character_identity: CharacterBattleIdentity::BattleInitWarlock,
            current_hp: 8,
            hit_point_maximum: 9,
            armor_class: 12,
            spell_level_1_count: 1,
            ..BattleInitProjectionFacts::empty()
        },
    }
}

#[must_use]
pub fn reject_battle_initialization_mixed_spell_and_pact_slot(
) -> RejectedCharacterBattleInitProjection {
    RejectedCharacterBattleInitProjection {
        reason: CharacterBattleInitProjectionRejection::MixedSpellAndPactSlot,
        facts: BattleInitProjectionFacts::empty(),
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
    PurePactMagicSlotExpenditure,
    FeatureResourceExpenditure,
    ZeroHpStableLifecycle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterBattleSettlementRejection {
    MixedSpellAndPactSlot,
    AmbiguousCreatedSpellSlotOrigin,
    CharacterSheetMismatch,
    MaximumHpDrift,
    ActiveWildShape,
    ActiveBattleState,
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
            CharacterBattleSettlementRejection::MixedSpellAndPactSlot => {
                "Battle handoff cannot project mixed Spell Slot and Pact Slot state without origin-distinct battle slots."
            }
            CharacterBattleSettlementRejection::AmbiguousCreatedSpellSlotOrigin => {
                "Battle handoff Spell Slot expenditure is origin-ambiguous for level 3."
            }
            CharacterBattleSettlementRejection::CharacterSheetMismatch => {
                "Battle handoff character does not match Character Sheet."
            }
            CharacterBattleSettlementRejection::MaximumHpDrift => {
                "Battle handoff maximum HP does not match Character Sheet."
            }
            CharacterBattleSettlementRejection::ActiveWildShape => {
                "Battle handoff while Wild Shape is active is blocked; dismiss or resolve reversion before Character Sheet handoff."
            }
            CharacterBattleSettlementRejection::ActiveBattleState => {
                "Battle handoff while active battle effects or Concentration are present is blocked; end or resolve battle-local effects before Character Sheet handoff."
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
            spent_hit_dice: 1,
            rest_feature_used: true,
            ..BattleSettlementFacts::empty()
        },
    }
}

#[must_use]
pub fn settle_battle_pure_pact_magic_slot_expenditure() -> AcceptedCharacterBattleSettlement {
    AcceptedCharacterBattleSettlement {
        kind: CharacterBattleSettlementAcceptance::PurePactMagicSlotExpenditure,
        facts: BattleSettlementFacts {
            current_hp: 8,
            pact_slots_expended: 1,
            ..BattleSettlementFacts::empty()
        },
    }
}

#[must_use]
pub fn reject_battle_settlement_mixed_spell_and_pact_slot() -> RejectedCharacterBattleSettlement {
    RejectedCharacterBattleSettlement {
        reason: CharacterBattleSettlementRejection::MixedSpellAndPactSlot,
        facts: BattleSettlementFacts::empty(),
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

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterBattleRouteSubjectFamily {
    SheetToBattleInitRouteSubject,
    BattleToSheetSettlementRouteSubject,
    HandoffResourceProjectionRouteSubject,
    HandoffFeatureResourceProjectionRouteSubject,
    HandoffSelectedReferenceRouteSubject,
    HandoffBattleMutationRouteSubject,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CharacterBattleRouteHoleFamily {
    HandoffIdentityMatchHoleFamily,
    HandoffHitPointProjectionHoleFamily,
    HandoffSpellResourceProjectionHoleFamily,
    HandoffFeatureResourceProjectionHoleFamily,
    HandoffSettlementConflictHoleFamily,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterBattleRouteFillFamily {
    HandoffSheetProjectionFill,
    HandoffBattleDeltaFill,
    HandoffResourceDeltaFill,
    HandoffSettlementRejectionFill,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterBattleRouteOwnerGroup {
    CharacterBattleSheetOwner,
    CharacterBattleBuildProjectionOwner,
    CharacterBattleInitProjectionOwner,
    CharacterBattleRuntimeOwner,
    CharacterBattleSettlementOwner,
    CharacterBattleResourceProjectionOwner,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CharacterBattleRouteEvent {
    RouteProjectCharacterSheetToBattle {
        subject: CharacterBattleRouteSubjectFamily,
        owner: CharacterBattleRouteOwnerGroup,
    },
    RouteEnterBattleRuntime {
        subject: CharacterBattleRouteSubjectFamily,
        owner: CharacterBattleRouteOwnerGroup,
    },
    RouteSettleBattleToCharacterSheet {
        subject: CharacterBattleRouteSubjectFamily,
        fill: CharacterBattleRouteFillFamily,
        holes: Vec<CharacterBattleRouteHoleFamily>,
        owner: CharacterBattleRouteOwnerGroup,
    },
    RouteRejectCharacterBattleHandoff {
        subject: CharacterBattleRouteSubjectFamily,
        fill: CharacterBattleRouteFillFamily,
        holes: Vec<CharacterBattleRouteHoleFamily>,
        owner: CharacterBattleRouteOwnerGroup,
    },
}

#[must_use]
pub fn route_project_character_sheet_to_battle(
    subject: CharacterBattleRouteSubjectFamily,
    owner: CharacterBattleRouteOwnerGroup,
) -> CharacterBattleRouteEvent {
    CharacterBattleRouteEvent::RouteProjectCharacterSheetToBattle { subject, owner }
}

#[must_use]
pub fn route_enter_battle_runtime(
    subject: CharacterBattleRouteSubjectFamily,
    owner: CharacterBattleRouteOwnerGroup,
) -> CharacterBattleRouteEvent {
    CharacterBattleRouteEvent::RouteEnterBattleRuntime { subject, owner }
}

#[must_use]
pub fn route_settle_battle_to_character_sheet(
    subject: CharacterBattleRouteSubjectFamily,
    fill: CharacterBattleRouteFillFamily,
    holes: Vec<CharacterBattleRouteHoleFamily>,
    owner: CharacterBattleRouteOwnerGroup,
) -> CharacterBattleRouteEvent {
    CharacterBattleRouteEvent::RouteSettleBattleToCharacterSheet {
        subject,
        fill,
        holes: sorted_holes(holes),
        owner,
    }
}

#[must_use]
pub fn route_reject_character_battle_handoff(
    subject: CharacterBattleRouteSubjectFamily,
    fill: CharacterBattleRouteFillFamily,
    holes: Vec<CharacterBattleRouteHoleFamily>,
    owner: CharacterBattleRouteOwnerGroup,
) -> CharacterBattleRouteEvent {
    CharacterBattleRouteEvent::RouteRejectCharacterBattleHandoff {
        subject,
        fill,
        holes: sorted_holes(holes),
        owner,
    }
}

#[must_use]
pub fn character_battle_route_payload(route: &[CharacterBattleRouteEvent]) -> String {
    route
        .iter()
        .map(route_event_payload)
        .collect::<Vec<_>>()
        .join("\n")
}

fn sorted_holes(
    mut holes: Vec<CharacterBattleRouteHoleFamily>,
) -> Vec<CharacterBattleRouteHoleFamily> {
    holes.sort();
    holes
}

fn route_event_payload(event: &CharacterBattleRouteEvent) -> String {
    match event {
        CharacterBattleRouteEvent::RouteProjectCharacterSheetToBattle { subject, owner } => {
            format!(
                "RouteProjectCharacterSheetToBattle subject={} owner={}",
                subject_ref(*subject),
                owner_ref(*owner)
            )
        }
        CharacterBattleRouteEvent::RouteEnterBattleRuntime { subject, owner } => {
            format!(
                "RouteEnterBattleRuntime subject={} owner={}",
                subject_ref(*subject),
                owner_ref(*owner)
            )
        }
        CharacterBattleRouteEvent::RouteSettleBattleToCharacterSheet {
            subject,
            fill,
            holes,
            owner,
        } => format!(
            "RouteSettleBattleToCharacterSheet subject={} fill={} holes={} owner={}",
            subject_ref(*subject),
            fill_ref(*fill),
            joined_holes(holes),
            owner_ref(*owner)
        ),
        CharacterBattleRouteEvent::RouteRejectCharacterBattleHandoff {
            subject,
            fill,
            holes,
            owner,
        } => format!(
            "RouteRejectCharacterBattleHandoff subject={} fill={} holes={} owner={}",
            subject_ref(*subject),
            fill_ref(*fill),
            joined_holes(holes),
            owner_ref(*owner)
        ),
    }
}

fn subject_ref(subject: CharacterBattleRouteSubjectFamily) -> &'static str {
    match subject {
        CharacterBattleRouteSubjectFamily::SheetToBattleInitRouteSubject => {
            "SheetToBattleInitRouteSubject"
        }
        CharacterBattleRouteSubjectFamily::BattleToSheetSettlementRouteSubject => {
            "BattleToSheetSettlementRouteSubject"
        }
        CharacterBattleRouteSubjectFamily::HandoffResourceProjectionRouteSubject => {
            "HandoffResourceProjectionRouteSubject"
        }
        CharacterBattleRouteSubjectFamily::HandoffFeatureResourceProjectionRouteSubject => {
            "HandoffFeatureResourceProjectionRouteSubject"
        }
        CharacterBattleRouteSubjectFamily::HandoffSelectedReferenceRouteSubject => {
            "HandoffSelectedReferenceRouteSubject"
        }
        CharacterBattleRouteSubjectFamily::HandoffBattleMutationRouteSubject => {
            "HandoffBattleMutationRouteSubject"
        }
    }
}

fn fill_ref(fill: CharacterBattleRouteFillFamily) -> &'static str {
    match fill {
        CharacterBattleRouteFillFamily::HandoffSheetProjectionFill => "HandoffSheetProjectionFill",
        CharacterBattleRouteFillFamily::HandoffBattleDeltaFill => "HandoffBattleDeltaFill",
        CharacterBattleRouteFillFamily::HandoffResourceDeltaFill => "HandoffResourceDeltaFill",
        CharacterBattleRouteFillFamily::HandoffSettlementRejectionFill => {
            "HandoffSettlementRejectionFill"
        }
    }
}

fn owner_ref(owner: CharacterBattleRouteOwnerGroup) -> &'static str {
    match owner {
        CharacterBattleRouteOwnerGroup::CharacterBattleSheetOwner => "CharacterBattleSheetOwner",
        CharacterBattleRouteOwnerGroup::CharacterBattleBuildProjectionOwner => {
            "CharacterBattleBuildProjectionOwner"
        }
        CharacterBattleRouteOwnerGroup::CharacterBattleInitProjectionOwner => {
            "CharacterBattleInitProjectionOwner"
        }
        CharacterBattleRouteOwnerGroup::CharacterBattleRuntimeOwner => {
            "CharacterBattleRuntimeOwner"
        }
        CharacterBattleRouteOwnerGroup::CharacterBattleSettlementOwner => {
            "CharacterBattleSettlementOwner"
        }
        CharacterBattleRouteOwnerGroup::CharacterBattleResourceProjectionOwner => {
            "CharacterBattleResourceProjectionOwner"
        }
    }
}

fn joined_holes(holes: &[CharacterBattleRouteHoleFamily]) -> String {
    if holes.is_empty() {
        return "none".to_string();
    }
    holes
        .iter()
        .map(|hole| match hole {
            CharacterBattleRouteHoleFamily::HandoffIdentityMatchHoleFamily => {
                "HandoffIdentityMatchHoleFamily"
            }
            CharacterBattleRouteHoleFamily::HandoffHitPointProjectionHoleFamily => {
                "HandoffHitPointProjectionHoleFamily"
            }
            CharacterBattleRouteHoleFamily::HandoffSpellResourceProjectionHoleFamily => {
                "HandoffSpellResourceProjectionHoleFamily"
            }
            CharacterBattleRouteHoleFamily::HandoffFeatureResourceProjectionHoleFamily => {
                "HandoffFeatureResourceProjectionHoleFamily"
            }
            CharacterBattleRouteHoleFamily::HandoffSettlementConflictHoleFamily => {
                "HandoffSettlementConflictHoleFamily"
            }
        })
        .collect::<Vec<_>>()
        .join(",")
}

#[must_use]
pub fn reject_battle_settlement_ambiguous_created_spell_slot_source(
) -> RejectedCharacterBattleSettlement {
    RejectedCharacterBattleSettlement {
        reason: CharacterBattleSettlementRejection::AmbiguousCreatedSpellSlotOrigin,
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
        reason: CharacterBattleSettlementRejection::CharacterSheetMismatch,
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
pub fn reject_battle_settlement_active_battle_state() -> RejectedCharacterBattleSettlement {
    RejectedCharacterBattleSettlement {
        reason: CharacterBattleSettlementRejection::ActiveBattleState,
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
