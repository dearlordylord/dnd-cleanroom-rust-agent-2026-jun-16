#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClassLevel {
    One,
    Two,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceUnit {
    MonksFocus,
    FontOfMagic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceKind {
    UseCount,
    PointPool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceRecovery {
    ShortOrLongRest,
    LongRest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClassResource {
    pub unit: ResourceUnit,
    pub kind: ResourceKind,
    pub maximum: u8,
    pub recovery: ResourceRecovery,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleFactKind {
    UncannyMetabolism,
    Metamagic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MartialArtsDie {
    pub dice: u8,
    pub size: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuleFact {
    pub kind: RuleFactKind,
    pub linked_resource: ResourceUnit,
    pub martial_arts_die: Option<MartialArtsDie>,
    pub level_bonus: Option<u8>,
    pub metamagic: Option<MetamagicSelection>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpellUseLimit {
    OnePerSpellUnlessOptionAllowsStacking,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetamagicRepeatability {
    Unique,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetamagicSelection {
    pub owner_level: ClassLevel,
    pub choice_count: u8,
    pub repeatability: MetamagicRepeatability,
    pub point_pool: ResourceUnit,
    pub spell_use_limit: SpellUseLimit,
    pub options: [MetamagicOptionFact; 2],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MetamagicOptionKey(pub u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetamagicStackingMode {
    CanCombineWithDifferentMetamagic,
    OnePerSpell,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MetamagicOptionFact {
    pub choice_key: MetamagicOptionKey,
    pub sorcery_point_cost: u8,
    pub stacking_mode: MetamagicStackingMode,
    pub effect: MetamagicEffect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetamagicEffect {
    DamageDiceReroll,
    SavingThrowDisadvantage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassFeatureProjectionFacts {
    pub resource: ClassResource,
    pub fact: RuleFact,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassFeatureProjection {
    pub resource: ClassResource,
    pub fact: RuleFact,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectionError {
    DuplicateMetamagicOption,
    MissingExtraCantrip,
    DuplicateExpertiseSkill,
    ExpertiseSkillNotOwned,
    DuplicateInvocationSelection,
    LockedInvocationReplacement,
    WrongWeaponMasteryChoiceCount,
    IllegalWeaponMasteryReselectionFacts,
    ExistingWeaponMasteryChoiceCountMismatch,
    RequestedWeaponMasteryChoiceCountMismatch,
    DuplicateExistingWeaponMasteryChoice,
    DuplicateRequestedWeaponMasteryChoice,
    IneligibleWeaponMasteryChoice,
    TooManyWeaponMasteryChanges,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FighterFightingStyleSelection {
    Initial(FightingStyleFeat),
    ReplacementOnLevelGain {
        previous: FightingStyleFeat,
        replacement: FightingStyleFeat,
        new_total_level: ClassLevel,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FightingStyleFeature {
    Fighter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FightingStyleFeat {
    Archery,
    Defense,
    GreatWeaponFighting,
    TwoWeaponFighting,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FightingStyleProjection {
    pub source_feature: FightingStyleFeature,
    pub selected_feat: FightingStyleFeat,
    pub replaced_feat: Option<FightingStyleFeat>,
    pub selected_fighting_style_feature_ref_count: u8,
    pub total_level: ClassLevel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectedClassFeatureFacts {
    pub choice_count: u8,
    pub resource_maximum: u8,
    pub known_form_count: u8,
    pub short_rest_refill: u8,
    pub long_rest_refills_all: bool,
    pub accepted: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectedClassFeatureProjection {
    pub choice_count: u8,
    pub resource_maximum: u8,
    pub known_form_count: u8,
    pub short_rest_refill: u8,
    pub long_rest_refills_all: bool,
    pub accepted: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpertiseSkill {
    Acrobatics,
    Perception,
    SleightOfHand,
    Stealth,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpertiseSelectionFacts {
    pub requested_skills: Vec<ExpertiseSkill>,
    pub owned_skill_proficiency_count: u8,
    pub total_level: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpertiseSelectionProjection {
    pub selected_skills: Vec<ExpertiseSkill>,
    pub selected_expertise_choice_count: u8,
    pub build_expertise_count: u8,
    pub owned_skill_proficiency_count: u8,
    pub total_level: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EldritchInvocation {
    ArmorOfShadows,
    DevilsSight,
    EldritchMind,
    PactBlade,
    RepellingBlastEldritchBlast,
    RepellingBlastPoisonSpray,
    ThirstingBlade,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WarlockInvocationSelectionFacts {
    pub selected_invocations: Vec<EldritchInvocation>,
    pub replacement_locked_by_prerequisite: bool,
    pub pact_magic_cantrip_count: u8,
    pub pact_magic_prepared_spell_count: u8,
    pub pact_magic_slot_count: u8,
    pub pact_magic_slot_level: u8,
    pub total_level: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WarlockInvocationSelectionProjection {
    pub selected_invocations: Vec<EldritchInvocation>,
    pub selected_invocation_count: u8,
    pub selected_class_choice_feature_ref_count: u8,
    pub pact_magic_cantrip_count: u8,
    pub pact_magic_prepared_spell_count: u8,
    pub pact_magic_slot_count: u8,
    pub pact_magic_slot_level: u8,
    pub total_level: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClassOrderFeature {
    DivineOrder,
    PrimalOrder,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderChoice {
    Protector,
    Thaumaturge,
    Magician,
    Warden,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cantrip {
    Guidance,
    Light,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TrainingProjection {
    pub martial_weapon_proficiency: bool,
    pub heavy_armor_training: bool,
    pub medium_armor_training: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ability {
    Intelligence,
    Wisdom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Skill {
    Arcana,
    Nature,
    Religion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AbilityCheckBonus {
    pub check_ability: Ability,
    pub skills: [Skill; 2],
    pub bonus_ability: Ability,
    pub minimum_bonus: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassOrderFacts {
    pub feature: ClassOrderFeature,
    pub selected_choice: OrderChoice,
    pub extra_cantrip: Option<Cantrip>,
    pub selected_order_option_count: u8,
    pub selected_suborder_class_choice_feature_count: u8,
    pub training: TrainingProjection,
    pub ability_check_bonus: Option<AbilityCheckBonus>,
    pub total_level: ClassLevel,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassOrderProjection {
    pub feature: ClassOrderFeature,
    pub selected_choice: OrderChoice,
    pub extra_cantrip: Option<Cantrip>,
    pub selected_order_option_count: u8,
    pub selected_suborder_class_choice_feature_count: u8,
    pub training: TrainingProjection,
    pub ability_check_bonus: Option<AbilityCheckBonus>,
    pub total_level: ClassLevel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeaponMasteryFeature {
    Barbarian,
    Fighter,
    Paladin,
    Ranger,
    Rogue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClassUnit {
    Barbarian,
    Fighter,
    Paladin,
    Ranger,
    Rogue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Weapon {
    Dagger,
    Flail,
    Longsword,
    Shortbow,
    Shortsword,
    Spear,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WeaponMasteryFacts {
    pub feature: WeaponMasteryFeature,
    pub class_unit: ClassUnit,
    pub choice_count: u8,
    pub build_mastery_feature_count: u8,
    pub open_hole_count: u8,
    pub total_level: ClassLevel,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeaponMasteryProjection {
    pub feature: WeaponMasteryFeature,
    pub class_unit: ClassUnit,
    pub selected_weapons: Vec<Weapon>,
    pub selected_mastery_choice_count: u8,
    pub build_mastery_feature_count: u8,
    pub open_hole_count: u8,
    pub total_level: ClassLevel,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeaponMasteryReselectionFacts<T> {
    pub choice_count: usize,
    pub long_rest_change_count: usize,
    pub current_weapons: Vec<T>,
    pub requested_weapons: Vec<T>,
    pub eligible_weapons: Vec<T>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeaponMasteryReselectionProjection<T> {
    pub selected_weapons: Vec<T>,
    pub changed_choice_count: usize,
    pub choice_count: usize,
    pub long_rest_change_count: usize,
}

pub fn class_feature_projection(
    facts: ClassFeatureProjectionFacts,
) -> Result<ClassFeatureProjection, ProjectionError> {
    if let Some(metamagic) = &facts.fact.metamagic {
        if metamagic.options[0].choice_key == metamagic.options[1].choice_key {
            return Err(ProjectionError::DuplicateMetamagicOption);
        }
    }

    Ok(ClassFeatureProjection {
        resource: facts.resource,
        fact: facts.fact,
    })
}

#[must_use]
pub fn fighter_fighting_style_projection(
    selection: FighterFightingStyleSelection,
) -> FightingStyleProjection {
    match selection {
        FighterFightingStyleSelection::Initial(selected_feat) => FightingStyleProjection {
            // cleanroom-input/raw/srd-5.2.1/Classes/Fighter.md,
            // "Level 1: Fighting Style"; cleanroom-input/raw/srd-5.2.1/Feats.md,
            // "Fighting Style Feats".
            source_feature: FightingStyleFeature::Fighter,
            selected_feat,
            replaced_feat: None,
            selected_fighting_style_feature_ref_count: 1,
            total_level: ClassLevel::One,
        },
        FighterFightingStyleSelection::ReplacementOnLevelGain {
            previous,
            replacement,
            new_total_level,
        } => FightingStyleProjection {
            // cleanroom-input/raw/srd-5.2.1/Classes/Fighter.md,
            // "Whenever you gain a Fighter level, you can replace the feat...".
            source_feature: FightingStyleFeature::Fighter,
            selected_feat: replacement,
            replaced_feat: Some(previous),
            selected_fighting_style_feature_ref_count: 1,
            total_level: new_total_level,
        },
    }
}

#[must_use]
pub fn selected_class_feature_projection(
    facts: SelectedClassFeatureFacts,
) -> SelectedClassFeatureProjection {
    // QNT: character-creation-class-feature-selected-identity.mbt.qnt.
    // Character Creation "Record Class Features" owns selected references at
    // the catalog/selection boundary; execution-facing behavior uses these
    // parsed support facts rather than authored identity.
    SelectedClassFeatureProjection {
        choice_count: facts.choice_count,
        resource_maximum: facts.resource_maximum,
        known_form_count: facts.known_form_count,
        short_rest_refill: facts.short_rest_refill,
        long_rest_refills_all: facts.long_rest_refills_all,
        accepted: facts.accepted,
    }
}

pub fn expertise_selection_projection(
    facts: ExpertiseSelectionFacts,
) -> Result<ExpertiseSelectionProjection, ProjectionError> {
    // RAW: cleanroom-input/raw/srd-5.2.1/Classes/Rogue.md "Level 1:
    // Expertise"; Rules-Glossary.md "Expertise".
    if has_duplicates(&facts.requested_skills) {
        return Err(ProjectionError::DuplicateExpertiseSkill);
    }
    if facts.requested_skills.len() > usize::from(facts.owned_skill_proficiency_count) {
        return Err(ProjectionError::ExpertiseSkillNotOwned);
    }

    Ok(ExpertiseSelectionProjection {
        selected_expertise_choice_count: facts.requested_skills.len() as u8,
        build_expertise_count: facts.requested_skills.len() as u8,
        selected_skills: facts.requested_skills,
        owned_skill_proficiency_count: facts.owned_skill_proficiency_count,
        total_level: facts.total_level,
    })
}

pub fn warlock_invocation_selection_projection(
    facts: WarlockInvocationSelectionFacts,
) -> Result<WarlockInvocationSelectionProjection, ProjectionError> {
    // RAW: cleanroom-input/raw/srd-5.2.1/Classes/Warlock.md "Level 1:
    // Eldritch Invocations" and "Level 1: Pact Magic".
    if facts.replacement_locked_by_prerequisite {
        return Err(ProjectionError::LockedInvocationReplacement);
    }
    if has_duplicates(&facts.selected_invocations) {
        return Err(ProjectionError::DuplicateInvocationSelection);
    }

    Ok(WarlockInvocationSelectionProjection {
        selected_invocation_count: facts.selected_invocations.len() as u8,
        selected_class_choice_feature_ref_count: 0,
        selected_invocations: facts.selected_invocations,
        pact_magic_cantrip_count: facts.pact_magic_cantrip_count,
        pact_magic_prepared_spell_count: facts.pact_magic_prepared_spell_count,
        pact_magic_slot_count: facts.pact_magic_slot_count,
        pact_magic_slot_level: facts.pact_magic_slot_level,
        total_level: facts.total_level,
    })
}

pub fn class_order_projection(facts: ClassOrderFacts) -> ClassOrderProjection {
    ClassOrderProjection {
        feature: facts.feature,
        selected_choice: facts.selected_choice,
        extra_cantrip: facts.extra_cantrip,
        selected_order_option_count: facts.selected_order_option_count,
        selected_suborder_class_choice_feature_count: facts
            .selected_suborder_class_choice_feature_count,
        training: facts.training,
        ability_check_bonus: facts.ability_check_bonus,
        total_level: facts.total_level,
    }
}

pub fn weapon_mastery_projection(
    facts: WeaponMasteryFacts,
    selected_weapons: &[Weapon],
) -> Result<WeaponMasteryProjection, ProjectionError> {
    if selected_weapons.len() != usize::from(facts.choice_count) {
        return Err(ProjectionError::WrongWeaponMasteryChoiceCount);
    }

    Ok(WeaponMasteryProjection {
        // cleanroom-input/raw/srd-5.2.1/Classes/Fighter.md,
        // "Level 1: Weapon Mastery"; equivalent level-1 Weapon Mastery
        // entries are cited for Barbarian, Paladin, Ranger, and Rogue.
        feature: facts.feature,
        class_unit: facts.class_unit,
        selected_weapons: selected_weapons.to_vec(),
        selected_mastery_choice_count: facts.choice_count,
        build_mastery_feature_count: facts.build_mastery_feature_count,
        open_hole_count: facts.open_hole_count,
        total_level: facts.total_level,
    })
}

#[must_use]
pub fn weapon_mastery_changed_choice_count<T: Copy + Eq>(
    current_weapons: &[T],
    requested_weapons: &[T],
) -> usize {
    requested_weapons
        .iter()
        .filter(|requested| !current_weapons.contains(requested))
        .count()
}

#[must_use]
pub fn can_apply_weapon_mastery_long_rest_reselection<T: Copy + Eq>(
    facts: &WeaponMasteryReselectionFacts<T>,
) -> bool {
    validate_weapon_mastery_reselection(facts).is_ok()
}

pub fn apply_weapon_mastery_long_rest_reselection<T: Copy + Eq>(
    facts: &WeaponMasteryReselectionFacts<T>,
) -> Result<WeaponMasteryReselectionProjection<T>, ProjectionError> {
    // QNT: cleanroom-input/qnt/shared-algebras/proofs/rule-core/
    // weapon-mastery-reselection.qnt; RAW: Paladin, Ranger, and Rogue
    // "Level 1: Weapon Mastery" Long Rest reselection clauses.
    validate_weapon_mastery_reselection(facts)?;

    Ok(WeaponMasteryReselectionProjection {
        selected_weapons: facts.requested_weapons.clone(),
        changed_choice_count: weapon_mastery_changed_choice_count(
            &facts.current_weapons,
            &facts.requested_weapons,
        ),
        choice_count: facts.choice_count,
        long_rest_change_count: facts.long_rest_change_count,
    })
}

fn validate_weapon_mastery_reselection<T: Copy + Eq>(
    facts: &WeaponMasteryReselectionFacts<T>,
) -> Result<(), ProjectionError> {
    if facts.choice_count == 0
        || facts.long_rest_change_count > facts.choice_count
        || facts.eligible_weapons.len() < facts.choice_count
    {
        return Err(ProjectionError::IllegalWeaponMasteryReselectionFacts);
    }
    if facts.current_weapons.len() != facts.choice_count {
        return Err(ProjectionError::ExistingWeaponMasteryChoiceCountMismatch);
    }
    if weapon_mastery_choices_have_duplicates(&facts.current_weapons) {
        return Err(ProjectionError::DuplicateExistingWeaponMasteryChoice);
    }
    if facts.requested_weapons.len() != facts.choice_count {
        return Err(ProjectionError::RequestedWeaponMasteryChoiceCountMismatch);
    }
    if weapon_mastery_choices_have_duplicates(&facts.requested_weapons) {
        return Err(ProjectionError::DuplicateRequestedWeaponMasteryChoice);
    }
    if facts
        .requested_weapons
        .iter()
        .any(|requested| !facts.eligible_weapons.contains(requested))
    {
        return Err(ProjectionError::IneligibleWeaponMasteryChoice);
    }
    if weapon_mastery_changed_choice_count(&facts.current_weapons, &facts.requested_weapons)
        > facts.long_rest_change_count
    {
        return Err(ProjectionError::TooManyWeaponMasteryChanges);
    }

    Ok(())
}

fn weapon_mastery_choices_have_duplicates<T: Eq>(weapons: &[T]) -> bool {
    weapons
        .iter()
        .enumerate()
        .any(|(index, weapon)| weapons[..index].contains(weapon))
}

fn has_duplicates<T: Eq>(items: &[T]) -> bool {
    items
        .iter()
        .enumerate()
        .any(|(index, item)| items[..index].contains(item))
}
