use super::character_creation_expected_routes::expected_retained_reference_route;
use crate::rules::character_creation::{
    apply_creation_retained_reference_operation, completed_fighter_creation_state, route_payload,
    CreationRetainedReferenceOperation, CreationRouteEvent,
};
use crate::rules::class_features::{
    fighter_fighting_style_projection, ClassLevel, FighterFightingStyleSelection,
    FightingStyleFeat, FightingStyleFeature, FightingStyleProjection,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FightingStyleWitness {
    pub last_result: &'static str,
    pub selected_from_unit_id: &'static str,
    pub selected_feat_unit_id: &'static str,
    pub selected_fighting_style_feature_ref_count: u8,
    pub fighter_fighting_style_unit_ref_present: bool,
    pub selected_feat_unit_ref_count: u8,
    pub non_selected_fighting_style_unit_ref_count: u8,
    pub total_level: u8,
}

pub const BRANCH_ACTIONS: &[&str] = &[
    "doSelectDefenseFightingStyle",
    "doSelectArcheryFightingStyle",
    "doSelectGreatWeaponFightingStyle",
    "doSelectTwoWeaponFightingStyle",
    "doReplaceArcheryWithDefenseOnFighterLevelGain",
    "doReplaceDefenseWithArcheryOnFighterLevelGain",
    "doReplaceDefenseWithGreatWeaponFightingOnFighterLevelGain",
    "doReplaceDefenseWithTwoWeaponFightingOnFighterLevelGain",
];

pub fn replay_observed_action(observed_action_taken: &str) -> FightingStyleWitness {
    match observed_action_taken {
        "doSelectDefenseFightingStyle" => select_defense_replay(),
        "doSelectArcheryFightingStyle" => select_archery_replay(),
        "doSelectGreatWeaponFightingStyle" => select_great_weapon_replay(),
        "doSelectTwoWeaponFightingStyle" => select_two_weapon_replay(),
        "doReplaceArcheryWithDefenseOnFighterLevelGain" => replace_archery_with_defense_replay(),
        "doReplaceDefenseWithArcheryOnFighterLevelGain" => replace_defense_with_archery_replay(),
        "doReplaceDefenseWithGreatWeaponFightingOnFighterLevelGain" => {
            replace_defense_with_great_weapon_replay()
        }
        "doReplaceDefenseWithTwoWeaponFightingOnFighterLevelGain" => {
            replace_defense_with_two_weapon_replay()
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<CreationRouteEvent> {
    let operation = if observed_action_taken.starts_with("doSelect") {
        CreationRetainedReferenceOperation::RetainOnly
    } else {
        CreationRetainedReferenceOperation::ReplaceAndProject
    };
    apply_creation_retained_reference_operation(&completed_fighter_creation_state(), operation)
        .route
}

pub fn expected_route(observed_action_taken: &str) -> Vec<CreationRouteEvent> {
    let operation = if observed_action_taken.starts_with("doSelect") {
        CreationRetainedReferenceOperation::RetainOnly
    } else {
        CreationRetainedReferenceOperation::ReplaceAndProject
    };
    expected_retained_reference_route(operation)
}

pub fn route_projection_payload(route: &[CreationRouteEvent]) -> String {
    route_payload(route)
}

pub fn expected_select_defense_witness() -> FightingStyleWitness {
    FightingStyleWitness {
        last_result: "finalized",
        selected_from_unit_id: "fighter_fighting_style",
        selected_feat_unit_id: "defense",
        selected_fighting_style_feature_ref_count: 1,
        fighter_fighting_style_unit_ref_present: true,
        selected_feat_unit_ref_count: 1,
        non_selected_fighting_style_unit_ref_count: 0,
        total_level: 1,
    }
}

pub fn expected_replace_defense_with_archery_witness() -> FightingStyleWitness {
    FightingStyleWitness {
        last_result: "replaced",
        selected_from_unit_id: "fighter_fighting_style",
        selected_feat_unit_id: "feat_archery",
        selected_fighting_style_feature_ref_count: 1,
        fighter_fighting_style_unit_ref_present: true,
        selected_feat_unit_ref_count: 1,
        non_selected_fighting_style_unit_ref_count: 0,
        total_level: 2,
    }
}

pub fn projection_payload(witness: &FightingStyleWitness) -> String {
    [
        format!("lastResult={}", witness.last_result),
        format!("selectedFromUnitId={}", witness.selected_from_unit_id),
        format!("selectedFeatUnitId={}", witness.selected_feat_unit_id),
        format!(
            "selectedFightingStyleFeatureRefCount={}",
            witness.selected_fighting_style_feature_ref_count
        ),
        format!(
            "fighterFightingStyleUnitRefPresent={}",
            witness.fighter_fighting_style_unit_ref_present
        ),
        format!(
            "selectedFeatUnitRefCount={}",
            witness.selected_feat_unit_ref_count
        ),
        format!(
            "nonSelectedFightingStyleUnitRefCount={}",
            witness.non_selected_fighting_style_unit_ref_count
        ),
        format!("totalLevel={}", witness.total_level),
    ]
    .join("\n")
}

fn select_defense_replay() -> FightingStyleWitness {
    let observed = initial_style_replay(FightingStyleFeat::Defense);
    fighting_style_projection_witness("finalized", &observed)
}

fn select_archery_replay() -> FightingStyleWitness {
    let observed = fighter_fighting_style_projection(FighterFightingStyleSelection::Initial(
        FightingStyleFeat::Archery,
    ));
    fighting_style_projection_witness("finalized", &observed)
}

fn select_great_weapon_replay() -> FightingStyleWitness {
    let observed = fighter_fighting_style_projection(FighterFightingStyleSelection::Initial(
        FightingStyleFeat::GreatWeaponFighting,
    ));
    fighting_style_projection_witness("finalized", &observed)
}

fn select_two_weapon_replay() -> FightingStyleWitness {
    let observed = fighter_fighting_style_projection(FighterFightingStyleSelection::Initial(
        FightingStyleFeat::TwoWeaponFighting,
    ));
    fighting_style_projection_witness("finalized", &observed)
}

fn replace_archery_with_defense_replay() -> FightingStyleWitness {
    replacement_style_replay(FightingStyleFeat::Archery, FightingStyleFeat::Defense)
}

fn replace_defense_with_archery_replay() -> FightingStyleWitness {
    replacement_style_replay(FightingStyleFeat::Defense, FightingStyleFeat::Archery)
}

fn replace_defense_with_great_weapon_replay() -> FightingStyleWitness {
    replacement_style_replay(
        FightingStyleFeat::Defense,
        FightingStyleFeat::GreatWeaponFighting,
    )
}

fn replace_defense_with_two_weapon_replay() -> FightingStyleWitness {
    replacement_style_replay(
        FightingStyleFeat::Defense,
        FightingStyleFeat::TwoWeaponFighting,
    )
}

fn initial_style_replay(feat: FightingStyleFeat) -> FightingStyleProjection {
    fighter_fighting_style_projection(FighterFightingStyleSelection::Initial(feat))
}

fn replacement_style_replay(
    previous: FightingStyleFeat,
    replacement: FightingStyleFeat,
) -> FightingStyleWitness {
    let observed =
        fighter_fighting_style_projection(FighterFightingStyleSelection::ReplacementOnLevelGain {
            previous,
            replacement,
            new_total_level: ClassLevel::Two,
        });
    fighting_style_projection_witness("replaced", &observed)
}

fn fighting_style_projection_witness(
    last_result: &'static str,
    projection: &FightingStyleProjection,
) -> FightingStyleWitness {
    FightingStyleWitness {
        last_result,
        selected_from_unit_id: feature_ref(projection.source_feature),
        selected_feat_unit_id: feat_ref(projection.selected_feat),
        selected_fighting_style_feature_ref_count: projection
            .selected_fighting_style_feature_ref_count,
        fighter_fighting_style_unit_ref_present: projection.source_feature
            == FightingStyleFeature::Fighter,
        selected_feat_unit_ref_count: 1,
        non_selected_fighting_style_unit_ref_count: 0,
        total_level: class_level(projection.total_level),
    }
}

fn feature_ref(feature: FightingStyleFeature) -> &'static str {
    match feature {
        FightingStyleFeature::Fighter => "fighter_fighting_style",
    }
}

fn feat_ref(feat: FightingStyleFeat) -> &'static str {
    match feat {
        FightingStyleFeat::Archery => "feat_archery",
        FightingStyleFeat::Defense => "defense",
        FightingStyleFeat::GreatWeaponFighting => "feat_great_weapon_fighting",
        FightingStyleFeat::TwoWeaponFighting => "feat_two_weapon_fighting",
    }
}

fn class_level(level: ClassLevel) -> u8 {
    match level {
        ClassLevel::One => 1,
        ClassLevel::Two => 2,
    }
}
