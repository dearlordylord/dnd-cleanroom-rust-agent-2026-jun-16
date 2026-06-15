use crate::rules::spell_shapes::{
    resolve_save_gated_spell_damage, resolve_spell_attack_hit, AttackSpellHitFacts,
    AttackSpellProfile, SaveGatedSpellDamageFacts, SaveGatedSpellProfile, SpellActiveEffect,
    SpellShapeOutcome, SpellShapeProjection, SpellShapeState,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttackSpellShapeWitness {
    pub target_hit_points: i16,
    pub spell_slot_spent_this_turn: bool,
    pub level_one_slots_remaining: i16,
    pub active_effect_kind: &'static str,
    pub active_effect_count: usize,
    pub scenario_outcome: &'static str,
    pub protocol_result: &'static str,
    pub invalid_reason: &'static str,
    pub protocol_hole_count: usize,
}

pub const BRANCH_ACTIONS: [&str; 6] = [
    "doChillTouchHitPointRegainPrevention",
    "doFireBoltHit",
    "doGuidingBoltNextAttackAdvantage",
    "doInflictWoundsFailedSave",
    "doInflictWoundsSuccessfulSave",
    "doShockingGraspOpportunityAttackDenied",
];

pub fn replay_observed_action(observed_action_taken: &str) -> AttackSpellShapeWitness {
    match observed_action_taken {
        "doChillTouchHitPointRegainPrevention" => spell_attack_hit(
            AttackSpellProfile::ChillTouch,
            SpellShapeOutcome::ChillTouchHit,
        ),
        "doFireBoltHit" => {
            spell_attack_hit(AttackSpellProfile::FireBolt, SpellShapeOutcome::FireBoltHit)
        }
        "doGuidingBoltNextAttackAdvantage" => spell_attack_hit(
            AttackSpellProfile::GuidingBolt,
            SpellShapeOutcome::GuidingBoltHit,
        ),
        "doInflictWoundsFailedSave" => inflict_wounds(false),
        "doInflictWoundsSuccessfulSave" => inflict_wounds(true),
        "doShockingGraspOpportunityAttackDenied" => spell_attack_hit(
            AttackSpellProfile::ShockingGrasp,
            SpellShapeOutcome::ShockingGraspHit,
        ),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> AttackSpellShapeWitness {
    replay_observed_action(observed_action_taken)
}

pub fn projection_payload(witness: &AttackSpellShapeWitness) -> String {
    [
        format!("targetHp={}", witness.target_hit_points),
        format!(
            "spellSlotSpentThisTurn={}",
            witness.spell_slot_spent_this_turn
        ),
        format!("level1SlotsRemaining={}", witness.level_one_slots_remaining),
        format!("activeEffectKind={}", witness.active_effect_kind),
        format!("activeEffectCount={}", witness.active_effect_count),
        format!("scenarioOutcome={}", witness.scenario_outcome),
        format!("protocolResult={}", witness.protocol_result),
        format!("invalidReason={}", witness.invalid_reason),
        format!("protocolHoleCount={}", witness.protocol_hole_count),
    ]
    .join("\n")
}

fn spell_attack_hit(
    spell: AttackSpellProfile,
    expected_outcome: SpellShapeOutcome,
) -> AttackSpellShapeWitness {
    let projection = resolve_spell_attack_hit(
        initial_state(),
        AttackSpellHitFacts {
            spell,
            damage_roll: 4,
        },
    );
    assert_eq!(projection.outcome, expected_outcome);
    witness_from_projection(projection)
}

fn inflict_wounds(saving_throw_succeeded: bool) -> AttackSpellShapeWitness {
    let projection = resolve_save_gated_spell_damage(
        initial_state(),
        SaveGatedSpellDamageFacts {
            spell: SaveGatedSpellProfile::InflictWounds,
            damage_roll: 6,
            saving_throw_succeeded,
        },
    );
    witness_from_projection(projection)
}

fn initial_state() -> SpellShapeState {
    SpellShapeState {
        target_hit_points: 12,
        spell_slot_spent_this_turn: false,
        level_one_slots_remaining: 2,
        active_effects: vec![],
    }
}

fn witness_from_projection(projection: SpellShapeProjection) -> AttackSpellShapeWitness {
    AttackSpellShapeWitness {
        target_hit_points: projection.state.target_hit_points,
        spell_slot_spent_this_turn: projection.state.spell_slot_spent_this_turn,
        level_one_slots_remaining: projection.state.level_one_slots_remaining,
        active_effect_kind: active_effect_kind(&projection.state.active_effects),
        active_effect_count: projection.state.active_effects.len(),
        scenario_outcome: scenario_outcome_ref(projection.outcome),
        protocol_result: "resolved",
        invalid_reason: "none",
        protocol_hole_count: 0,
    }
}

fn active_effect_kind(effects: &[SpellActiveEffect]) -> &'static str {
    match effects {
        [] => "none",
        [SpellActiveEffect::HitPointRegainPrevented] => "hitPointRegainPrevented",
        [SpellActiveEffect::NextAttackRollAgainstTargetAdvantage] => "nextAttackRollAgainstSelf",
        [SpellActiveEffect::OpportunityAttackDenied] => "opportunityAttackDenied",
        _ => "multiple",
    }
}

fn scenario_outcome_ref(outcome: SpellShapeOutcome) -> &'static str {
    match outcome {
        SpellShapeOutcome::ChillTouchHit => "ChillTouch",
        SpellShapeOutcome::FireBoltHit => "FireBolt",
        SpellShapeOutcome::GuidingBoltHit => "GuidingBolt",
        SpellShapeOutcome::InflictWoundsFailedSave => "InflictWoundsFailure",
        SpellShapeOutcome::InflictWoundsSuccessfulSave => "InflictWoundsSuccess",
        SpellShapeOutcome::ShockingGraspHit => "ShockingGrasp",
    }
}
