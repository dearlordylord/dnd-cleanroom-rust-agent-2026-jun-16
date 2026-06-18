use crate::rules::character_battle_handoff::{
    reject_battle_settlement_active_wild_shape,
    reject_battle_settlement_ambiguous_created_spell_slot_source,
    reject_battle_settlement_maximum_hp_drift,
    reject_battle_settlement_mismatched_character_identity,
    reject_battle_settlement_stable_recovery_progress, settle_battle_feature_resource_expenditure,
    settle_battle_hit_points_conditions_slots_and_preserved_sheet_state,
    settle_battle_zero_hp_stable_lifecycle, CharacterBattleSettlement,
    CharacterBattleSettlementAcceptance, CharacterBattleSettlementRejection,
    ZeroHitPointLifecycleState,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterBattleSettlementWitness {
    pub last_result: &'static str,
    pub accepted: bool,
    pub message: &'static str,
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
    pub zero_hp_state: &'static str,
    pub zero_hp_successes: i16,
    pub zero_hp_failures: i16,
    pub stable_recovery_elapsed: i16,
    pub replay_index: u8,
}

pub const BRANCH_ACTIONS: [&str; 8] = [
    "doSettleHitPointsConditionsSlotsAndPreservedSheetState",
    "doSettleFeatureResourceExpenditure",
    "doRejectAmbiguousCreatedSpellSlotSource",
    "doRejectMismatchedCharacterIdentity",
    "doRejectMaximumHpDrift",
    "doRejectActiveWildShapeHandoff",
    "doRejectStableRecoveryProgressHandoff",
    "doSettleZeroHpStableLifecycle",
];

pub fn replay_observed_action(observed_action_taken: &str) -> CharacterBattleSettlementWitness {
    match observed_action_taken {
        "doSettleHitPointsConditionsSlotsAndPreservedSheetState" => {
            settle_hit_points_conditions_slots_and_preserved_sheet_state()
        }
        "doSettleFeatureResourceExpenditure" => settle_feature_resource_expenditure(),
        "doRejectAmbiguousCreatedSpellSlotSource" => reject_ambiguous_created_spell_slot_source(),
        "doRejectMismatchedCharacterIdentity" => reject_mismatched_character_identity(),
        "doRejectMaximumHpDrift" => reject_maximum_hp_drift(),
        "doRejectActiveWildShapeHandoff" => reject_active_wild_shape_handoff(),
        "doRejectStableRecoveryProgressHandoff" => reject_stable_recovery_progress_handoff(),
        "doSettleZeroHpStableLifecycle" => settle_zero_hp_stable_lifecycle(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> CharacterBattleSettlementWitness {
    replay_observed_action(observed_action_taken)
}

pub fn projection_payload(witness: &CharacterBattleSettlementWitness) -> String {
    [
        format!("lastResult={}", witness.last_result),
        format!("accepted={}", witness.accepted),
        format!("message={}", witness.message),
        format!("currentHp={}", witness.current_hp),
        format!("temporaryHitPoints={}", witness.temporary_hit_points),
        format!("poisoned={}", witness.poisoned),
        format!("prone={}", witness.prone),
        format!("spellLevel1Expended={}", witness.spell_level_1_expended),
        format!("createdLevel3Capacity={}", witness.created_level_3_capacity),
        format!("createdLevel3Expended={}", witness.created_level_3_expended),
        format!("pactSlotsExpended={}", witness.pact_slots_expended),
        format!(
            "featureResourceExpended={}",
            witness.feature_resource_expended
        ),
        format!("spentHitDice={}", witness.spent_hit_dice),
        format!("restFeatureUsed={}", witness.rest_feature_used),
        format!("buildUnchanged={}", witness.build_unchanged),
        format!("zeroHpState={}", witness.zero_hp_state),
        format!("zeroHpSuccesses={}", witness.zero_hp_successes),
        format!("zeroHpFailures={}", witness.zero_hp_failures),
        format!("stableRecoveryElapsed={}", witness.stable_recovery_elapsed),
        format!("replayIndex={}", witness.replay_index),
    ]
    .join("\n")
}

fn settle_hit_points_conditions_slots_and_preserved_sheet_state() -> CharacterBattleSettlementWitness
{
    witness_from_settlement(
        settle_battle_hit_points_conditions_slots_and_preserved_sheet_state(),
        1,
    )
}

fn settle_feature_resource_expenditure() -> CharacterBattleSettlementWitness {
    witness_from_settlement(settle_battle_feature_resource_expenditure(), 2)
}

fn reject_ambiguous_created_spell_slot_source() -> CharacterBattleSettlementWitness {
    witness_from_settlement(
        reject_battle_settlement_ambiguous_created_spell_slot_source(),
        3,
    )
}

fn reject_mismatched_character_identity() -> CharacterBattleSettlementWitness {
    witness_from_settlement(reject_battle_settlement_mismatched_character_identity(), 4)
}

fn reject_maximum_hp_drift() -> CharacterBattleSettlementWitness {
    witness_from_settlement(reject_battle_settlement_maximum_hp_drift(), 5)
}

fn reject_active_wild_shape_handoff() -> CharacterBattleSettlementWitness {
    witness_from_settlement(reject_battle_settlement_active_wild_shape(), 6)
}

fn reject_stable_recovery_progress_handoff() -> CharacterBattleSettlementWitness {
    witness_from_settlement(reject_battle_settlement_stable_recovery_progress(), 7)
}

fn settle_zero_hp_stable_lifecycle() -> CharacterBattleSettlementWitness {
    witness_from_settlement(settle_battle_zero_hp_stable_lifecycle(), 8)
}

fn witness_from_settlement(
    settlement: impl Into<CharacterBattleSettlement>,
    replay_index: u8,
) -> CharacterBattleSettlementWitness {
    let settlement = settlement.into();
    let facts = settlement.facts();

    CharacterBattleSettlementWitness {
        last_result: last_result_ref(&settlement),
        accepted: settlement.accepted(),
        message: settlement.message().unwrap_or("none"),
        current_hp: facts.current_hp,
        temporary_hit_points: facts.temporary_hit_points,
        poisoned: facts.poisoned,
        prone: facts.prone,
        spell_level_1_expended: facts.spell_level_1_expended,
        created_level_3_capacity: facts.created_level_3_capacity,
        created_level_3_expended: facts.created_level_3_expended,
        pact_slots_expended: facts.pact_slots_expended,
        feature_resource_expended: facts.feature_resource_expended,
        spent_hit_dice: facts.spent_hit_dice,
        rest_feature_used: facts.rest_feature_used,
        build_unchanged: facts.build_unchanged,
        zero_hp_state: zero_hp_lifecycle_ref(facts.zero_hp_lifecycle),
        zero_hp_successes: facts.zero_hp_successes,
        zero_hp_failures: facts.zero_hp_failures,
        stable_recovery_elapsed: facts.stable_recovery_elapsed,
        replay_index,
    }
}

fn last_result_ref(settlement: &CharacterBattleSettlement) -> &'static str {
    match settlement {
        CharacterBattleSettlement::Accepted(accepted) => match accepted.kind() {
            CharacterBattleSettlementAcceptance::HitPointsConditionsSlotsAndPreservedSheetState => {
                "settle-hit-points-conditions-slots-and-preserved-sheet-state"
            }
            CharacterBattleSettlementAcceptance::FeatureResourceExpenditure => {
                "settle-feature-resource-expenditure"
            }
            CharacterBattleSettlementAcceptance::ZeroHpStableLifecycle => {
                "settle-zero-hp-stable-lifecycle"
            }
        },
        CharacterBattleSettlement::Rejected(rejected) => match rejected.reason() {
            CharacterBattleSettlementRejection::AmbiguousCreatedSpellSlotSource => {
                "ambiguous-created-spell-slot-source-rejected"
            }
            CharacterBattleSettlementRejection::MismatchedCharacterIdentity => {
                "mismatched-character-identity-rejected"
            }
            CharacterBattleSettlementRejection::MaximumHpDrift => "maximum-hp-drift-rejected",
            CharacterBattleSettlementRejection::ActiveWildShape => {
                "active-wild-shape-handoff-rejected"
            }
            CharacterBattleSettlementRejection::StableRecoveryProgress => {
                "stable-recovery-progress-rejected"
            }
        },
    }
}

fn zero_hp_lifecycle_ref(state: ZeroHitPointLifecycleState) -> &'static str {
    match state {
        ZeroHitPointLifecycleState::None => "none",
        ZeroHitPointLifecycleState::Stable => "stable",
    }
}
