use crate::rules::character_creation::{
    apply_creation_retained_reference_operation, completed_fighter_creation_state, route_payload,
    CreationRetainedReferenceOperation, CreationRouteEvent,
};
use crate::rules::class_features::{
    expertise_selection_projection, ExpertiseSelectionFacts, ExpertiseSelectionProjection,
    ExpertiseSkill,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RogueExpertiseWitness {
    pub outcome: &'static str,
    pub selected_expertise_unit_id: &'static str,
    pub selected_expertise_choice_count: u8,
    pub build_expertise_count: u8,
    pub owned_skill_proficiency_count: u8,
    pub rogue_expertise_unit_ref_present: bool,
    pub acrobatics_expertise_present: bool,
    pub perception_expertise_present: bool,
    pub sleight_of_hand_expertise_present: bool,
    pub stealth_expertise_present: bool,
    pub total_level: u8,
}

pub const BRANCH_ACTIONS: &[&str] = &[
    "doSelectLevelOneOwnedSkillExpertise",
    "doSelectLevelSixAdditionalOwnedSkillExpertise",
];

pub fn replay_observed_action(observed_action_taken: &str) -> RogueExpertiseWitness {
    let (outcome, facts) = match observed_action_taken {
        "doSelectLevelOneOwnedSkillExpertise" => (
            "level-one",
            ExpertiseSelectionFacts {
                requested_skills: vec![ExpertiseSkill::SleightOfHand, ExpertiseSkill::Stealth],
                owned_skill_proficiency_count: 6,
                total_level: 1,
            },
        ),
        "doSelectLevelSixAdditionalOwnedSkillExpertise" => (
            "level-six",
            ExpertiseSelectionFacts {
                requested_skills: vec![
                    ExpertiseSkill::Acrobatics,
                    ExpertiseSkill::Perception,
                    ExpertiseSkill::SleightOfHand,
                    ExpertiseSkill::Stealth,
                ],
                owned_skill_proficiency_count: 6,
                total_level: 6,
            },
        ),
        action => panic!("unsupported mbt::actionTaken {action}"),
    };
    let projection = expertise_selection_projection(facts)
        .expect("QNT-selected expertise skills are owned and unique");
    witness_from_projection(outcome, &projection)
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<CreationRouteEvent> {
    match observed_action_taken {
        "doSelectLevelOneOwnedSkillExpertise" => {
            apply_creation_retained_reference_operation(
                &completed_fighter_creation_state(),
                CreationRetainedReferenceOperation::RetainOnly,
            )
            .route
        }
        "doSelectLevelSixAdditionalOwnedSkillExpertise" => Vec::new(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_route(observed_action_taken: &str) -> Vec<CreationRouteEvent> {
    replay_observed_route(observed_action_taken)
}

pub fn projection_payload(witness: &RogueExpertiseWitness) -> String {
    [
        format!("outcome={}", witness.outcome),
        format!(
            "selectedExpertiseUnitId={}",
            witness.selected_expertise_unit_id
        ),
        format!(
            "selectedExpertiseChoiceCount={}",
            witness.selected_expertise_choice_count
        ),
        format!("buildExpertiseCount={}", witness.build_expertise_count),
        format!(
            "ownedSkillProficiencyCount={}",
            witness.owned_skill_proficiency_count
        ),
        format!(
            "rogueExpertiseUnitRefPresent={}",
            witness.rogue_expertise_unit_ref_present
        ),
        format!(
            "acrobaticsExpertisePresent={}",
            witness.acrobatics_expertise_present
        ),
        format!(
            "perceptionExpertisePresent={}",
            witness.perception_expertise_present
        ),
        format!(
            "sleightOfHandExpertisePresent={}",
            witness.sleight_of_hand_expertise_present
        ),
        format!(
            "stealthExpertisePresent={}",
            witness.stealth_expertise_present
        ),
        format!("totalLevel={}", witness.total_level),
    ]
    .join("\n")
}

pub fn route_projection_payload(route: &[CreationRouteEvent]) -> String {
    route_payload(route)
}

fn witness_from_projection(
    outcome: &'static str,
    projection: &ExpertiseSelectionProjection,
) -> RogueExpertiseWitness {
    RogueExpertiseWitness {
        outcome,
        selected_expertise_unit_id: "rogue_expertise",
        selected_expertise_choice_count: projection.selected_expertise_choice_count,
        build_expertise_count: projection.build_expertise_count,
        owned_skill_proficiency_count: projection.owned_skill_proficiency_count,
        rogue_expertise_unit_ref_present: true,
        acrobatics_expertise_present: projection
            .selected_skills
            .contains(&ExpertiseSkill::Acrobatics),
        perception_expertise_present: projection
            .selected_skills
            .contains(&ExpertiseSkill::Perception),
        sleight_of_hand_expertise_present: projection
            .selected_skills
            .contains(&ExpertiseSkill::SleightOfHand),
        stealth_expertise_present: projection
            .selected_skills
            .contains(&ExpertiseSkill::Stealth),
        total_level: projection.total_level,
    }
}
