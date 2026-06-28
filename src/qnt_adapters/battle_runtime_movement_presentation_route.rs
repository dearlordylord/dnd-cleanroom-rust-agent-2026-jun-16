use crate::rules::battle_reducer_spine::{
    discover_generic_route_subject_observed, generic_route_subject_for_current_actor,
    resolve_battle_subject_observed, start_battle_observed, BattleEntrypointTrace,
    BattleGenericRouteFill, BattleMovementPresentationRouteState, BattleResolutionRequest,
    BattleSetup, BattleState, BattleSubjectKind,
};

use super::battle_runtime_reducer_route::{
    observed_reducer_route, route_discover_battle_acts_from_route_holes,
    route_resolve_battle_subject_from_route_result,
    route_resolve_battle_subject_without_fill_from_route_result, route_start_battle,
    ReducerRouteEvent, ReducerRouteFillKind, ReducerRouteHoleKind, ReducerRouteOwnerGroup,
    ReducerRouteResolutionOutcome, ReducerRouteSubjectFamily,
};

pub const CONNECTOR_ACTIONS: [&str; 3] = [
    "doRouteMovementReplacementLandingWitness",
    "doRouteForcedCreatureMovementPresentation",
    "doRouteObjectPushPresentation",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MovementPresentationRouteFact {
    Resource(MovementPresentationResourceFact),
    Table(MovementPresentationTableFact),
    ObjectBoundary(MovementPresentationObjectBoundaryFact),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MovementPresentationResourceFact {
    MovementReplacementFixedBudgetSpend,
    MovementReplacementDistanceProjection,
    ForcedMovementNoOwnMovementResource,
    ForcedMovementDistanceProjection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(clippy::enum_variant_names)]
pub enum MovementPresentationTableFact {
    TableSuppliedMovementPathWitness,
    LandingSpacePresentationWitness,
    DirectionAwayFromSourcePresentationWitness,
    AreaGeometryPresentationWitness,
    AudibleEffectPresentationWitness,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(clippy::enum_variant_names)]
pub enum MovementPresentationObjectBoundaryFact {
    ObjectUnsecuredBoundary,
    ObjectEntirelyWithinAreaBoundary,
    ObjectPushProjection,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MovementPresentationRouteWitness {
    pub route_state: BattleMovementPresentationRouteState,
    pub facts: Vec<MovementPresentationRouteFact>,
}

pub fn replay_observed_action(observed_action_taken: &str) -> MovementPresentationRouteWitness {
    replay_observed_state_and_route(observed_action_taken).0
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    replay_observed_state_and_route(observed_action_taken).1
}

pub fn expected_witness(observed_action_taken: &str) -> MovementPresentationRouteWitness {
    expected_for_action(observed_action_taken).0
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    expected_for_action(observed_action_taken).1
}

pub fn projection_payload(witness: &MovementPresentationRouteWitness) -> String {
    [
        format!("routeState={}", route_state_ref(witness.route_state)),
        format!("facts={}", facts_payload(&witness.facts)),
    ]
    .join("\n")
}

fn replay_observed_state_and_route(
    observed_action_taken: &str,
) -> (MovementPresentationRouteWitness, Vec<ReducerRouteEvent>) {
    let mut trace = BattleEntrypointTrace::default();
    let state = start_battle_observed(BattleSetup::standard(), &mut trace).state;
    let state = match observed_action_taken {
        "doRouteMovementReplacementLandingWitness" => route_movement_replacement(state, &mut trace),
        "doRouteForcedCreatureMovementPresentation" => route_forced_movement(state, &mut trace),
        "doRouteObjectPushPresentation" => route_object_push(state, &mut trace),
        action => panic!("unsupported movement-presentation route action {action}"),
    };
    (
        MovementPresentationRouteWitness {
            route_state: state.movement_presentation_route,
            facts: expected_facts(observed_action_taken),
        },
        observed_reducer_route(
            &trace,
            &[ReducerRouteSubjectFamily::MovementPresentationRoute],
        ),
    )
}

fn route_movement_replacement(
    state: BattleState,
    trace: &mut BattleEntrypointTrace,
) -> BattleState {
    let state = discover_then_resolve(
        state,
        BattleSubjectKind::MovementPresentationReplacementMovement,
        BattleGenericRouteFill::Movement { accepted: true },
        trace,
    );
    resolve_without_fill(
        state,
        BattleSubjectKind::MovementPresentationReplacementTable,
        trace,
    )
}

fn route_forced_movement(state: BattleState, trace: &mut BattleEntrypointTrace) -> BattleState {
    let state = discover_then_resolve(
        state,
        BattleSubjectKind::MovementPresentationForcedSavingThrow,
        BattleGenericRouteFill::SavingThrowOutcome,
        trace,
    );
    let state = resolve(
        state,
        BattleSubjectKind::MovementPresentationForcedMovement,
        BattleGenericRouteFill::Movement { accepted: true },
        trace,
    );
    resolve_without_fill(
        state,
        BattleSubjectKind::MovementPresentationForcedTable,
        trace,
    )
}

fn route_object_push(state: BattleState, trace: &mut BattleEntrypointTrace) -> BattleState {
    let state = discover_then_resolve_without_fill(
        state,
        BattleSubjectKind::MovementPresentationObjectBoundary,
        trace,
    );
    resolve_without_fill(
        state,
        BattleSubjectKind::MovementPresentationObjectTable,
        trace,
    )
}

fn discover_then_resolve_without_fill(
    state: BattleState,
    kind: BattleSubjectKind,
    trace: &mut BattleEntrypointTrace,
) -> BattleState {
    let (state, subject) = discover_generic_route_subject_observed(state, kind, trace);
    resolve_subject(state, subject, BattleGenericRouteFill::WithoutFill, trace)
}

fn discover_then_resolve(
    state: BattleState,
    kind: BattleSubjectKind,
    fill: BattleGenericRouteFill,
    trace: &mut BattleEntrypointTrace,
) -> BattleState {
    let (state, subject) = discover_generic_route_subject_observed(state, kind, trace);
    resolve_subject(state, subject, fill, trace)
}

fn resolve_without_fill(
    state: BattleState,
    kind: BattleSubjectKind,
    trace: &mut BattleEntrypointTrace,
) -> BattleState {
    resolve(state, kind, BattleGenericRouteFill::WithoutFill, trace)
}

fn resolve(
    state: BattleState,
    kind: BattleSubjectKind,
    fill: BattleGenericRouteFill,
    trace: &mut BattleEntrypointTrace,
) -> BattleState {
    let subject = generic_route_subject_for_current_actor(&state, kind);
    resolve_subject(state, subject, fill, trace)
}

fn resolve_subject(
    state: BattleState,
    subject: crate::rules::battle_reducer_spine::BattleSubject,
    fill: BattleGenericRouteFill,
    trace: &mut BattleEntrypointTrace,
) -> BattleState {
    resolve_battle_subject_observed(
        state,
        BattleResolutionRequest::generic_route(subject, fill)
            .expect("movement-presentation route subject should accept generic route fill"),
        trace,
    )
    .into_state()
}

fn expected_for_action(action: &str) -> (MovementPresentationRouteWitness, Vec<ReducerRouteEvent>) {
    let mut route = vec![route_start_battle(ReducerRouteOwnerGroup::ActionEconomy)];
    let state = match action {
        "doRouteMovementReplacementLandingWitness" => {
            route.extend(movement_replacement_route());
            BattleMovementPresentationRouteState::MovementReplacementLandingPresented
        }
        "doRouteForcedCreatureMovementPresentation" => {
            route.extend(forced_creature_movement_route());
            BattleMovementPresentationRouteState::ForcedCreatureMovementPresented
        }
        "doRouteObjectPushPresentation" => {
            route.extend(object_push_presentation_route());
            BattleMovementPresentationRouteState::ObjectPushPresented
        }
        action => panic!("unsupported expected movement-presentation route action {action}"),
    };
    (
        MovementPresentationRouteWitness {
            route_state: state,
            facts: expected_facts(action),
        },
        route,
    )
}

fn movement_replacement_route() -> Vec<ReducerRouteEvent> {
    vec![
        discover(
            vec![ReducerRouteHoleKind::Movement],
            ReducerRouteOwnerGroup::MovementResource,
        ),
        resolve_with_fill(
            ReducerRouteFillKind::Movement,
            Vec::new(),
            ReducerRouteOwnerGroup::MovementResource,
        ),
        resolve_without_fill_event(ReducerRouteOwnerGroup::TablePresentation),
    ]
}

fn forced_creature_movement_route() -> Vec<ReducerRouteEvent> {
    vec![
        discover(
            vec![
                ReducerRouteHoleKind::Movement,
                ReducerRouteHoleKind::SavingThrowOutcome,
            ],
            ReducerRouteOwnerGroup::SavingThrowOutcome,
        ),
        resolve_with_fill(
            ReducerRouteFillKind::SavingThrowOutcome,
            vec![ReducerRouteHoleKind::Movement],
            ReducerRouteOwnerGroup::SavingThrowOutcome,
        ),
        resolve_with_fill(
            ReducerRouteFillKind::Movement,
            Vec::new(),
            ReducerRouteOwnerGroup::MovementResource,
        ),
        resolve_without_fill_event(ReducerRouteOwnerGroup::TablePresentation),
    ]
}

fn object_push_presentation_route() -> Vec<ReducerRouteEvent> {
    vec![
        discover(Vec::new(), ReducerRouteOwnerGroup::ObjectTargetBoundary),
        resolve_without_fill_event(ReducerRouteOwnerGroup::ObjectTargetBoundary),
        resolve_without_fill_event(ReducerRouteOwnerGroup::TablePresentation),
    ]
}

fn discover(holes: Vec<ReducerRouteHoleKind>, owner: ReducerRouteOwnerGroup) -> ReducerRouteEvent {
    route_discover_battle_acts_from_route_holes(
        ReducerRouteSubjectFamily::MovementPresentationRoute,
        holes,
        owner,
    )
}

fn resolve_with_fill(
    fill: ReducerRouteFillKind,
    holes: Vec<ReducerRouteHoleKind>,
    owner: ReducerRouteOwnerGroup,
) -> ReducerRouteEvent {
    route_resolve_battle_subject_from_route_result(
        ReducerRouteSubjectFamily::MovementPresentationRoute,
        fill,
        if holes.is_empty() {
            ReducerRouteResolutionOutcome::Resolved
        } else {
            ReducerRouteResolutionOutcome::NeedsHoles
        },
        holes,
        owner,
    )
}

fn resolve_without_fill_event(owner: ReducerRouteOwnerGroup) -> ReducerRouteEvent {
    route_resolve_battle_subject_without_fill_from_route_result(
        ReducerRouteSubjectFamily::MovementPresentationRoute,
        ReducerRouteResolutionOutcome::Resolved,
        Vec::new(),
        owner,
    )
}

fn expected_facts(action: &str) -> Vec<MovementPresentationRouteFact> {
    match action {
        "doRouteMovementReplacementLandingWitness" => movement_replacement_facts(),
        "doRouteForcedCreatureMovementPresentation" => forced_creature_movement_facts(),
        "doRouteObjectPushPresentation" => object_push_presentation_facts(),
        action => panic!("unsupported movement-presentation facts action {action}"),
    }
}

fn movement_replacement_facts() -> Vec<MovementPresentationRouteFact> {
    vec![
        MovementPresentationRouteFact::Resource(
            MovementPresentationResourceFact::MovementReplacementFixedBudgetSpend,
        ),
        MovementPresentationRouteFact::Resource(
            MovementPresentationResourceFact::MovementReplacementDistanceProjection,
        ),
        MovementPresentationRouteFact::Table(
            MovementPresentationTableFact::TableSuppliedMovementPathWitness,
        ),
        MovementPresentationRouteFact::Table(
            MovementPresentationTableFact::LandingSpacePresentationWitness,
        ),
    ]
}

fn forced_creature_movement_facts() -> Vec<MovementPresentationRouteFact> {
    vec![
        MovementPresentationRouteFact::Resource(
            MovementPresentationResourceFact::ForcedMovementNoOwnMovementResource,
        ),
        MovementPresentationRouteFact::Resource(
            MovementPresentationResourceFact::ForcedMovementDistanceProjection,
        ),
        MovementPresentationRouteFact::Table(
            MovementPresentationTableFact::DirectionAwayFromSourcePresentationWitness,
        ),
        MovementPresentationRouteFact::Table(
            MovementPresentationTableFact::AreaGeometryPresentationWitness,
        ),
    ]
}

fn object_push_presentation_facts() -> Vec<MovementPresentationRouteFact> {
    vec![
        MovementPresentationRouteFact::ObjectBoundary(
            MovementPresentationObjectBoundaryFact::ObjectUnsecuredBoundary,
        ),
        MovementPresentationRouteFact::ObjectBoundary(
            MovementPresentationObjectBoundaryFact::ObjectEntirelyWithinAreaBoundary,
        ),
        MovementPresentationRouteFact::ObjectBoundary(
            MovementPresentationObjectBoundaryFact::ObjectPushProjection,
        ),
        MovementPresentationRouteFact::Table(
            MovementPresentationTableFact::DirectionAwayFromSourcePresentationWitness,
        ),
        MovementPresentationRouteFact::Table(
            MovementPresentationTableFact::AudibleEffectPresentationWitness,
        ),
    ]
}

fn route_state_ref(state: BattleMovementPresentationRouteState) -> &'static str {
    match state {
        BattleMovementPresentationRouteState::Inactive => "FreshMovementPresentationRouteSurface",
        BattleMovementPresentationRouteState::MovementReplacementLandingPresented => {
            "MovementReplacementLandingRouteSurface"
        }
        BattleMovementPresentationRouteState::ForcedCreatureMovementPresented => {
            "ForcedCreatureMovementPresentationRouteSurface"
        }
        BattleMovementPresentationRouteState::ObjectPushPresented => {
            "ObjectPushPresentationRouteSurface"
        }
    }
}

fn facts_payload(facts: &[MovementPresentationRouteFact]) -> String {
    if facts.is_empty() {
        return "none".to_string();
    }
    facts.iter().map(fact_ref).collect::<Vec<_>>().join(",")
}

fn fact_ref(fact: &MovementPresentationRouteFact) -> &'static str {
    match *fact {
        MovementPresentationRouteFact::Resource(fact) => resource_fact_ref(fact),
        MovementPresentationRouteFact::Table(fact) => table_fact_ref(fact),
        MovementPresentationRouteFact::ObjectBoundary(fact) => object_boundary_fact_ref(fact),
    }
}

fn resource_fact_ref(fact: MovementPresentationResourceFact) -> &'static str {
    match fact {
        MovementPresentationResourceFact::MovementReplacementFixedBudgetSpend => {
            "MovementReplacementFixedBudgetSpend"
        }
        MovementPresentationResourceFact::MovementReplacementDistanceProjection => {
            "MovementReplacementDistanceProjection"
        }
        MovementPresentationResourceFact::ForcedMovementNoOwnMovementResource => {
            "ForcedMovementNoOwnMovementResource"
        }
        MovementPresentationResourceFact::ForcedMovementDistanceProjection => {
            "ForcedMovementDistanceProjection"
        }
    }
}

