use crate::rules::hit_points::{
    hit_point_maximum_projection, HitPointMaximumFacts, HitPointMaximumProjection,
};

use super::character_sheet_reducer_route::{
    initial_sheet_build_route, route_project_character_sheet_facts,
    route_record_character_sheet_facts, CharacterSheetRouteEvent, CharacterSheetRouteFactFamily,
    CharacterSheetRouteOwnerGroup, CharacterSheetRouteSubjectFamily,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HitPointMaximumWitness {
    pub last_result: &'static str,
    pub normal_hit_point_maximum: i16,
    pub effective_hit_point_maximum: i16,
    pub hit_dice_total: i16,
    pub hit_point_maximum_reduction: i16,
    pub replay_index: u8,
}

pub const BRANCH_ACTIONS: [&str; 6] = [
    "doProjectFighterLevelOne",
    "doProjectFighterLevelTwo",
    "doProjectWizardFighterMulticlass",
    "doProjectMinimumHigherLevelGain",
    "doProjectSorcererDraconicResilience",
    "doProjectReducedEffectiveMaximum",
];

pub fn replay_observed_action(observed_action_taken: &str) -> HitPointMaximumWitness {
    match observed_action_taken {
        "doProjectFighterLevelOne" => fighter_level_one(),
        "doProjectFighterLevelTwo" => fighter_level_two(),
        "doProjectWizardFighterMulticlass" => wizard_fighter_multiclass(),
        "doProjectMinimumHigherLevelGain" => minimum_higher_level_gain(),
        "doProjectSorcererDraconicResilience" => sorcerer_draconic_resilience(),
        "doProjectReducedEffectiveMaximum" => reduced_effective_maximum(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> HitPointMaximumWitness {
    replay_observed_action(observed_action_taken)
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<CharacterSheetRouteEvent> {
    target_observed_route(observed_action_taken)
}

pub fn expected_route(observed_action_taken: &str) -> Vec<CharacterSheetRouteEvent> {
    copied_connector_expected_route(observed_action_taken)
}

fn target_observed_route(observed_action_taken: &str) -> Vec<CharacterSheetRouteEvent> {
    route_after_project_count(route_replay_index(observed_action_taken))
}

fn copied_connector_expected_route(observed_action_taken: &str) -> Vec<CharacterSheetRouteEvent> {
    route_after_project_count(route_replay_index(observed_action_taken))
}

fn route_replay_index(observed_action_taken: &str) -> usize {
    match observed_action_taken {
        "doProjectFighterLevelOne" => 1,
        "doProjectFighterLevelTwo" => 2,
        "doProjectWizardFighterMulticlass" => 3,
        "doProjectMinimumHigherLevelGain" => 4,
        "doProjectSorcererDraconicResilience" => 5,
        "doProjectReducedEffectiveMaximum" => 6,
        action => panic!("unsupported route mbt::actionTaken {action}"),
    }
}

pub fn projection_payload(witness: &HitPointMaximumWitness) -> String {
    [
        format!("lastResult={}", witness.last_result),
        format!("normalHitPointMaximum={}", witness.normal_hit_point_maximum),
        format!(
            "effectiveHitPointMaximum={}",
            witness.effective_hit_point_maximum
        ),
        format!("hitDiceTotal={}", witness.hit_dice_total),
        format!(
            "hitPointMaximumReduction={}",
            witness.hit_point_maximum_reduction
        ),
        format!("replayIndex={}", witness.replay_index),
    ]
    .join("\n")
}

fn route_after_project_count(project_count: usize) -> Vec<CharacterSheetRouteEvent> {
    let mut route = initial_sheet_build_route();
    for _ in 0..project_count {
        append_project_hit_point_maximum_route(&mut route);
    }
    route
}

fn append_project_hit_point_maximum_route(route: &mut Vec<CharacterSheetRouteEvent>) {
    route.push(route_project_character_sheet_facts(
        CharacterSheetRouteSubjectFamily::SheetHitPoint,
        CharacterSheetRouteOwnerGroup::CharacterSheetHitPoint,
    ));
    route.push(route_record_character_sheet_facts(
        CharacterSheetRouteSubjectFamily::SheetHitPoint,
        vec![CharacterSheetRouteFactFamily::SheetHitPointMaximumArithmeticInput],
        CharacterSheetRouteOwnerGroup::CharacterSheetBuildProjection,
    ));
}

fn fighter_level_one() -> HitPointMaximumWitness {
    hit_point_maximum_witness("fighter-level-one", facts(10, 1, vec![], 0, 0), 1)
}

fn fighter_level_two() -> HitPointMaximumWitness {
    hit_point_maximum_witness("fighter-level-two", facts(10, 1, vec![10], 0, 0), 2)
}

fn wizard_fighter_multiclass() -> HitPointMaximumWitness {
    hit_point_maximum_witness("wizard-fighter-multiclass", facts(6, 1, vec![10], 0, 0), 3)
}

fn minimum_higher_level_gain() -> HitPointMaximumWitness {
    hit_point_maximum_witness("minimum-higher-level-gain", facts(6, -4, vec![6], 0, 0), 4)
}

fn sorcerer_draconic_resilience() -> HitPointMaximumWitness {
    hit_point_maximum_witness(
        "sorcerer-draconic-resilience",
        facts(6, 1, vec![6, 6], 3, 0),
        5,
    )
}

fn reduced_effective_maximum() -> HitPointMaximumWitness {
    hit_point_maximum_witness("reduced-effective-maximum", facts(10, 1, vec![10], 0, 3), 6)
}

fn facts(
    starting_hit_point_die: i16,
    constitution_modifier: i16,
    fixed_higher_level_hit_point_dice: Vec<i16>,
    hit_point_maximum_bonus: i16,
    hit_point_maximum_reduction: i16,
) -> HitPointMaximumFacts {
    HitPointMaximumFacts {
        starting_hit_point_die,
        constitution_modifier,
        fixed_higher_level_hit_point_dice,
        hit_point_maximum_bonus,
        hit_point_maximum_reduction,
    }
}

fn hit_point_maximum_witness(
    last_result: &'static str,
    facts: HitPointMaximumFacts,
    replay_index: u8,
) -> HitPointMaximumWitness {
    let projection =
        hit_point_maximum_projection(facts).expect("hit point maximum witness facts must be legal");
    witness_from_projection(last_result, replay_index, projection)
}

fn witness_from_projection(
    last_result: &'static str,
    replay_index: u8,
    projection: HitPointMaximumProjection,
) -> HitPointMaximumWitness {
    HitPointMaximumWitness {
        last_result,
        normal_hit_point_maximum: projection.normal_hit_point_maximum,
        effective_hit_point_maximum: projection.effective_hit_point_maximum,
        hit_dice_total: projection.hit_dice_total,
        hit_point_maximum_reduction: projection.hit_point_maximum_reduction,
        replay_index,
    }
}
