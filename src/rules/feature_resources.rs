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
pub enum ResourcePoolError {
    IllegalPool,
    NegativeSpend,
    NegativeRestoredHitPoints,
    NegativeRemovedConditionCount,
    InsufficientRemaining,
    SpendOverflow,
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
