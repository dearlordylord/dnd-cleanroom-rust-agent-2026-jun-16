#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClassLevel {
    One,
    Two,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeatureSet {
    MonkLevelTwo,
    SorcererLevelTwo {
        metamagic_options: [MetamagicOption; 2],
    },
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
pub enum MetamagicOption {
    EmpoweredSpell,
    HeightenedSpell,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MetamagicOptionFact {
    pub option: MetamagicOption,
    pub sorcery_point_cost: u8,
    pub stacking_mode: MetamagicStackingMode,
    pub effect: MetamagicEffect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetamagicStackingMode {
    CanCombineWithDifferentMetamagic,
    OnePerSpell,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetamagicEffect {
    DamageDiceReroll,
    SavingThrowDisadvantage,
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
pub enum OrderSelection {
    Cleric(ClericDivineOrder),
    Druid(DruidPrimalOrder),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClericDivineOrder {
    Protector,
    Thaumaturge { extra_cantrip: Cantrip },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DruidPrimalOrder {
    Magician { extra_cantrip: Cantrip },
    Warden,
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

pub fn level_two_feature_projection(
    feature_set: FeatureSet,
) -> Result<ClassFeatureProjection, ProjectionError> {
    match feature_set {
        FeatureSet::MonkLevelTwo => Ok(monk_level_two_projection()),
        FeatureSet::SorcererLevelTwo { metamagic_options } => {
            sorcerer_level_two_projection(metamagic_options)
        }
    }
}

#[must_use]
pub fn level_one_order_projection(selection: OrderSelection) -> ClassOrderProjection {
    match selection {
        OrderSelection::Cleric(choice) => cleric_divine_order_projection(choice),
        OrderSelection::Druid(choice) => druid_primal_order_projection(choice),
    }
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
pub fn cleric_divine_order_projection(choice: ClericDivineOrder) -> ClassOrderProjection {
    match choice {
        ClericDivineOrder::Protector => ClassOrderProjection {
            // cleanroom-input/raw/srd-5.2.1/Classes/Cleric.md,
            // "Level 1: Divine Order", Protector.
            feature: ClassOrderFeature::DivineOrder,
            selected_choice: OrderChoice::Protector,
            extra_cantrip: None,
            selected_order_option_count: 1,
            selected_suborder_class_choice_feature_count: 0,
            training: TrainingProjection {
                martial_weapon_proficiency: true,
                heavy_armor_training: true,
                medium_armor_training: true,
            },
            ability_check_bonus: None,
            total_level: ClassLevel::One,
        },
        ClericDivineOrder::Thaumaturge { extra_cantrip } => ClassOrderProjection {
            // cleanroom-input/raw/srd-5.2.1/Classes/Cleric.md,
            // "Level 1: Divine Order", Thaumaturge.
            feature: ClassOrderFeature::DivineOrder,
            selected_choice: OrderChoice::Thaumaturge,
            extra_cantrip: Some(extra_cantrip),
            selected_order_option_count: 1,
            selected_suborder_class_choice_feature_count: 0,
            training: TrainingProjection {
                martial_weapon_proficiency: false,
                heavy_armor_training: false,
                medium_armor_training: true,
            },
            ability_check_bonus: Some(AbilityCheckBonus {
                check_ability: Ability::Intelligence,
                skills: [Skill::Arcana, Skill::Religion],
                bonus_ability: Ability::Wisdom,
                minimum_bonus: 1,
            }),
            total_level: ClassLevel::One,
        },
    }
}

#[must_use]
pub fn druid_primal_order_projection(choice: DruidPrimalOrder) -> ClassOrderProjection {
    match choice {
        DruidPrimalOrder::Magician { extra_cantrip } => ClassOrderProjection {
            // cleanroom-input/raw/srd-5.2.1/Classes/Druid.md,
            // "Level 1: Primal Order", Magician.
            feature: ClassOrderFeature::PrimalOrder,
            selected_choice: OrderChoice::Magician,
            extra_cantrip: Some(extra_cantrip),
            selected_order_option_count: 1,
            selected_suborder_class_choice_feature_count: 0,
            training: TrainingProjection {
                martial_weapon_proficiency: false,
                heavy_armor_training: false,
                medium_armor_training: false,
            },
            ability_check_bonus: Some(AbilityCheckBonus {
                check_ability: Ability::Intelligence,
                skills: [Skill::Arcana, Skill::Nature],
                bonus_ability: Ability::Wisdom,
                minimum_bonus: 1,
            }),
            total_level: ClassLevel::One,
        },
        DruidPrimalOrder::Warden => ClassOrderProjection {
            // cleanroom-input/raw/srd-5.2.1/Classes/Druid.md,
            // "Level 1: Primal Order", Warden.
            feature: ClassOrderFeature::PrimalOrder,
            selected_choice: OrderChoice::Warden,
            extra_cantrip: None,
            selected_order_option_count: 1,
            selected_suborder_class_choice_feature_count: 0,
            training: TrainingProjection {
                martial_weapon_proficiency: true,
                heavy_armor_training: false,
                medium_armor_training: true,
            },
            ability_check_bonus: None,
            total_level: ClassLevel::One,
        },
    }
}

#[must_use]
pub fn monk_level_two_projection() -> ClassFeatureProjection {
    ClassFeatureProjection {
        // cleanroom-input/raw/srd-5.2.1/Classes/Monk.md,
        // "Level 2: Monk's Focus" and "Level 2: Uncanny Metabolism".
        resource: ClassResource {
            unit: ResourceUnit::MonksFocus,
            kind: ResourceKind::UseCount,
            maximum: 2,
            recovery: ResourceRecovery::ShortOrLongRest,
        },
        fact: RuleFact {
            kind: RuleFactKind::UncannyMetabolism,
            linked_resource: ResourceUnit::MonksFocus,
            martial_arts_die: Some(MartialArtsDie { dice: 1, size: 6 }),
            level_bonus: Some(2),
            metamagic: None,
        },
    }
}

pub fn sorcerer_level_two_projection(
    metamagic_options: [MetamagicOption; 2],
) -> Result<ClassFeatureProjection, ProjectionError> {
    if metamagic_options[0] == metamagic_options[1] {
        return Err(ProjectionError::DuplicateMetamagicOption);
    }

    Ok(ClassFeatureProjection {
        // cleanroom-input/raw/srd-5.2.1/Classes/Sorcerer.md,
        // "Level 2: Font of Magic", "Level 2: Metamagic",
        // "Empowered Spell", and "Heightened Spell".
        resource: ClassResource {
            unit: ResourceUnit::FontOfMagic,
            kind: ResourceKind::PointPool,
            maximum: 2,
            recovery: ResourceRecovery::LongRest,
        },
        fact: RuleFact {
            kind: RuleFactKind::Metamagic,
            linked_resource: ResourceUnit::FontOfMagic,
            martial_arts_die: None,
            level_bonus: None,
            metamagic: Some(MetamagicSelection {
                owner_level: ClassLevel::Two,
                choice_count: 2,
                repeatability: MetamagicRepeatability::Unique,
                point_pool: ResourceUnit::FontOfMagic,
                spell_use_limit: SpellUseLimit::OnePerSpellUnlessOptionAllowsStacking,
                options: [
                    metamagic_option_fact(metamagic_options[0]),
                    metamagic_option_fact(metamagic_options[1]),
                ],
            }),
        },
    })
}

#[must_use]
pub fn metamagic_option_fact(option: MetamagicOption) -> MetamagicOptionFact {
    match option {
        MetamagicOption::EmpoweredSpell => MetamagicOptionFact {
            option,
            sorcery_point_cost: 1,
            stacking_mode: MetamagicStackingMode::CanCombineWithDifferentMetamagic,
            effect: MetamagicEffect::DamageDiceReroll,
        },
        MetamagicOption::HeightenedSpell => MetamagicOptionFact {
            option,
            sorcery_point_cost: 2,
            stacking_mode: MetamagicStackingMode::OnePerSpell,
            effect: MetamagicEffect::SavingThrowDisadvantage,
        },
    }
}
