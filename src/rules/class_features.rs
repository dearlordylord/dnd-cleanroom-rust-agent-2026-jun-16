#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClassLevel {
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
