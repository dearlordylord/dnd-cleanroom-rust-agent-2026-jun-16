use crate::rules::character_sheet::{
    apply_arcane_recovery, complete_long_rest_slot_benefits, empty_slot_facts,
    full_caster_spell_slot_table, ordinary_slots_from_table, pact_magic_spell_slot_table,
    pact_slots_from_table, recover_pact_slots, FullCasterSpellSlotTableLevel,
    PactMagicSpellSlotTableLevel, SheetSlotFacts,
};

use super::character_sheet_reducer_route::{
    initial_sheet_build_route, route_complete_character_sheet_rest,
    route_resolve_character_sheet_subject, CharacterSheetRouteEvent, CharacterSheetRouteFillFamily,
    CharacterSheetRouteHoleFamily, CharacterSheetRouteOwnerGroup, CharacterSheetRouteSubjectFamily,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArcaneRecoveryWitness {
    pub feature_unit_id: &'static str,
    pub first_level_spell_slots_expended: i16,
    pub second_level_spell_slots_expended: i16,
    pub pact_slots_expended: i16,
    pub arcane_recovery_used_since_long_rest: bool,
    pub accepted: bool,
    pub message: &'static str,
    pub recovered_combined_slot_levels: i16,
    pub last_result: &'static str,
}

pub const BRANCH_ACTIONS: [&str; 3] = [
    "doRecoverSecondLevelSpellSlot",
    "doResetArcaneRecoveryOnLongRest",
    "doRejectPactSlotArcaneRecovery",
];

pub fn replay_observed_action(observed_action_taken: &str) -> ArcaneRecoveryWitness {
    match observed_action_taken {
        "doRecoverSecondLevelSpellSlot" => recover_second_level_spell_slot(),
        "doResetArcaneRecoveryOnLongRest" => reset_arcane_recovery_on_long_rest(),
        "doRejectPactSlotArcaneRecovery" => reject_pact_slot_arcane_recovery(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> ArcaneRecoveryWitness {
    match observed_action_taken {
        "doRecoverSecondLevelSpellSlot" => ArcaneRecoveryWitness {
            feature_unit_id: "wizard_arcane_recovery",
            first_level_spell_slots_expended: 2,
            second_level_spell_slots_expended: 0,
            pact_slots_expended: 0,
            arcane_recovery_used_since_long_rest: true,
            accepted: true,
            message: "none",
            recovered_combined_slot_levels: 2,
            last_result: "short-rest-recovered",
        },
        "doResetArcaneRecoveryOnLongRest" => ArcaneRecoveryWitness {
            feature_unit_id: "wizard_arcane_recovery",
            first_level_spell_slots_expended: 0,
            second_level_spell_slots_expended: 0,
            pact_slots_expended: 0,
            arcane_recovery_used_since_long_rest: false,
            accepted: true,
            message: "none",
            recovered_combined_slot_levels: 0,
            last_result: "long-rest-reset",
        },
        "doRejectPactSlotArcaneRecovery" => ArcaneRecoveryWitness {
            feature_unit_id: "wizard_arcane_recovery",
            first_level_spell_slots_expended: 0,
            second_level_spell_slots_expended: 0,
            pact_slots_expended: 1,
            arcane_recovery_used_since_long_rest: false,
            accepted: false,
            message: "Arcane Recovery cannot refund more Spell Slots than are expended.",
            recovered_combined_slot_levels: 0,
            last_result: "pact-slot-rejected",
        },
        action => panic!("unsupported expected mbt::actionTaken {action}"),
    }
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<CharacterSheetRouteEvent> {
    let mut route = initial_sheet_build_route();
    match observed_action_taken {
        "doRecoverSecondLevelSpellSlot" | "doResetArcaneRecoveryOnLongRest" => {
            route.push(route_complete_character_sheet_rest(
                CharacterSheetRouteSubjectFamily::SheetSpellResource,
                CharacterSheetRouteFillFamily::RecoverySelection,
                Vec::new(),
                CharacterSheetRouteOwnerGroup::CharacterSheetSpellSlot,
            ));
        }
        "doRejectPactSlotArcaneRecovery" => {
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

pub fn projection_payload(witness: &ArcaneRecoveryWitness) -> String {
    [
        format!("featureUnitId={}", witness.feature_unit_id),
        format!(
            "firstLevelSpellSlotsExpended={}",
            witness.first_level_spell_slots_expended
        ),
        format!(
            "secondLevelSpellSlotsExpended={}",
            witness.second_level_spell_slots_expended
        ),
        format!("pactSlotsExpended={}", witness.pact_slots_expended),
        format!(
            "arcaneRecoveryUsedSinceLongRest={}",
            witness.arcane_recovery_used_since_long_rest
        ),
        format!("accepted={}", witness.accepted),
        format!("message={}", witness.message),
        format!(
            "recoveredCombinedSlotLevels={}",
            witness.recovered_combined_slot_levels
        ),
        format!("lastResult={}", witness.last_result),
    ]
    .join("\n")
}

fn recover_second_level_spell_slot() -> ArcaneRecoveryWitness {
    let sheet = apply_arcane_recovery(
        recover_pact_slots(arcane_recovery_level2_spent_slots()),
        0,
        1,
    );
    witness_from_sheet("short-rest-recovered", true, "none", 2, sheet)
}

fn reset_arcane_recovery_on_long_rest() -> ArcaneRecoveryWitness {
    let sheet = complete_long_rest_slot_benefits(empty_slot_facts());
    witness_from_sheet("long-rest-reset", true, "none", 0, sheet)
}

fn reject_pact_slot_arcane_recovery() -> ArcaneRecoveryWitness {
    let mut sheet = empty_slot_facts();
    sheet.pact = pact_slots_from_table(
        pact_magic_spell_slot_table(PactMagicSpellSlotTableLevel::One),
        1,
    );
    witness_from_sheet(
        "pact-slot-rejected",
        false,
        "Arcane Recovery cannot refund more Spell Slots than are expended.",
        0,
        sheet,
    )
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

fn witness_from_sheet(
    last_result: &'static str,
    accepted: bool,
    message: &'static str,
    recovered_combined_slot_levels: i16,
    sheet: SheetSlotFacts,
) -> ArcaneRecoveryWitness {
    ArcaneRecoveryWitness {
        feature_unit_id: "wizard_arcane_recovery",
        first_level_spell_slots_expended: sheet.ordinary.level1_expended,
        second_level_spell_slots_expended: sheet.ordinary.level2_expended,
        pact_slots_expended: sheet.pact.expended,
        arcane_recovery_used_since_long_rest: sheet.arcane_recovery_used_since_long_rest,
        accepted,
        message,
        recovered_combined_slot_levels,
        last_result,
    }
}
