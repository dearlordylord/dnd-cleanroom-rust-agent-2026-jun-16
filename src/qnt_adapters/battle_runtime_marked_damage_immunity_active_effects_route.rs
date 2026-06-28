use crate::rules::battle_reducer_spine::{
    discover_generic_route_subject_observed, generic_route_subject_for_current_actor,
    resolve_battle_subject_observed, start_battle_observed, BattleEntrypointTrace,
    BattleGenericRouteFill, BattleResolutionRequest, BattleSetup, BattleState, BattleSubjectKind,
};

use super::battle_runtime_reducer_route::{
    observed_reducer_route, route_discover_battle_acts_from_route_holes,
    route_resolve_battle_subject_from_route_result,
    route_resolve_battle_subject_without_fill_from_route_result, route_start_battle,
    ReducerRouteEvent, ReducerRouteFillKind, ReducerRouteHoleKind, ReducerRouteOwnerGroup,
    ReducerRouteResolutionOutcome, ReducerRouteSubjectFamily,
};

pub const CONNECTOR_ACTIONS: [&str; 9] = [
    "doAdmitMarkedDamageRider",
    "doProjectMarkedDamageRiderOnHit",
    "doOpenMarkedDamageRiderTransferAfterTargetDrop",
    "doTransferMarkedDamageRiderSameTurn",
    "doTransferMarkedDamageRiderLaterTurn",
    "doAdmitConditionImmunityTemporaryHitPoints",
    "doProjectConditionImmunity",
    "doGrantTurnStartTemporaryHitPoints",
    "doCleanupConditionImmunityTemporaryHitPoints",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarkedDamageImmunityRouteFact {
    TransferAwaitsMarkedTargetDrop,
    AttackRollHitMarkedTarget,
    ExtraDamageOnMarkedTargetHit,
    TransferAvailableAfterMarkedTargetDrops,
    TransferAvailableOnSameTurn,
    TransferAvailableOnLaterTurn,
    TransferConsumesBonusAction,
    TransferResetsAwaitingMarkedTargetDrop,
    FrightenedConditionImmunity,
    ConditionImmunityApplied,
    ConditionApplicationRejectedByImmunity,
    TurnStartTemporaryHitPointGrantScheduled,
    TurnStartTemporaryHitPointsGranted,
    TemporaryHitPointsUseNonStackingChoice,
    ConcentrationCleanupPreventsLaterTurnStartGrant,
    BattleActiveEffectProjectionOwner,
    BattleConditionLifecycleProjectionOwner,
    BattleTemporaryHitPointProjectionOwner,
    BattleTurnBoundaryProjectionOwner,
    ImmunityBattleConcentrationCleanupOwner,
    ImmunityBattleActiveEffectCleanupOwner,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarkedDamageImmunityRouteWitness {
    pub facts: Vec<MarkedDamageImmunityRouteFact>,
}

pub fn replay_observed_action(observed_action_taken: &str) -> MarkedDamageImmunityRouteWitness {
    MarkedDamageImmunityRouteWitness {
        facts: expected_facts(observed_action_taken),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> MarkedDamageImmunityRouteWitness {
    replay_observed_action(observed_action_taken)
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    let mut trace = BattleEntrypointTrace::default();
    let mut state = start_battle_observed(BattleSetup::standard(), &mut trace).state;

    for step in observed_steps(observed_action_taken) {
        state = replay_step(state, step, &mut trace);
    }

    observed_reducer_route(
        &trace,
        &[
            ReducerRouteSubjectFamily::MarkedEffect,
            ReducerRouteSubjectFamily::ConditionImmunityActiveEffect,
        ],
    )
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    let mut route = vec![route_start_battle(ReducerRouteOwnerGroup::ActionEconomy)];
    route.extend(expected_events(observed_action_taken));
    route
}

pub fn projection_payload(witness: &MarkedDamageImmunityRouteWitness) -> String {
    format!("facts={}", facts_payload(&witness.facts))
}

#[derive(Debug, Clone, Copy)]
enum ReplayStep {
    Discover(BattleSubjectKind),
    Resolve(BattleSubjectKind, BattleGenericRouteFill),
}

fn replay_step(
    state: BattleState,
    step: ReplayStep,
    trace: &mut BattleEntrypointTrace,
) -> BattleState {
    match step {
        ReplayStep::Discover(kind) => discover_generic_route_subject_observed(state, kind, trace).0,
        ReplayStep::Resolve(kind, fill) => {
            let subject = generic_route_subject_for_current_actor(&state, kind);
            let request = BattleResolutionRequest::generic_route(subject, fill)
                .expect("SQNT-07A route step should accept its copied generic fill");
            resolve_battle_subject_observed(state, request, trace).into_state()
        }
    }
}

fn discover(kind: BattleSubjectKind) -> ReplayStep {
    ReplayStep::Discover(kind)
}

fn resolve(kind: BattleSubjectKind, fill: BattleGenericRouteFill) -> ReplayStep {
    ReplayStep::Resolve(kind, fill)
}

fn observed_steps(action: &str) -> Vec<ReplayStep> {
    let mut steps = Vec::new();
    match action {
        "doAdmitMarkedDamageRider" => steps.extend(marked_rider_admission_steps()),
        "doProjectMarkedDamageRiderOnHit" => {
            steps.extend(marked_rider_admission_steps());
            steps.extend(marked_rider_attack_projection_steps());
        }
        "doOpenMarkedDamageRiderTransferAfterTargetDrop" => {
            steps.extend(marked_rider_admission_steps());
            steps.extend(marked_rider_transfer_availability_steps());
        }
        "doTransferMarkedDamageRiderSameTurn" => {
            steps.extend(marked_rider_admission_steps());
            steps.extend(marked_rider_transfer_availability_steps());
            steps.extend(marked_rider_transfer_steps());
        }
        "doTransferMarkedDamageRiderLaterTurn" => {
            steps.extend(marked_rider_admission_steps());
            steps.extend(marked_rider_transfer_availability_steps());
            steps.push(resolve(
                BattleSubjectKind::MarkedDamageRiderLaterTurnBoundary,
                BattleGenericRouteFill::WithoutFill,
            ));
            steps.extend(marked_rider_transfer_steps());
        }
        "doAdmitConditionImmunityTemporaryHitPoints" => {
            steps.extend(immunity_temporary_hit_point_admission_steps());
        }
        "doProjectConditionImmunity" => {
            steps.extend(immunity_temporary_hit_point_admission_steps());
            steps.extend(condition_immunity_projection_steps());
        }
        "doGrantTurnStartTemporaryHitPoints" => {
            steps.extend(immunity_temporary_hit_point_admission_steps());
            steps.extend(turn_start_temporary_hit_point_steps());
        }
        "doCleanupConditionImmunityTemporaryHitPoints" => {
            steps.extend(immunity_temporary_hit_point_admission_steps());
            steps.extend(immunity_temporary_hit_point_cleanup_steps());
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
    steps
}

fn marked_rider_admission_steps() -> Vec<ReplayStep> {
    vec![
        discover(BattleSubjectKind::MarkedDamageRiderAdmissionTargetChoice),
        resolve(
            BattleSubjectKind::MarkedDamageRiderAdmissionTargetChoice,
            BattleGenericRouteFill::TargetChoice,
        ),
        resolve(
            BattleSubjectKind::MarkedDamageRiderActiveEffectAdmission,
            BattleGenericRouteFill::WithoutFill,
        ),
        resolve(
            BattleSubjectKind::MarkedDamageRiderConcentrationAdmission,
            BattleGenericRouteFill::WithoutFill,
        ),
    ]
}

fn marked_rider_attack_projection_steps() -> Vec<ReplayStep> {
    vec![
        discover(BattleSubjectKind::MarkedDamageRiderAttackTargetChoice),
        resolve(
            BattleSubjectKind::MarkedDamageRiderAttackTargetChoice,
            BattleGenericRouteFill::TargetChoice,
        ),
        resolve(
            BattleSubjectKind::MarkedDamageRiderAttackRoll,
            BattleGenericRouteFill::AttackRoll,
        ),
        resolve(
            BattleSubjectKind::MarkedDamageRiderAttackDamage,
            BattleGenericRouteFill::RolledDice,
        ),
    ]
}

fn marked_rider_transfer_availability_steps() -> Vec<ReplayStep> {
    vec![
        resolve(
            BattleSubjectKind::MarkedDamageRiderTargetDropTransferAvailabilityHitPoint,
            BattleGenericRouteFill::WithoutFill,
        ),
        resolve(
            BattleSubjectKind::MarkedDamageRiderTargetDropTransferAvailabilityActiveEffect,
            BattleGenericRouteFill::WithoutFill,
        ),
    ]
}

fn marked_rider_transfer_steps() -> Vec<ReplayStep> {
    vec![
        discover(BattleSubjectKind::MarkedDamageRiderTransferTargetChoice),
        resolve(
            BattleSubjectKind::MarkedDamageRiderTransferTargetChoice,
            BattleGenericRouteFill::TargetChoice,
        ),
        resolve(
            BattleSubjectKind::MarkedDamageRiderTransferActiveEffect,
            BattleGenericRouteFill::WithoutFill,
        ),
    ]
}

fn immunity_temporary_hit_point_admission_steps() -> Vec<ReplayStep> {
    vec![
        discover(BattleSubjectKind::ConditionImmunityTemporaryHitPointAdmissionTargetChoice),
        resolve(
            BattleSubjectKind::ConditionImmunityTemporaryHitPointAdmissionTargetChoice,
            BattleGenericRouteFill::TargetChoice,
        ),
        resolve(
            BattleSubjectKind::ConditionImmunityTemporaryHitPointActiveEffectAdmission,
            BattleGenericRouteFill::WithoutFill,
        ),
        resolve(
            BattleSubjectKind::ConditionImmunityTemporaryHitPointConcentrationAdmission,
            BattleGenericRouteFill::WithoutFill,
        ),
    ]
}

fn condition_immunity_projection_steps() -> Vec<ReplayStep> {
    vec![
        resolve(
            BattleSubjectKind::ConditionImmunityTemporaryHitPointConditionLifecycleProjection,
            BattleGenericRouteFill::WithoutFill,
        ),
        resolve(
            BattleSubjectKind::ConditionImmunityTemporaryHitPointActiveEffectProjection,
            BattleGenericRouteFill::WithoutFill,
        ),
    ]
}

fn turn_start_temporary_hit_point_steps() -> Vec<ReplayStep> {
    vec![
        discover(BattleSubjectKind::ConditionImmunityTemporaryHitPointTurnStartTemporaryHitPoint),
        resolve(
            BattleSubjectKind::ConditionImmunityTemporaryHitPointTurnStartTemporaryHitPoint,
            BattleGenericRouteFill::WithoutFill,
        ),
    ]
}

fn immunity_temporary_hit_point_cleanup_steps() -> Vec<ReplayStep> {
    vec![
        resolve(
            BattleSubjectKind::ConditionImmunityTemporaryHitPointConcentrationCleanup,
            BattleGenericRouteFill::WithoutFill,
        ),
        resolve(
            BattleSubjectKind::ConditionImmunityTemporaryHitPointActiveEffectCleanup,
            BattleGenericRouteFill::WithoutFill,
        ),
    ]
}

fn expected_events(action: &str) -> Vec<ReducerRouteEvent> {
    let mut route = Vec::new();
    match action {
        "doAdmitMarkedDamageRider" => route.extend(expected_marked_rider_admission_route()),
        "doProjectMarkedDamageRiderOnHit" => {
            route.extend(expected_marked_rider_admission_route());
            route.extend(expected_marked_rider_attack_projection_route());
        }
        "doOpenMarkedDamageRiderTransferAfterTargetDrop" => {
            route.extend(expected_marked_rider_admission_route());
            route.extend(expected_marked_rider_transfer_availability_route());
        }
        "doTransferMarkedDamageRiderSameTurn" => {
            route.extend(expected_marked_rider_admission_route());
            route.extend(expected_marked_rider_transfer_availability_route());
            route.extend(expected_marked_rider_transfer_route());
        }
        "doTransferMarkedDamageRiderLaterTurn" => {
            route.extend(expected_marked_rider_admission_route());
            route.extend(expected_marked_rider_transfer_availability_route());
            route.push(expected_resolve_without_fill(
                ReducerRouteSubjectFamily::MarkedEffect,
                ReducerRouteOwnerGroup::TurnBoundary,
            ));
            route.extend(expected_marked_rider_transfer_route());
        }
        "doAdmitConditionImmunityTemporaryHitPoints" => {
            route.extend(expected_immunity_temporary_hit_point_admission_route());
        }
        "doProjectConditionImmunity" => {
            route.extend(expected_immunity_temporary_hit_point_admission_route());
            route.extend(expected_condition_immunity_projection_route());
        }
        "doGrantTurnStartTemporaryHitPoints" => {
            route.extend(expected_immunity_temporary_hit_point_admission_route());
            route.extend(expected_turn_start_temporary_hit_point_route());
        }
        "doCleanupConditionImmunityTemporaryHitPoints" => {
            route.extend(expected_immunity_temporary_hit_point_admission_route());
            route.extend(expected_immunity_temporary_hit_point_cleanup_route());
        }
        action => panic!("unsupported expected route mbt::actionTaken {action}"),
    }
    route
}

fn expected_marked_rider_admission_route() -> Vec<ReducerRouteEvent> {
    vec![
        expected_discover(
            ReducerRouteSubjectFamily::MarkedEffect,
            vec![ReducerRouteHoleKind::TargetChoice],
            ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
        ),
        expected_resolve(
            ReducerRouteSubjectFamily::MarkedEffect,
            ReducerRouteFillKind::TargetChoice,
            Vec::new(),
            ReducerRouteResolutionOutcome::Resolved,
            ReducerRouteOwnerGroup::TargetSelection,
        ),
        expected_resolve_without_fill(
            ReducerRouteSubjectFamily::MarkedEffect,
            ReducerRouteOwnerGroup::ActiveEffect,
        ),
        expected_resolve_without_fill(
            ReducerRouteSubjectFamily::MarkedEffect,
            ReducerRouteOwnerGroup::Concentration,
        ),
    ]
}

fn expected_marked_rider_attack_projection_route() -> Vec<ReducerRouteEvent> {
    vec![
        expected_discover(
            ReducerRouteSubjectFamily::MarkedEffect,
            vec![ReducerRouteHoleKind::TargetChoice],
            ReducerRouteOwnerGroup::ActionEconomy,
        ),
        expected_resolve(
            ReducerRouteSubjectFamily::MarkedEffect,
            ReducerRouteFillKind::TargetChoice,
            vec![ReducerRouteHoleKind::AttackRoll],
            ReducerRouteResolutionOutcome::NeedsHoles,
            ReducerRouteOwnerGroup::TargetSelection,
        ),
        expected_resolve(
            ReducerRouteSubjectFamily::MarkedEffect,
            ReducerRouteFillKind::AttackRoll,
            vec![ReducerRouteHoleKind::RolledDice],
            ReducerRouteResolutionOutcome::NeedsHoles,
            ReducerRouteOwnerGroup::AttackRoll,
        ),
        expected_resolve(
            ReducerRouteSubjectFamily::MarkedEffect,
            ReducerRouteFillKind::RolledDice,
            Vec::new(),
            ReducerRouteResolutionOutcome::Resolved,
            ReducerRouteOwnerGroup::HitPoint,
        ),
    ]
}

fn expected_marked_rider_transfer_availability_route() -> Vec<ReducerRouteEvent> {
    vec![
        expected_resolve_without_fill(
            ReducerRouteSubjectFamily::MarkedEffect,
            ReducerRouteOwnerGroup::HitPointAndZeroHpLifecycle,
        ),
        expected_resolve_without_fill(
            ReducerRouteSubjectFamily::MarkedEffect,
            ReducerRouteOwnerGroup::ActiveEffect,
        ),
    ]
}

fn expected_marked_rider_transfer_route() -> Vec<ReducerRouteEvent> {
    vec![
        expected_discover(
            ReducerRouteSubjectFamily::MarkedEffect,
            vec![ReducerRouteHoleKind::TargetChoice],
            ReducerRouteOwnerGroup::ActionEconomy,
        ),
        expected_resolve(
            ReducerRouteSubjectFamily::MarkedEffect,
            ReducerRouteFillKind::TargetChoice,
            Vec::new(),
            ReducerRouteResolutionOutcome::Resolved,
            ReducerRouteOwnerGroup::TargetSelection,
        ),
        expected_resolve_without_fill(
            ReducerRouteSubjectFamily::MarkedEffect,
            ReducerRouteOwnerGroup::ActiveEffect,
        ),
    ]
}

fn expected_immunity_temporary_hit_point_admission_route() -> Vec<ReducerRouteEvent> {
    vec![
        expected_discover(
            ReducerRouteSubjectFamily::ConditionImmunityActiveEffect,
            vec![ReducerRouteHoleKind::TargetChoice],
            ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
        ),
        expected_resolve(
            ReducerRouteSubjectFamily::ConditionImmunityActiveEffect,
            ReducerRouteFillKind::TargetChoice,
            Vec::new(),
            ReducerRouteResolutionOutcome::Resolved,
            ReducerRouteOwnerGroup::TargetSelection,
        ),
        expected_resolve_without_fill(
            ReducerRouteSubjectFamily::ConditionImmunityActiveEffect,
            ReducerRouteOwnerGroup::ActiveEffect,
        ),
        expected_resolve_without_fill(
            ReducerRouteSubjectFamily::ConditionImmunityActiveEffect,
            ReducerRouteOwnerGroup::Concentration,
        ),
    ]
}

fn expected_condition_immunity_projection_route() -> Vec<ReducerRouteEvent> {
    vec![
        expected_resolve_without_fill(
            ReducerRouteSubjectFamily::ConditionImmunityActiveEffect,
            ReducerRouteOwnerGroup::ConditionLifecycle,
        ),
        expected_resolve_without_fill(
            ReducerRouteSubjectFamily::ConditionImmunityActiveEffect,
            ReducerRouteOwnerGroup::ActiveEffect,
        ),
    ]
}

fn expected_turn_start_temporary_hit_point_route() -> Vec<ReducerRouteEvent> {
    vec![
        expected_discover(
            ReducerRouteSubjectFamily::ConditionImmunityActiveEffect,
            Vec::new(),
            ReducerRouteOwnerGroup::TurnBoundary,
        ),
        expected_resolve_without_fill(
            ReducerRouteSubjectFamily::ConditionImmunityActiveEffect,
            ReducerRouteOwnerGroup::TemporaryHitPoint,
        ),
    ]
}

fn expected_immunity_temporary_hit_point_cleanup_route() -> Vec<ReducerRouteEvent> {
    vec![
        expected_resolve_without_fill(
            ReducerRouteSubjectFamily::ConditionImmunityActiveEffect,
            ReducerRouteOwnerGroup::Concentration,
        ),
        expected_resolve_without_fill(
            ReducerRouteSubjectFamily::ConditionImmunityActiveEffect,
            ReducerRouteOwnerGroup::ActiveEffect,
        ),
    ]
}

fn expected_discover(
    subject: ReducerRouteSubjectFamily,
    holes: Vec<ReducerRouteHoleKind>,
    owner: ReducerRouteOwnerGroup,
) -> ReducerRouteEvent {
    route_discover_battle_acts_from_route_holes(subject, holes, owner)
}

fn expected_resolve(
    subject: ReducerRouteSubjectFamily,
    fill: ReducerRouteFillKind,
    holes: Vec<ReducerRouteHoleKind>,
    outcome: ReducerRouteResolutionOutcome,
    owner: ReducerRouteOwnerGroup,
) -> ReducerRouteEvent {
    route_resolve_battle_subject_from_route_result(subject, fill, outcome, holes, owner)
}

fn expected_resolve_without_fill(
    subject: ReducerRouteSubjectFamily,
    owner: ReducerRouteOwnerGroup,
) -> ReducerRouteEvent {
    route_resolve_battle_subject_without_fill_from_route_result(
        subject,
        ReducerRouteResolutionOutcome::Resolved,
        Vec::new(),
        owner,
    )
}

fn expected_facts(action: &str) -> Vec<MarkedDamageImmunityRouteFact> {
    let mut facts = Vec::new();
    match action {
        "doAdmitMarkedDamageRider" => {
            facts.push(MarkedDamageImmunityRouteFact::TransferAwaitsMarkedTargetDrop);
        }
        "doProjectMarkedDamageRiderOnHit" => {
            facts.push(MarkedDamageImmunityRouteFact::TransferAwaitsMarkedTargetDrop);
            facts.push(MarkedDamageImmunityRouteFact::AttackRollHitMarkedTarget);
            facts.push(MarkedDamageImmunityRouteFact::ExtraDamageOnMarkedTargetHit);
        }
        "doOpenMarkedDamageRiderTransferAfterTargetDrop" => {
            facts.push(MarkedDamageImmunityRouteFact::TransferAwaitsMarkedTargetDrop);
            facts.push(MarkedDamageImmunityRouteFact::TransferAvailableAfterMarkedTargetDrops);
        }
        "doTransferMarkedDamageRiderSameTurn" => {
            facts.push(MarkedDamageImmunityRouteFact::TransferAwaitsMarkedTargetDrop);
            facts.push(MarkedDamageImmunityRouteFact::TransferAvailableAfterMarkedTargetDrops);
            facts.push(MarkedDamageImmunityRouteFact::TransferAvailableOnSameTurn);
            facts.push(MarkedDamageImmunityRouteFact::TransferConsumesBonusAction);
            facts.push(MarkedDamageImmunityRouteFact::TransferResetsAwaitingMarkedTargetDrop);
        }
        "doTransferMarkedDamageRiderLaterTurn" => {
            facts.push(MarkedDamageImmunityRouteFact::TransferAwaitsMarkedTargetDrop);
            facts.push(MarkedDamageImmunityRouteFact::TransferAvailableAfterMarkedTargetDrops);
            facts.push(MarkedDamageImmunityRouteFact::TransferAvailableOnLaterTurn);
            facts.push(MarkedDamageImmunityRouteFact::TransferConsumesBonusAction);
            facts.push(MarkedDamageImmunityRouteFact::TransferResetsAwaitingMarkedTargetDrop);
        }
        "doAdmitConditionImmunityTemporaryHitPoints" => {
            facts.extend(immunity_admission_facts());
        }
        "doProjectConditionImmunity" => {
            facts.extend(immunity_admission_facts());
            facts.push(MarkedDamageImmunityRouteFact::ConditionApplicationRejectedByImmunity);
            facts.push(MarkedDamageImmunityRouteFact::BattleConditionLifecycleProjectionOwner);
        }
        "doGrantTurnStartTemporaryHitPoints" => {
            facts.extend(immunity_admission_facts());
            facts.push(MarkedDamageImmunityRouteFact::TurnStartTemporaryHitPointsGranted);
            facts.push(MarkedDamageImmunityRouteFact::TemporaryHitPointsUseNonStackingChoice);
            facts.push(MarkedDamageImmunityRouteFact::BattleTurnBoundaryProjectionOwner);
            facts.push(MarkedDamageImmunityRouteFact::BattleTemporaryHitPointProjectionOwner);
        }
        "doCleanupConditionImmunityTemporaryHitPoints" => {
            facts.extend(immunity_admission_facts());
            facts.push(
                MarkedDamageImmunityRouteFact::ConcentrationCleanupPreventsLaterTurnStartGrant,
            );
            facts.push(MarkedDamageImmunityRouteFact::ImmunityBattleConcentrationCleanupOwner);
            facts.push(MarkedDamageImmunityRouteFact::ImmunityBattleActiveEffectCleanupOwner);
        }
        action => panic!("unsupported route facts action {action}"),
    }
    facts
}

fn immunity_admission_facts() -> Vec<MarkedDamageImmunityRouteFact> {
    vec![
        MarkedDamageImmunityRouteFact::FrightenedConditionImmunity,
        MarkedDamageImmunityRouteFact::ConditionImmunityApplied,
        MarkedDamageImmunityRouteFact::TurnStartTemporaryHitPointGrantScheduled,
        MarkedDamageImmunityRouteFact::BattleActiveEffectProjectionOwner,
    ]
}

fn facts_payload(facts: &[MarkedDamageImmunityRouteFact]) -> String {
    facts.iter().map(fact_ref).collect::<Vec<_>>().join(",")
}

fn fact_ref(fact: &MarkedDamageImmunityRouteFact) -> &'static str {
    match fact {
        MarkedDamageImmunityRouteFact::TransferAwaitsMarkedTargetDrop => {
            "TransferAwaitsMarkedTargetDrop"
        }
        MarkedDamageImmunityRouteFact::AttackRollHitMarkedTarget => "AttackRollHitMarkedTarget",
        MarkedDamageImmunityRouteFact::ExtraDamageOnMarkedTargetHit => {
            "ExtraDamageOnMarkedTargetHit"
        }
        MarkedDamageImmunityRouteFact::TransferAvailableAfterMarkedTargetDrops => {
            "TransferAvailableAfterMarkedTargetDrops"
        }
        MarkedDamageImmunityRouteFact::TransferAvailableOnSameTurn => "TransferAvailableOnSameTurn",
        MarkedDamageImmunityRouteFact::TransferAvailableOnLaterTurn => {
            "TransferAvailableOnLaterTurn"
        }
        MarkedDamageImmunityRouteFact::TransferConsumesBonusAction => "TransferConsumesBonusAction",
        MarkedDamageImmunityRouteFact::TransferResetsAwaitingMarkedTargetDrop => {
            "TransferResetsAwaitingMarkedTargetDrop"
        }
        MarkedDamageImmunityRouteFact::FrightenedConditionImmunity => "FrightenedConditionImmunity",
        MarkedDamageImmunityRouteFact::ConditionImmunityApplied => "ConditionImmunityApplied",
        MarkedDamageImmunityRouteFact::ConditionApplicationRejectedByImmunity => {
            "ConditionApplicationRejectedByImmunity"
        }
        MarkedDamageImmunityRouteFact::TurnStartTemporaryHitPointGrantScheduled => {
            "TurnStartTemporaryHitPointGrantScheduled"
        }
        MarkedDamageImmunityRouteFact::TurnStartTemporaryHitPointsGranted => {
            "TurnStartTemporaryHitPointsGranted"
        }
        MarkedDamageImmunityRouteFact::TemporaryHitPointsUseNonStackingChoice => {
            "TemporaryHitPointsUseNonStackingChoice"
        }
        MarkedDamageImmunityRouteFact::ConcentrationCleanupPreventsLaterTurnStartGrant => {
            "ConcentrationCleanupPreventsLaterTurnStartGrant"
        }
        MarkedDamageImmunityRouteFact::BattleActiveEffectProjectionOwner => {
            "BattleActiveEffectProjectionOwner"
        }
        MarkedDamageImmunityRouteFact::BattleConditionLifecycleProjectionOwner => {
            "BattleConditionLifecycleProjectionOwner"
        }
        MarkedDamageImmunityRouteFact::BattleTemporaryHitPointProjectionOwner => {
            "BattleTemporaryHitPointProjectionOwner"
        }
        MarkedDamageImmunityRouteFact::BattleTurnBoundaryProjectionOwner => {
            "BattleTurnBoundaryProjectionOwner"
        }
        MarkedDamageImmunityRouteFact::ImmunityBattleConcentrationCleanupOwner => {
            "ImmunityBattleConcentrationCleanupOwner"
        }
        MarkedDamageImmunityRouteFact::ImmunityBattleActiveEffectCleanupOwner => {
            "ImmunityBattleActiveEffectCleanupOwner"
        }
    }
}
