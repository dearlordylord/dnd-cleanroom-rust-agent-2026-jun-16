use crate::rules::class_features::{
    weapon_mastery_projection, ClassLevel, ClassUnit, Weapon, WeaponMasteryClass,
    WeaponMasteryFeature, WeaponMasteryProjection,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeaponMasteryWitness {
    pub last_result: &'static str,
    pub feature_unit_id: &'static str,
    pub class_unit_id: &'static str,
    pub first_weapon_unit_id: &'static str,
    pub second_weapon_unit_id: &'static str,
    pub third_weapon_unit_id: &'static str,
    pub selected_mastery_choice_count: u8,
    pub build_mastery_feature_count: u8,
    pub open_hole_count: u8,
    pub feature_unit_ref_present: bool,
    pub first_weapon_unit_ref_present: bool,
    pub second_weapon_unit_ref_present: bool,
    pub third_weapon_unit_ref_present: bool,
    pub total_level: u8,
}

pub fn replay_observed_action(observed_action_taken: &str) -> WeaponMasteryWitness {
    match observed_action_taken {
        "doFinalizeBarbarianWeaponMastery" => barbarian_replay(),
        "doFinalizeFighterWeaponMastery" => fighter_replay(),
        "doFinalizePaladinWeaponMastery" => paladin_replay(),
        "doFinalizeRangerWeaponMastery" => ranger_replay(),
        "doFinalizeRogueWeaponMastery" => rogue_replay(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_barbarian_witness() -> WeaponMasteryWitness {
    WeaponMasteryWitness {
        last_result: "barbarianFinalized",
        feature_unit_id: "barbarian_weapon_mastery",
        class_unit_id: "class_barbarian",
        first_weapon_unit_id: "weapon_longsword",
        second_weapon_unit_id: "weapon_dagger",
        third_weapon_unit_id: "none",
        selected_mastery_choice_count: 2,
        build_mastery_feature_count: 2,
        open_hole_count: 0,
        feature_unit_ref_present: true,
        first_weapon_unit_ref_present: true,
        second_weapon_unit_ref_present: true,
        third_weapon_unit_ref_present: false,
        total_level: 1,
    }
}

pub fn expected_fighter_witness() -> WeaponMasteryWitness {
    WeaponMasteryWitness {
        last_result: "fighterFinalized",
        feature_unit_id: "fighter_weapon_mastery",
        class_unit_id: "class_fighter",
        first_weapon_unit_id: "weapon_longsword",
        second_weapon_unit_id: "weapon_spear",
        third_weapon_unit_id: "weapon_flail",
        selected_mastery_choice_count: 3,
        build_mastery_feature_count: 3,
        open_hole_count: 0,
        feature_unit_ref_present: true,
        first_weapon_unit_ref_present: true,
        second_weapon_unit_ref_present: true,
        third_weapon_unit_ref_present: true,
        total_level: 1,
    }
}

pub fn expected_paladin_witness() -> WeaponMasteryWitness {
    WeaponMasteryWitness {
        last_result: "paladinFinalized",
        feature_unit_id: "paladin_weapon_mastery",
        class_unit_id: "class_paladin",
        first_weapon_unit_id: "weapon_longsword",
        second_weapon_unit_id: "weapon_dagger",
        third_weapon_unit_id: "none",
        selected_mastery_choice_count: 2,
        build_mastery_feature_count: 2,
        open_hole_count: 0,
        feature_unit_ref_present: true,
        first_weapon_unit_ref_present: true,
        second_weapon_unit_ref_present: true,
        third_weapon_unit_ref_present: false,
        total_level: 1,
    }
}

pub fn expected_ranger_witness() -> WeaponMasteryWitness {
    WeaponMasteryWitness {
        last_result: "rangerFinalized",
        feature_unit_id: "ranger_weapon_mastery",
        class_unit_id: "class_ranger",
        first_weapon_unit_id: "weapon_longsword",
        second_weapon_unit_id: "weapon_dagger",
        third_weapon_unit_id: "none",
        selected_mastery_choice_count: 2,
        build_mastery_feature_count: 2,
        open_hole_count: 0,
        feature_unit_ref_present: true,
        first_weapon_unit_ref_present: true,
        second_weapon_unit_ref_present: true,
        third_weapon_unit_ref_present: false,
        total_level: 1,
    }
}

pub fn expected_rogue_witness() -> WeaponMasteryWitness {
    WeaponMasteryWitness {
        last_result: "rogueFinalized",
        feature_unit_id: "rogue_weapon_mastery",
        class_unit_id: "class_rogue",
        first_weapon_unit_id: "weapon_dagger",
        second_weapon_unit_id: "weapon_shortsword",
        third_weapon_unit_id: "none",
        selected_mastery_choice_count: 2,
        build_mastery_feature_count: 2,
        open_hole_count: 0,
        feature_unit_ref_present: true,
        first_weapon_unit_ref_present: true,
        second_weapon_unit_ref_present: true,
        third_weapon_unit_ref_present: false,
        total_level: 1,
    }
}

pub fn projection_payload(witness: &WeaponMasteryWitness) -> String {
    [
        format!("lastResult={}", witness.last_result),
        format!("featureUnitId={}", witness.feature_unit_id),
        format!("classUnitId={}", witness.class_unit_id),
        format!("firstWeaponUnitId={}", witness.first_weapon_unit_id),
        format!("secondWeaponUnitId={}", witness.second_weapon_unit_id),
        format!("thirdWeaponUnitId={}", witness.third_weapon_unit_id),
        format!(
            "selectedMasteryChoiceCount={}",
            witness.selected_mastery_choice_count
        ),
        format!(
            "buildMasteryFeatureCount={}",
            witness.build_mastery_feature_count
        ),
        format!("openHoleCount={}", witness.open_hole_count),
        format!("featureUnitRefPresent={}", witness.feature_unit_ref_present),
        format!(
            "firstWeaponUnitRefPresent={}",
            witness.first_weapon_unit_ref_present
        ),
        format!(
            "secondWeaponUnitRefPresent={}",
            witness.second_weapon_unit_ref_present
        ),
        format!(
            "thirdWeaponUnitRefPresent={}",
            witness.third_weapon_unit_ref_present
        ),
        format!("totalLevel={}", witness.total_level),
    ]
    .join("\n")
}

fn barbarian_replay() -> WeaponMasteryWitness {
    let observed = weapon_mastery_projection(
        WeaponMasteryClass::Barbarian,
        &[Weapon::Longsword, Weapon::Dagger],
    )
    .expect("barbarian selects two weapon mastery choices");
    weapon_mastery_witness("barbarianFinalized", &observed)
}

fn fighter_replay() -> WeaponMasteryWitness {
    let observed = weapon_mastery_projection(
        WeaponMasteryClass::Fighter,
        &[Weapon::Longsword, Weapon::Spear, Weapon::Flail],
    )
    .expect("fighter selects three weapon mastery choices");
    weapon_mastery_witness("fighterFinalized", &observed)
}

fn paladin_replay() -> WeaponMasteryWitness {
    let observed = weapon_mastery_projection(
        WeaponMasteryClass::Paladin,
        &[Weapon::Longsword, Weapon::Dagger],
    )
    .expect("paladin selects two weapon mastery choices");
    weapon_mastery_witness("paladinFinalized", &observed)
}

fn ranger_replay() -> WeaponMasteryWitness {
    let observed = weapon_mastery_projection(
        WeaponMasteryClass::Ranger,
        &[Weapon::Longsword, Weapon::Dagger],
    )
    .expect("ranger selects two weapon mastery choices");
    weapon_mastery_witness("rangerFinalized", &observed)
}

fn rogue_replay() -> WeaponMasteryWitness {
    let observed = weapon_mastery_projection(
        WeaponMasteryClass::Rogue,
        &[Weapon::Dagger, Weapon::Shortsword],
    )
    .expect("rogue selects two weapon mastery choices");
    weapon_mastery_witness("rogueFinalized", &observed)
}

fn weapon_mastery_witness(
    last_result: &'static str,
    projection: &WeaponMasteryProjection,
) -> WeaponMasteryWitness {
    let first = projection.selected_weapons.first().copied();
    let second = projection.selected_weapons.get(1).copied();
    let third = projection.selected_weapons.get(2).copied();

    WeaponMasteryWitness {
        last_result,
        feature_unit_id: feature_ref(projection.feature),
        class_unit_id: class_ref(projection.class_unit),
        first_weapon_unit_id: first.map(weapon_ref).unwrap_or("none"),
        second_weapon_unit_id: second.map(weapon_ref).unwrap_or("none"),
        third_weapon_unit_id: third.map(weapon_ref).unwrap_or("none"),
        selected_mastery_choice_count: projection.selected_mastery_choice_count,
        build_mastery_feature_count: projection.build_mastery_feature_count,
        open_hole_count: projection.open_hole_count,
        feature_unit_ref_present: true,
        first_weapon_unit_ref_present: first.is_some(),
        second_weapon_unit_ref_present: second.is_some(),
        third_weapon_unit_ref_present: third.is_some(),
        total_level: class_level(projection.total_level),
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

fn class_level(level: ClassLevel) -> u8 {
    match level {
        ClassLevel::One => 1,
        ClassLevel::Two => 2,
    }
}
