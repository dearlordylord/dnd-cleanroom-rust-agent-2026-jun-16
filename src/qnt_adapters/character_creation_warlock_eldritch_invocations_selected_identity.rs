use super::character_creation_expected_routes::expected_retained_reference_route;
use crate::rules::character_creation::{
    apply_creation_retained_reference_operation, completed_fighter_creation_state, route_payload,
    CreationRetainedReferenceOperation, CreationRouteEvent,
};
use crate::rules::class_features::{
    warlock_invocation_selection_projection, EldritchInvocation, ProjectionError,
    WarlockInvocationSelectionFacts,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WarlockInvocationWitness {
    pub outcome: &'static str,
    pub selected_from_unit_id: &'static str,
    pub selected_invocation_count: u8,
    pub selected_class_choice_feature_ref_count: u8,
    pub warlock_invocations_unit_ref_present: bool,
    pub armor_of_shadows_invocation_present: bool,
    pub pact_blade_invocation_present: bool,
    pub devils_sight_invocation_present: bool,
    pub eldritch_mind_invocation_present: bool,
    pub thirsting_blade_invocation_present: bool,
    pub repelling_blast_eldritch_blast_present: bool,
    pub repelling_blast_poison_spray_present: bool,
    pub armor_of_shadows_unit_ref_present: bool,
    pub pact_magic_cantrip_count: u8,
    pub pact_magic_prepared_spell_count: u8,
    pub pact_magic_slot_count: u8,
    pub pact_magic_slot_level: u8,
    pub total_level: u8,
    pub locked_replacement_rejected: bool,
    pub duplicate_non_repeatable_rejected: bool,
    pub duplicate_repeatable_choice_rejected: bool,
}

pub const BRANCH_ACTIONS: &[&str] = &[
    "doSelectLevelOneArmorOfShadows",
    "doGainLevelTwoInvocations",
    "doReplaceArmorWithEldritchMindOnWarlockLevelGain",
    "doReplaceRepeatableInvocationByChoice",
    "doRejectPrerequisiteRetainedInvocationReplacement",
    "doRejectDuplicateInvocationSelections",
];

pub fn replay_observed_action(observed_action_taken: &str) -> WarlockInvocationWitness {
    let scenario = scenario(observed_action_taken);
    match warlock_invocation_selection_projection(scenario.facts.clone()) {
        Ok(projection) => WarlockInvocationWitness {
            outcome: scenario.outcome,
            selected_from_unit_id: "warlock_eldritch_invocations",
            selected_invocation_count: projection.selected_invocation_count,
            selected_class_choice_feature_ref_count: projection
                .selected_class_choice_feature_ref_count,
            warlock_invocations_unit_ref_present: true,
            armor_of_shadows_invocation_present: projection
                .selected_invocations
                .contains(&EldritchInvocation::ArmorOfShadows),
            pact_blade_invocation_present: projection
                .selected_invocations
                .contains(&EldritchInvocation::PactBlade),
            devils_sight_invocation_present: projection
                .selected_invocations
                .contains(&EldritchInvocation::DevilsSight),
            eldritch_mind_invocation_present: projection
                .selected_invocations
                .contains(&EldritchInvocation::EldritchMind),
            thirsting_blade_invocation_present: projection
                .selected_invocations
                .contains(&EldritchInvocation::ThirstingBlade),
            repelling_blast_eldritch_blast_present: projection
                .selected_invocations
                .contains(&EldritchInvocation::RepellingBlastEldritchBlast),
            repelling_blast_poison_spray_present: projection
                .selected_invocations
                .contains(&EldritchInvocation::RepellingBlastPoisonSpray),
            armor_of_shadows_unit_ref_present: projection
                .selected_invocations
                .contains(&EldritchInvocation::ArmorOfShadows),
            pact_magic_cantrip_count: projection.pact_magic_cantrip_count,
            pact_magic_prepared_spell_count: projection.pact_magic_prepared_spell_count,
            pact_magic_slot_count: projection.pact_magic_slot_count,
            pact_magic_slot_level: projection.pact_magic_slot_level,
            total_level: projection.total_level,
            locked_replacement_rejected: false,
            duplicate_non_repeatable_rejected: false,
            duplicate_repeatable_choice_rejected: false,
        },
        Err(error) => rejected_witness(scenario.outcome, scenario.facts, error),
    }
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<CreationRouteEvent> {
    let operation = match observed_action_taken {
        "doSelectLevelOneArmorOfShadows" => CreationRetainedReferenceOperation::RetainAndProject,
        "doGainLevelTwoInvocations"
        | "doReplaceArmorWithEldritchMindOnWarlockLevelGain"
        | "doReplaceRepeatableInvocationByChoice" => {
            CreationRetainedReferenceOperation::ReplaceAndProject
        }
        "doRejectPrerequisiteRetainedInvocationReplacement"
        | "doRejectDuplicateInvocationSelections" => {
            CreationRetainedReferenceOperation::RejectSelection
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    };
    apply_creation_retained_reference_operation(&completed_fighter_creation_state(), operation)
        .route
}

pub fn expected_route(observed_action_taken: &str) -> Vec<CreationRouteEvent> {
    let operation = match observed_action_taken {
        "doSelectLevelOneArmorOfShadows" => CreationRetainedReferenceOperation::RetainAndProject,
        "doGainLevelTwoInvocations"
        | "doReplaceArmorWithEldritchMindOnWarlockLevelGain"
        | "doReplaceRepeatableInvocationByChoice" => {
            CreationRetainedReferenceOperation::ReplaceAndProject
        }
        "doRejectPrerequisiteRetainedInvocationReplacement"
        | "doRejectDuplicateInvocationSelections" => {
            CreationRetainedReferenceOperation::RejectSelection
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    };
    expected_retained_reference_route(operation)
}

pub fn projection_payload(witness: &WarlockInvocationWitness) -> String {
    [
        format!("outcome={}", witness.outcome),
        format!("selectedFromUnitId={}", witness.selected_from_unit_id),
        format!(
            "selectedInvocationCount={}",
            witness.selected_invocation_count
        ),
        format!(
            "selectedClassChoiceFeatureRefCount={}",
            witness.selected_class_choice_feature_ref_count
        ),
        format!(
            "warlockInvocationsUnitRefPresent={}",
            witness.warlock_invocations_unit_ref_present
        ),
        format!(
            "armorOfShadowsInvocationPresent={}",
            witness.armor_of_shadows_invocation_present
        ),
        format!(
            "pactBladeInvocationPresent={}",
            witness.pact_blade_invocation_present
        ),
        format!(
            "devilsSightInvocationPresent={}",
            witness.devils_sight_invocation_present
        ),
        format!(
            "eldritchMindInvocationPresent={}",
            witness.eldritch_mind_invocation_present
        ),
        format!(
            "thirstingBladeInvocationPresent={}",
            witness.thirsting_blade_invocation_present
        ),
        format!(
            "repellingBlastEldritchBlastPresent={}",
            witness.repelling_blast_eldritch_blast_present
        ),
        format!(
            "repellingBlastPoisonSprayPresent={}",
            witness.repelling_blast_poison_spray_present
        ),
        format!(
            "armorOfShadowsUnitRefPresent={}",
            witness.armor_of_shadows_unit_ref_present
        ),
        format!("pactMagicCantripCount={}", witness.pact_magic_cantrip_count),
        format!(
            "pactMagicPreparedSpellCount={}",
            witness.pact_magic_prepared_spell_count
        ),
        format!("pactMagicSlotCount={}", witness.pact_magic_slot_count),
        format!("pactMagicSlotLevel={}", witness.pact_magic_slot_level),
        format!("totalLevel={}", witness.total_level),
        format!(
            "lockedReplacementRejected={}",
            witness.locked_replacement_rejected
        ),
        format!(
            "duplicateNonRepeatableRejected={}",
            witness.duplicate_non_repeatable_rejected
        ),
        format!(
            "duplicateRepeatableChoiceRejected={}",
            witness.duplicate_repeatable_choice_rejected
        ),
    ]
    .join("\n")
}

pub fn route_projection_payload(route: &[CreationRouteEvent]) -> String {
    route_payload(route)
}

#[derive(Debug, Clone)]
struct Scenario {
    outcome: &'static str,
    facts: WarlockInvocationSelectionFacts,
}

fn scenario(action: &str) -> Scenario {
    match action {
        "doSelectLevelOneArmorOfShadows" => Scenario {
            outcome: "level-one-selected",
            facts: facts(vec![EldritchInvocation::ArmorOfShadows], 2, 2, 1, 1, 1),
        },
        "doGainLevelTwoInvocations" => Scenario {
            outcome: "level-two-gained",
            facts: facts(
                vec![
                    EldritchInvocation::ArmorOfShadows,
                    EldritchInvocation::PactBlade,
                    EldritchInvocation::DevilsSight,
                ],
                2,
                3,
                2,
                1,
                2,
            ),
        },
        "doReplaceArmorWithEldritchMindOnWarlockLevelGain" => Scenario {
            outcome: "non-repeatable-replaced",
            facts: facts(
                vec![
                    EldritchInvocation::PactBlade,
                    EldritchInvocation::DevilsSight,
                    EldritchInvocation::EldritchMind,
                ],
                2,
                4,
                2,
                2,
                3,
            ),
        },
        "doReplaceRepeatableInvocationByChoice" => Scenario {
            outcome: "repeatable-replaced",
            facts: facts(
                vec![
                    EldritchInvocation::ArmorOfShadows,
                    EldritchInvocation::DevilsSight,
                    EldritchInvocation::RepellingBlastEldritchBlast,
                ],
                2,
                4,
                2,
                2,
                3,
            ),
        },
        "doRejectPrerequisiteRetainedInvocationReplacement" => {
            let mut facts = facts(
                vec![
                    EldritchInvocation::ArmorOfShadows,
                    EldritchInvocation::PactBlade,
                    EldritchInvocation::DevilsSight,
                    EldritchInvocation::EldritchMind,
                    EldritchInvocation::ThirstingBlade,
                ],
                3,
                6,
                2,
                3,
                5,
            );
            facts.replacement_locked_by_prerequisite = true;
            Scenario {
                outcome: "locked-replacement-rejected",
                facts,
            }
        }
        "doRejectDuplicateInvocationSelections" => Scenario {
            outcome: "duplicate-selection-rejected",
            facts: facts(
                vec![
                    EldritchInvocation::ArmorOfShadows,
                    EldritchInvocation::ArmorOfShadows,
                ],
                2,
                2,
                1,
                1,
                1,
            ),
        },
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

fn facts(
    selected_invocations: Vec<EldritchInvocation>,
    pact_magic_cantrip_count: u8,
    pact_magic_prepared_spell_count: u8,
    pact_magic_slot_count: u8,
    pact_magic_slot_level: u8,
    total_level: u8,
) -> WarlockInvocationSelectionFacts {
    WarlockInvocationSelectionFacts {
        selected_invocations,
        replacement_locked_by_prerequisite: false,
        pact_magic_cantrip_count,
        pact_magic_prepared_spell_count,
        pact_magic_slot_count,
        pact_magic_slot_level,
        total_level,
    }
}

fn rejected_witness(
    outcome: &'static str,
    facts: WarlockInvocationSelectionFacts,
    error: ProjectionError,
) -> WarlockInvocationWitness {
    WarlockInvocationWitness {
        outcome,
        selected_from_unit_id: "warlock_eldritch_invocations",
        selected_invocation_count: facts.selected_invocations.len() as u8,
        selected_class_choice_feature_ref_count: 0,
        warlock_invocations_unit_ref_present: true,
        armor_of_shadows_invocation_present: facts
            .selected_invocations
            .contains(&EldritchInvocation::ArmorOfShadows),
        pact_blade_invocation_present: facts
            .selected_invocations
            .contains(&EldritchInvocation::PactBlade),
        devils_sight_invocation_present: facts
            .selected_invocations
            .contains(&EldritchInvocation::DevilsSight),
        eldritch_mind_invocation_present: facts
            .selected_invocations
            .contains(&EldritchInvocation::EldritchMind),
        thirsting_blade_invocation_present: facts
            .selected_invocations
            .contains(&EldritchInvocation::ThirstingBlade),
        repelling_blast_eldritch_blast_present: facts
            .selected_invocations
            .contains(&EldritchInvocation::RepellingBlastEldritchBlast),
        repelling_blast_poison_spray_present: facts
            .selected_invocations
            .contains(&EldritchInvocation::RepellingBlastPoisonSpray),
        armor_of_shadows_unit_ref_present: facts
            .selected_invocations
            .contains(&EldritchInvocation::ArmorOfShadows),
        pact_magic_cantrip_count: facts.pact_magic_cantrip_count,
        pact_magic_prepared_spell_count: facts.pact_magic_prepared_spell_count,
        pact_magic_slot_count: facts.pact_magic_slot_count,
        pact_magic_slot_level: facts.pact_magic_slot_level,
        total_level: facts.total_level,
        locked_replacement_rejected: error == ProjectionError::LockedInvocationReplacement,
        duplicate_non_repeatable_rejected: error == ProjectionError::DuplicateInvocationSelection,
        duplicate_repeatable_choice_rejected: error
            == ProjectionError::DuplicateInvocationSelection,
    }
}
