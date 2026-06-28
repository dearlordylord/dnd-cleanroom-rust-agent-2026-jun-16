use crate::rules::character_battle_handoff::{
    reject_battle_settlement_active_battle_state, reject_battle_settlement_active_wild_shape,
    reject_battle_settlement_ambiguous_created_spell_slot_source,
    reject_battle_settlement_maximum_hp_drift,
    reject_battle_settlement_mismatched_character_identity,
    reject_battle_settlement_mixed_spell_and_pact_slot,
    reject_battle_settlement_stable_recovery_progress, route_enter_battle_runtime,
    route_project_character_sheet_to_battle, route_record_character_battle_handoff_facts,
    route_reject_character_battle_handoff, route_settle_battle_to_character_sheet,
    settle_battle_feature_resource_expenditure,
    settle_battle_hit_points_conditions_slots_and_preserved_sheet_state,
    settle_battle_pure_pact_magic_slot_expenditure, settle_battle_zero_hp_stable_lifecycle,
    CharacterBattleRouteEvent, CharacterBattleRouteFillFamily,
    CharacterBattleRouteHandoffFactFamily, CharacterBattleRouteHoleFamily,
    CharacterBattleRouteOwnerGroup, CharacterBattleRouteSubjectFamily, CharacterBattleSettlement,
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

pub const BRANCH_ACTIONS: [&str; 11] = [
    "doSettleHitPointsConditionsSlotsAndPreservedSheetState",
    "doSettlePurePactMagicSlotExpenditure",
    "doRejectMixedSpellAndPactSlotSettlement",
    "doSettleFeatureResourceExpenditure",
    "doRejectMismatchedCharacterIdentity",
    "doRejectMaximumHpDrift",
    "doRejectActiveWildShapeHandoff",
    "doRejectActiveBattleStateHandoff",
    "doRejectStableRecoveryProgressHandoff",
    "doSettleZeroHpStableLifecycle",
    "doRejectAmbiguousCreatedSpellSlotSource",
];

pub fn replay_observed_action(observed_action_taken: &str) -> CharacterBattleSettlementWitness {
    match observed_action_taken {
        "doSettleHitPointsConditionsSlotsAndPreservedSheetState" => {
            settle_hit_points_conditions_slots_and_preserved_sheet_state()
        }
        "doSettlePurePactMagicSlotExpenditure" => settle_pure_pact_magic_slot_expenditure(),
        "doRejectMixedSpellAndPactSlotSettlement" => reject_mixed_spell_and_pact_slot_settlement(),
        "doSettleFeatureResourceExpenditure" => settle_feature_resource_expenditure(),
        "doRejectAmbiguousCreatedSpellSlotSource" => reject_ambiguous_created_spell_slot_source(),
        "doRejectMismatchedCharacterIdentity" => reject_mismatched_character_identity(),
        "doRejectMaximumHpDrift" => reject_maximum_hp_drift(),
        "doRejectActiveWildShapeHandoff" => reject_active_wild_shape_handoff(),
        "doRejectActiveBattleStateHandoff" => reject_active_battle_state_handoff(),
        "doRejectStableRecoveryProgressHandoff" => reject_stable_recovery_progress_handoff(),
        "doSettleZeroHpStableLifecycle" => settle_zero_hp_stable_lifecycle(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> CharacterBattleSettlementWitness {
    match observed_action_taken {
        "doSettleHitPointsConditionsSlotsAndPreservedSheetState" => {
            expected_settle_hit_points_conditions_slots_and_preserved_sheet_state()
        }
        "doSettlePurePactMagicSlotExpenditure" => {
            expected_settle_pure_pact_magic_slot_expenditure()
        }
        "doRejectMixedSpellAndPactSlotSettlement" => {
            expected_reject_mixed_spell_and_pact_slot_settlement()
        }
        "doSettleFeatureResourceExpenditure" => expected_settle_feature_resource_expenditure(),
        "doRejectAmbiguousCreatedSpellSlotSource" => {
            expected_reject_ambiguous_created_spell_slot_source()
        }
        "doRejectMismatchedCharacterIdentity" => expected_reject_mismatched_character_identity(),
        "doRejectMaximumHpDrift" => expected_reject_maximum_hp_drift(),
        "doRejectActiveWildShapeHandoff" => expected_reject_active_wild_shape_handoff(),
        "doRejectActiveBattleStateHandoff" => expected_reject_active_battle_state_handoff(),
        "doRejectStableRecoveryProgressHandoff" => {
            expected_reject_stable_recovery_progress_handoff()
        }
        "doSettleZeroHpStableLifecycle" => expected_settle_zero_hp_stable_lifecycle(),
        action => panic!("unsupported expected mbt::actionTaken {action}"),
    }
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<CharacterBattleRouteEvent> {
    match observed_action_taken {
        "doSettleHitPointsConditionsSlotsAndPreservedSheetState" => {
            route_after_settle_hit_points_conditions_slots_and_preserved_sheet_state()
        }
        "doSettlePurePactMagicSlotExpenditure" => {
            route_after_settle_pure_pact_magic_slot_expenditure()
        }
        "doRejectMixedSpellAndPactSlotSettlement" => {
            route_after_reject_mixed_spell_and_pact_slot_settlement()
        }
        "doSettleFeatureResourceExpenditure" => route_after_settle_feature_resource_expenditure(),
        "doRejectAmbiguousCreatedSpellSlotSource" => {
            route_after_reject_ambiguous_created_spell_slot_source()
        }
        "doRejectMismatchedCharacterIdentity" => route_after_reject_mismatched_character_identity(),
        "doRejectMaximumHpDrift" => route_after_reject_maximum_hp_drift(),
        "doRejectActiveWildShapeHandoff" => route_after_reject_active_wild_shape_handoff(),
        "doRejectActiveBattleStateHandoff" => route_after_reject_active_battle_state_handoff(),
        "doRejectStableRecoveryProgressHandoff" => {
            route_after_reject_stable_recovery_progress_handoff()
        }
        "doSettleZeroHpStableLifecycle" => route_after_settle_zero_hp_stable_lifecycle(),
        action => panic!("unsupported route mbt::actionTaken {action}"),
    }
}

pub fn expected_route(observed_action_taken: &str) -> Vec<CharacterBattleRouteEvent> {
    match observed_action_taken {
        "doSettleHitPointsConditionsSlotsAndPreservedSheetState" => {
            expected_route_after_settle_hit_points_conditions_slots_and_preserved_sheet_state()
        }
        "doSettlePurePactMagicSlotExpenditure" => {
            expected_route_after_settle_pure_pact_magic_slot_expenditure()
        }
        "doRejectMixedSpellAndPactSlotSettlement" => {
            expected_route_after_reject_mixed_spell_and_pact_slot_settlement()
        }
        "doSettleFeatureResourceExpenditure" => {
            expected_route_after_settle_feature_resource_expenditure()
        }
        "doRejectAmbiguousCreatedSpellSlotSource" => {
            expected_route_after_reject_ambiguous_created_spell_slot_source()
        }
        "doRejectMismatchedCharacterIdentity" => {
            expected_route_after_reject_mismatched_character_identity()
        }
        "doRejectMaximumHpDrift" => expected_route_after_reject_maximum_hp_drift(),
        "doRejectActiveWildShapeHandoff" => expected_route_after_reject_active_wild_shape_handoff(),
        "doRejectActiveBattleStateHandoff" => {
            expected_route_after_reject_active_battle_state_handoff()
        }
        "doRejectStableRecoveryProgressHandoff" => {
            expected_route_after_reject_stable_recovery_progress_handoff()
        }
        "doSettleZeroHpStableLifecycle" => expected_route_after_settle_zero_hp_stable_lifecycle(),
        action => panic!("unsupported expected route mbt::actionTaken {action}"),
    }
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
    witness_from_settlement(settle_battle_feature_resource_expenditure(), 4)
}

fn reject_ambiguous_created_spell_slot_source() -> CharacterBattleSettlementWitness {
    witness_from_settlement(
        reject_battle_settlement_ambiguous_created_spell_slot_source(),
        5,
    )
}

fn reject_mismatched_character_identity() -> CharacterBattleSettlementWitness {
    witness_from_settlement(reject_battle_settlement_mismatched_character_identity(), 6)
}

fn reject_maximum_hp_drift() -> CharacterBattleSettlementWitness {
    witness_from_settlement(reject_battle_settlement_maximum_hp_drift(), 7)
}

fn reject_active_wild_shape_handoff() -> CharacterBattleSettlementWitness {
    witness_from_settlement(reject_battle_settlement_active_wild_shape(), 8)
}

fn reject_active_battle_state_handoff() -> CharacterBattleSettlementWitness {
    witness_from_settlement(reject_battle_settlement_active_battle_state(), 9)
}

fn reject_stable_recovery_progress_handoff() -> CharacterBattleSettlementWitness {
    witness_from_settlement(reject_battle_settlement_stable_recovery_progress(), 10)
}

fn settle_zero_hp_stable_lifecycle() -> CharacterBattleSettlementWitness {
    witness_from_settlement(settle_battle_zero_hp_stable_lifecycle(), 11)
}

fn settle_pure_pact_magic_slot_expenditure() -> CharacterBattleSettlementWitness {
    witness_from_settlement(settle_battle_pure_pact_magic_slot_expenditure(), 2)
}

fn reject_mixed_spell_and_pact_slot_settlement() -> CharacterBattleSettlementWitness {
    witness_from_settlement(reject_battle_settlement_mixed_spell_and_pact_slot(), 3)
}

fn expected_settle_hit_points_conditions_slots_and_preserved_sheet_state(
) -> CharacterBattleSettlementWitness {
    CharacterBattleSettlementWitness {
        last_result: "settle-hit-points-conditions-slots-and-preserved-sheet-state",
        accepted: true,
        message: "none",
        current_hp: 6,
        temporary_hit_points: 3,
        poisoned: true,
        prone: true,
        spell_level_1_expended: 2,
        created_level_3_capacity: 0,
        created_level_3_expended: 0,
        pact_slots_expended: 0,
        feature_resource_expended: 0,
        spent_hit_dice: 1,
        rest_feature_used: true,
        build_unchanged: true,
        zero_hp_state: "none",
        zero_hp_successes: 0,
        zero_hp_failures: 0,
        stable_recovery_elapsed: 0,
        replay_index: 1,
    }
}

fn expected_settle_feature_resource_expenditure() -> CharacterBattleSettlementWitness {
    CharacterBattleSettlementWitness {
        last_result: "settle-feature-resource-expenditure",
        accepted: true,
        message: "none",
        current_hp: 24,
        temporary_hit_points: 0,
        poisoned: false,
        prone: false,
        spell_level_1_expended: 0,
        created_level_3_capacity: 0,
        created_level_3_expended: 0,
        pact_slots_expended: 0,
        feature_resource_expended: 3,
        spent_hit_dice: 0,
        rest_feature_used: false,
        build_unchanged: true,
        zero_hp_state: "none",
        zero_hp_successes: 0,
        zero_hp_failures: 0,
        stable_recovery_elapsed: 0,
        replay_index: 4,
    }
}

fn expected_settle_pure_pact_magic_slot_expenditure() -> CharacterBattleSettlementWitness {
    CharacterBattleSettlementWitness {
        last_result: "settle-pure-pact-magic-slot-expenditure",
        accepted: true,
        message: "none",
        current_hp: 8,
        pact_slots_expended: 1,
        replay_index: 2,
        ..empty_expected_settlement_witness()
    }
}

fn expected_reject_mixed_spell_and_pact_slot_settlement() -> CharacterBattleSettlementWitness {
    rejected_expected_settlement_witness(
        "mixed-spell-and-pact-slot-settlement-rejected",
        "Battle handoff cannot project mixed Spell Slot and Pact Slot state without origin-distinct battle slots.",
        3,
    )
}

fn expected_reject_ambiguous_created_spell_slot_source() -> CharacterBattleSettlementWitness {
    CharacterBattleSettlementWitness {
        last_result: "ambiguous-created-spell-slot-source-rejected",
        accepted: false,
        message: "Battle handoff Spell Slot expenditure is origin-ambiguous for level 3.",
        created_level_3_capacity: 1,
        replay_index: 5,
        ..empty_expected_settlement_witness()
    }
}

fn expected_reject_mismatched_character_identity() -> CharacterBattleSettlementWitness {
    rejected_expected_settlement_witness(
        "mismatched-character-identity-rejected",
        "Battle handoff character identity does not match Character Sheet.",
        6,
    )
}

fn expected_reject_maximum_hp_drift() -> CharacterBattleSettlementWitness {
    rejected_expected_settlement_witness(
        "maximum-hp-drift-rejected",
        "Battle handoff maximum HP does not match Character Sheet.",
        7,
    )
}

fn expected_reject_active_wild_shape_handoff() -> CharacterBattleSettlementWitness {
    rejected_expected_settlement_witness(
        "active-wild-shape-handoff-rejected",
        "Battle handoff while Wild Shape is active is blocked; dismiss or resolve reversion before Character Sheet handoff.",
        8,
    )
}

fn expected_reject_active_battle_state_handoff() -> CharacterBattleSettlementWitness {
    rejected_expected_settlement_witness(
        "active-battle-state-handoff-rejected",
        "Battle handoff while active battle effects or Concentration are present is blocked; end or resolve battle-local effects before Character Sheet handoff.",
        9,
    )
}

fn expected_reject_stable_recovery_progress_handoff() -> CharacterBattleSettlementWitness {
    CharacterBattleSettlementWitness {
        last_result: "stable-recovery-progress-rejected",
        accepted: false,
        message: "Battle handoff cannot preserve in-progress Stable recovery timers.",
        zero_hp_state: "stable",
        stable_recovery_elapsed: 1,
        replay_index: 10,
        ..empty_expected_settlement_witness()
    }
}

fn expected_settle_zero_hp_stable_lifecycle() -> CharacterBattleSettlementWitness {
    CharacterBattleSettlementWitness {
        last_result: "settle-zero-hp-stable-lifecycle",
        accepted: true,
        message: "none",
        zero_hp_state: "stable",
        replay_index: 11,
        ..empty_expected_settlement_witness()
    }
}

fn rejected_expected_settlement_witness(
    last_result: &'static str,
    message: &'static str,
    replay_index: u8,
) -> CharacterBattleSettlementWitness {
    CharacterBattleSettlementWitness {
        last_result,
        accepted: false,
        message,
        replay_index,
        ..empty_expected_settlement_witness()
    }
}

fn empty_expected_settlement_witness() -> CharacterBattleSettlementWitness {
    CharacterBattleSettlementWitness {
        last_result: "",
        accepted: false,
        message: "",
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
        zero_hp_state: "none",
        zero_hp_successes: 0,
        zero_hp_failures: 0,
        stable_recovery_elapsed: 0,
        replay_index: 0,
    }
}

fn initial_battle_settlement_route() -> Vec<CharacterBattleRouteEvent> {
    vec![
        route_project_character_sheet_to_battle(
            CharacterBattleRouteSubjectFamily::SheetToBattleInitRouteSubject,
            CharacterBattleRouteOwnerGroup::CharacterBattleSheetOwner,
        ),
        route_enter_battle_runtime(
            CharacterBattleRouteSubjectFamily::HandoffBattleMutationRouteSubject,
            CharacterBattleRouteOwnerGroup::CharacterBattleRuntimeOwner,
        ),
    ]
}

fn route_after_settle_hit_points_conditions_slots_and_preserved_sheet_state(
) -> Vec<CharacterBattleRouteEvent> {
    let mut route = initial_battle_settlement_route();
    append_settle_hit_points_conditions_slots_and_preserved_sheet_state_route(&mut route);
    route
}

fn route_after_settle_feature_resource_expenditure() -> Vec<CharacterBattleRouteEvent> {
    let mut route = route_after_reject_mixed_spell_and_pact_slot_settlement();
    append_settle_feature_resource_expenditure_route(&mut route);
    route
}

fn route_after_settle_pure_pact_magic_slot_expenditure() -> Vec<CharacterBattleRouteEvent> {
    let mut route = route_after_settle_hit_points_conditions_slots_and_preserved_sheet_state();
    append_settle_pure_pact_magic_slot_expenditure_route(&mut route);
    route
}

fn route_after_reject_mixed_spell_and_pact_slot_settlement() -> Vec<CharacterBattleRouteEvent> {
    let mut route = route_after_settle_pure_pact_magic_slot_expenditure();
    append_reject_mixed_spell_and_pact_slot_settlement_route(&mut route);
    route
}

fn route_after_reject_ambiguous_created_spell_slot_source() -> Vec<CharacterBattleRouteEvent> {
    let mut route = route_after_settle_feature_resource_expenditure();
    append_reject_ambiguous_created_spell_slot_source_route(&mut route);
    route
}

fn route_after_reject_mismatched_character_identity() -> Vec<CharacterBattleRouteEvent> {
    let mut route = route_after_reject_ambiguous_created_spell_slot_source();
    append_reject_identity_mismatch_route(&mut route);
    route
}

fn route_after_reject_maximum_hp_drift() -> Vec<CharacterBattleRouteEvent> {
    let mut route = route_after_reject_mismatched_character_identity();
    append_reject_maximum_hp_drift_route(&mut route);
    route
}

fn route_after_reject_active_wild_shape_handoff() -> Vec<CharacterBattleRouteEvent> {
    let mut route = route_after_reject_maximum_hp_drift();
    append_reject_settlement_conflict_route(&mut route);
    route
}

fn route_after_reject_stable_recovery_progress_handoff() -> Vec<CharacterBattleRouteEvent> {
    let mut route = route_after_reject_active_battle_state_handoff();
    append_reject_settlement_conflict_route(&mut route);
    route
}

fn route_after_reject_active_battle_state_handoff() -> Vec<CharacterBattleRouteEvent> {
    let mut route = route_after_reject_active_wild_shape_handoff();
    append_reject_settlement_conflict_route(&mut route);
    route
}

fn route_after_settle_zero_hp_stable_lifecycle() -> Vec<CharacterBattleRouteEvent> {
    let mut route = route_after_reject_stable_recovery_progress_handoff();
    append_settle_zero_hp_stable_lifecycle_route(&mut route);
    route
}

fn expected_route_after_settle_hit_points_conditions_slots_and_preserved_sheet_state(
) -> Vec<CharacterBattleRouteEvent> {
    vec![
        expected_project_character_sheet_to_battle(
            CharacterBattleRouteSubjectFamily::SheetToBattleInitRouteSubject,
            CharacterBattleRouteOwnerGroup::CharacterBattleSheetOwner,
        ),
        expected_enter_battle_runtime(
            CharacterBattleRouteSubjectFamily::HandoffBattleMutationRouteSubject,
            CharacterBattleRouteOwnerGroup::CharacterBattleRuntimeOwner,
        ),
        expected_settle_battle_to_character_sheet(
            CharacterBattleRouteSubjectFamily::BattleToSheetSettlementRouteSubject,
            CharacterBattleRouteFillFamily::HandoffBattleDeltaFill,
            Vec::new(),
            CharacterBattleRouteOwnerGroup::CharacterBattleSettlementOwner,
        ),
        expected_settle_battle_to_character_sheet(
            CharacterBattleRouteSubjectFamily::HandoffResourceProjectionRouteSubject,
            CharacterBattleRouteFillFamily::HandoffResourceDeltaFill,
            Vec::new(),
            CharacterBattleRouteOwnerGroup::CharacterBattleResourceProjectionOwner,
        ),
        route_record_character_battle_handoff_facts(
            CharacterBattleRouteSubjectFamily::HandoffResourceProjectionRouteSubject,
            vec![
                CharacterBattleRouteHandoffFactFamily::HandoffSourceExactSpellSlotDeltaFact,
                CharacterBattleRouteHandoffFactFamily::HandoffSourceExactPactSlotDeltaFact,
            ],
            CharacterBattleRouteOwnerGroup::CharacterBattleResourceProjectionOwner,
        ),
        expected_project_character_sheet_to_battle(
            CharacterBattleRouteSubjectFamily::SheetToBattleInitRouteSubject,
            CharacterBattleRouteOwnerGroup::CharacterBattleSheetOwner,
        ),
    ]
}

fn expected_route_after_settle_feature_resource_expenditure() -> Vec<CharacterBattleRouteEvent> {
    let mut route = expected_route_after_reject_mixed_spell_and_pact_slot_settlement();
    route.push(expected_settle_battle_to_character_sheet(
        CharacterBattleRouteSubjectFamily::HandoffFeatureResourceProjectionRouteSubject,
        CharacterBattleRouteFillFamily::HandoffResourceDeltaFill,
        Vec::new(),
        CharacterBattleRouteOwnerGroup::CharacterBattleResourceProjectionOwner,
    ));
    route.push(route_record_character_battle_handoff_facts(
        CharacterBattleRouteSubjectFamily::HandoffFeatureResourceProjectionRouteSubject,
        vec![CharacterBattleRouteHandoffFactFamily::HandoffFeatureResourceDeltaFact],
        CharacterBattleRouteOwnerGroup::CharacterBattleResourceProjectionOwner,
    ));
    route
}

fn expected_route_after_settle_pure_pact_magic_slot_expenditure() -> Vec<CharacterBattleRouteEvent>
{
    let mut route =
        expected_route_after_settle_hit_points_conditions_slots_and_preserved_sheet_state();
    route.push(expected_settle_battle_to_character_sheet(
        CharacterBattleRouteSubjectFamily::HandoffResourceProjectionRouteSubject,
        CharacterBattleRouteFillFamily::HandoffResourceDeltaFill,
        Vec::new(),
        CharacterBattleRouteOwnerGroup::CharacterBattleResourceProjectionOwner,
    ));
    route
}

fn expected_route_after_reject_mixed_spell_and_pact_slot_settlement(
) -> Vec<CharacterBattleRouteEvent> {
    let mut route = expected_route_after_settle_pure_pact_magic_slot_expenditure();
    route.push(expected_reject_character_battle_handoff(
        CharacterBattleRouteSubjectFamily::HandoffResourceProjectionRouteSubject,
        CharacterBattleRouteFillFamily::HandoffSettlementRejectionFill,
        vec![
            CharacterBattleRouteHoleFamily::HandoffSpellResourceProjectionHoleFamily,
            CharacterBattleRouteHoleFamily::HandoffSettlementConflictHoleFamily,
        ],
        CharacterBattleRouteOwnerGroup::CharacterBattleResourceProjectionOwner,
    ));
    route.push(route_record_character_battle_handoff_facts(
        CharacterBattleRouteSubjectFamily::HandoffResourceProjectionRouteSubject,
        vec![CharacterBattleRouteHandoffFactFamily::HandoffSettlementConflictFact],
        CharacterBattleRouteOwnerGroup::CharacterBattleResourceProjectionOwner,
    ));
    route
}

fn expected_route_after_reject_ambiguous_created_spell_slot_source(
) -> Vec<CharacterBattleRouteEvent> {
    let mut route = expected_route_after_settle_feature_resource_expenditure();
    route.push(expected_reject_character_battle_handoff(
        CharacterBattleRouteSubjectFamily::HandoffResourceProjectionRouteSubject,
        CharacterBattleRouteFillFamily::HandoffSettlementRejectionFill,
        vec![
            CharacterBattleRouteHoleFamily::HandoffSpellResourceProjectionHoleFamily,
            CharacterBattleRouteHoleFamily::HandoffSettlementConflictHoleFamily,
        ],
        CharacterBattleRouteOwnerGroup::CharacterBattleResourceProjectionOwner,
    ));
    route.push(route_record_character_battle_handoff_facts(
        CharacterBattleRouteSubjectFamily::HandoffResourceProjectionRouteSubject,
        vec![CharacterBattleRouteHandoffFactFamily::HandoffSettlementConflictFact],
        CharacterBattleRouteOwnerGroup::CharacterBattleResourceProjectionOwner,
    ));
    route
}

fn expected_route_after_reject_mismatched_character_identity() -> Vec<CharacterBattleRouteEvent> {
    let mut route = expected_route_after_reject_ambiguous_created_spell_slot_source();
    route.push(expected_reject_character_battle_handoff(
        CharacterBattleRouteSubjectFamily::BattleToSheetSettlementRouteSubject,
        CharacterBattleRouteFillFamily::HandoffSettlementRejectionFill,
        vec![CharacterBattleRouteHoleFamily::HandoffIdentityMatchHoleFamily],
        CharacterBattleRouteOwnerGroup::CharacterBattleSettlementOwner,
    ));
    route
}

fn expected_route_after_reject_maximum_hp_drift() -> Vec<CharacterBattleRouteEvent> {
    let mut route = expected_route_after_reject_mismatched_character_identity();
    route.push(expected_reject_character_battle_handoff(
        CharacterBattleRouteSubjectFamily::BattleToSheetSettlementRouteSubject,
        CharacterBattleRouteFillFamily::HandoffSettlementRejectionFill,
        vec![CharacterBattleRouteHoleFamily::HandoffHitPointProjectionHoleFamily],
        CharacterBattleRouteOwnerGroup::CharacterBattleSettlementOwner,
    ));
    route
}

fn expected_route_after_reject_active_wild_shape_handoff() -> Vec<CharacterBattleRouteEvent> {
    let mut route = expected_route_after_reject_maximum_hp_drift();
    route.push(expected_reject_character_battle_handoff(
        CharacterBattleRouteSubjectFamily::BattleToSheetSettlementRouteSubject,
        CharacterBattleRouteFillFamily::HandoffSettlementRejectionFill,
        vec![CharacterBattleRouteHoleFamily::HandoffSettlementConflictHoleFamily],
        CharacterBattleRouteOwnerGroup::CharacterBattleSettlementOwner,
    ));
    route.push(route_record_character_battle_handoff_facts(
        CharacterBattleRouteSubjectFamily::BattleToSheetSettlementRouteSubject,
        vec![CharacterBattleRouteHandoffFactFamily::HandoffSettlementConflictFact],
        CharacterBattleRouteOwnerGroup::CharacterBattleSettlementOwner,
    ));
    route
}

fn expected_route_after_reject_stable_recovery_progress_handoff() -> Vec<CharacterBattleRouteEvent>
{
    let mut route = expected_route_after_reject_active_battle_state_handoff();
    route.push(expected_reject_character_battle_handoff(
        CharacterBattleRouteSubjectFamily::BattleToSheetSettlementRouteSubject,
        CharacterBattleRouteFillFamily::HandoffSettlementRejectionFill,
        vec![CharacterBattleRouteHoleFamily::HandoffSettlementConflictHoleFamily],
        CharacterBattleRouteOwnerGroup::CharacterBattleSettlementOwner,
    ));
    route.push(route_record_character_battle_handoff_facts(
        CharacterBattleRouteSubjectFamily::BattleToSheetSettlementRouteSubject,
        vec![CharacterBattleRouteHandoffFactFamily::HandoffSettlementConflictFact],
        CharacterBattleRouteOwnerGroup::CharacterBattleSettlementOwner,
    ));
    route
}

fn expected_route_after_reject_active_battle_state_handoff() -> Vec<CharacterBattleRouteEvent> {
    let mut route = expected_route_after_reject_active_wild_shape_handoff();
    route.push(expected_reject_character_battle_handoff(
        CharacterBattleRouteSubjectFamily::BattleToSheetSettlementRouteSubject,
        CharacterBattleRouteFillFamily::HandoffSettlementRejectionFill,
        vec![CharacterBattleRouteHoleFamily::HandoffSettlementConflictHoleFamily],
        CharacterBattleRouteOwnerGroup::CharacterBattleSettlementOwner,
    ));
    route.push(route_record_character_battle_handoff_facts(
        CharacterBattleRouteSubjectFamily::BattleToSheetSettlementRouteSubject,
        vec![CharacterBattleRouteHandoffFactFamily::HandoffSettlementConflictFact],
        CharacterBattleRouteOwnerGroup::CharacterBattleSettlementOwner,
    ));
    route
}

fn expected_route_after_settle_zero_hp_stable_lifecycle() -> Vec<CharacterBattleRouteEvent> {
    let mut route = expected_route_after_reject_stable_recovery_progress_handoff();
    route.push(expected_settle_battle_to_character_sheet(
        CharacterBattleRouteSubjectFamily::BattleToSheetSettlementRouteSubject,
        CharacterBattleRouteFillFamily::HandoffBattleDeltaFill,
        Vec::new(),
        CharacterBattleRouteOwnerGroup::CharacterBattleSettlementOwner,
    ));
    route.push(route_record_character_battle_handoff_facts(
        CharacterBattleRouteSubjectFamily::BattleToSheetSettlementRouteSubject,
        vec![CharacterBattleRouteHandoffFactFamily::HandoffZeroHpStableLifecycleFact],
        CharacterBattleRouteOwnerGroup::CharacterBattleSettlementOwner,
    ));
    route
}

fn append_settle_hit_points_conditions_slots_and_preserved_sheet_state_route(
    route: &mut Vec<CharacterBattleRouteEvent>,
) {
    route.push(route_settle_battle_to_character_sheet(
        CharacterBattleRouteSubjectFamily::BattleToSheetSettlementRouteSubject,
        CharacterBattleRouteFillFamily::HandoffBattleDeltaFill,
        Vec::new(),
        CharacterBattleRouteOwnerGroup::CharacterBattleSettlementOwner,
    ));
    route.push(route_settle_battle_to_character_sheet(
        CharacterBattleRouteSubjectFamily::HandoffResourceProjectionRouteSubject,
        CharacterBattleRouteFillFamily::HandoffResourceDeltaFill,
        Vec::new(),
        CharacterBattleRouteOwnerGroup::CharacterBattleResourceProjectionOwner,
    ));
    route.push(route_record_character_battle_handoff_facts(
        CharacterBattleRouteSubjectFamily::HandoffResourceProjectionRouteSubject,
        vec![
            CharacterBattleRouteHandoffFactFamily::HandoffSourceExactSpellSlotDeltaFact,
            CharacterBattleRouteHandoffFactFamily::HandoffSourceExactPactSlotDeltaFact,
        ],
        CharacterBattleRouteOwnerGroup::CharacterBattleResourceProjectionOwner,
    ));
    route.push(route_project_character_sheet_to_battle(
        CharacterBattleRouteSubjectFamily::SheetToBattleInitRouteSubject,
        CharacterBattleRouteOwnerGroup::CharacterBattleSheetOwner,
    ));
}

fn append_settle_feature_resource_expenditure_route(route: &mut Vec<CharacterBattleRouteEvent>) {
    route.push(route_settle_battle_to_character_sheet(
        CharacterBattleRouteSubjectFamily::HandoffFeatureResourceProjectionRouteSubject,
        CharacterBattleRouteFillFamily::HandoffResourceDeltaFill,
        Vec::new(),
        CharacterBattleRouteOwnerGroup::CharacterBattleResourceProjectionOwner,
    ));
    route.push(route_record_character_battle_handoff_facts(
        CharacterBattleRouteSubjectFamily::HandoffFeatureResourceProjectionRouteSubject,
        vec![CharacterBattleRouteHandoffFactFamily::HandoffFeatureResourceDeltaFact],
        CharacterBattleRouteOwnerGroup::CharacterBattleResourceProjectionOwner,
    ));
}

fn append_settle_pure_pact_magic_slot_expenditure_route(
    route: &mut Vec<CharacterBattleRouteEvent>,
) {
    route.push(route_settle_battle_to_character_sheet(
        CharacterBattleRouteSubjectFamily::HandoffResourceProjectionRouteSubject,
        CharacterBattleRouteFillFamily::HandoffResourceDeltaFill,
        Vec::new(),
        CharacterBattleRouteOwnerGroup::CharacterBattleResourceProjectionOwner,
    ));
}

fn append_reject_mixed_spell_and_pact_slot_settlement_route(
    route: &mut Vec<CharacterBattleRouteEvent>,
) {
    route.push(route_reject_character_battle_handoff(
        CharacterBattleRouteSubjectFamily::HandoffResourceProjectionRouteSubject,
        CharacterBattleRouteFillFamily::HandoffSettlementRejectionFill,
        vec![
            CharacterBattleRouteHoleFamily::HandoffSpellResourceProjectionHoleFamily,
            CharacterBattleRouteHoleFamily::HandoffSettlementConflictHoleFamily,
        ],
        CharacterBattleRouteOwnerGroup::CharacterBattleResourceProjectionOwner,
    ));
    route.push(route_record_character_battle_handoff_facts(
        CharacterBattleRouteSubjectFamily::HandoffResourceProjectionRouteSubject,
        vec![CharacterBattleRouteHandoffFactFamily::HandoffSettlementConflictFact],
        CharacterBattleRouteOwnerGroup::CharacterBattleResourceProjectionOwner,
    ));
}

fn append_reject_ambiguous_created_spell_slot_source_route(
    route: &mut Vec<CharacterBattleRouteEvent>,
) {
    route.push(route_reject_character_battle_handoff(
        CharacterBattleRouteSubjectFamily::HandoffResourceProjectionRouteSubject,
        CharacterBattleRouteFillFamily::HandoffSettlementRejectionFill,
        vec![
            CharacterBattleRouteHoleFamily::HandoffSpellResourceProjectionHoleFamily,
            CharacterBattleRouteHoleFamily::HandoffSettlementConflictHoleFamily,
        ],
        CharacterBattleRouteOwnerGroup::CharacterBattleResourceProjectionOwner,
    ));
    route.push(route_record_character_battle_handoff_facts(
        CharacterBattleRouteSubjectFamily::HandoffResourceProjectionRouteSubject,
        vec![CharacterBattleRouteHandoffFactFamily::HandoffSettlementConflictFact],
        CharacterBattleRouteOwnerGroup::CharacterBattleResourceProjectionOwner,
    ));
}

fn append_reject_identity_mismatch_route(route: &mut Vec<CharacterBattleRouteEvent>) {
    route.push(route_reject_character_battle_handoff(
        CharacterBattleRouteSubjectFamily::BattleToSheetSettlementRouteSubject,
        CharacterBattleRouteFillFamily::HandoffSettlementRejectionFill,
        vec![CharacterBattleRouteHoleFamily::HandoffIdentityMatchHoleFamily],
        CharacterBattleRouteOwnerGroup::CharacterBattleSettlementOwner,
    ));
}

fn append_reject_maximum_hp_drift_route(route: &mut Vec<CharacterBattleRouteEvent>) {
    route.push(route_reject_character_battle_handoff(
        CharacterBattleRouteSubjectFamily::BattleToSheetSettlementRouteSubject,
        CharacterBattleRouteFillFamily::HandoffSettlementRejectionFill,
        vec![CharacterBattleRouteHoleFamily::HandoffHitPointProjectionHoleFamily],
        CharacterBattleRouteOwnerGroup::CharacterBattleSettlementOwner,
    ));
}

fn append_reject_settlement_conflict_route(route: &mut Vec<CharacterBattleRouteEvent>) {
    route.push(route_reject_character_battle_handoff(
        CharacterBattleRouteSubjectFamily::BattleToSheetSettlementRouteSubject,
        CharacterBattleRouteFillFamily::HandoffSettlementRejectionFill,
        vec![CharacterBattleRouteHoleFamily::HandoffSettlementConflictHoleFamily],
        CharacterBattleRouteOwnerGroup::CharacterBattleSettlementOwner,
    ));
    route.push(route_record_character_battle_handoff_facts(
        CharacterBattleRouteSubjectFamily::BattleToSheetSettlementRouteSubject,
        vec![CharacterBattleRouteHandoffFactFamily::HandoffSettlementConflictFact],
        CharacterBattleRouteOwnerGroup::CharacterBattleSettlementOwner,
    ));
}

fn append_settle_zero_hp_stable_lifecycle_route(route: &mut Vec<CharacterBattleRouteEvent>) {
    route.push(route_settle_battle_to_character_sheet(
        CharacterBattleRouteSubjectFamily::BattleToSheetSettlementRouteSubject,
        CharacterBattleRouteFillFamily::HandoffBattleDeltaFill,
        Vec::new(),
        CharacterBattleRouteOwnerGroup::CharacterBattleSettlementOwner,
    ));
    route.push(route_record_character_battle_handoff_facts(
        CharacterBattleRouteSubjectFamily::BattleToSheetSettlementRouteSubject,
        vec![CharacterBattleRouteHandoffFactFamily::HandoffZeroHpStableLifecycleFact],
        CharacterBattleRouteOwnerGroup::CharacterBattleSettlementOwner,
    ));
}

fn expected_project_character_sheet_to_battle(
    subject: CharacterBattleRouteSubjectFamily,
    owner: CharacterBattleRouteOwnerGroup,
) -> CharacterBattleRouteEvent {
    CharacterBattleRouteEvent::RouteProjectCharacterSheetToBattle { subject, owner }
}

fn expected_enter_battle_runtime(
    subject: CharacterBattleRouteSubjectFamily,
    owner: CharacterBattleRouteOwnerGroup,
) -> CharacterBattleRouteEvent {
    CharacterBattleRouteEvent::RouteEnterBattleRuntime { subject, owner }
}

fn expected_settle_battle_to_character_sheet(
    subject: CharacterBattleRouteSubjectFamily,
    fill: CharacterBattleRouteFillFamily,
    mut holes: Vec<CharacterBattleRouteHoleFamily>,
    owner: CharacterBattleRouteOwnerGroup,
) -> CharacterBattleRouteEvent {
    holes.sort();
    CharacterBattleRouteEvent::RouteSettleBattleToCharacterSheet {
        subject,
        fill,
        holes,
        owner,
    }
}

fn expected_reject_character_battle_handoff(
    subject: CharacterBattleRouteSubjectFamily,
    fill: CharacterBattleRouteFillFamily,
    mut holes: Vec<CharacterBattleRouteHoleFamily>,
    owner: CharacterBattleRouteOwnerGroup,
) -> CharacterBattleRouteEvent {
    holes.sort();
    CharacterBattleRouteEvent::RouteRejectCharacterBattleHandoff {
        subject,
        fill,
        holes,
        owner,
    }
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
        message: settlement_message_ref(&settlement),
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

fn settlement_message_ref(settlement: &CharacterBattleSettlement) -> &'static str {
    match settlement {
        CharacterBattleSettlement::Accepted(_) => "none",
        CharacterBattleSettlement::Rejected(rejected) => match rejected.reason() {
            CharacterBattleSettlementRejection::CharacterSheetMismatch => {
                "Battle handoff character identity does not match Character Sheet."
            }
            _ => rejected.message(),
        },
    }
}

fn last_result_ref(settlement: &CharacterBattleSettlement) -> &'static str {
    match settlement {
        CharacterBattleSettlement::Accepted(accepted) => match accepted.kind() {
            CharacterBattleSettlementAcceptance::HitPointsConditionsSlotsAndPreservedSheetState => {
                "settle-hit-points-conditions-slots-and-preserved-sheet-state"
            }
            CharacterBattleSettlementAcceptance::PurePactMagicSlotExpenditure => {
                "settle-pure-pact-magic-slot-expenditure"
            }
            CharacterBattleSettlementAcceptance::FeatureResourceExpenditure => {
                "settle-feature-resource-expenditure"
            }
            CharacterBattleSettlementAcceptance::ZeroHpStableLifecycle => {
                "settle-zero-hp-stable-lifecycle"
            }
        },
        CharacterBattleSettlement::Rejected(rejected) => match rejected.reason() {
            CharacterBattleSettlementRejection::MixedSpellAndPactSlot => {
                "mixed-spell-and-pact-slot-settlement-rejected"
            }
            CharacterBattleSettlementRejection::AmbiguousCreatedSpellSlotOrigin => {
                "ambiguous-created-spell-slot-source-rejected"
            }
            CharacterBattleSettlementRejection::CharacterSheetMismatch => {
                "mismatched-character-identity-rejected"
            }
            CharacterBattleSettlementRejection::MaximumHpDrift => "maximum-hp-drift-rejected",
            CharacterBattleSettlementRejection::ActiveWildShape => {
                "active-wild-shape-handoff-rejected"
            }
            CharacterBattleSettlementRejection::ActiveBattleState => {
                "active-battle-state-handoff-rejected"
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
