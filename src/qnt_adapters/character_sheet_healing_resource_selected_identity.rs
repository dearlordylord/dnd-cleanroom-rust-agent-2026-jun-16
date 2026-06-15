use crate::rules::feature_resources::{
    apply_lay_on_hands_resource, resource_pool_remaining, FeatureResourceHitPoints,
    LayOnHandsProjection, ResourcePoolFacts,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HealingResourceWitness {
    pub source_hp: i16,
    pub target_hp: i16,
    pub target_poisoned: bool,
    pub pool_expended: i16,
    pub pool_remaining: i16,
    pub last_result: &'static str,
}

pub fn replay_observed_action(observed_action_taken: &str) -> HealingResourceWitness {
    match observed_action_taken {
        "doLayOnHandsRestoreHpAndRemovePoisoned" => lay_on_hands_restore_hp_and_remove_poisoned(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_lay_on_hands_witness() -> HealingResourceWitness {
    HealingResourceWitness {
        source_hp: 12,
        target_hp: 5,
        target_poisoned: false,
        pool_expended: 7,
        pool_remaining: 3,
        last_result: "resolved",
    }
}

pub fn projection_payload(witness: &HealingResourceWitness) -> String {
    [
        format!("sourceHp={}", witness.source_hp),
        format!("targetHp={}", witness.target_hp),
        format!("targetPoisoned={}", witness.target_poisoned),
        format!("poolExpended={}", witness.pool_expended),
        format!("poolRemaining={}", witness.pool_remaining),
        format!("lastResult={}", witness.last_result),
    ]
    .join("\n")
}

fn lay_on_hands_restore_hp_and_remove_poisoned() -> HealingResourceWitness {
    let target_poisoned = true;
    let projection = apply_lay_on_hands_resource(
        ResourcePoolFacts {
            capacity: 10,
            expended: 0,
        },
        FeatureResourceHitPoints {
            current_hit_points: 3,
            hit_point_maximum: 12,
            temporary_hit_points: 0,
        },
        2,
        1,
    )
    .expect("lay on hands witness spend must be legal");

    healing_resource_witness(12, target_poisoned, "resolved", projection)
}

fn healing_resource_witness(
    source_hp: i16,
    target_poisoned: bool,
    last_result: &'static str,
    projection: LayOnHandsProjection,
) -> HealingResourceWitness {
    HealingResourceWitness {
        source_hp,
        target_hp: projection.target_hit_points.current_hit_points,
        target_poisoned: target_poisoned && !projection.condition_removed,
        pool_expended: projection.healing_pool.expended,
        pool_remaining: resource_pool_remaining(projection.healing_pool),
        last_result,
    }
}
