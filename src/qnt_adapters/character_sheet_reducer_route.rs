#[allow(dead_code, clippy::enum_variant_names)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterSheetRouteSubjectFamily {
    SheetState,
    SheetHitPoint,
    SheetRest,
    SheetFeatureResource,
    SheetSpellResource,
    SheetBuildFactsProjection,
    SheetArmorClassProjection,
    SheetAbilityCheckProjection,
    SheetSelectedReferenceProjection,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CharacterSheetRouteHoleFamily {
    HitDiceSpend,
    RestBenefitChoice,
    ResourceSpend,
    RecoveryChoice,
    ProjectionChoice,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterSheetRouteFillFamily {
    HitDiceSpend,
    RestDuration,
    ResourceSpend,
    RecoverySelection,
    ProjectionSelection,
}

#[allow(dead_code, clippy::enum_variant_names)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterSheetRouteOwnerGroup {
    CharacterSheetState,
    CharacterSheetHitPoint,
    CharacterSheetHitDice,
    CharacterSheetSpellSlot,
    CharacterSheetPactSlot,
    CharacterSheetFeatureResource,
    CharacterSheetBuildProjection,
    CharacterSheetSelectedReference,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CharacterSheetRouteEvent {
    CreateCharacterSheet {
        owner: CharacterSheetRouteOwnerGroup,
    },
    ProjectCharacterSheetFacts {
        subject: CharacterSheetRouteSubjectFamily,
        owner: CharacterSheetRouteOwnerGroup,
    },
    RetainCharacterSheetSelectedReferences {
        subject: CharacterSheetRouteSubjectFamily,
        owner: CharacterSheetRouteOwnerGroup,
    },
    ResolveCharacterSheetSubject {
        subject: CharacterSheetRouteSubjectFamily,
        fill: CharacterSheetRouteFillFamily,
        holes: Vec<CharacterSheetRouteHoleFamily>,
        owner: CharacterSheetRouteOwnerGroup,
    },
    CompleteCharacterSheetRest {
        subject: CharacterSheetRouteSubjectFamily,
        fill: CharacterSheetRouteFillFamily,
        holes: Vec<CharacterSheetRouteHoleFamily>,
        owner: CharacterSheetRouteOwnerGroup,
    },
}

#[must_use]
pub fn route_create_character_sheet(
    owner: CharacterSheetRouteOwnerGroup,
) -> CharacterSheetRouteEvent {
    CharacterSheetRouteEvent::CreateCharacterSheet { owner }
}

#[must_use]
pub fn route_project_character_sheet_facts(
    subject: CharacterSheetRouteSubjectFamily,
    owner: CharacterSheetRouteOwnerGroup,
) -> CharacterSheetRouteEvent {
    CharacterSheetRouteEvent::ProjectCharacterSheetFacts { subject, owner }
}

#[must_use]
pub fn route_retain_character_sheet_selected_references(
    subject: CharacterSheetRouteSubjectFamily,
    owner: CharacterSheetRouteOwnerGroup,
) -> CharacterSheetRouteEvent {
    CharacterSheetRouteEvent::RetainCharacterSheetSelectedReferences { subject, owner }
}

#[must_use]
pub fn route_resolve_character_sheet_subject(
    subject: CharacterSheetRouteSubjectFamily,
    fill: CharacterSheetRouteFillFamily,
    holes: Vec<CharacterSheetRouteHoleFamily>,
    owner: CharacterSheetRouteOwnerGroup,
) -> CharacterSheetRouteEvent {
    CharacterSheetRouteEvent::ResolveCharacterSheetSubject {
        subject,
        fill,
        holes: sorted_holes(holes),
        owner,
    }
}

#[must_use]
pub fn route_complete_character_sheet_rest(
    subject: CharacterSheetRouteSubjectFamily,
    fill: CharacterSheetRouteFillFamily,
    holes: Vec<CharacterSheetRouteHoleFamily>,
    owner: CharacterSheetRouteOwnerGroup,
) -> CharacterSheetRouteEvent {
    CharacterSheetRouteEvent::CompleteCharacterSheetRest {
        subject,
        fill,
        holes: sorted_holes(holes),
        owner,
    }
}

#[must_use]
pub fn initial_sheet_build_route() -> Vec<CharacterSheetRouteEvent> {
    vec![
        route_create_character_sheet(CharacterSheetRouteOwnerGroup::CharacterSheetState),
        route_project_character_sheet_facts(
            CharacterSheetRouteSubjectFamily::SheetBuildFactsProjection,
            CharacterSheetRouteOwnerGroup::CharacterSheetBuildProjection,
        ),
    ]
}

#[must_use]
pub fn character_sheet_route_payload(route: &[CharacterSheetRouteEvent]) -> String {
    route
        .iter()
        .map(|event| format!("{event:?}"))
        .collect::<Vec<_>>()
        .join("\n")
}

fn sorted_holes(
    mut holes: Vec<CharacterSheetRouteHoleFamily>,
) -> Vec<CharacterSheetRouteHoleFamily> {
    holes.sort();
    holes
}
