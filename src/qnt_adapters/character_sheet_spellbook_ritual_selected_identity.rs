use crate::rules::spellbook_rituals::{
    spellbook_ritual_invocation, PreparationRequirement, RequiredSpellAccess, SpellSlotCost,
    SpellbookRitualAccess, SpellbookRitualAdept, SpellbookRitualFacts, SpellbookRitualResult,
    SpellbookSpellKind,
};

use super::character_sheet_reducer_route::{
    initial_sheet_build_route, route_resolve_character_sheet_subject,
    route_retain_character_sheet_selected_references, CharacterSheetRouteEvent,
    CharacterSheetRouteFillFamily, CharacterSheetRouteHoleFamily, CharacterSheetRouteOwnerGroup,
    CharacterSheetRouteSubjectFamily,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpellbookRitualWitness {
    pub feature_unit_id: &'static str,
    pub spell_id: &'static str,
    pub spellcasting_source_unit_id: &'static str,
    pub spellbook_contains_ritual: bool,
    pub prepared_contains_ritual: bool,
    pub invocation_accepted: bool,
    pub spell_slot_cost_kind: &'static str,
    pub preparation_requirement: &'static str,
    pub required_spell_access: &'static str,
    pub additional_casting_time_minutes: i16,
    pub requires_reading_spellbook: bool,
    pub first_level_spell_slots_expended: i16,
    pub last_result: &'static str,
}

pub const BRANCH_ACTIONS: [&str; 5] = [
    "doInvokeSpellbookRitual",
    "doRejectPreparedOnlyRitual",
    "doRejectNonRitualSpellbookSpell",
    "doRejectMissingRitualAccessFeature",
    "doRejectNonLeveledRitualSpellbookSpell",
];

pub fn replay_observed_action(observed_action_taken: &str) -> SpellbookRitualWitness {
    match observed_action_taken {
        "doInvokeSpellbookRitual" => invoke_spellbook_ritual(),
        "doRejectPreparedOnlyRitual" => reject_prepared_only_ritual(),
        "doRejectNonRitualSpellbookSpell" => reject_non_ritual_spellbook_spell(),
        "doRejectMissingRitualAccessFeature" => reject_missing_ritual_access_feature(),
        "doRejectNonLeveledRitualSpellbookSpell" => reject_non_leveled_ritual_spellbook_spell(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> SpellbookRitualWitness {
    match observed_action_taken {
        "doInvokeSpellbookRitual" => expected(
            "wizard_ritual_adept",
            "detect_magic",
            "class_wizard",
            true,
            false,
            true,
            "none",
            "not_required",
            "spellbook",
            10,
            true,
            0,
            "invoked",
        ),
        "doRejectPreparedOnlyRitual" => expected(
            "wizard_ritual_adept",
            "detect_magic",
            "class_wizard",
            false,
            true,
            false,
            "none",
            "none",
            "none",
            0,
            false,
            0,
            "prepared_only_rejected",
        ),
        "doRejectNonRitualSpellbookSpell" => expected(
            "none",
            "none",
            "none",
            false,
            false,
            false,
            "none",
            "none",
            "none",
            0,
            false,
            0,
            "non_ritual_rejected",
        ),
        "doRejectMissingRitualAccessFeature" => expected(
            "none",
            "none",
            "none",
            true,
            false,
            false,
            "none",
            "none",
            "none",
            0,
            false,
            0,
            "missing_feature_rejected",
        ),
        "doRejectNonLeveledRitualSpellbookSpell" => expected(
            "none",
            "none",
            "none",
            true,
            false,
            false,
            "none",
            "none",
            "none",
            0,
            false,
            0,
            "non_leveled_rejected",
        ),
        action => panic!("unsupported expected mbt::actionTaken {action}"),
    }
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<CharacterSheetRouteEvent> {
    match observed_action_taken {
        "doInvokeSpellbookRitual" => spellbook_ritual_route(Vec::new()),
        "doRejectPreparedOnlyRitual"
        | "doRejectNonRitualSpellbookSpell"
        | "doRejectMissingRitualAccessFeature"
        | "doRejectNonLeveledRitualSpellbookSpell" => {
            spellbook_ritual_route(vec![CharacterSheetRouteHoleFamily::ProjectionChoice])
        }
        action => panic!("unsupported route mbt::actionTaken {action}"),
    }
}

pub fn expected_route(observed_action_taken: &str) -> Vec<CharacterSheetRouteEvent> {
    match observed_action_taken {
        "doInvokeSpellbookRitual" => expected_spellbook_ritual_route(Vec::new()),
        "doRejectPreparedOnlyRitual"
        | "doRejectNonRitualSpellbookSpell"
        | "doRejectMissingRitualAccessFeature"
        | "doRejectNonLeveledRitualSpellbookSpell" => {
            expected_spellbook_ritual_route(vec![CharacterSheetRouteHoleFamily::ProjectionChoice])
        }
        action => panic!("unsupported expected route mbt::actionTaken {action}"),
    }
}

pub fn projection_payload(witness: &SpellbookRitualWitness) -> String {
    [
        format!("featureUnitId={}", witness.feature_unit_id),
        format!("spellId={}", witness.spell_id),
        format!(
            "spellcastingSourceUnitId={}",
            witness.spellcasting_source_unit_id
        ),
        format!(
            "spellbookContainsRitual={}",
            witness.spellbook_contains_ritual
        ),
        format!(
            "preparedContainsRitual={}",
            witness.prepared_contains_ritual
        ),
        format!("invocationAccepted={}", witness.invocation_accepted),
        format!("spellSlotCostKind={}", witness.spell_slot_cost_kind),
        format!("preparationRequirement={}", witness.preparation_requirement),
        format!("requiredSpellAccess={}", witness.required_spell_access),
        format!(
            "additionalCastingTimeMinutes={}",
            witness.additional_casting_time_minutes
        ),
        format!(
            "requiresReadingSpellbook={}",
            witness.requires_reading_spellbook
        ),
        format!(
            "firstLevelSpellSlotsExpended={}",
            witness.first_level_spell_slots_expended
        ),
        format!("lastResult={}", witness.last_result),
    ]
    .join("\n")
}

fn spellbook_ritual_route(
    holes: Vec<CharacterSheetRouteHoleFamily>,
) -> Vec<CharacterSheetRouteEvent> {
    let mut route = initial_sheet_build_route();
    route.push(route_retain_character_sheet_selected_references(
        CharacterSheetRouteSubjectFamily::SheetSelectedReferenceProjection,
        CharacterSheetRouteOwnerGroup::CharacterSheetSelectedReference,
    ));
    route.push(route_resolve_character_sheet_subject(
        CharacterSheetRouteSubjectFamily::SheetSpellResource,
        CharacterSheetRouteFillFamily::ProjectionSelection,
        holes,
        CharacterSheetRouteOwnerGroup::CharacterSheetSelectedReference,
    ));
    route
}

fn expected_spellbook_ritual_route(
    holes: Vec<CharacterSheetRouteHoleFamily>,
) -> Vec<CharacterSheetRouteEvent> {
    let mut route = initial_sheet_build_route();
    route.push(route_retain_character_sheet_selected_references(
        CharacterSheetRouteSubjectFamily::SheetSelectedReferenceProjection,
        CharacterSheetRouteOwnerGroup::CharacterSheetSelectedReference,
    ));
    route.push(route_resolve_character_sheet_subject(
        CharacterSheetRouteSubjectFamily::SheetSpellResource,
        CharacterSheetRouteFillFamily::ProjectionSelection,
        holes,
        CharacterSheetRouteOwnerGroup::CharacterSheetSelectedReference,
    ));
    route
}

fn invoke_spellbook_ritual() -> SpellbookRitualWitness {
    let projection = spellbook_ritual_invocation(facts(
        SpellbookSpellKind::LevelOnePlusRitual,
        SpellbookRitualAccess::InSpellbook,
        SpellbookRitualAdept::Present,
    ));
    accepted_witness("invoked", true, false, projection)
}

fn reject_prepared_only_ritual() -> SpellbookRitualWitness {
    let projection = spellbook_ritual_invocation(facts(
        SpellbookSpellKind::LevelOnePlusRitual,
        SpellbookRitualAccess::PreparedOnly,
        SpellbookRitualAdept::Present,
    ));
    rejected_witness(
        "wizard_ritual_adept",
        "detect_magic",
        "class_wizard",
        false,
        true,
        "prepared_only_rejected",
        projection,
    )
}

fn reject_non_ritual_spellbook_spell() -> SpellbookRitualWitness {
    let projection = spellbook_ritual_invocation(facts(
        SpellbookSpellKind::LevelOnePlusNonRitual,
        SpellbookRitualAccess::InSpellbook,
        SpellbookRitualAdept::Present,
    ));
    rejected_witness(
        "none",
        "none",
        "none",
        false,
        false,
        "non_ritual_rejected",
        projection,
    )
}

fn reject_missing_ritual_access_feature() -> SpellbookRitualWitness {
    let projection = spellbook_ritual_invocation(facts(
        SpellbookSpellKind::LevelOnePlusRitual,
        SpellbookRitualAccess::InSpellbook,
        SpellbookRitualAdept::Missing,
    ));
    rejected_witness(
        "none",
        "none",
        "none",
        true,
        false,
        "missing_feature_rejected",
        projection,
    )
}

fn reject_non_leveled_ritual_spellbook_spell() -> SpellbookRitualWitness {
    let projection = spellbook_ritual_invocation(facts(
        SpellbookSpellKind::NonLeveled,
        SpellbookRitualAccess::InSpellbook,
        SpellbookRitualAdept::Present,
    ));
    rejected_witness(
        "none",
        "none",
        "none",
        true,
        false,
        "non_leveled_rejected",
        projection,
    )
}

fn facts(
    spell_kind: SpellbookSpellKind,
    access: SpellbookRitualAccess,
    ritual_adept: SpellbookRitualAdept,
) -> SpellbookRitualFacts {
    SpellbookRitualFacts {
        spell_kind,
        access,
        ritual_adept,
    }
}

#[allow(clippy::too_many_arguments)]
fn expected(
    feature_unit_id: &'static str,
    spell_id: &'static str,
    spellcasting_source_unit_id: &'static str,
    spellbook_contains_ritual: bool,
    prepared_contains_ritual: bool,
    invocation_accepted: bool,
    spell_slot_cost_kind: &'static str,
    preparation_requirement: &'static str,
    required_spell_access: &'static str,
    additional_casting_time_minutes: i16,
    requires_reading_spellbook: bool,
    first_level_spell_slots_expended: i16,
    last_result: &'static str,
) -> SpellbookRitualWitness {
    SpellbookRitualWitness {
        feature_unit_id,
        spell_id,
        spellcasting_source_unit_id,
        spellbook_contains_ritual,
        prepared_contains_ritual,
        invocation_accepted,
        spell_slot_cost_kind,
        preparation_requirement,
        required_spell_access,
        additional_casting_time_minutes,
        requires_reading_spellbook,
        first_level_spell_slots_expended,
        last_result,
    }
}

fn accepted_witness(
    last_result: &'static str,
    spellbook_contains_ritual: bool,
    prepared_contains_ritual: bool,
    projection: SpellbookRitualResult,
) -> SpellbookRitualWitness {
    match projection {
        SpellbookRitualResult::Accepted(invocation) => SpellbookRitualWitness {
            feature_unit_id: "wizard_ritual_adept",
            spell_id: "detect_magic",
            spellcasting_source_unit_id: "class_wizard",
            spellbook_contains_ritual,
            prepared_contains_ritual,
            invocation_accepted: true,
            spell_slot_cost_kind: spell_slot_cost_kind(invocation.spell_slot_cost),
            preparation_requirement: preparation_requirement(invocation.preparation_requirement),
            required_spell_access: required_access(invocation.required_access),
            additional_casting_time_minutes: invocation.additional_casting_time_minutes,
            requires_reading_spellbook: invocation.requires_reading_spellbook,
            first_level_spell_slots_expended: invocation.first_level_spell_slots_expended,
            last_result,
        },
        SpellbookRitualResult::Rejected => panic!("accepted witness facts must be accepted"),
    }
}

fn rejected_witness(
    feature_unit_id: &'static str,
    spell_id: &'static str,
    spellcasting_source_unit_id: &'static str,
    spellbook_contains_ritual: bool,
    prepared_contains_ritual: bool,
    last_result: &'static str,
    projection: SpellbookRitualResult,
) -> SpellbookRitualWitness {
    assert_eq!(projection, SpellbookRitualResult::Rejected);

    SpellbookRitualWitness {
        feature_unit_id,
        spell_id,
        spellcasting_source_unit_id,
        spellbook_contains_ritual,
        prepared_contains_ritual,
        invocation_accepted: false,
        spell_slot_cost_kind: "none",
        preparation_requirement: "none",
        required_spell_access: "none",
        additional_casting_time_minutes: 0,
        requires_reading_spellbook: false,
        first_level_spell_slots_expended: 0,
        last_result,
    }
}

fn spell_slot_cost_kind(spell_slot_cost: SpellSlotCost) -> &'static str {
    match spell_slot_cost {
        SpellSlotCost::None => "none",
    }
}

fn preparation_requirement(preparation_requirement: PreparationRequirement) -> &'static str {
    match preparation_requirement {
        PreparationRequirement::NotRequired => "not_required",
    }
}

fn required_access(required_access: RequiredSpellAccess) -> &'static str {
    match required_access {
        RequiredSpellAccess::Spellbook => "spellbook",
    }
}
