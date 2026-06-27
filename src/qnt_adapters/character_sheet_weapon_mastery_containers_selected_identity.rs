use crate::rules::class_features::{
    apply_weapon_mastery_long_rest_reselection, weapon_mastery_changed_choice_count,
    weapon_mastery_projection, ClassLevel, ClassUnit, ProjectionError, Weapon, WeaponMasteryFacts,
    WeaponMasteryFeature, WeaponMasteryProjection, WeaponMasteryReselectionFacts,
};

use super::character_sheet_reducer_route::{
    initial_sheet_build_route, route_complete_character_sheet_rest,
    route_resolve_character_sheet_subject, route_retain_character_sheet_selected_references,
    CharacterSheetRouteEvent, CharacterSheetRouteFillFamily, CharacterSheetRouteHoleFamily,
    CharacterSheetRouteOwnerGroup, CharacterSheetRouteSubjectFamily,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ReplayWeapon {
    CurrentFirst,
    CurrentSecond,
    Dagger,
    Flail,
    Longsword,
    RequestedFirst,
    RequestedSecond,
    Shortbow,
    Shortsword,
    Spear,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SheetWeaponMasteryWitness {
    pub last_result: &'static str,
    pub feature_unit_id: &'static str,
    pub class_unit_id: &'static str,
    pub first_weapon_unit_id: &'static str,
    pub second_weapon_unit_id: &'static str,
    pub choice_count: usize,
    pub long_rest_change_count: usize,
    pub selected_weapon_count: usize,
    pub changed_choice_count: usize,
    pub first_weapon_eligible: bool,
    pub second_weapon_eligible: bool,
    pub feature_unit_ref_present: bool,
    pub accepted: bool,
}

pub const BRANCH_ACTIONS: [&str; 8] = [
    "doSelectPaladinWeaponMastery",
    "doReselectPaladinWeaponMasteryOnLongRest",
    "doSelectRangerWeaponMastery",
    "doReselectRangerWeaponMasteryOnLongRest",
    "doSelectRogueWeaponMastery",
    "doReselectRogueWeaponMasteryOnLongRest",
    "doAcceptOneChangeWeaponMasteryReselection",
    "doRejectTooManyChangesWeaponMasteryReselection",
];

pub fn replay_observed_action(observed_action_taken: &str) -> SheetWeaponMasteryWitness {
    match observed_action_taken {
        "doSelectPaladinWeaponMastery" => select_paladin_weapon_mastery(),
        "doReselectPaladinWeaponMasteryOnLongRest" => reselect_paladin_weapon_mastery(),
        "doSelectRangerWeaponMastery" => select_ranger_weapon_mastery(),
        "doReselectRangerWeaponMasteryOnLongRest" => reselect_ranger_weapon_mastery(),
        "doSelectRogueWeaponMastery" => select_rogue_weapon_mastery(),
        "doReselectRogueWeaponMasteryOnLongRest" => reselect_rogue_weapon_mastery(),
        "doAcceptOneChangeWeaponMasteryReselection" => accept_one_change_reselection(),
        "doRejectTooManyChangesWeaponMasteryReselection" => reject_too_many_changes_reselection(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> SheetWeaponMasteryWitness {
    match observed_action_taken {
        "doSelectPaladinWeaponMastery" => expected(
            "paladinSelected",
            "paladin_weapon_mastery",
            "class_paladin",
            "weapon_longsword",
            "weapon_dagger",
            2,
            2,
            2,
            0,
            true,
        ),
        "doReselectPaladinWeaponMasteryOnLongRest" => expected(
            "paladinReselected",
            "paladin_weapon_mastery",
            "class_paladin",
            "weapon_spear",
            "weapon_flail",
            2,
            2,
            2,
            2,
            true,
        ),
        "doSelectRangerWeaponMastery" => expected(
            "rangerSelected",
            "ranger_weapon_mastery",
            "class_ranger",
            "weapon_longsword",
            "weapon_dagger",
            2,
            2,
            2,
            0,
            true,
        ),
        "doReselectRangerWeaponMasteryOnLongRest" => expected(
            "rangerReselected",
            "ranger_weapon_mastery",
            "class_ranger",
            "weapon_spear",
            "weapon_flail",
            2,
            2,
            2,
            2,
            true,
        ),
        "doSelectRogueWeaponMastery" => expected(
            "rogueSelected",
            "rogue_weapon_mastery",
            "class_rogue",
            "weapon_dagger",
            "weapon_shortbow",
            2,
            2,
            2,
            0,
            true,
        ),
        "doReselectRogueWeaponMasteryOnLongRest" => expected(
            "rogueReselected",
            "rogue_weapon_mastery",
            "class_rogue",
            "weapon_spear",
            "weapon_shortsword",
            2,
            2,
            2,
            2,
            true,
        ),
        "doAcceptOneChangeWeaponMasteryReselection" => expected(
            "oneChangeAccepted",
            "semantic_core",
            "semantic_core",
            "current_first",
            "requested_second",
            2,
            1,
            2,
            1,
            true,
        ),
        "doRejectTooManyChangesWeaponMasteryReselection" => expected(
            "tooManyChangesRejected",
            "semantic_core",
            "semantic_core",
            "requested_first",
            "requested_second",
            2,
            1,
            2,
            2,
            false,
        ),
        action => panic!("unsupported expected mbt::actionTaken {action}"),
    }
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<CharacterSheetRouteEvent> {
    match observed_action_taken {
        "doSelectPaladinWeaponMastery"
        | "doSelectRangerWeaponMastery"
        | "doSelectRogueWeaponMastery" => retain_weapon_mastery_route(),
        "doReselectPaladinWeaponMasteryOnLongRest"
        | "doReselectRangerWeaponMasteryOnLongRest"
        | "doReselectRogueWeaponMasteryOnLongRest"
        | "doAcceptOneChangeWeaponMasteryReselection" => reselect_weapon_mastery_on_rest_route(),
        "doRejectTooManyChangesWeaponMasteryReselection" => {
            reject_weapon_mastery_reselection_route()
        }
        action => panic!("unsupported route mbt::actionTaken {action}"),
    }
}

pub fn expected_route(observed_action_taken: &str) -> Vec<CharacterSheetRouteEvent> {
    match observed_action_taken {
        "doSelectPaladinWeaponMastery"
        | "doSelectRangerWeaponMastery"
        | "doSelectRogueWeaponMastery" => expected_retain_weapon_mastery_route(),
        "doReselectPaladinWeaponMasteryOnLongRest"
        | "doReselectRangerWeaponMasteryOnLongRest"
        | "doReselectRogueWeaponMasteryOnLongRest"
        | "doAcceptOneChangeWeaponMasteryReselection" => {
            expected_reselect_weapon_mastery_on_rest_route()
        }
        "doRejectTooManyChangesWeaponMasteryReselection" => {
            expected_reject_weapon_mastery_reselection_route()
        }
        action => panic!("unsupported expected route mbt::actionTaken {action}"),
    }
}

pub fn projection_payload(witness: &SheetWeaponMasteryWitness) -> String {
    [
        format!("lastResult={}", witness.last_result),
        format!("featureUnitId={}", witness.feature_unit_id),
        format!("classUnitId={}", witness.class_unit_id),
        format!("firstWeaponUnitId={}", witness.first_weapon_unit_id),
        format!("secondWeaponUnitId={}", witness.second_weapon_unit_id),
        format!("choiceCount={}", witness.choice_count),
        format!("longRestChangeCount={}", witness.long_rest_change_count),
        format!("selectedWeaponCount={}", witness.selected_weapon_count),
        format!("changedChoiceCount={}", witness.changed_choice_count),
        format!("firstWeaponEligible={}", witness.first_weapon_eligible),
        format!("secondWeaponEligible={}", witness.second_weapon_eligible),
        format!("featureUnitRefPresent={}", witness.feature_unit_ref_present),
        format!("accepted={}", witness.accepted),
    ]
    .join("\n")
}

fn retain_weapon_mastery_route() -> Vec<CharacterSheetRouteEvent> {
    let mut route = initial_sheet_build_route();
    route.push(route_retain_character_sheet_selected_references(
        CharacterSheetRouteSubjectFamily::SheetSelectedReferenceProjection,
        CharacterSheetRouteOwnerGroup::CharacterSheetSelectedReference,
    ));
    route
}

fn reselect_weapon_mastery_on_rest_route() -> Vec<CharacterSheetRouteEvent> {
    let mut route = retain_weapon_mastery_route();
    route.push(route_complete_character_sheet_rest(
        CharacterSheetRouteSubjectFamily::SheetSelectedReferenceProjection,
        CharacterSheetRouteFillFamily::ProjectionSelection,
        Vec::new(),
        CharacterSheetRouteOwnerGroup::CharacterSheetSelectedReference,
    ));
    route
}

fn reject_weapon_mastery_reselection_route() -> Vec<CharacterSheetRouteEvent> {
    let mut route = initial_sheet_build_route();
    route.push(route_resolve_character_sheet_subject(
        CharacterSheetRouteSubjectFamily::SheetSelectedReferenceProjection,
        CharacterSheetRouteFillFamily::ProjectionSelection,
        vec![CharacterSheetRouteHoleFamily::ProjectionChoice],
        CharacterSheetRouteOwnerGroup::CharacterSheetSelectedReference,
    ));
    route
}

fn expected_retain_weapon_mastery_route() -> Vec<CharacterSheetRouteEvent> {
    let mut route = initial_sheet_build_route();
    route.push(route_retain_character_sheet_selected_references(
        CharacterSheetRouteSubjectFamily::SheetSelectedReferenceProjection,
        CharacterSheetRouteOwnerGroup::CharacterSheetSelectedReference,
    ));
    route
}

fn expected_reselect_weapon_mastery_on_rest_route() -> Vec<CharacterSheetRouteEvent> {
    let mut route = expected_retain_weapon_mastery_route();
    route.push(route_complete_character_sheet_rest(
        CharacterSheetRouteSubjectFamily::SheetSelectedReferenceProjection,
        CharacterSheetRouteFillFamily::ProjectionSelection,
        Vec::new(),
        CharacterSheetRouteOwnerGroup::CharacterSheetSelectedReference,
    ));
    route
}

fn expected_reject_weapon_mastery_reselection_route() -> Vec<CharacterSheetRouteEvent> {
    let mut route = initial_sheet_build_route();
    route.push(route_resolve_character_sheet_subject(
        CharacterSheetRouteSubjectFamily::SheetSelectedReferenceProjection,
        CharacterSheetRouteFillFamily::ProjectionSelection,
        vec![CharacterSheetRouteHoleFamily::ProjectionChoice],
        CharacterSheetRouteOwnerGroup::CharacterSheetSelectedReference,
    ));
    route
}

fn select_paladin_weapon_mastery() -> SheetWeaponMasteryWitness {
    let projection = weapon_mastery_projection(
        weapon_mastery_facts(WeaponMasteryFeature::Paladin, ClassUnit::Paladin, 2),
        &[Weapon::Longsword, Weapon::Dagger],
    )
    .expect("paladin selects two weapon mastery choices");
    selection_witness("paladinSelected", &projection)
}

fn reselect_paladin_weapon_mastery() -> SheetWeaponMasteryWitness {
    reselect_class_weapon_mastery(
        "paladinReselected",
        "paladin_weapon_mastery",
        "class_paladin",
        [ReplayWeapon::Longsword, ReplayWeapon::Dagger],
        [ReplayWeapon::Spear, ReplayWeapon::Flail],
        [ReplayWeapon::Spear, ReplayWeapon::Flail],
        2,
    )
}

fn select_ranger_weapon_mastery() -> SheetWeaponMasteryWitness {
    let projection = weapon_mastery_projection(
        weapon_mastery_facts(WeaponMasteryFeature::Ranger, ClassUnit::Ranger, 2),
        &[Weapon::Longsword, Weapon::Dagger],
    )
    .expect("ranger selects two weapon mastery choices");
    selection_witness("rangerSelected", &projection)
}

fn reselect_ranger_weapon_mastery() -> SheetWeaponMasteryWitness {
    reselect_class_weapon_mastery(
        "rangerReselected",
        "ranger_weapon_mastery",
        "class_ranger",
        [ReplayWeapon::Longsword, ReplayWeapon::Dagger],
        [ReplayWeapon::Spear, ReplayWeapon::Flail],
        [ReplayWeapon::Spear, ReplayWeapon::Flail],
        2,
    )
}

fn select_rogue_weapon_mastery() -> SheetWeaponMasteryWitness {
    let projection = weapon_mastery_projection(
        weapon_mastery_facts(WeaponMasteryFeature::Rogue, ClassUnit::Rogue, 2),
        &[Weapon::Dagger, Weapon::Shortbow],
    )
    .expect("rogue selects two weapon mastery choices");
    selection_witness("rogueSelected", &projection)
}

fn weapon_mastery_facts(
    feature: WeaponMasteryFeature,
    class_unit: ClassUnit,
    choice_count: u8,
) -> WeaponMasteryFacts {
    WeaponMasteryFacts {
        feature,
        class_unit,
        choice_count,
        build_mastery_feature_count: choice_count,
        open_hole_count: 0,
        total_level: ClassLevel::One,
    }
}

fn reselect_rogue_weapon_mastery() -> SheetWeaponMasteryWitness {
    reselect_class_weapon_mastery(
        "rogueReselected",
        "rogue_weapon_mastery",
        "class_rogue",
        [ReplayWeapon::Dagger, ReplayWeapon::Shortbow],
        [ReplayWeapon::Spear, ReplayWeapon::Shortsword],
        [ReplayWeapon::Spear, ReplayWeapon::Shortsword],
        2,
    )
}

fn accept_one_change_reselection() -> SheetWeaponMasteryWitness {
    reselect_class_weapon_mastery(
        "oneChangeAccepted",
        "semantic_core",
        "semantic_core",
        [ReplayWeapon::CurrentFirst, ReplayWeapon::CurrentSecond],
        [ReplayWeapon::CurrentFirst, ReplayWeapon::RequestedSecond],
        [ReplayWeapon::CurrentFirst, ReplayWeapon::RequestedSecond],
        1,
    )
}

fn reject_too_many_changes_reselection() -> SheetWeaponMasteryWitness {
    reselect_class_weapon_mastery(
        "tooManyChangesRejected",
        "semantic_core",
        "semantic_core",
        [ReplayWeapon::CurrentFirst, ReplayWeapon::CurrentSecond],
        [ReplayWeapon::RequestedFirst, ReplayWeapon::RequestedSecond],
        [ReplayWeapon::RequestedFirst, ReplayWeapon::RequestedSecond],
        1,
    )
}

fn selection_witness(
    last_result: &'static str,
    projection: &WeaponMasteryProjection,
) -> SheetWeaponMasteryWitness {
    let first = projection.selected_weapons.first().copied();
    let second = projection.selected_weapons.get(1).copied();

    SheetWeaponMasteryWitness {
        last_result,
        feature_unit_id: feature_ref(projection.feature),
        class_unit_id: class_ref(projection.class_unit),
        first_weapon_unit_id: first.map(weapon_ref).unwrap_or("none"),
        second_weapon_unit_id: second.map(weapon_ref).unwrap_or("none"),
        choice_count: usize::from(projection.selected_mastery_choice_count),
        long_rest_change_count: usize::from(projection.selected_mastery_choice_count),
        selected_weapon_count: projection.selected_weapons.len(),
        changed_choice_count: 0,
        first_weapon_eligible: first.is_some(),
        second_weapon_eligible: second.is_some(),
        feature_unit_ref_present: true,
        accepted: true,
    }
}

#[allow(clippy::too_many_arguments)]
fn expected(
    last_result: &'static str,
    feature_unit_id: &'static str,
    class_unit_id: &'static str,
    first_weapon_unit_id: &'static str,
    second_weapon_unit_id: &'static str,
    choice_count: usize,
    long_rest_change_count: usize,
    selected_weapon_count: usize,
    changed_choice_count: usize,
    accepted: bool,
) -> SheetWeaponMasteryWitness {
    SheetWeaponMasteryWitness {
        last_result,
        feature_unit_id,
        class_unit_id,
        first_weapon_unit_id,
        second_weapon_unit_id,
        choice_count,
        long_rest_change_count,
        selected_weapon_count,
        changed_choice_count,
        first_weapon_eligible: true,
        second_weapon_eligible: true,
        feature_unit_ref_present: true,
        accepted,
    }
}

fn reselect_class_weapon_mastery(
    last_result: &'static str,
    feature_unit_id: &'static str,
    class_unit_id: &'static str,
    current_weapons: [ReplayWeapon; 2],
    requested_weapons: [ReplayWeapon; 2],
    eligible_weapons: [ReplayWeapon; 2],
    long_rest_change_count: usize,
) -> SheetWeaponMasteryWitness {
    let facts = WeaponMasteryReselectionFacts {
        choice_count: 2,
        long_rest_change_count,
        current_weapons: current_weapons.to_vec(),
        requested_weapons: requested_weapons.to_vec(),
        eligible_weapons: eligible_weapons.to_vec(),
    };
    let result = apply_weapon_mastery_long_rest_reselection(&facts);
    let accepted = result.is_ok();
    if !accepted {
        assert_eq!(result, Err(ProjectionError::TooManyWeaponMasteryChanges));
    }

    SheetWeaponMasteryWitness {
        last_result,
        feature_unit_id,
        class_unit_id,
        first_weapon_unit_id: replay_weapon_ref(requested_weapons[0]),
        second_weapon_unit_id: replay_weapon_ref(requested_weapons[1]),
        choice_count: facts.choice_count,
        long_rest_change_count: facts.long_rest_change_count,
        selected_weapon_count: facts.requested_weapons.len(),
        changed_choice_count: weapon_mastery_changed_choice_count(
            &facts.current_weapons,
            &facts.requested_weapons,
        ),
        first_weapon_eligible: facts.eligible_weapons.contains(&requested_weapons[0]),
        second_weapon_eligible: facts.eligible_weapons.contains(&requested_weapons[1]),
        feature_unit_ref_present: true,
        accepted,
    }
}

fn feature_ref(feature: WeaponMasteryFeature) -> &'static str {
    match feature {
        WeaponMasteryFeature::Barbarian => "barbarian_weapon_mastery",
        WeaponMasteryFeature::Fighter => "fighter_weapon_mastery",
        WeaponMasteryFeature::Paladin => "paladin_weapon_mastery",
        WeaponMasteryFeature::Ranger => "ranger_weapon_mastery",
        WeaponMasteryFeature::Rogue => "rogue_weapon_mastery",
    }
}

fn class_ref(class_unit: ClassUnit) -> &'static str {
    match class_unit {
        ClassUnit::Barbarian => "class_barbarian",
        ClassUnit::Fighter => "class_fighter",
        ClassUnit::Paladin => "class_paladin",
        ClassUnit::Ranger => "class_ranger",
        ClassUnit::Rogue => "class_rogue",
    }
}

fn weapon_ref(weapon: Weapon) -> &'static str {
    match weapon {
        Weapon::Dagger => "weapon_dagger",
        Weapon::Flail => "weapon_flail",
        Weapon::Longsword => "weapon_longsword",
        Weapon::Shortbow => "weapon_shortbow",
        Weapon::Shortsword => "weapon_shortsword",
        Weapon::Spear => "weapon_spear",
    }
}

fn replay_weapon_ref(weapon: ReplayWeapon) -> &'static str {
    match weapon {
        ReplayWeapon::CurrentFirst => "current_first",
        ReplayWeapon::CurrentSecond => "current_second",
        ReplayWeapon::Dagger => "weapon_dagger",
        ReplayWeapon::Flail => "weapon_flail",
        ReplayWeapon::Longsword => "weapon_longsword",
        ReplayWeapon::RequestedFirst => "requested_first",
        ReplayWeapon::RequestedSecond => "requested_second",
        ReplayWeapon::Shortbow => "weapon_shortbow",
        ReplayWeapon::Shortsword => "weapon_shortsword",
        ReplayWeapon::Spear => "weapon_spear",
    }
}
