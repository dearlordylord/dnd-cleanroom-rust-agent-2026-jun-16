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
    pub defense_unit_ref_present: bool,
    pub archery_unit_ref_present: bool,
    pub total_level: u8,
}

pub fn replay_observed_action(observed_action_taken: &str) -> FightingStyleWitness {
    match observed_action_taken {
        "doSelectDefenseFightingStyle" => select_defense_replay(),
        "doReplaceDefenseWithArcheryOnFighterLevelGain" => replace_defense_with_archery_replay(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_select_defense_witness() -> FightingStyleWitness {
    FightingStyleWitness {
        last_result: "finalized",
        selected_from_unit_id: "fighter_fighting_style",
        selected_feat_unit_id: "defense",
        selected_fighting_style_feature_ref_count: 1,
        fighter_fighting_style_unit_ref_present: true,
        defense_unit_ref_present: true,
        archery_unit_ref_present: false,
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
        defense_unit_ref_present: false,
        archery_unit_ref_present: true,
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
        format!("defenseUnitRefPresent={}", witness.defense_unit_ref_present),
        format!("archeryUnitRefPresent={}", witness.archery_unit_ref_present),
        format!("totalLevel={}", witness.total_level),
    ]
    .join("\n")
}

fn select_defense_replay() -> FightingStyleWitness {
    let observed = fighter_fighting_style_projection(FighterFightingStyleSelection::Initial(
        FightingStyleFeat::Defense,
    ));
    fighting_style_projection_witness("finalized", &observed)
}

fn replace_defense_with_archery_replay() -> FightingStyleWitness {
    let observed =
        fighter_fighting_style_projection(FighterFightingStyleSelection::ReplacementOnLevelGain {
            previous: FightingStyleFeat::Defense,
            replacement: FightingStyleFeat::Archery,
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
        defense_unit_ref_present: projection.selected_feat == FightingStyleFeat::Defense,
        archery_unit_ref_present: projection.selected_feat == FightingStyleFeat::Archery,
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
    }
}

fn class_level(level: ClassLevel) -> u8 {
    match level {
        ClassLevel::One => 1,
        ClassLevel::Two => 2,
    }
}
