use crate::rules::class_features::{
    level_two_feature_projection, ClassFeatureProjection, ClassLevel, FeatureSet, MetamagicEffect,
    MetamagicOption, MetamagicRepeatability, MetamagicStackingMode, ResourceKind, ResourceRecovery,
    ResourceUnit, RuleFactKind, SpellUseLimit,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectionWitness {
    pub last_result: &'static str,
    pub resource_unit_id: &'static str,
    pub resource_kind: &'static str,
    pub resource_maximum: u8,
    pub short_rest_refills_all: bool,
    pub long_rest_refills_all: bool,
    pub source_fact_kind: &'static str,
    pub linked_resource_unit_id: &'static str,
    pub known_option_count: u8,
    pub spell_use_limit: &'static str,
    pub martial_arts_die_source_unit_id: &'static str,
    pub martial_arts_die_dice: u8,
    pub martial_arts_die_size: u8,
    pub monk_level_bonus: u8,
    pub metamagic_owner_class_level: u8,
    pub metamagic_choice_count: u8,
    pub metamagic_selection_repeatability: &'static str,
    pub metamagic_sorcery_point_pool_id: &'static str,
    pub first_metamagic_option_id: &'static str,
    pub first_metamagic_sorcery_point_cost: u8,
    pub first_metamagic_stacking_mode: &'static str,
    pub first_metamagic_effect_kind: &'static str,
    pub second_metamagic_option_id: &'static str,
    pub second_metamagic_sorcery_point_cost: u8,
    pub second_metamagic_stacking_mode: &'static str,
    pub second_metamagic_effect_kind: &'static str,
    pub replay_index: u8,
}

pub fn replay_observed_action(observed_action_taken: &str) -> ProjectionWitness {
    match observed_action_taken {
        "doProjectMonkFocusAndUncannyMetabolism" => monk_replay(),
        "doProjectSorcererFontAndMetamagic" => sorcerer_replay_after_monk(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_monk_witness() -> ProjectionWitness {
    ProjectionWitness {
        last_result: "monk-focus-uncanny-metabolism",
        resource_unit_id: "monk_monks_focus",
        resource_kind: "use_count",
        resource_maximum: 2,
        short_rest_refills_all: true,
        long_rest_refills_all: true,
        source_fact_kind: "uncanny_metabolism",
        linked_resource_unit_id: "monk_monks_focus",
        known_option_count: 0,
        spell_use_limit: "none",
        martial_arts_die_source_unit_id: "monk_martial_arts",
        martial_arts_die_dice: 1,
        martial_arts_die_size: 6,
        monk_level_bonus: 2,
        metamagic_owner_class_level: 0,
        metamagic_choice_count: 0,
        metamagic_selection_repeatability: "none",
        metamagic_sorcery_point_pool_id: "none",
        first_metamagic_option_id: "none",
        first_metamagic_sorcery_point_cost: 0,
        first_metamagic_stacking_mode: "none",
        first_metamagic_effect_kind: "none",
        second_metamagic_option_id: "none",
        second_metamagic_sorcery_point_cost: 0,
        second_metamagic_stacking_mode: "none",
        second_metamagic_effect_kind: "none",
        replay_index: 1,
    }
}

pub fn expected_sorcerer_witness() -> ProjectionWitness {
    ProjectionWitness {
        last_result: "sorcerer-font-metamagic",
        resource_unit_id: "sorcerer_font_of_magic",
        resource_kind: "point_pool",
        resource_maximum: 2,
        short_rest_refills_all: false,
        long_rest_refills_all: true,
        source_fact_kind: "sorcerer_metamagic",
        linked_resource_unit_id: "sorcerer_font_of_magic",
        known_option_count: 2,
        spell_use_limit: "one_per_spell_unless_option_allows_stacking",
        martial_arts_die_source_unit_id: "none",
        martial_arts_die_dice: 0,
        martial_arts_die_size: 0,
        monk_level_bonus: 0,
        metamagic_owner_class_level: 2,
        metamagic_choice_count: 2,
        metamagic_selection_repeatability: "unique",
        metamagic_sorcery_point_pool_id: "sorcery_points",
        first_metamagic_option_id: "sorcerer_empowered_spell",
        first_metamagic_sorcery_point_cost: 1,
        first_metamagic_stacking_mode: "can_combine_with_different_metamagic",
        first_metamagic_effect_kind: "damage_dice_reroll",
        second_metamagic_option_id: "sorcerer_heightened_spell",
        second_metamagic_sorcery_point_cost: 2,
        second_metamagic_stacking_mode: "one_per_spell",
        second_metamagic_effect_kind: "saving_throw_disadvantage",
        replay_index: 2,
    }
}

pub fn projection_payload(witness: &ProjectionWitness) -> String {
    [
        format!("lastResult={}", witness.last_result),
        format!("resourceUnitId={}", witness.resource_unit_id),
        format!("resourceKind={}", witness.resource_kind),
        format!("resourceMaximum={}", witness.resource_maximum),
        format!("shortRestRefillsAll={}", witness.short_rest_refills_all),
        format!("longRestRefillsAll={}", witness.long_rest_refills_all),
        format!("sourceFactKind={}", witness.source_fact_kind),
        format!("linkedResourceUnitId={}", witness.linked_resource_unit_id),
        format!("knownOptionCount={}", witness.known_option_count),
        format!("spellUseLimit={}", witness.spell_use_limit),
        format!(
            "martialArtsDieSourceUnitId={}",
            witness.martial_arts_die_source_unit_id
        ),
        format!("martialArtsDieDice={}", witness.martial_arts_die_dice),
        format!("martialArtsDieSize={}", witness.martial_arts_die_size),
        format!("monkLevelBonus={}", witness.monk_level_bonus),
        format!(
            "metamagicOwnerClassLevel={}",
            witness.metamagic_owner_class_level
        ),
        format!("metamagicChoiceCount={}", witness.metamagic_choice_count),
        format!(
            "metamagicSelectionRepeatability={}",
            witness.metamagic_selection_repeatability
        ),
        format!(
            "metamagicSorceryPointPoolId={}",
            witness.metamagic_sorcery_point_pool_id
        ),
        format!(
            "firstMetamagicOptionId={}",
            witness.first_metamagic_option_id
        ),
        format!(
            "firstMetamagicSorceryPointCost={}",
            witness.first_metamagic_sorcery_point_cost
        ),
        format!(
            "firstMetamagicStackingMode={}",
            witness.first_metamagic_stacking_mode
        ),
        format!(
            "firstMetamagicEffectKind={}",
            witness.first_metamagic_effect_kind
        ),
        format!(
            "secondMetamagicOptionId={}",
            witness.second_metamagic_option_id
        ),
        format!(
            "secondMetamagicSorceryPointCost={}",
            witness.second_metamagic_sorcery_point_cost
        ),
        format!(
            "secondMetamagicStackingMode={}",
            witness.second_metamagic_stacking_mode
        ),
        format!(
            "secondMetamagicEffectKind={}",
            witness.second_metamagic_effect_kind
        ),
        format!("replayIndex={}", witness.replay_index),
    ]
    .join("\n")
}

fn monk_replay() -> ProjectionWitness {
    let observed = level_two_feature_projection(FeatureSet::MonkLevelTwo)
        .expect("level two monk projection is infallible");
    monk_projection_witness(&observed)
}

fn sorcerer_replay_after_monk() -> ProjectionWitness {
    let observed = level_two_feature_projection(FeatureSet::SorcererLevelTwo {
        metamagic_options: [
            MetamagicOption::EmpoweredSpell,
            MetamagicOption::HeightenedSpell,
        ],
    })
    .expect("selected metamagic options are unique");
    sorcerer_projection_witness(&observed)
}

fn monk_projection_witness(projection: &ClassFeatureProjection) -> ProjectionWitness {
    let die = projection
        .fact
        .martial_arts_die
        .expect("monk projection carries a martial arts die");

    ProjectionWitness {
        last_result: "monk-focus-uncanny-metabolism",
        resource_unit_id: unit_ref(projection.resource.unit),
        resource_kind: resource_kind(projection.resource.kind),
        resource_maximum: projection.resource.maximum,
        short_rest_refills_all: projection.resource.recovery == ResourceRecovery::ShortOrLongRest,
        long_rest_refills_all: true,
        source_fact_kind: rule_fact_kind(projection.fact.kind),
        linked_resource_unit_id: unit_ref(projection.fact.linked_resource),
        known_option_count: 0,
        spell_use_limit: "none",
        martial_arts_die_source_unit_id: "monk_martial_arts",
        martial_arts_die_dice: die.dice,
        martial_arts_die_size: die.size,
        monk_level_bonus: projection
            .fact
            .level_bonus
            .expect("monk projection carries a level bonus"),
        metamagic_owner_class_level: 0,
        metamagic_choice_count: 0,
        metamagic_selection_repeatability: "none",
        metamagic_sorcery_point_pool_id: "none",
        first_metamagic_option_id: "none",
        first_metamagic_sorcery_point_cost: 0,
        first_metamagic_stacking_mode: "none",
        first_metamagic_effect_kind: "none",
        second_metamagic_option_id: "none",
        second_metamagic_sorcery_point_cost: 0,
        second_metamagic_stacking_mode: "none",
        second_metamagic_effect_kind: "none",
        replay_index: 1,
    }
}

fn sorcerer_projection_witness(projection: &ClassFeatureProjection) -> ProjectionWitness {
    let metamagic = projection
        .fact
        .metamagic
        .as_ref()
        .expect("sorcerer projection carries metamagic facts");

    ProjectionWitness {
        last_result: "sorcerer-font-metamagic",
        resource_unit_id: unit_ref(projection.resource.unit),
        resource_kind: resource_kind(projection.resource.kind),
        resource_maximum: projection.resource.maximum,
        short_rest_refills_all: projection.resource.recovery == ResourceRecovery::ShortOrLongRest,
        long_rest_refills_all: true,
        source_fact_kind: rule_fact_kind(projection.fact.kind),
        linked_resource_unit_id: unit_ref(projection.fact.linked_resource),
        known_option_count: metamagic.choice_count,
        spell_use_limit: spell_use_limit(metamagic.spell_use_limit),
        martial_arts_die_source_unit_id: "none",
        martial_arts_die_dice: 0,
        martial_arts_die_size: 0,
        monk_level_bonus: 0,
        metamagic_owner_class_level: class_level(metamagic.owner_level),
        metamagic_choice_count: metamagic.choice_count,
        metamagic_selection_repeatability: metamagic_repeatability(metamagic.repeatability),
        metamagic_sorcery_point_pool_id: "sorcery_points",
        first_metamagic_option_id: metamagic_option_ref(metamagic.options[0].option),
        first_metamagic_sorcery_point_cost: metamagic.options[0].sorcery_point_cost,
        first_metamagic_stacking_mode: metamagic_stacking_mode(metamagic.options[0].stacking_mode),
        first_metamagic_effect_kind: metamagic_effect(metamagic.options[0].effect),
        second_metamagic_option_id: metamagic_option_ref(metamagic.options[1].option),
        second_metamagic_sorcery_point_cost: metamagic.options[1].sorcery_point_cost,
        second_metamagic_stacking_mode: metamagic_stacking_mode(metamagic.options[1].stacking_mode),
        second_metamagic_effect_kind: metamagic_effect(metamagic.options[1].effect),
        replay_index: 2,
    }
}

fn unit_ref(unit: ResourceUnit) -> &'static str {
    match unit {
        ResourceUnit::MonksFocus => "monk_monks_focus",
        ResourceUnit::FontOfMagic => "sorcerer_font_of_magic",
    }
}

fn resource_kind(kind: ResourceKind) -> &'static str {
    match kind {
        ResourceKind::UseCount => "use_count",
        ResourceKind::PointPool => "point_pool",
    }
}

fn rule_fact_kind(kind: RuleFactKind) -> &'static str {
    match kind {
        RuleFactKind::UncannyMetabolism => "uncanny_metabolism",
        RuleFactKind::Metamagic => "sorcerer_metamagic",
    }
}

fn class_level(level: ClassLevel) -> u8 {
    match level {
        ClassLevel::Two => 2,
    }
}

fn spell_use_limit(limit: SpellUseLimit) -> &'static str {
    match limit {
        SpellUseLimit::OnePerSpellUnlessOptionAllowsStacking => {
            "one_per_spell_unless_option_allows_stacking"
        }
    }
}

fn metamagic_repeatability(repeatability: MetamagicRepeatability) -> &'static str {
    match repeatability {
        MetamagicRepeatability::Unique => "unique",
    }
}

fn metamagic_option_ref(option: MetamagicOption) -> &'static str {
    match option {
        MetamagicOption::EmpoweredSpell => "sorcerer_empowered_spell",
        MetamagicOption::HeightenedSpell => "sorcerer_heightened_spell",
    }
}

fn metamagic_stacking_mode(mode: MetamagicStackingMode) -> &'static str {
    match mode {
        MetamagicStackingMode::CanCombineWithDifferentMetamagic => {
            "can_combine_with_different_metamagic"
        }
        MetamagicStackingMode::OnePerSpell => "one_per_spell",
    }
}

fn metamagic_effect(effect: MetamagicEffect) -> &'static str {
    match effect {
        MetamagicEffect::DamageDiceReroll => "damage_dice_reroll",
        MetamagicEffect::SavingThrowDisadvantage => "saving_throw_disadvantage",
    }
}
