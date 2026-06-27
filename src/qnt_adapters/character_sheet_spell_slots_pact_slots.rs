use crate::rules::character_sheet::{
    apply_arcane_recovery, apply_magical_cunning, can_apply_magical_cunning,
    can_recover_ordinary_spell_slots, complete_long_rest_slot_benefits, empty_slot_facts,
    full_caster_spell_slot_table, ordinary_slots_from_table, pact_magic_spell_slot_table,
    pact_slot_expenditure_within_capacity, pact_slots_from_table, recover_pact_slots,
    FullCasterSpellSlotTableLevel, PactMagicSpellSlotTableLevel, SheetSlotFacts,
};

use super::character_sheet_reducer_route::{
    initial_sheet_build_route, route_complete_character_sheet_rest,
    route_resolve_character_sheet_subject, CharacterSheetRouteEvent, CharacterSheetRouteFillFamily,
    CharacterSheetRouteHoleFamily, CharacterSheetRouteOwnerGroup, CharacterSheetRouteSubjectFamily,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpellSlotsPactSlotsWitness {
    pub last_result: &'static str,
    pub accepted: bool,
    pub message: &'static str,
    pub ordinary_level1_capacity: i16,
    pub ordinary_level1_expended: i16,
    pub ordinary_level2_capacity: i16,
    pub ordinary_level2_expended: i16,
    pub created_level1_capacity: i16,
    pub created_level1_expended: i16,
    pub pact_slot_level: i16,
    pub pact_slot_capacity: i16,
    pub pact_slot_expended: i16,
    pub arcane_recovery_used_since_long_rest: bool,
    pub magical_cunning_used_since_long_rest: bool,
    pub replay_index: u8,
}

pub const BRANCH_ACTIONS: [&str; 11] = [
    "doRejectMismatchedOrdinarySpellSlotCapacity",
    "doRejectPactSlotExpenditureOverCapacity",
    "doShortRestRestoresPactSlotsOnly",
    "doShortRestArcaneRecoveryRefundsOrdinarySpellSlot",
    "doCompleteLongRestRestoresOrdinaryPactAndClearsCreatedSlots",
    "doInterruptShortRestNoSlotBenefit",
    "doInterruptLongRestBeforeOneHourNoSlotBenefit",
    "doInterruptLongRestWithShortRestSlotBenefits",
    "doMagicalCunningRecoversPactSlots",
    "doRejectMagicalCunningWithoutExpendedPactSlots",
    "doRejectArcaneRecoveryPactSlotRefund",
];

pub fn replay_observed_action(observed_action_taken: &str) -> SpellSlotsPactSlotsWitness {
    match observed_action_taken {
        "doRejectMismatchedOrdinarySpellSlotCapacity" => {
            reject_mismatched_ordinary_spell_slot_capacity()
        }
        "doRejectPactSlotExpenditureOverCapacity" => reject_pact_slot_expenditure_over_capacity(),
        "doShortRestRestoresPactSlotsOnly" => short_rest_restores_pact_slots_only(),
        "doShortRestArcaneRecoveryRefundsOrdinarySpellSlot" => {
            short_rest_arcane_recovery_refunds_ordinary_spell_slot()
        }
        "doCompleteLongRestRestoresOrdinaryPactAndClearsCreatedSlots" => complete_long_rest_slots(),
        "doInterruptShortRestNoSlotBenefit" => interrupt_short_rest_no_slot_benefit(),
        "doInterruptLongRestBeforeOneHourNoSlotBenefit" => {
            interrupt_long_rest_before_one_hour_no_slot_benefit()
        }
        "doInterruptLongRestWithShortRestSlotBenefits" => {
            interrupt_long_rest_with_short_rest_slot_benefits()
        }
        "doMagicalCunningRecoversPactSlots" => magical_cunning_recovers_pact_slots(),
        "doRejectMagicalCunningWithoutExpendedPactSlots" => {
            reject_magical_cunning_without_expended_pact_slots()
        }
        "doRejectArcaneRecoveryPactSlotRefund" => reject_arcane_recovery_pact_slot_refund(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> SpellSlotsPactSlotsWitness {
    match observed_action_taken {
        "doRejectMismatchedOrdinarySpellSlotCapacity" => expected(
            "ordinary-capacity-mismatch-rejected",
            false,
            "Spell Slot state does not match build capacity for level 1.",
            mismatched_ordinary_level1_capacity(),
            1,
        ),
        "doRejectPactSlotExpenditureOverCapacity" => expected(
            "pact-expenditure-over-capacity-rejected",
            false,
            "Pact Slot state must match Pact Magic build capacity.",
            pact_slot_expended_over_capacity(),
            2,
        ),
        "doShortRestRestoresPactSlotsOnly" => expected(
            "short-rest-restores-pact-slots-only",
            true,
            "none",
            recover_pact_slots(short_rest_pact_spent_slots()),
            3,
        ),
        "doShortRestArcaneRecoveryRefundsOrdinarySpellSlot" => expected(
            "short-rest-arcane-recovery-refunds-ordinary-spell-slot",
            true,
            "none",
            apply_arcane_recovery(
                recover_pact_slots(arcane_recovery_level2_spent_slots()),
                0,
                1,
            ),
            4,
        ),
        "doCompleteLongRestRestoresOrdinaryPactAndClearsCreatedSlots" => expected(
            "long-rest-restores-ordinary-pact-and-clears-created-slots",
            true,
            "none",
            complete_long_rest_slot_benefits(long_rest_created_slot_spent_slots()),
            5,
        ),
        "doInterruptShortRestNoSlotBenefit" => expected(
            "short-rest-interrupted-no-slot-benefit",
            true,
            "none",
            short_rest_pact_spent_slots(),
            6,
        ),
        "doInterruptLongRestBeforeOneHourNoSlotBenefit" => expected(
            "long-rest-interrupted-before-one-hour-no-slot-benefit",
            true,
            "none",
            short_rest_pact_spent_slots(),
            7,
        ),
        "doInterruptLongRestWithShortRestSlotBenefits" => expected(
            "long-rest-interrupted-with-short-rest-slot-benefits",
            true,
            "none",
            recover_pact_slots(short_rest_pact_spent_slots()),
            8,
        ),
        "doMagicalCunningRecoversPactSlots" => expected(
            "magical-cunning-recovers-pact-slots",
            true,
            "none",
            apply_magical_cunning(magical_cunning_spent_pact_slots()),
            9,
        ),
        "doRejectMagicalCunningWithoutExpendedPactSlots" => expected(
            "magical-cunning-no-expended-pact-slots-rejected",
            false,
            "Magical Cunning must recover expended Pact Slots.",
            magical_cunning_fresh_pact_slots(),
            10,
        ),
        "doRejectArcaneRecoveryPactSlotRefund" => expected(
            "arcane-recovery-pact-slot-refund-rejected",
            false,
            "Arcane Recovery cannot refund more Spell Slots than are expended.",
            arcane_recovery_no_expended_ordinary_slots(),
            11,
        ),
        action => panic!("unsupported expected mbt::actionTaken {action}"),
    }
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<CharacterSheetRouteEvent> {
    let mut route = initial_sheet_build_route();
    match observed_action_taken {
        "doRejectMismatchedOrdinarySpellSlotCapacity" => {
            route.push(reject_spell_resource_route(
                CharacterSheetRouteOwnerGroup::CharacterSheetSpellSlot,
            ));
        }
        "doRejectPactSlotExpenditureOverCapacity" => {
            route.push(reject_spell_resource_route(
                CharacterSheetRouteOwnerGroup::CharacterSheetPactSlot,
            ));
        }
        "doShortRestRestoresPactSlotsOnly" | "doInterruptLongRestWithShortRestSlotBenefits" => {
            route.push(complete_restored_slot_route(
                CharacterSheetRouteOwnerGroup::CharacterSheetPactSlot,
            ));
        }
        "doShortRestArcaneRecoveryRefundsOrdinarySpellSlot" => {
            route.push(route_complete_character_sheet_rest(
                CharacterSheetRouteSubjectFamily::SheetSpellResource,
                CharacterSheetRouteFillFamily::RecoverySelection,
                Vec::new(),
                CharacterSheetRouteOwnerGroup::CharacterSheetSpellSlot,
            ));
        }
        "doCompleteLongRestRestoresOrdinaryPactAndClearsCreatedSlots" => {
            route.push(complete_restored_slot_route(
                CharacterSheetRouteOwnerGroup::CharacterSheetSpellSlot,
            ));
            route.push(complete_restored_slot_route(
                CharacterSheetRouteOwnerGroup::CharacterSheetPactSlot,
            ));
        }
        "doInterruptShortRestNoSlotBenefit" | "doInterruptLongRestBeforeOneHourNoSlotBenefit" => {
            route.push(route_complete_character_sheet_rest(
                CharacterSheetRouteSubjectFamily::SheetRest,
                CharacterSheetRouteFillFamily::RestDuration,
                vec![CharacterSheetRouteHoleFamily::RestBenefitChoice],
                CharacterSheetRouteOwnerGroup::CharacterSheetSpellSlot,
            ));
        }
        "doMagicalCunningRecoversPactSlots" => {
            route.push(route_resolve_character_sheet_subject(
                CharacterSheetRouteSubjectFamily::SheetFeatureResource,
                CharacterSheetRouteFillFamily::RecoverySelection,
                Vec::new(),
                CharacterSheetRouteOwnerGroup::CharacterSheetFeatureResource,
            ));
            route.push(route_resolve_character_sheet_subject(
                CharacterSheetRouteSubjectFamily::SheetSpellResource,
                CharacterSheetRouteFillFamily::RecoverySelection,
                Vec::new(),
                CharacterSheetRouteOwnerGroup::CharacterSheetPactSlot,
            ));
        }
        "doRejectMagicalCunningWithoutExpendedPactSlots"
        | "doRejectArcaneRecoveryPactSlotRefund" => {
            route.push(route_resolve_character_sheet_subject(
                CharacterSheetRouteSubjectFamily::SheetSpellResource,
                CharacterSheetRouteFillFamily::RecoverySelection,
                vec![CharacterSheetRouteHoleFamily::RecoveryChoice],
                CharacterSheetRouteOwnerGroup::CharacterSheetPactSlot,
            ));
        }
        action => panic!("unsupported route mbt::actionTaken {action}"),
    }
    route
}

pub fn expected_route(observed_action_taken: &str) -> Vec<CharacterSheetRouteEvent> {
    replay_observed_route(observed_action_taken)
}

pub fn projection_payload(witness: &SpellSlotsPactSlotsWitness) -> String {
    [
        format!("lastResult={}", witness.last_result),
        format!("accepted={}", witness.accepted),
        format!("message={}", witness.message),
        format!(
            "ordinaryLevel1Capacity={}",
            witness.ordinary_level1_capacity
        ),
        format!(
            "ordinaryLevel1Expended={}",
            witness.ordinary_level1_expended
        ),
        format!(
            "ordinaryLevel2Capacity={}",
            witness.ordinary_level2_capacity
        ),
        format!(
            "ordinaryLevel2Expended={}",
            witness.ordinary_level2_expended
        ),
        format!("createdLevel1Capacity={}", witness.created_level1_capacity),
        format!("createdLevel1Expended={}", witness.created_level1_expended),
        format!("pactSlotLevel={}", witness.pact_slot_level),
        format!("pactSlotCapacity={}", witness.pact_slot_capacity),
        format!("pactSlotExpended={}", witness.pact_slot_expended),
        format!(
            "arcaneRecoveryUsedSinceLongRest={}",
            witness.arcane_recovery_used_since_long_rest
        ),
        format!(
            "magicalCunningUsedSinceLongRest={}",
            witness.magical_cunning_used_since_long_rest
        ),
        format!("replayIndex={}", witness.replay_index),
    ]
    .join("\n")
}

fn reject_mismatched_ordinary_spell_slot_capacity() -> SpellSlotsPactSlotsWitness {
    let sheet = mismatched_ordinary_level1_capacity();
    record_projection(
        "ordinary-capacity-mismatch-rejected",
        false,
        "Spell Slot state does not match build capacity for level 1.",
        sheet,
        1,
    )
}

fn reject_pact_slot_expenditure_over_capacity() -> SpellSlotsPactSlotsWitness {
    let sheet = pact_slot_expended_over_capacity();
    assert!(!pact_slot_expenditure_within_capacity(sheet));
    record_projection(
        "pact-expenditure-over-capacity-rejected",
        false,
        "Pact Slot state must match Pact Magic build capacity.",
        sheet,
        2,
    )
}

fn short_rest_restores_pact_slots_only() -> SpellSlotsPactSlotsWitness {
    record_projection(
        "short-rest-restores-pact-slots-only",
        true,
        "none",
        recover_pact_slots(short_rest_pact_spent_slots()),
        3,
    )
}

fn short_rest_arcane_recovery_refunds_ordinary_spell_slot() -> SpellSlotsPactSlotsWitness {
    let recovered_pact = recover_pact_slots(arcane_recovery_level2_spent_slots());
    assert!(can_recover_ordinary_spell_slots(recovered_pact, 0, 1));
    record_projection(
        "short-rest-arcane-recovery-refunds-ordinary-spell-slot",
        true,
        "none",
        apply_arcane_recovery(recovered_pact, 0, 1),
        4,
    )
}

fn complete_long_rest_slots() -> SpellSlotsPactSlotsWitness {
    record_projection(
        "long-rest-restores-ordinary-pact-and-clears-created-slots",
        true,
        "none",
        complete_long_rest_slot_benefits(long_rest_created_slot_spent_slots()),
        5,
    )
}

fn interrupt_short_rest_no_slot_benefit() -> SpellSlotsPactSlotsWitness {
    record_projection(
        "short-rest-interrupted-no-slot-benefit",
        true,
        "none",
        short_rest_pact_spent_slots(),
        6,
    )
}

fn interrupt_long_rest_before_one_hour_no_slot_benefit() -> SpellSlotsPactSlotsWitness {
    record_projection(
        "long-rest-interrupted-before-one-hour-no-slot-benefit",
        true,
        "none",
        short_rest_pact_spent_slots(),
        7,
    )
}

fn interrupt_long_rest_with_short_rest_slot_benefits() -> SpellSlotsPactSlotsWitness {
    record_projection(
        "long-rest-interrupted-with-short-rest-slot-benefits",
        true,
        "none",
        recover_pact_slots(short_rest_pact_spent_slots()),
        8,
    )
}

fn magical_cunning_recovers_pact_slots() -> SpellSlotsPactSlotsWitness {
    let sheet = magical_cunning_spent_pact_slots();
    assert!(can_apply_magical_cunning(sheet));
    record_projection(
        "magical-cunning-recovers-pact-slots",
        true,
        "none",
        apply_magical_cunning(sheet),
        9,
    )
}

fn reject_magical_cunning_without_expended_pact_slots() -> SpellSlotsPactSlotsWitness {
    let sheet = magical_cunning_fresh_pact_slots();
    assert!(!can_apply_magical_cunning(sheet));
    record_projection(
        "magical-cunning-no-expended-pact-slots-rejected",
        false,
        "Magical Cunning must recover expended Pact Slots.",
        sheet,
        10,
    )
}

fn reject_arcane_recovery_pact_slot_refund() -> SpellSlotsPactSlotsWitness {
    let sheet = arcane_recovery_no_expended_ordinary_slots();
    assert!(!can_recover_ordinary_spell_slots(sheet, 1, 0));
    record_projection(
        "arcane-recovery-pact-slot-refund-rejected",
        false,
        "Arcane Recovery cannot refund more Spell Slots than are expended.",
        sheet,
        11,
    )
}

fn short_rest_pact_spent_slots() -> SheetSlotFacts {
    SheetSlotFacts {
        ordinary: ordinary_slots_from_table(
            full_caster_spell_slot_table(FullCasterSpellSlotTableLevel::One),
            1,
            0,
            0,
            0,
        ),
        pact: pact_slots_from_table(
            pact_magic_spell_slot_table(PactMagicSpellSlotTableLevel::One),
            1,
        ),
        arcane_recovery_used_since_long_rest: false,
        magical_cunning_used_since_long_rest: false,
    }
}

fn arcane_recovery_level2_spent_slots() -> SheetSlotFacts {
    SheetSlotFacts {
        ordinary: ordinary_slots_from_table(
            full_caster_spell_slot_table(FullCasterSpellSlotTableLevel::Four),
            2,
            1,
            0,
            0,
        ),
        pact: pact_slots_from_table(
            pact_magic_spell_slot_table(PactMagicSpellSlotTableLevel::One),
            1,
        ),
        arcane_recovery_used_since_long_rest: false,
        magical_cunning_used_since_long_rest: false,
    }
}

fn long_rest_created_slot_spent_slots() -> SheetSlotFacts {
    SheetSlotFacts {
        ordinary: ordinary_slots_from_table(
            full_caster_spell_slot_table(FullCasterSpellSlotTableLevel::Two),
            2,
            0,
            1,
            0,
        ),
        pact: pact_slots_from_table(
            pact_magic_spell_slot_table(PactMagicSpellSlotTableLevel::Two),
            1,
        ),
        arcane_recovery_used_since_long_rest: false,
        magical_cunning_used_since_long_rest: false,
    }
}

fn mismatched_ordinary_level1_capacity() -> SheetSlotFacts {
    SheetSlotFacts {
        ordinary: ordinary_slots_from_table(
            full_caster_spell_slot_table(FullCasterSpellSlotTableLevel::Two),
            0,
            0,
            0,
            0,
        ),
        ..empty_slot_facts()
    }
}

fn pact_slot_expended_over_capacity() -> SheetSlotFacts {
    SheetSlotFacts {
        pact: pact_slots_from_table(
            pact_magic_spell_slot_table(PactMagicSpellSlotTableLevel::Two),
            3,
        ),
        ..empty_slot_facts()
    }
}

fn magical_cunning_spent_pact_slots() -> SheetSlotFacts {
    SheetSlotFacts {
        pact: pact_slots_from_table(
            pact_magic_spell_slot_table(PactMagicSpellSlotTableLevel::Two),
            2,
        ),
        ..empty_slot_facts()
    }
}

fn magical_cunning_fresh_pact_slots() -> SheetSlotFacts {
    SheetSlotFacts {
        pact: pact_slots_from_table(
            pact_magic_spell_slot_table(PactMagicSpellSlotTableLevel::Two),
            0,
        ),
        ..empty_slot_facts()
    }
}

fn arcane_recovery_no_expended_ordinary_slots() -> SheetSlotFacts {
    let mut sheet = arcane_recovery_level2_spent_slots();
    sheet.ordinary.level1_expended = 0;
    sheet.ordinary.level2_expended = 0;
    sheet
}

fn reject_spell_resource_route(owner: CharacterSheetRouteOwnerGroup) -> CharacterSheetRouteEvent {
    route_resolve_character_sheet_subject(
        CharacterSheetRouteSubjectFamily::SheetSpellResource,
        CharacterSheetRouteFillFamily::ResourceSpend,
        vec![CharacterSheetRouteHoleFamily::ResourceSpend],
        owner,
    )
}

fn complete_restored_slot_route(owner: CharacterSheetRouteOwnerGroup) -> CharacterSheetRouteEvent {
    route_complete_character_sheet_rest(
        CharacterSheetRouteSubjectFamily::SheetSpellResource,
        CharacterSheetRouteFillFamily::RestDuration,
        Vec::new(),
        owner,
    )
}

fn expected(
    last_result: &'static str,
    accepted: bool,
    message: &'static str,
    sheet: SheetSlotFacts,
    replay_index: u8,
) -> SpellSlotsPactSlotsWitness {
    record_projection(last_result, accepted, message, sheet, replay_index)
}

fn record_projection(
    last_result: &'static str,
    accepted: bool,
    message: &'static str,
    sheet: SheetSlotFacts,
    replay_index: u8,
) -> SpellSlotsPactSlotsWitness {
    SpellSlotsPactSlotsWitness {
        last_result,
        accepted,
        message,
        ordinary_level1_capacity: sheet.ordinary.level1_capacity,
        ordinary_level1_expended: sheet.ordinary.level1_expended,
        ordinary_level2_capacity: sheet.ordinary.level2_capacity,
        ordinary_level2_expended: sheet.ordinary.level2_expended,
        created_level1_capacity: sheet.ordinary.created_level1_capacity,
        created_level1_expended: sheet.ordinary.created_level1_expended,
        pact_slot_level: sheet.pact.slot_level,
        pact_slot_capacity: sheet.pact.capacity,
        pact_slot_expended: sheet.pact.expended,
        arcane_recovery_used_since_long_rest: sheet.arcane_recovery_used_since_long_rest,
        magical_cunning_used_since_long_rest: sheet.magical_cunning_used_since_long_rest,
        replay_index,
    }
}
