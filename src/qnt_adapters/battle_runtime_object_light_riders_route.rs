use crate::rules::battle_reducer_spine::{
    discover_generic_route_subject_observed, object_light_route_subject_for_current_actor,
    resolve_battle_subject_observed, start_battle_observed, BattleEntrypointTrace,
    BattleGenericRouteFill, BattleObjectLightEmitterAttachment, BattleObjectLightEmitterSource,
    BattleObjectLightProjectionState, BattleObjectLightRouteState, BattleResolutionRequest,
    BattleSetup, BattleState, BattleSubjectKind,
};

use super::battle_runtime_reducer_route::{
    observed_reducer_route, route_discover_battle_acts_from_route_holes,
    route_resolve_battle_subject_from_route_result,
    route_resolve_battle_subject_without_fill_from_route_result, route_start_battle,
    ReducerRouteEvent, ReducerRouteFillKind, ReducerRouteHoleKind, ReducerRouteOwnerGroup,
    ReducerRouteResolutionOutcome, ReducerRouteSubjectFamily,
};

pub const CONNECTOR_ACTIONS: [&str; 9] = [
    "doAdmitObjectAttachedEmitter",
    "doRejectObjectAttachedEmitterWithoutObjectWitness",
    "doAdmitHeldLightEmitter",
    "doProjectObjectAttachedBrightDimLight",
    "doProjectHeldBrightDimLight",
    "doCleanupObjectAttachedEmitterOnReplacement",
    "doCleanupHeldEmitterOnHurl",
    "doCleanupObjectAttachedEmitterOnDuration",
    "doRecordTableOwnedGeometryAndCoverWitnesses",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectLightRouteFact {
    EmitterSource(ObjectLightEmitterSourceFact),
    Admission(ObjectLightEmitterAdmissionFact),
    Attachment(ObjectLightEmitterAttachmentFact),
    Projection(ObjectLightProjectionFact),
    CleanedUpBy {
        cleanup: ObjectLightCleanupFact,
        owner: ObjectLightCleanupOwnerFact,
    },
    TableWitness(ObjectLightTableWitnessFact),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectLightEmitterSourceFact {
    ObjectAttachedEmitterSource,
    HeldEmitterSource,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectLightEmitterAdmissionFact {
    ObjectTargetAdmittedByTableWitness,
    ObjectTargetRejectedByTableWitness,
    EmitterEffectAdmitted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectLightEmitterAttachmentFact {
    AttachedToObject,
    HeldByCaster,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectLightProjectionFact {
    BrightLightProjection,
    DimLightProjection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectLightCleanupFact {
    Replacement,
    Hurl,
    Duration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectLightCleanupOwnerFact {
    BattleActiveEffectCleanupOwner,
    BattleTurnBoundaryCleanupOwner,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectLightTableWitnessFact {
    ObjectValidityAdmission,
    ObjectGeometry,
    CoverGeometry,
    OpaqueBlockerGeometry,
    ColorPresentation,
    ObjectDurabilityBoundary,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectLightRouteWitness {
    pub route_state: BattleObjectLightRouteState,
    pub object_attached: bool,
    pub held: bool,
    pub bright_light_projected: bool,
    pub dim_light_projected: bool,
    pub replacement_cleaned_up: bool,
    pub hurl_cleaned_up: bool,
    pub duration_cleaned_up: bool,
    pub facts: Vec<ObjectLightRouteFact>,
}

pub fn replay_observed_action(observed_action_taken: &str) -> ObjectLightRouteWitness {
    replay_observed_state_and_route(observed_action_taken).0
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    replay_observed_state_and_route(observed_action_taken).1
}

pub fn expected_witness(observed_action_taken: &str) -> ObjectLightRouteWitness {
    expected_for_action(observed_action_taken).0
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    expected_for_action(observed_action_taken).1
}

pub fn projection_payload(witness: &ObjectLightRouteWitness) -> String {
    [
        format!("routeState={}", route_state_ref(witness.route_state)),
        format!("objectAttached={}", witness.object_attached),
        format!("held={}", witness.held),
        format!("brightLightProjected={}", witness.bright_light_projected),
        format!("dimLightProjected={}", witness.dim_light_projected),
        format!("replacementCleanedUp={}", witness.replacement_cleaned_up),
        format!("hurlCleanedUp={}", witness.hurl_cleaned_up),
        format!("durationCleanedUp={}", witness.duration_cleaned_up),
        format!("facts={}", facts_payload(&witness.facts)),
    ]
    .join("\n")
}

fn replay_observed_state_and_route(
    observed_action_taken: &str,
) -> (ObjectLightRouteWitness, Vec<ReducerRouteEvent>) {
    let mut trace = BattleEntrypointTrace::default();
    let state = start_battle_observed(BattleSetup::standard(), &mut trace).state;
    let state = match observed_action_taken {
        "doAdmitObjectAttachedEmitter" => admit_object_attached_emitter(state, &mut trace),
        "doRejectObjectAttachedEmitterWithoutObjectWitness" => {
            reject_object_attached_emitter(state, &mut trace)
        }
        "doAdmitHeldLightEmitter" => admit_held_light_emitter(state, &mut trace),
        "doProjectObjectAttachedBrightDimLight" => {
            let state = admit_object_attached_emitter(state, &mut trace);
            resolve_without_fill(state, BattleSubjectKind::ObjectLightProjection, &mut trace)
        }
        "doProjectHeldBrightDimLight" => {
            let state = admit_held_light_emitter(state, &mut trace);
            resolve_without_fill(state, BattleSubjectKind::ObjectLightProjection, &mut trace)
        }
        "doCleanupObjectAttachedEmitterOnReplacement" => {
            let state = admit_object_attached_emitter(state, &mut trace);
            resolve_without_fill(
                state,
                BattleSubjectKind::ObjectLightReplacementCleanup,
                &mut trace,
            )
        }
        "doCleanupHeldEmitterOnHurl" => {
            let state = admit_held_light_emitter(state, &mut trace);
            resolve_without_fill(state, BattleSubjectKind::ObjectLightHurlCleanup, &mut trace)
        }
        "doCleanupObjectAttachedEmitterOnDuration" => {
            let state = admit_object_attached_emitter(state, &mut trace);
            let state = resolve_without_fill(
                state,
                BattleSubjectKind::ObjectLightDurationTurnBoundary,
                &mut trace,
            );
            resolve_without_fill(
                state,
                BattleSubjectKind::ObjectLightDurationActiveEffectCleanup,
                &mut trace,
            )
        }
        "doRecordTableOwnedGeometryAndCoverWitnesses" => resolve_without_fill(
            state,
            BattleSubjectKind::ObjectLightTableWitness,
            &mut trace,
        ),
        action => panic!("unsupported object/light route action {action}"),
    };
    (
        witness_from_state(state, expected_facts(observed_action_taken)),
        observed_reducer_route(&trace, &[ReducerRouteSubjectFamily::ObjectLightRider]),
    )
}

fn admit_object_attached_emitter(
    state: BattleState,
    trace: &mut BattleEntrypointTrace,
) -> BattleState {
    let (state, subject) = discover_generic_route_subject_observed(
        state,
        BattleSubjectKind::ObjectLightObjectAttachedAdmission,
        trace,
    );
    let state = resolve_with_fill(
        state,
        subject.kind,
        BattleGenericRouteFill::TargetChoice,
        trace,
    );
    resolve_without_fill(state, BattleSubjectKind::ObjectLightEmitterAdmission, trace)
}

fn reject_object_attached_emitter(
    state: BattleState,
    trace: &mut BattleEntrypointTrace,
) -> BattleState {
    let (state, subject) = discover_generic_route_subject_observed(
        state,
        BattleSubjectKind::ObjectLightObjectAttachedRejection,
        trace,
    );
    resolve_with_fill(
        state,
        subject.kind,
        BattleGenericRouteFill::TargetChoice,
        trace,
    )
}

fn admit_held_light_emitter(state: BattleState, trace: &mut BattleEntrypointTrace) -> BattleState {
    resolve_without_fill(
        state,
        BattleSubjectKind::ObjectLightHeldEmitterAdmission,
        trace,
    )
}

fn resolve_without_fill(
    state: BattleState,
    kind: BattleSubjectKind,
    trace: &mut BattleEntrypointTrace,
) -> BattleState {
    resolve_with_fill(state, kind, BattleGenericRouteFill::WithoutFill, trace)
}

fn resolve_with_fill(
    state: BattleState,
    kind: BattleSubjectKind,
    fill: BattleGenericRouteFill,
    trace: &mut BattleEntrypointTrace,
) -> BattleState {
    let subject = object_light_route_subject_for_current_actor(&state, kind);
    resolve_battle_subject_observed(
        state,
        BattleResolutionRequest::generic_route(subject, fill)
            .expect("object/light route subject should accept generic fill"),
        trace,
    )
    .into_state()
}

fn expected_for_action(action: &str) -> (ObjectLightRouteWitness, Vec<ReducerRouteEvent>) {
    let mut route = vec![route_start_battle(ReducerRouteOwnerGroup::ActionEconomy)];
    let facts = expected_facts(action);
    let route_state = match action {
        "doAdmitObjectAttachedEmitter" => {
            route.extend(object_admission_route());
            route.extend(emitter_admission_route());
            active_object_attached(BattleObjectLightProjectionState::NotProjected)
        }
        "doRejectObjectAttachedEmitterWithoutObjectWitness" => {
            route.extend(object_admission_route());
            BattleObjectLightRouteState::ObjectTargetRejected
        }
        "doAdmitHeldLightEmitter" => {
            route.extend(emitter_admission_route());
            active_held(BattleObjectLightProjectionState::NotProjected)
        }
        "doProjectObjectAttachedBrightDimLight" => {
            route.extend(object_admission_route());
            route.extend(emitter_admission_route());
            route.extend(light_projection_route());
            active_object_attached(BattleObjectLightProjectionState::BrightDimProjected)
        }
        "doProjectHeldBrightDimLight" => {
            route.extend(emitter_admission_route());
            route.extend(light_projection_route());
            active_held(BattleObjectLightProjectionState::BrightDimProjected)
        }
        "doCleanupObjectAttachedEmitterOnReplacement" => {
            route.extend(object_admission_route());
            route.extend(emitter_admission_route());
            route.extend(replacement_cleanup_route());
            replacement_cleaned_up()
        }
        "doCleanupHeldEmitterOnHurl" => {
            route.extend(emitter_admission_route());
            route.extend(hurl_cleanup_route());
            hurl_cleaned_up()
        }
        "doCleanupObjectAttachedEmitterOnDuration" => {
            route.extend(object_admission_route());
            route.extend(emitter_admission_route());
            route.extend(duration_cleanup_route());
            duration_cleaned_up()
        }
        "doRecordTableOwnedGeometryAndCoverWitnesses" => {
            route.extend(table_geometry_witness_route());
            BattleObjectLightRouteState::Inactive
        }
        action => panic!("unsupported object/light route action {action}"),
    };
    (witness_from_state_value(route_state, facts), route)
}

fn object_admission_route() -> Vec<ReducerRouteEvent> {
    vec![
        route_discover_battle_acts_from_route_holes(
            ReducerRouteSubjectFamily::ObjectLightRider,
            vec![ReducerRouteHoleKind::TargetChoice],
            ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
        ),
        route_resolve_battle_subject_from_route_result(
            ReducerRouteSubjectFamily::ObjectLightRider,
            ReducerRouteFillKind::TargetChoice,
            ReducerRouteResolutionOutcome::Resolved,
            Vec::new(),
            ReducerRouteOwnerGroup::ObjectTargetBoundary,
        ),
    ]
}

fn emitter_admission_route() -> Vec<ReducerRouteEvent> {
    vec![route_resolve_battle_subject_without_fill_from_route_result(
        ReducerRouteSubjectFamily::ObjectLightRider,
        ReducerRouteResolutionOutcome::Resolved,
        Vec::new(),
        ReducerRouteOwnerGroup::ActiveEffect,
    )]
}

fn light_projection_route() -> Vec<ReducerRouteEvent> {
    vec![route_resolve_battle_subject_without_fill_from_route_result(
        ReducerRouteSubjectFamily::ObjectLightRider,
        ReducerRouteResolutionOutcome::Resolved,
        Vec::new(),
        ReducerRouteOwnerGroup::LightProjection,
    )]
}

fn replacement_cleanup_route() -> Vec<ReducerRouteEvent> {
    vec![route_resolve_battle_subject_without_fill_from_route_result(
        ReducerRouteSubjectFamily::ObjectLightRider,
        ReducerRouteResolutionOutcome::Resolved,
        Vec::new(),
        ReducerRouteOwnerGroup::ActiveEffect,
    )]
}

fn hurl_cleanup_route() -> Vec<ReducerRouteEvent> {
    replacement_cleanup_route()
}

fn duration_cleanup_route() -> Vec<ReducerRouteEvent> {
    vec![
        route_resolve_battle_subject_without_fill_from_route_result(
            ReducerRouteSubjectFamily::ObjectLightRider,
            ReducerRouteResolutionOutcome::Resolved,
            Vec::new(),
            ReducerRouteOwnerGroup::TurnBoundary,
        ),
        route_resolve_battle_subject_without_fill_from_route_result(
            ReducerRouteSubjectFamily::ObjectLightRider,
            ReducerRouteResolutionOutcome::Resolved,
            Vec::new(),
            ReducerRouteOwnerGroup::ActiveEffect,
        ),
    ]
}

fn table_geometry_witness_route() -> Vec<ReducerRouteEvent> {
    vec![route_resolve_battle_subject_without_fill_from_route_result(
        ReducerRouteSubjectFamily::ObjectLightRider,
        ReducerRouteResolutionOutcome::Resolved,
        Vec::new(),
        ReducerRouteOwnerGroup::AreaShape,
    )]
}

fn expected_facts(action: &str) -> Vec<ObjectLightRouteFact> {
    match action {
        "doAdmitObjectAttachedEmitter" => object_attached_admission_facts(),
        "doRejectObjectAttachedEmitterWithoutObjectWitness" => object_rejected_facts(),
        "doAdmitHeldLightEmitter" => held_admission_facts(),
        "doProjectObjectAttachedBrightDimLight" => {
            let mut facts = object_attached_admission_facts();
            facts.extend(bright_dim_projection_facts());
            facts
        }
        "doProjectHeldBrightDimLight" => {
            let mut facts = held_admission_facts();
            facts.extend(bright_dim_projection_facts());
            facts
        }
        "doCleanupObjectAttachedEmitterOnReplacement" => {
            let mut facts = object_attached_admission_facts();
            facts.extend(replacement_cleanup_facts());
            facts
        }
        "doCleanupHeldEmitterOnHurl" => {
            let mut facts = held_admission_facts();
            facts.extend(hurl_cleanup_facts());
            facts
        }
        "doCleanupObjectAttachedEmitterOnDuration" => {
            let mut facts = object_attached_admission_facts();
            facts.extend(duration_cleanup_facts());
            facts
        }
        "doRecordTableOwnedGeometryAndCoverWitnesses" => table_geometry_witness_facts(),
        action => panic!("unsupported object/light route action {action}"),
    }
}

fn object_attached_admission_facts() -> Vec<ObjectLightRouteFact> {
    vec![
        ObjectLightRouteFact::EmitterSource(
            ObjectLightEmitterSourceFact::ObjectAttachedEmitterSource,
        ),
        ObjectLightRouteFact::Admission(
            ObjectLightEmitterAdmissionFact::ObjectTargetAdmittedByTableWitness,
        ),
        ObjectLightRouteFact::Admission(ObjectLightEmitterAdmissionFact::EmitterEffectAdmitted),
        ObjectLightRouteFact::Attachment(ObjectLightEmitterAttachmentFact::AttachedToObject),
        ObjectLightRouteFact::TableWitness(ObjectLightTableWitnessFact::ObjectValidityAdmission),
    ]
}

fn object_rejected_facts() -> Vec<ObjectLightRouteFact> {
    vec![
        ObjectLightRouteFact::EmitterSource(
            ObjectLightEmitterSourceFact::ObjectAttachedEmitterSource,
        ),
        ObjectLightRouteFact::Admission(
            ObjectLightEmitterAdmissionFact::ObjectTargetRejectedByTableWitness,
        ),
        ObjectLightRouteFact::TableWitness(ObjectLightTableWitnessFact::ObjectValidityAdmission),
    ]
}

fn held_admission_facts() -> Vec<ObjectLightRouteFact> {
    vec![
        ObjectLightRouteFact::EmitterSource(ObjectLightEmitterSourceFact::HeldEmitterSource),
        ObjectLightRouteFact::Admission(ObjectLightEmitterAdmissionFact::EmitterEffectAdmitted),
        ObjectLightRouteFact::Attachment(ObjectLightEmitterAttachmentFact::HeldByCaster),
    ]
}

fn bright_dim_projection_facts() -> Vec<ObjectLightRouteFact> {
    vec![
        ObjectLightRouteFact::Projection(ObjectLightProjectionFact::BrightLightProjection),
        ObjectLightRouteFact::Projection(ObjectLightProjectionFact::DimLightProjection),
    ]
}

fn replacement_cleanup_facts() -> Vec<ObjectLightRouteFact> {
    vec![ObjectLightRouteFact::CleanedUpBy {
        cleanup: ObjectLightCleanupFact::Replacement,
        owner: ObjectLightCleanupOwnerFact::BattleActiveEffectCleanupOwner,
    }]
}

fn hurl_cleanup_facts() -> Vec<ObjectLightRouteFact> {
    vec![ObjectLightRouteFact::CleanedUpBy {
        cleanup: ObjectLightCleanupFact::Hurl,
        owner: ObjectLightCleanupOwnerFact::BattleActiveEffectCleanupOwner,
    }]
}

fn duration_cleanup_facts() -> Vec<ObjectLightRouteFact> {
    vec![
        ObjectLightRouteFact::CleanedUpBy {
            cleanup: ObjectLightCleanupFact::Duration,
            owner: ObjectLightCleanupOwnerFact::BattleTurnBoundaryCleanupOwner,
        },
        ObjectLightRouteFact::CleanedUpBy {
            cleanup: ObjectLightCleanupFact::Duration,
            owner: ObjectLightCleanupOwnerFact::BattleActiveEffectCleanupOwner,
        },
    ]
}

fn table_geometry_witness_facts() -> Vec<ObjectLightRouteFact> {
    vec![
        ObjectLightRouteFact::TableWitness(ObjectLightTableWitnessFact::ObjectGeometry),
        ObjectLightRouteFact::TableWitness(ObjectLightTableWitnessFact::CoverGeometry),
        ObjectLightRouteFact::TableWitness(ObjectLightTableWitnessFact::OpaqueBlockerGeometry),
        ObjectLightRouteFact::TableWitness(ObjectLightTableWitnessFact::ColorPresentation),
        ObjectLightRouteFact::TableWitness(ObjectLightTableWitnessFact::ObjectDurabilityBoundary),
    ]
}

fn active_object_attached(
    projection: BattleObjectLightProjectionState,
) -> BattleObjectLightRouteState {
    BattleObjectLightRouteState::Active(
        crate::rules::battle_reducer_spine::BattleObjectLightEmitter {
            source: BattleObjectLightEmitterSource::ObjectAttached,
            attachment: BattleObjectLightEmitterAttachment::AttachedToObject,
            projection,
        },
    )
}

fn active_held(projection: BattleObjectLightProjectionState) -> BattleObjectLightRouteState {
    BattleObjectLightRouteState::Active(
        crate::rules::battle_reducer_spine::BattleObjectLightEmitter {
            source: BattleObjectLightEmitterSource::Held,
            attachment: BattleObjectLightEmitterAttachment::HeldByCaster,
            projection,
        },
    )
}

fn replacement_cleaned_up() -> BattleObjectLightRouteState {
    let BattleObjectLightRouteState::Active(emitter) =
        active_object_attached(BattleObjectLightProjectionState::NotProjected)
    else {
        unreachable!("active_object_attached must return Active")
    };
    BattleObjectLightRouteState::ReplacementCleanedUp(emitter)
}

fn hurl_cleaned_up() -> BattleObjectLightRouteState {
    let BattleObjectLightRouteState::Active(emitter) =
        active_held(BattleObjectLightProjectionState::NotProjected)
    else {
        unreachable!("active_held must return Active")
    };
    BattleObjectLightRouteState::HurlCleanedUp(emitter)
}

fn duration_cleaned_up() -> BattleObjectLightRouteState {
    let BattleObjectLightRouteState::Active(emitter) =
        active_object_attached(BattleObjectLightProjectionState::NotProjected)
    else {
        unreachable!("active_object_attached must return Active")
    };
    BattleObjectLightRouteState::DurationCleanedUp(emitter)
}

fn witness_from_state(
    state: BattleState,
    facts: Vec<ObjectLightRouteFact>,
) -> ObjectLightRouteWitness {
    witness_from_state_value(state.object_light_route, facts)
}

fn witness_from_state_value(
    route_state: BattleObjectLightRouteState,
    facts: Vec<ObjectLightRouteFact>,
) -> ObjectLightRouteWitness {
    let emitter = match route_state {
        BattleObjectLightRouteState::Active(emitter)
        | BattleObjectLightRouteState::ReplacementCleanedUp(emitter)
        | BattleObjectLightRouteState::HurlCleanedUp(emitter)
        | BattleObjectLightRouteState::DurationTurnBoundaryExpired(emitter)
        | BattleObjectLightRouteState::DurationCleanedUp(emitter) => Some(emitter),
        BattleObjectLightRouteState::Inactive
        | BattleObjectLightRouteState::ObjectTargetAdmitted
        | BattleObjectLightRouteState::ObjectTargetRejected => None,
    };
    ObjectLightRouteWitness {
        route_state,
        object_attached: emitter.is_some_and(|emitter| {
            emitter.attachment == BattleObjectLightEmitterAttachment::AttachedToObject
        }),
        held: emitter.is_some_and(|emitter| {
            emitter.attachment == BattleObjectLightEmitterAttachment::HeldByCaster
        }),
        bright_light_projected: emitter.is_some_and(|emitter| {
            emitter.projection == BattleObjectLightProjectionState::BrightDimProjected
        }),
        dim_light_projected: emitter.is_some_and(|emitter| {
            emitter.projection == BattleObjectLightProjectionState::BrightDimProjected
        }),
        replacement_cleaned_up: matches!(
            route_state,
            BattleObjectLightRouteState::ReplacementCleanedUp(_)
        ),
        hurl_cleaned_up: matches!(route_state, BattleObjectLightRouteState::HurlCleanedUp(_)),
        duration_cleaned_up: matches!(
            route_state,
            BattleObjectLightRouteState::DurationCleanedUp(_)
        ),
        facts,
    }
}

fn route_state_ref(state: BattleObjectLightRouteState) -> &'static str {
    match state {
        BattleObjectLightRouteState::Inactive => "Inactive",
        BattleObjectLightRouteState::ObjectTargetAdmitted => "ObjectTargetAdmitted",
        BattleObjectLightRouteState::ObjectTargetRejected => "ObjectTargetRejected",
        BattleObjectLightRouteState::Active(_) => "Active",
        BattleObjectLightRouteState::ReplacementCleanedUp(_) => "ReplacementCleanedUp",
        BattleObjectLightRouteState::HurlCleanedUp(_) => "HurlCleanedUp",
        BattleObjectLightRouteState::DurationTurnBoundaryExpired(_) => {
            "DurationTurnBoundaryExpired"
        }
        BattleObjectLightRouteState::DurationCleanedUp(_) => "DurationCleanedUp",
    }
}

fn facts_payload(facts: &[ObjectLightRouteFact]) -> String {
    if facts.is_empty() {
        return "none".to_string();
    }
    facts.iter().map(fact_ref).collect::<Vec<_>>().join(",")
}

fn fact_ref(fact: &ObjectLightRouteFact) -> &'static str {
    match *fact {
        ObjectLightRouteFact::EmitterSource(source) => emitter_source_fact_ref(source),
        ObjectLightRouteFact::Admission(admission) => admission_fact_ref(admission),
        ObjectLightRouteFact::Attachment(attachment) => attachment_fact_ref(attachment),
        ObjectLightRouteFact::Projection(projection) => projection_fact_ref(projection),
        ObjectLightRouteFact::CleanedUpBy { cleanup, owner } => cleanup_fact_ref(cleanup, owner),
        ObjectLightRouteFact::TableWitness(witness) => table_witness_fact_ref(witness),
    }
}

fn emitter_source_fact_ref(fact: ObjectLightEmitterSourceFact) -> &'static str {
    match fact {
        ObjectLightEmitterSourceFact::ObjectAttachedEmitterSource => "ObjectAttachedEmitterSource",
        ObjectLightEmitterSourceFact::HeldEmitterSource => "HeldEmitterSource",
    }
}

fn admission_fact_ref(fact: ObjectLightEmitterAdmissionFact) -> &'static str {
    match fact {
        ObjectLightEmitterAdmissionFact::ObjectTargetAdmittedByTableWitness => {
            "ObjectTargetAdmittedByTableWitness"
        }
        ObjectLightEmitterAdmissionFact::ObjectTargetRejectedByTableWitness => {
            "ObjectTargetRejectedByTableWitness"
        }
        ObjectLightEmitterAdmissionFact::EmitterEffectAdmitted => "EmitterEffectAdmitted",
    }
}

fn attachment_fact_ref(fact: ObjectLightEmitterAttachmentFact) -> &'static str {
    match fact {
        ObjectLightEmitterAttachmentFact::AttachedToObject => "AttachedToObject",
        ObjectLightEmitterAttachmentFact::HeldByCaster => "HeldByCaster",
    }
}

fn projection_fact_ref(fact: ObjectLightProjectionFact) -> &'static str {
    match fact {
        ObjectLightProjectionFact::BrightLightProjection => "BrightLightProjection",
        ObjectLightProjectionFact::DimLightProjection => "DimLightProjection",
    }
}

fn cleanup_fact_ref(
    cleanup: ObjectLightCleanupFact,
    owner: ObjectLightCleanupOwnerFact,
) -> &'static str {
    match (cleanup, owner) {
        (
            ObjectLightCleanupFact::Replacement,
            ObjectLightCleanupOwnerFact::BattleActiveEffectCleanupOwner,
        ) => "ReplacementCleanup:BattleActiveEffectCleanupOwner",
        (
            ObjectLightCleanupFact::Hurl,
            ObjectLightCleanupOwnerFact::BattleActiveEffectCleanupOwner,
        ) => "HurlCleanup:BattleActiveEffectCleanupOwner",
        (
            ObjectLightCleanupFact::Duration,
            ObjectLightCleanupOwnerFact::BattleTurnBoundaryCleanupOwner,
        ) => "DurationCleanup:BattleTurnBoundaryCleanupOwner",
        (
            ObjectLightCleanupFact::Duration,
            ObjectLightCleanupOwnerFact::BattleActiveEffectCleanupOwner,
        ) => "DurationCleanup:BattleActiveEffectCleanupOwner",
        (
            ObjectLightCleanupFact::Replacement | ObjectLightCleanupFact::Hurl,
            ObjectLightCleanupOwnerFact::BattleTurnBoundaryCleanupOwner,
        ) => "UnsupportedCleanupOwner",
    }
}

fn table_witness_fact_ref(fact: ObjectLightTableWitnessFact) -> &'static str {
    match fact {
        ObjectLightTableWitnessFact::ObjectValidityAdmission => "ObjectValidityAdmissionWitness",
        ObjectLightTableWitnessFact::ObjectGeometry => "ObjectGeometryWitness",
        ObjectLightTableWitnessFact::CoverGeometry => "CoverGeometryWitness",
        ObjectLightTableWitnessFact::OpaqueBlockerGeometry => "OpaqueBlockerGeometryWitness",
        ObjectLightTableWitnessFact::ColorPresentation => "ColorPresentationWitness",
        ObjectLightTableWitnessFact::ObjectDurabilityBoundary => "ObjectDurabilityBoundaryWitness",
    }
}
