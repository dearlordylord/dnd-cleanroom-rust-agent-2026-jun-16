use std::collections::BTreeSet;

use crate::rules::character_creation::{
    CreationRetainedReferenceOperation, CreationRouteEvent, CreationRouteFillFamily,
    CreationRouteHoleFamily, CreationRouteOwnerGroup, CreationRouteSubjectFamily,
};

#[must_use]
pub fn expected_completed_fighter_creation_route() -> Vec<CreationRouteEvent> {
    let mut route = vec![
        CreationRouteEvent::CreateCharacterDraft {
            owner: CreationRouteOwnerGroup::CharacterDraft,
        },
        CreationRouteEvent::DiscoverCreationHoles {
            subject: CreationRouteSubjectFamily::DraftState,
            holes: holes(&[
                CreationRouteHoleFamily::DraftStructure,
                CreationRouteHoleFamily::AbilityScore,
            ]),
            owner: CreationRouteOwnerGroup::CreationHoleFrontier,
        },
    ];

    append_apply_and_discover(
        &mut route,
        &[
            CreationRouteFillFamily::ChoiceSet,
            CreationRouteFillFamily::AbilityScoreAssignment,
        ],
        &[
            CreationRouteHoleFamily::UnitChoice,
            CreationRouteHoleFamily::EquipmentSelection,
        ],
        CreationRouteOwnerGroup::CharacterDraft,
    );
    append_apply_and_discover(
        &mut route,
        &[
            CreationRouteFillFamily::ChoiceSet,
            CreationRouteFillFamily::EquipmentSelection,
        ],
        &[CreationRouteHoleFamily::EquipmentSelection],
        CreationRouteOwnerGroup::CreationSupportProfileAdmission,
    );
    append_apply_and_discover(
        &mut route,
        &[CreationRouteFillFamily::EquipmentSelection],
        &[CreationRouteHoleFamily::Loadout],
        CreationRouteOwnerGroup::CreationSupportProfileAdmission,
    );
    append_apply_and_discover(
        &mut route,
        &[CreationRouteFillFamily::LoadoutSelection],
        &[],
        CreationRouteOwnerGroup::CharacterDraft,
    );
    route.push(CreationRouteEvent::FinalizeCharacterDraft {
        subject: CreationRouteSubjectFamily::Finalization,
        owner: CreationRouteOwnerGroup::CharacterBuild,
    });

    route
}

#[must_use]
pub fn expected_retained_reference_route(
    operation: CreationRetainedReferenceOperation,
) -> Vec<CreationRouteEvent> {
    let mut route = expected_completed_fighter_creation_route();
    match operation {
        CreationRetainedReferenceOperation::RetainOnly => {
            append_retain_selected_references(&mut route);
        }
        CreationRetainedReferenceOperation::RetainAndProject => {
            append_retain_selected_references(&mut route);
            append_project_build_facts(&mut route);
        }
        CreationRetainedReferenceOperation::ProjectBuildFacts => {
            append_project_build_facts(&mut route);
        }
        CreationRetainedReferenceOperation::ReplaceAndProject => {
            route.push(CreationRouteEvent::ApplyCreationFillBatch {
                subject: CreationRouteSubjectFamily::RetainedReference,
                fills: fills(&[CreationRouteFillFamily::ChoiceSet]),
                holes: BTreeSet::new(),
                owner: CreationRouteOwnerGroup::CharacterBuild,
            });
            append_retain_selected_references(&mut route);
            append_project_build_facts(&mut route);
        }
        CreationRetainedReferenceOperation::RejectSelection => {
            route.push(CreationRouteEvent::ApplyCreationFillBatch {
                subject: CreationRouteSubjectFamily::RetainedReference,
                fills: fills(&[CreationRouteFillFamily::ChoiceSet]),
                holes: holes(&[CreationRouteHoleFamily::UnitChoice]),
                owner: CreationRouteOwnerGroup::CreationSupportProfileAdmission,
            });
        }
    }

    route
}

#[must_use]
pub fn expected_project_build_facts_route() -> Vec<CreationRouteEvent> {
    let mut route = expected_completed_fighter_creation_route();
    append_project_build_facts(&mut route);
    route
}

fn append_apply_and_discover(
    route: &mut Vec<CreationRouteEvent>,
    fill_families: &[CreationRouteFillFamily],
    hole_families: &[CreationRouteHoleFamily],
    owner: CreationRouteOwnerGroup,
) {
    let holes = holes(hole_families);
    route.push(CreationRouteEvent::ApplyCreationFillBatch {
        subject: CreationRouteSubjectFamily::FillBatch,
        fills: fills(fill_families),
        holes: holes.clone(),
        owner,
    });
    route.push(CreationRouteEvent::DiscoverCreationHoles {
        subject: CreationRouteSubjectFamily::OptionDiscovery,
        holes,
        owner: CreationRouteOwnerGroup::CreationHoleFrontier,
    });
}

fn append_retain_selected_references(route: &mut Vec<CreationRouteEvent>) {
    route.push(CreationRouteEvent::RetainCreationRetainedReferences {
        subject: CreationRouteSubjectFamily::RetainedReference,
        owner: CreationRouteOwnerGroup::CreationRetainedReference,
    });
}

fn append_project_build_facts(route: &mut Vec<CreationRouteEvent>) {
    route.push(CreationRouteEvent::ProjectCharacterBuildFacts {
        subject: CreationRouteSubjectFamily::BuildProjection,
        owner: CreationRouteOwnerGroup::CharacterBuild,
    });
}

fn holes(families: &[CreationRouteHoleFamily]) -> BTreeSet<CreationRouteHoleFamily> {
    families.iter().copied().collect()
}

fn fills(families: &[CreationRouteFillFamily]) -> BTreeSet<CreationRouteFillFamily> {
    families.iter().copied().collect()
}
