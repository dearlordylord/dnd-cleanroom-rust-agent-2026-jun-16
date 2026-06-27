use crate::rules::character_sheet::{
    project_sheet_class_feature, SheetClassFeatureFacts, SheetClassFeatureProjection,
};

use super::character_sheet_reducer_route::{
    initial_sheet_build_route, route_project_character_sheet_facts,
    route_retain_character_sheet_selected_references, CharacterSheetRouteEvent,
    CharacterSheetRouteOwnerGroup, CharacterSheetRouteSubjectFamily,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SheetClassFeatureWitness {
    pub last_result: &'static str,
    pub feature_unit_id: &'static str,
    pub spellcasting_source_unit_id: &'static str,
    pub expected_spell_present: bool,
    pub spell_count: i16,
    pub ability_check_bonus: i16,
    pub land: &'static str,
    pub accepted: bool,
}

pub const BRANCH_ACTIONS: [&str; 8] = [
    "doProjectBardJackOfAllTrades",
    "doProjectClericLifeDomainSpells",
    "doProjectDruidCircleLandSpells",
    "doProjectPaladinOathDevotionSpells",
    "doProjectPaladinsSmite",
    "doProjectRangerFavoredEnemy",
    "doProjectSorcererDraconicSpells",
    "doProjectWarlockFiendSpells",
];

pub fn replay_observed_action(observed_action_taken: &str) -> SheetClassFeatureWitness {
    match observed_action_taken {
        "doProjectBardJackOfAllTrades" => class_feature_witness(
            "bard-jack-of-all-trades",
            "bard_jack_of_all_trades",
            "none",
            "none",
            SheetClassFeatureFacts {
                expected_spell_present: false,
                spell_count: 0,
                ability_check_bonus: 1,
                spellcasting_source_present: false,
                land_choice_present: false,
            },
        ),
        "doProjectClericLifeDomainSpells" => class_feature_witness(
            "cleric-life-domain-spells",
            "cleric_life_domain_spells",
            "none",
            "none",
            spell_feature_facts(4, false, false),
        ),
        "doProjectDruidCircleLandSpells" => class_feature_witness(
            "druid-circle-land-spells",
            "druid_circle_of_the_land_spells",
            "class_druid",
            "temperate",
            spell_feature_facts(3, true, true),
        ),
        "doProjectPaladinOathDevotionSpells" => class_feature_witness(
            "paladin-oath-devotion-spells",
            "paladin_oath_of_devotion_spells",
            "none",
            "none",
            spell_feature_facts(2, false, false),
        ),
        "doProjectPaladinsSmite" => class_feature_witness(
            "paladins-smite",
            "paladin_paladins_smite",
            "none",
            "none",
            spell_feature_facts(1, false, false),
        ),
        "doProjectRangerFavoredEnemy" => class_feature_witness(
            "ranger-favored-enemy",
            "ranger_favored_enemy",
            "none",
            "none",
            spell_feature_facts(1, false, false),
        ),
        "doProjectSorcererDraconicSpells" => class_feature_witness(
            "sorcerer-draconic-spells",
            "sorcerer_draconic_spells",
            "none",
            "none",
            spell_feature_facts(4, false, false),
        ),
        "doProjectWarlockFiendSpells" => class_feature_witness(
            "warlock-fiend-spells",
            "warlock_fiend_spells",
            "none",
            "none",
            spell_feature_facts(4, false, false),
        ),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> SheetClassFeatureWitness {
    match observed_action_taken {
        "doProjectBardJackOfAllTrades" => expected(
            "bard-jack-of-all-trades",
            "bard_jack_of_all_trades",
            "none",
            false,
            0,
            1,
            "none",
        ),
        "doProjectClericLifeDomainSpells" => expected(
            "cleric-life-domain-spells",
            "cleric_life_domain_spells",
            "none",
            true,
            4,
            0,
            "none",
        ),
        "doProjectDruidCircleLandSpells" => expected(
            "druid-circle-land-spells",
            "druid_circle_of_the_land_spells",
            "class_druid",
            true,
            3,
            0,
            "temperate",
        ),
        "doProjectPaladinOathDevotionSpells" => expected(
            "paladin-oath-devotion-spells",
            "paladin_oath_of_devotion_spells",
            "none",
            true,
            2,
            0,
            "none",
        ),
        "doProjectPaladinsSmite" => expected(
            "paladins-smite",
            "paladin_paladins_smite",
            "none",
            true,
            1,
            0,
            "none",
        ),
        "doProjectRangerFavoredEnemy" => expected(
            "ranger-favored-enemy",
            "ranger_favored_enemy",
            "none",
            true,
            1,
            0,
            "none",
        ),
        "doProjectSorcererDraconicSpells" => expected(
            "sorcerer-draconic-spells",
            "sorcerer_draconic_spells",
            "none",
            true,
            4,
            0,
            "none",
        ),
        "doProjectWarlockFiendSpells" => expected(
            "warlock-fiend-spells",
            "warlock_fiend_spells",
            "none",
            true,
            4,
            0,
            "none",
        ),
        action => panic!("unsupported expected mbt::actionTaken {action}"),
    }
}

pub fn replay_observed_route(_observed_action_taken: &str) -> Vec<CharacterSheetRouteEvent> {
    let mut route = initial_sheet_build_route();
    route.push(route_retain_character_sheet_selected_references(
        CharacterSheetRouteSubjectFamily::SheetSelectedReferenceProjection,
        CharacterSheetRouteOwnerGroup::CharacterSheetSelectedReference,
    ));
    route.push(route_project_character_sheet_facts(
        CharacterSheetRouteSubjectFamily::SheetSelectedReferenceProjection,
        CharacterSheetRouteOwnerGroup::CharacterSheetBuildProjection,
    ));
    route
}

pub fn expected_route(observed_action_taken: &str) -> Vec<CharacterSheetRouteEvent> {
    replay_observed_route(observed_action_taken)
}

pub fn projection_payload(witness: &SheetClassFeatureWitness) -> String {
    [
        format!("lastResult={}", witness.last_result),
        format!("featureUnitId={}", witness.feature_unit_id),
        format!(
            "spellcastingSourceUnitId={}",
            witness.spellcasting_source_unit_id
        ),
        format!("expectedSpellPresent={}", witness.expected_spell_present),
        format!("spellCount={}", witness.spell_count),
        format!("abilityCheckBonus={}", witness.ability_check_bonus),
        format!("land={}", witness.land),
        format!("accepted={}", witness.accepted),
    ]
    .join("\n")
}

fn spell_feature_facts(
    spell_count: i16,
    spellcasting_source_present: bool,
    land_choice_present: bool,
) -> SheetClassFeatureFacts {
    SheetClassFeatureFacts {
        expected_spell_present: true,
        spell_count,
        ability_check_bonus: 0,
        spellcasting_source_present,
        land_choice_present,
    }
}

fn class_feature_witness(
    last_result: &'static str,
    feature_unit_id: &'static str,
    spellcasting_source_unit_id: &'static str,
    land: &'static str,
    facts: SheetClassFeatureFacts,
) -> SheetClassFeatureWitness {
    let projection = project_sheet_class_feature(facts);
    witness_from_projection(
        last_result,
        feature_unit_id,
        spellcasting_source_unit_id,
        land,
        projection,
    )
}

fn witness_from_projection(
    last_result: &'static str,
    feature_unit_id: &'static str,
    spellcasting_source_unit_id: &'static str,
    land: &'static str,
    projection: SheetClassFeatureProjection,
) -> SheetClassFeatureWitness {
    SheetClassFeatureWitness {
        last_result,
        feature_unit_id,
        spellcasting_source_unit_id,
        expected_spell_present: projection.expected_spell_present,
        spell_count: projection.spell_count,
        ability_check_bonus: projection.ability_check_bonus,
        land,
        accepted: projection.accepted,
    }
}

fn expected(
    last_result: &'static str,
    feature_unit_id: &'static str,
    spellcasting_source_unit_id: &'static str,
    expected_spell_present: bool,
    spell_count: i16,
    ability_check_bonus: i16,
    land: &'static str,
) -> SheetClassFeatureWitness {
    SheetClassFeatureWitness {
        last_result,
        feature_unit_id,
        spellcasting_source_unit_id,
        expected_spell_present,
        spell_count,
        ability_check_bonus,
        land,
        accepted: true,
    }
}