fn table_fact_ref(fact: MovementPresentationTableFact) -> &'static str {
    match fact {
        MovementPresentationTableFact::TableSuppliedMovementPathWitness => {
            "TableSuppliedMovementPathWitness"
        }
        MovementPresentationTableFact::LandingSpacePresentationWitness => {
            "LandingSpacePresentationWitness"
        }
        MovementPresentationTableFact::DirectionAwayFromSourcePresentationWitness => {
            "DirectionAwayFromSourcePresentationWitness"
        }
        MovementPresentationTableFact::AreaGeometryPresentationWitness => {
            "AreaGeometryPresentationWitness"
        }
        MovementPresentationTableFact::AudibleEffectPresentationWitness => {
            "AudibleEffectPresentationWitness"
        }
    }
}

fn object_boundary_fact_ref(fact: MovementPresentationObjectBoundaryFact) -> &'static str {
    match fact {
        MovementPresentationObjectBoundaryFact::ObjectUnsecuredBoundary => {
            "ObjectUnsecuredBoundary"
        }
        MovementPresentationObjectBoundaryFact::ObjectEntirelyWithinAreaBoundary => {
            "ObjectEntirelyWithinAreaBoundary"
        }
        MovementPresentationObjectBoundaryFact::ObjectPushProjection => "ObjectPushProjection",
    }
}
