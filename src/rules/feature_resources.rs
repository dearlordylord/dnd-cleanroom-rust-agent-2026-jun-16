#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResourcePoolFacts {
    pub capacity: i16,
    pub expended: i16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FeatureResourceHitPoints {
    pub current_hit_points: i16,
    pub hit_point_maximum: i16,
    pub temporary_hit_points: i16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayOnHandsProjection {
    pub healing_pool: ResourcePoolFacts,
    pub target_hit_points: FeatureResourceHitPoints,
    pub condition_removed: bool,
    pub pool_spend: i16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FeatureResourceRestState {
    pub lay_on_hands_pool: ResourcePoolFacts,
    pub wild_shape_expended: i16,
    pub monk_focus_expended: i16,
    pub sorcery_points: ResourcePoolFacts,
    pub created_spell_slots: ResourcePoolFacts,
    pub uncanny_used_since_long_rest: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FontOfMagicCreatedSlotProjection {
    pub sorcery_points: ResourcePoolFacts,
    pub created_spell_slot: ResourcePoolFacts,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FontOfMagicSlotToPointsProjection {
    pub sorcery_points: ResourcePoolFacts,
    pub ordinary_spell_slot: ResourcePoolFacts,
    pub created_spell_slot: ResourcePoolFacts,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UncannyMetabolismProjection {
    pub focus_points: ResourcePoolFacts,
    pub hit_points: FeatureResourceHitPoints,
    pub used_since_long_rest: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MetamagicSharedSorceryPoints {
    pub sorcery_points: ResourcePoolFacts,
    pub known_options: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MetamagicSharedSorceryPointProjection {
    pub sorcery_points: ResourcePoolFacts,
    pub known_options: u8,
    pub shared_resource_expended: i16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourcePoolError {
    IllegalPool,
    NegativeSpend,
    NegativeRecovered,
    NegativeRestoredHitPoints,
    NegativeRemovedConditionCount,
    InsufficientRemaining,
    RecoveryExceedsExpended,
    SpendOverflow,
    CapacityOverflow,
    IllegalSpellSlotLevel,
    AmbiguousSpellSlotSource,
    NoSpellSlotAvailable,
    InsufficientSorcererLevel,
    UncannyMetabolismAlreadyUsed,
}

pub const LAY_ON_HANDS_CONDITION_REMOVAL_COST: i16 = 5;

#[must_use]
pub fn legal_resource_pool(pool: ResourcePoolFacts) -> bool {
    pool.capacity >= 0 && pool.expended >= 0 && pool.expended <= pool.capacity
}

#[must_use]
pub fn resource_pool_remaining(pool: ResourcePoolFacts) -> i16 {
    pool.capacity - pool.expended
}

#[must_use]
pub fn can_spend_resource_pool(pool: ResourcePoolFacts, spend: i16) -> bool {
    legal_resource_pool(pool) && spend >= 0 && spend <= resource_pool_remaining(pool)
}

#[must_use]
pub fn can_recover_resource_pool(pool: ResourcePoolFacts, recovered: i16) -> bool {
    legal_resource_pool(pool) && recovered >= 0 && recovered <= pool.expended
}

pub fn spend_resource_pool(
    pool: ResourcePoolFacts,
    spend: i16,
) -> Result<ResourcePoolFacts, ResourcePoolError> {
    // cleanroom-input/qnt/shared-algebras/proofs/rule-core/feature-resource-pool.qnt.
    if !legal_resource_pool(pool) {
        return Err(ResourcePoolError::IllegalPool);
    }
    if spend < 0 {
        return Err(ResourcePoolError::NegativeSpend);
    }
    if spend > resource_pool_remaining(pool) {
        return Err(ResourcePoolError::InsufficientRemaining);
    }

    Ok(ResourcePoolFacts {
        capacity: pool.capacity,
        expended: pool
            .expended
            .checked_add(spend)
            .ok_or(ResourcePoolError::SpendOverflow)?,
    })
}

pub fn recover_resource_pool(
    pool: ResourcePoolFacts,
    recovered: i16,
) -> Result<ResourcePoolFacts, ResourcePoolError> {
    // QNT: cleanroom-input/qnt/shared-algebras/proofs/rule-core/feature-resource-pool.qnt.
    if !legal_resource_pool(pool) {
        return Err(ResourcePoolError::IllegalPool);
    }
    if recovered < 0 {
        return Err(ResourcePoolError::NegativeRecovered);
    }
    if recovered > pool.expended {
        return Err(ResourcePoolError::RecoveryExceedsExpended);
    }

    Ok(ResourcePoolFacts {
        capacity: pool.capacity,
        expended: pool.expended - recovered,
    })
}

#[must_use]
pub fn recover_all_resource_pool(pool: ResourcePoolFacts) -> ResourcePoolFacts {
    // QNT: cleanroom-input/qnt/shared-algebras/proofs/rule-core/feature-resource-pool.qnt.
    ResourcePoolFacts {
        capacity: pool.capacity,
        expended: 0,
    }
}

pub fn lay_on_hands_pool_spend(
    restored_hit_points: i16,
    removed_condition_count: i16,
) -> Result<i16, ResourcePoolError> {
    // QNT: cleanroom-input/qnt/shared-algebras/proofs/rule-core/lay-on-hands-resource.qnt;
    // RAW: cleanroom-input/raw/srd-5.2.1/Classes/Paladin.md "Level 1: Lay On Hands".
    if restored_hit_points < 0 {
        return Err(ResourcePoolError::NegativeRestoredHitPoints);
    }
    if removed_condition_count < 0 {
        return Err(ResourcePoolError::NegativeRemovedConditionCount);
    }

    let condition_spend = removed_condition_count
        .checked_mul(LAY_ON_HANDS_CONDITION_REMOVAL_COST)
        .ok_or(ResourcePoolError::SpendOverflow)?;
    restored_hit_points
        .checked_add(condition_spend)
        .ok_or(ResourcePoolError::SpendOverflow)
}

pub fn can_apply_lay_on_hands_resource(
    healing_pool: ResourcePoolFacts,
    restored_hit_points: i16,
    removed_condition_count: i16,
) -> bool {
    match lay_on_hands_pool_spend(restored_hit_points, removed_condition_count) {
        Ok(spend) => can_spend_resource_pool(healing_pool, spend),
        Err(_) => false,
    }
}

pub fn apply_lay_on_hands_resource(
    healing_pool: ResourcePoolFacts,
    target_hit_points: FeatureResourceHitPoints,
    restored_hit_points: i16,
    removed_condition_count: i16,
) -> Result<LayOnHandsProjection, ResourcePoolError> {
    // QNT: cleanroom-input/qnt/shared-algebras/proofs/rule-core/lay-on-hands-resource.qnt
    // and feature-resource-hit-point-healing.qnt; RAW: Paladin "Level 1: Lay On
    // Hands" and Playing-the-Game.md "Healing".
    let pool_spend = lay_on_hands_pool_spend(restored_hit_points, removed_condition_count)?;
    let healing_pool = spend_resource_pool(healing_pool, pool_spend)?;

    Ok(LayOnHandsProjection {
        healing_pool,
        target_hit_points: apply_feature_resource_hit_point_healing(
            target_hit_points,
            restored_hit_points,
        ),
        condition_removed: removed_condition_count > 0,
        pool_spend,
    })
}

#[must_use]
pub fn legal_feature_resource_rest_state(state: FeatureResourceRestState) -> bool {
    legal_resource_pool(state.lay_on_hands_pool)
        && state.wild_shape_expended >= 0
        && state.monk_focus_expended >= 0
        && legal_resource_pool(state.sorcery_points)
        && legal_resource_pool(state.created_spell_slots)
}

#[must_use]
pub fn recover_one_expended_use(expended: i16) -> i16 {
    // QNT: cleanroom-input/qnt/shared-algebras/proofs/rule-core/feature-resource-pool.qnt;
    // RAW: cleanroom-input/raw/srd-5.2.1/Classes/Druid.md "Level 2: Wild Shape".
    if expended > 0 {
        expended - 1
    } else {
        0
    }
}

pub fn complete_short_rest_feature_resource_rest_state(
    state: FeatureResourceRestState,
) -> Result<FeatureResourceRestState, ResourcePoolError> {
    // QNT: cleanroom-input/qnt/shared-algebras/proofs/rule-core/feature-resource-pool.qnt;
    // RAW: Druid.md "Level 2: Wild Shape" and Monk.md "Level 2: Monk's Focus".
    if !legal_feature_resource_rest_state(state) {
        return Err(ResourcePoolError::IllegalPool);
    }

    Ok(FeatureResourceRestState {
        wild_shape_expended: recover_one_expended_use(state.wild_shape_expended),
        monk_focus_expended: 0,
        ..state
    })
}

pub fn complete_long_rest_feature_resource_rest_state(
    state: FeatureResourceRestState,
) -> Result<FeatureResourceRestState, ResourcePoolError> {
    // QNT: cleanroom-input/qnt/shared-algebras/proofs/rule-core/feature-resource-pool.qnt;
    // RAW: Paladin.md "Level 1: Lay On Hands", Sorcerer.md "Level 2: Font of
    // Magic", and Monk.md "Level 2: Uncanny Metabolism".
    if !legal_feature_resource_rest_state(state) {
        return Err(ResourcePoolError::IllegalPool);
    }

    Ok(FeatureResourceRestState {
        lay_on_hands_pool: recover_all_resource_pool(state.lay_on_hands_pool),
        wild_shape_expended: 0,
        monk_focus_expended: 0,
        sorcery_points: recover_all_resource_pool(state.sorcery_points),
        created_spell_slots: ResourcePoolFacts {
            capacity: 0,
            expended: 0,
        },
        uncanny_used_since_long_rest: false,
    })
}

#[must_use]
pub fn font_of_magic_spell_slot_point_gain(spell_slot_level: i16) -> i16 {
    // QNT: cleanroom-input/qnt/shared-algebras/proofs/rule-core/font-of-magic-resource.qnt;
    // RAW: cleanroom-input/raw/srd-5.2.1/Classes/Sorcerer.md
    // "Converting Spell Slots to Sorcery Points".
    spell_slot_level
}

#[must_use]
pub fn legal_font_of_magic_convertible_spell_slot_level(spell_slot_level: i16) -> bool {
    (1..=9).contains(&spell_slot_level)
}

#[must_use]
pub fn font_of_magic_spell_slot_source_requires_choice(
    ordinary_spell_slot: ResourcePoolFacts,
    created_spell_slot: ResourcePoolFacts,
) -> bool {
    resource_pool_remaining(ordinary_spell_slot) > 0
        && resource_pool_remaining(created_spell_slot) > 0
}

#[must_use]
pub fn can_convert_font_of_magic_slot_to_sorcery_points(
    sorcery_points: ResourcePoolFacts,
    spell_slot_level: i16,
    ordinary_spell_slot: ResourcePoolFacts,
    created_spell_slot: ResourcePoolFacts,
) -> bool {
    convert_font_of_magic_slot_to_sorcery_points(
        sorcery_points,
        spell_slot_level,
        ordinary_spell_slot,
        created_spell_slot,
    )
    .is_ok()
}

pub fn convert_font_of_magic_slot_to_sorcery_points(
    sorcery_points: ResourcePoolFacts,
    spell_slot_level: i16,
    ordinary_spell_slot: ResourcePoolFacts,
    created_spell_slot: ResourcePoolFacts,
) -> Result<FontOfMagicSlotToPointsProjection, ResourcePoolError> {
    // QNT: cleanroom-input/qnt/shared-algebras/proofs/rule-core/font-of-magic-resource.qnt;
    // RAW: cleanroom-input/raw/srd-5.2.1/Classes/Sorcerer.md "Level 2: Font of Magic".
    if !legal_font_of_magic_convertible_spell_slot_level(spell_slot_level) {
        return Err(ResourcePoolError::IllegalSpellSlotLevel);
    }
    if !legal_resource_pool(ordinary_spell_slot) || !legal_resource_pool(created_spell_slot) {
        return Err(ResourcePoolError::IllegalPool);
    }
    if resource_pool_remaining(ordinary_spell_slot) + resource_pool_remaining(created_spell_slot)
        <= 0
    {
        return Err(ResourcePoolError::NoSpellSlotAvailable);
    }
    if font_of_magic_spell_slot_source_requires_choice(ordinary_spell_slot, created_spell_slot) {
        return Err(ResourcePoolError::AmbiguousSpellSlotSource);
    }

    let recovered = font_of_magic_spell_slot_point_gain(spell_slot_level);
    let sorcery_points = recover_resource_pool(sorcery_points, recovered)?;
    if resource_pool_remaining(ordinary_spell_slot) > 0 {
        Ok(FontOfMagicSlotToPointsProjection {
            sorcery_points,
            ordinary_spell_slot: spend_resource_pool(ordinary_spell_slot, 1)?,
            created_spell_slot,
        })
    } else {
        Ok(FontOfMagicSlotToPointsProjection {
            sorcery_points,
            ordinary_spell_slot,
            created_spell_slot: spend_resource_pool(created_spell_slot, 1)?,
        })
    }
}

#[must_use]
pub fn font_of_magic_spell_slot_creation_cost(spell_slot_level: i16) -> i16 {
    match spell_slot_level {
        1 => 2,
        2 => 3,
        3 => 5,
        4 => 6,
        5 => 7,
        _ => 0,
    }
}

#[must_use]
pub fn font_of_magic_spell_slot_creation_minimum_sorcerer_level(spell_slot_level: i16) -> i16 {
    match spell_slot_level {
        1 => 2,
        2 => 3,
        3 => 5,
        4 => 7,
        5 => 9,
        _ => 0,
    }
}

#[must_use]
pub fn legal_font_of_magic_created_spell_slot_level(spell_slot_level: i16) -> bool {
    (1..=5).contains(&spell_slot_level)
}

#[must_use]
pub fn can_create_font_of_magic_spell_slot(
    sorcery_points: ResourcePoolFacts,
    created_spell_slot: ResourcePoolFacts,
    sorcerer_level: i16,
    spell_slot_level: i16,
) -> bool {
    create_font_of_magic_spell_slot(
        sorcery_points,
        created_spell_slot,
        sorcerer_level,
        spell_slot_level,
    )
    .is_ok()
}

pub fn create_font_of_magic_spell_slot(
    sorcery_points: ResourcePoolFacts,
    created_spell_slot: ResourcePoolFacts,
    sorcerer_level: i16,
    spell_slot_level: i16,
) -> Result<FontOfMagicCreatedSlotProjection, ResourcePoolError> {
    // QNT: cleanroom-input/qnt/shared-algebras/proofs/rule-core/font-of-magic-resource.qnt;
    // RAW: cleanroom-input/raw/srd-5.2.1/Classes/Sorcerer.md
    // "Creating Spell Slots".
    if !legal_font_of_magic_created_spell_slot_level(spell_slot_level) {
        return Err(ResourcePoolError::IllegalSpellSlotLevel);
    }
    if !legal_resource_pool(created_spell_slot) {
        return Err(ResourcePoolError::IllegalPool);
    }
    if sorcerer_level < font_of_magic_spell_slot_creation_minimum_sorcerer_level(spell_slot_level) {
        return Err(ResourcePoolError::InsufficientSorcererLevel);
    }

    Ok(FontOfMagicCreatedSlotProjection {
        sorcery_points: spend_resource_pool(
            sorcery_points,
            font_of_magic_spell_slot_creation_cost(spell_slot_level),
        )?,
        created_spell_slot: ResourcePoolFacts {
            capacity: created_spell_slot
                .capacity
                .checked_add(1)
                .ok_or(ResourcePoolError::CapacityOverflow)?,
            expended: created_spell_slot.expended,
        },
    })
}

#[must_use]
pub fn can_use_uncanny_metabolism(used_since_long_rest: bool) -> bool {
    !used_since_long_rest
}

#[must_use]
pub fn uncanny_metabolism_healing(monk_level: i16, martial_arts_roll: i16) -> i16 {
    // QNT: cleanroom-input/qnt/shared-algebras/proofs/rule-core/uncanny-metabolism-resource.qnt;
    // RAW: cleanroom-input/raw/srd-5.2.1/Classes/Monk.md
    // "Level 2: Uncanny Metabolism".
    monk_level + martial_arts_roll
}

pub fn apply_uncanny_metabolism(
    used_since_long_rest: bool,
    focus_points: ResourcePoolFacts,
    hit_points: FeatureResourceHitPoints,
    monk_level: i16,
    martial_arts_roll: i16,
) -> Result<UncannyMetabolismProjection, ResourcePoolError> {
    if !can_use_uncanny_metabolism(used_since_long_rest) {
        return Err(ResourcePoolError::UncannyMetabolismAlreadyUsed);
    }
    if !legal_resource_pool(focus_points) {
        return Err(ResourcePoolError::IllegalPool);
    }

    Ok(UncannyMetabolismProjection {
        focus_points: recover_all_resource_pool(focus_points),
        hit_points: apply_feature_resource_hit_point_healing(
            hit_points,
            uncanny_metabolism_healing(monk_level, martial_arts_roll),
        ),
        used_since_long_rest: true,
    })
}

pub fn project_metamagic_shared_sorcery_points(
    facts: MetamagicSharedSorceryPoints,
) -> Result<MetamagicSharedSorceryPointProjection, ResourcePoolError> {
    // RAW: cleanroom-input/raw/srd-5.2.1/Classes/Sorcerer.md "Level 2: Font of
    // Magic" and "Level 2: Metamagic".
    if !legal_resource_pool(facts.sorcery_points) {
        return Err(ResourcePoolError::IllegalPool);
    }

    Ok(MetamagicSharedSorceryPointProjection {
        sorcery_points: facts.sorcery_points,
        known_options: facts.known_options,
        shared_resource_expended: facts.sorcery_points.expended,
    })
}

#[must_use]
pub fn apply_feature_resource_hit_point_healing(
    hit_points: FeatureResourceHitPoints,
    raw_healing: i16,
) -> FeatureResourceHitPoints {
    // QNT: cleanroom-input/qnt/shared-algebras/proofs/rule-core/
    // feature-resource-hit-point-healing.qnt and hit-point-recovery.qnt.
    if raw_healing <= 0 {
        return hit_points;
    }

    FeatureResourceHitPoints {
        current_hit_points: clamp_hit_points(
            hit_points.current_hit_points + raw_healing,
            hit_points.hit_point_maximum,
        ),
        hit_point_maximum: hit_points.hit_point_maximum,
        temporary_hit_points: hit_points.temporary_hit_points,
    }
}

#[must_use]
pub fn apply_temporary_hit_points(
    current_temporary_hit_points: i16,
    granted_temporary_hit_points: i16,
) -> i16 {
    // RAW: cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md
    // "Temporary Hit Points" and "They Don't Stack".
    current_temporary_hit_points.max(granted_temporary_hit_points)
}

fn clamp_hit_points(hit_points: i16, hit_point_maximum: i16) -> i16 {
    if hit_points < 0 {
        0
    } else if hit_points > hit_point_maximum {
        hit_point_maximum
    } else {
        hit_points
    }
}
