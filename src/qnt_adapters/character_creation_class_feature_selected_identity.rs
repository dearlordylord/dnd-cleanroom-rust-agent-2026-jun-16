use super::character_creation_expected_routes::expected_retained_reference_route;
use crate::rules::character_creation::{
    apply_creation_retained_reference_operation, completed_fighter_creation_state, route_payload,
    CreationRetainedReferenceOperation, CreationRouteEvent,
};
use crate::rules::class_features::{
    selected_class_feature_projection, SelectedClassFeatureFacts, SelectedClassFeatureProjection,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectedClassFeatureWitness {
    pub outcome: &'static str,
    pub feature_unit_id: &'static str,
    pub linked_unit_id: &'static str,
    pub choice_count: u8,
    pub resource_maximum: u8,
    pub known_form_count: u8,
    pub short_rest_refill: u8,
    pub long_rest_refills_all: bool,
    pub accepted: bool,
}

pub const BRANCH_ACTIONS: &[&str] = &[
    "doSelectBardExpertise",
    "doProjectClericChannelDivinity",
    "doProjectDruidWildShape",
    "doProjectDruidWildCompanion",
    "doProjectMonksFocus",
    "doProjectMonkUncannyMetabolism",
    "doSelectPaladinFightingStyle",
    "doSelectRangerDeftExplorer",
    "doSelectRangerFightingStyle",
    "doProjectWarlockPactMagic",
    "doSelectWizardScholar",
    "doSelectWizardEvocationSavant",
];

pub fn replay_observed_action(observed_action_taken: &str) -> SelectedClassFeatureWitness {
    let spec = selected_feature_spec(observed_action_taken);
    let projection = selected_class_feature_projection(spec.facts);
    witness_from_projection(spec, &projection)
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<CreationRouteEvent> {
    let operation = selected_feature_spec(observed_action_taken).route_operation;
    apply_creation_retained_reference_operation(&completed_fighter_creation_state(), operation)
        .route
}

pub fn expected_route(observed_action_taken: &str) -> Vec<CreationRouteEvent> {
    let operation = match observed_action_taken {
        "doSelectBardExpertise"
        | "doSelectPaladinFightingStyle"
        | "doSelectRangerDeftExplorer"
        | "doSelectRangerFightingStyle"
        | "doSelectWizardScholar"
        | "doSelectWizardEvocationSavant" => CreationRetainedReferenceOperation::RetainOnly,
        "doProjectClericChannelDivinity"
        | "doProjectDruidWildShape"
        | "doProjectDruidWildCompanion"
        | "doProjectMonksFocus"
        | "doProjectMonkUncannyMetabolism"
        | "doProjectWarlockPactMagic" => CreationRetainedReferenceOperation::RetainAndProject,
        action => panic!("unsupported mbt::actionTaken {action}"),
    };
    expected_retained_reference_route(operation)
}

pub fn projection_payload(witness: &SelectedClassFeatureWitness) -> String {
    [
        format!("outcome={}", witness.outcome),
        format!("featureUnitId={}", witness.feature_unit_id),
        format!("linkedUnitId={}", witness.linked_unit_id),
        format!("choiceCount={}", witness.choice_count),
        format!("resourceMaximum={}", witness.resource_maximum),
        format!("knownFormCount={}", witness.known_form_count),
        format!("shortRestRefill={}", witness.short_rest_refill),
        format!("longRestRefillsAll={}", witness.long_rest_refills_all),
        format!("accepted={}", witness.accepted),
    ]
    .join("\n")
}

pub fn route_projection_payload(route: &[CreationRouteEvent]) -> String {
    route_payload(route)
}

#[derive(Debug, Clone, Copy)]
struct SelectedFeatureSpec {
    outcome: &'static str,
    feature_unit_id: &'static str,
    linked_unit_id: &'static str,
    facts: SelectedClassFeatureFacts,
    route_operation: CreationRetainedReferenceOperation,
}

fn selected_feature_spec(action: &str) -> SelectedFeatureSpec {
    match action {
        "doSelectBardExpertise" => retain("bard_expertise", "none", 2),
        "doProjectClericChannelDivinity" => project(
            "cleric_channel_divinity",
            "none",
            SelectedClassFeatureFacts {
                choice_count: 2,
                resource_maximum: 2,
                known_form_count: 0,
                short_rest_refill: 1,
                long_rest_refills_all: true,
                accepted: true,
            },
        ),
        "doProjectDruidWildShape" => project(
            "druid_wild_shape",
            "none",
            SelectedClassFeatureFacts {
                choice_count: 0,
                resource_maximum: 2,
                known_form_count: 4,
                short_rest_refill: 1,
                long_rest_refills_all: true,
                accepted: true,
            },
        ),
        "doProjectDruidWildCompanion" => project(
            "druid_wild_companion",
            "druid_wild_shape",
            SelectedClassFeatureFacts {
                choice_count: 2,
                resource_maximum: 0,
                known_form_count: 0,
                short_rest_refill: 0,
                long_rest_refills_all: false,
                accepted: true,
            },
        ),
        "doProjectMonksFocus" => project(
            "monk_monks_focus",
            "none",
            SelectedClassFeatureFacts {
                choice_count: 3,
                resource_maximum: 2,
                known_form_count: 0,
                short_rest_refill: 0,
                long_rest_refills_all: true,
                accepted: true,
            },
        ),
        "doProjectMonkUncannyMetabolism" => project(
            "monk_uncanny_metabolism",
            "monk_monks_focus",
            SelectedClassFeatureFacts {
                choice_count: 0,
                resource_maximum: 0,
                known_form_count: 6,
                short_rest_refill: 0,
                long_rest_refills_all: true,
                accepted: true,
            },
        ),
        "doSelectPaladinFightingStyle" => retain("paladin_fighting_style", "defense", 2),
        "doSelectRangerDeftExplorer" => retain("ranger_deft_explorer", "none", 3),
        "doSelectRangerFightingStyle" => retain("ranger_fighting_style", "guidance", 3),
        "doProjectWarlockPactMagic" => project(
            "warlock_pact_magic",
            "class_warlock",
            SelectedClassFeatureFacts {
                choice_count: 4,
                resource_maximum: 1,
                known_form_count: 1,
                short_rest_refill: 1,
                long_rest_refills_all: true,
                accepted: true,
            },
        ),
        "doSelectWizardScholar" => retain("wizard_scholar", "none", 1),
        "doSelectWizardEvocationSavant" => retain("wizard_evocation_savant", "shatter", 2),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

fn retain(
    feature_unit_id: &'static str,
    linked_unit_id: &'static str,
    choice_count: u8,
) -> SelectedFeatureSpec {
    SelectedFeatureSpec {
        outcome: feature_unit_id,
        feature_unit_id,
        linked_unit_id,
        facts: SelectedClassFeatureFacts {
            choice_count,
            resource_maximum: 0,
            known_form_count: 0,
            short_rest_refill: 0,
            long_rest_refills_all: false,
            accepted: true,
        },
        route_operation: CreationRetainedReferenceOperation::RetainOnly,
    }
}

fn project(
    feature_unit_id: &'static str,
    linked_unit_id: &'static str,
    facts: SelectedClassFeatureFacts,
) -> SelectedFeatureSpec {
    SelectedFeatureSpec {
        outcome: feature_unit_id,
        feature_unit_id,
        linked_unit_id,
        facts,
        route_operation: CreationRetainedReferenceOperation::RetainAndProject,
    }
}

fn witness_from_projection(
    spec: SelectedFeatureSpec,
    projection: &SelectedClassFeatureProjection,
) -> SelectedClassFeatureWitness {
    SelectedClassFeatureWitness {
        outcome: spec.outcome,
        feature_unit_id: spec.feature_unit_id,
        linked_unit_id: spec.linked_unit_id,
        choice_count: projection.choice_count,
        resource_maximum: projection.resource_maximum,
        known_form_count: projection.known_form_count,
        short_rest_refill: projection.short_rest_refill,
        long_rest_refills_all: projection.long_rest_refills_all,
        accepted: projection.accepted,
    }
}
