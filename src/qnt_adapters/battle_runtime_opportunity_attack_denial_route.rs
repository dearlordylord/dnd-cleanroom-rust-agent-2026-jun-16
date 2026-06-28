use crate::rules::battle_reducer_spine::{
    discover_spell_attack_damage_subject_observed, reaction_interdiction_route_subject_for_target,
    resolve_battle_subject_observed, start_battle_observed, Actor, AttackRollFacts,
    BattleEntrypointTrace, BattleGenericRouteFill, BattleOpportunityAttackDenialEffect,
    BattleResolutionRequest, BattleResolutionResult, BattleSetup, BattleSpellActiveEffectKind,
    BattleSpellAttackFill, BattleState, BattleSubject, BattleSubjectKind,
};

use super::battle_runtime_reducer_route::{
    observed_reducer_route, route_discover_battle_acts, route_resolve_battle_subject,
    route_resolve_battle_subject_without_fill, route_start_battle, ReducerRouteEvent,
    ReducerRouteFillKind, ReducerRouteOwnerGroup, ReducerRouteSubjectFamily,
};

pub const CONNECTOR_ACTIONS: [&str; 4] = [
    "doAdmitAttackHitOpportunityAttackDenialEffect",
    "doProjectOpportunityAttackDenialIntoMovementReactionDiscovery",
    "doExpireActiveDenialAtAffectedTargetTurnStart",
    "doExpireProjectedDenialAtAffectedTargetTurnStart",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReactionInterdictionRouteFact {
    SourceHostAttackHitDamageOutcome,
    CarrierHostOutcomeTargetCarrier,
    ScopeAffectedTargetReactionsOnly,
    TriggerFamilyOpportunityAttack,
    ExpirationUntilAffectedTargetNextTurnStarts,
    Admitted,
    ProjectedToMovementDiscovery,
    ProjectedToReactionDiscovery,
    Expired,
    CleanedUp,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpportunityAttackDenialRouteWitness {
    pub target_hit_points_after_damage: i16,
    pub active_after_attack_hit: bool,
    pub active_after_admission: bool,
    pub projected_to_movement_discovery: bool,
    pub projected_to_reaction_discovery: bool,
    pub active_after_projection: bool,
    pub expired_at_boundary: bool,
    pub active_after_cleanup: bool,
    pub facts: Vec<ReactionInterdictionRouteFact>,
}

pub fn replay_observed_action(observed_action_taken: &str) -> OpportunityAttackDenialRouteWitness {
    replay_observed_state_and_route(observed_action_taken).0
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    replay_observed_state_and_route(observed_action_taken).1
}

pub fn expected_witness(observed_action_taken: &str) -> OpportunityAttackDenialRouteWitness {
    match observed_action_taken {
        "doAdmitAttackHitOpportunityAttackDenialEffect" => admission_witness(),
        "doProjectOpportunityAttackDenialIntoMovementReactionDiscovery" => projected_witness(),
        "doExpireActiveDenialAtAffectedTargetTurnStart" => expired_witness(false),
        "doExpireProjectedDenialAtAffectedTargetTurnStart" => expired_witness(true),
        action => panic!("unsupported route connector action {action}"),
    }
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doAdmitAttackHitOpportunityAttackDenialEffect" => admission_route(),
        "doProjectOpportunityAttackDenialIntoMovementReactionDiscovery" => {
            let mut route = admission_route();
            route.extend(project_route());
            route
        }
        "doExpireActiveDenialAtAffectedTargetTurnStart" => {
            let mut route = admission_route();
            route.extend(expire_and_cleanup_route());
            route
        }
        "doExpireProjectedDenialAtAffectedTargetTurnStart" => {
            let mut route = admission_route();
            route.extend(project_route());
            route.extend(expire_and_cleanup_route());
            route
        }
        action => panic!("unsupported route connector action {action}"),
    }
}

pub fn projection_payload(witness: &OpportunityAttackDenialRouteWitness) -> String {
    [
        format!(
            "targetHitPointsAfterDamage={}",
            witness.target_hit_points_after_damage
        ),
        format!("activeAfterAttackHit={}", witness.active_after_attack_hit),
        format!("activeAfterAdmission={}", witness.active_after_admission),
        format!(
            "projectedToMovementDiscovery={}",
            witness.projected_to_movement_discovery
        ),
        format!(
            "projectedToReactionDiscovery={}",
            witness.projected_to_reaction_discovery
        ),
        format!("activeAfterProjection={}", witness.active_after_projection),
        format!("expiredAtBoundary={}", witness.expired_at_boundary),
        format!("activeAfterCleanup={}", witness.active_after_cleanup),
        format!("facts={}", facts_payload(&witness.facts)),
    ]
    .join("\n")
}

fn replay_observed_state_and_route(
    observed_action_taken: &str,
) -> (OpportunityAttackDenialRouteWitness, Vec<ReducerRouteEvent>) {
    let mut trace = BattleEntrypointTrace::default();
    let state = start_battle_observed(opportunity_attack_denial_setup(), &mut trace).state;
    let state = resolve_denial_spell_attack_hit(state, &mut trace);
    let target_hit_points_after_damage = state.skeleton.hp;
    let active_after_attack_hit = opportunity_attack_denied(&state);

    let state = resolve_reaction_interdiction_without_fill(
        state,
        BattleSubjectKind::ReactionInterdictionActiveEffectAdmission,
        &mut trace,
    );
    let active_after_admission = opportunity_attack_denied(&state);
    let mut facts = admission_facts();

    let (state, projected_to_movement_discovery, projected_to_reaction_discovery) =
        match observed_action_taken {
            "doProjectOpportunityAttackDenialIntoMovementReactionDiscovery"
            | "doExpireProjectedDenialAtAffectedTargetTurnStart" => {
                let state = resolve_reaction_interdiction_without_fill(
                    state,
                    BattleSubjectKind::ReactionInterdictionMovementProjection,
                    &mut trace,
                );
                let state = resolve_reaction_interdiction_without_fill(
                    state,
                    BattleSubjectKind::ReactionInterdictionReactionDiscoveryProjection,
                    &mut trace,
                );
                facts.extend(projection_facts());
                (state, true, true)
            }
            "doAdmitAttackHitOpportunityAttackDenialEffect"
            | "doExpireActiveDenialAtAffectedTargetTurnStart" => (state, false, false),
            action => panic!("unsupported route connector action {action}"),
        };
    let active_after_projection = opportunity_attack_denied(&state);

    let (state, expired_at_boundary) = match observed_action_taken {
        "doExpireActiveDenialAtAffectedTargetTurnStart"
        | "doExpireProjectedDenialAtAffectedTargetTurnStart" => {
            let state = resolve_reaction_interdiction_without_fill(
                state,
                BattleSubjectKind::ReactionInterdictionDurationExpiry,
                &mut trace,
            );
            let expired = opportunity_attack_denial_effect(&state)
                == BattleOpportunityAttackDenialEffect::Expired;
            let state = resolve_reaction_interdiction_without_fill(
                state,
                BattleSubjectKind::ReactionInterdictionActiveEffectCleanup,
                &mut trace,
            );
            facts.extend(expiry_facts());
            (state, expired)
        }
        "doAdmitAttackHitOpportunityAttackDenialEffect"
        | "doProjectOpportunityAttackDenialIntoMovementReactionDiscovery" => (state, false),
        action => panic!("unsupported route connector action {action}"),
    };

    (
        OpportunityAttackDenialRouteWitness {
            target_hit_points_after_damage,
            active_after_attack_hit,
            active_after_admission,
            projected_to_movement_discovery,
            projected_to_reaction_discovery,
            active_after_projection,
            expired_at_boundary,
            active_after_cleanup: opportunity_attack_denied(&state),
            facts,
        },
        observed_reducer_route(
            &trace,
            &[
                ReducerRouteSubjectFamily::SpellAttack,
                ReducerRouteSubjectFamily::ReactionInterdiction,
            ],
        ),
    )
}

fn resolve_denial_spell_attack_hit(
    state: BattleState,
    trace: &mut BattleEntrypointTrace,
) -> BattleState {
    let (state, subject) = discover_spell_attack_damage_subject_observed(
        state,
        false,
        BattleSpellActiveEffectKind::OpportunityAttackDenied,
        trace,
    );
    let result = resolve_battle_subject_observed(
        state,
        BattleResolutionRequest::spell_attack(
            subject,
            BattleSpellAttackFill::TargetChoice(Actor::Skeleton),
        )
        .expect("opportunity-attack-denial spell attack should accept target choice"),
        trace,
    );
    let (state, subject) = continuation(result, "target choice");
    let result = resolve_battle_subject_observed(
        state,
        BattleResolutionRequest::spell_attack(
            subject,
            BattleSpellAttackFill::AttackRoll(AttackRollFacts {
                total: 20,
                natural_d20: 15,
            }),
        )
        .expect("opportunity-attack-denial spell attack should accept attack roll"),
        trace,
    );
    let (state, subject) = continuation(result, "attack roll");
    let result = resolve_battle_subject_observed(
        state,
        BattleResolutionRequest::spell_attack(subject, BattleSpellAttackFill::DamageRoll(4))
            .expect("opportunity-attack-denial spell attack should accept damage roll"),
        trace,
    );
    resolved_state(result, "damage roll")
}

fn resolve_reaction_interdiction_without_fill(
    state: BattleState,
    kind: BattleSubjectKind,
    trace: &mut BattleEntrypointTrace,
) -> BattleState {
    let subject = reaction_interdiction_route_subject_for_target(&state, kind, Actor::Skeleton);
    let result = resolve_battle_subject_observed(
        state,
        BattleResolutionRequest::generic_route(subject, BattleGenericRouteFill::WithoutFill)
            .expect("reaction-interdiction route subject should accept without-fill"),
        trace,
    );
    resolved_state(result, "reaction-interdiction route")
}

fn opportunity_attack_denial_setup() -> BattleSetup {
    let mut setup = BattleSetup::standard();
    setup.initiative.still_to_act.waiting = vec![Actor::Skeleton];
    setup.skeleton.hp = 12;
    setup.skeleton.max_hp = 12;
    setup.skeleton.armor_class = 10;
    setup
}

fn continuation(result: BattleResolutionResult, context: &str) -> (BattleState, BattleSubject) {
    result
        .into_needs_holes()
        .map(|needs| (needs.state, needs.subject))
        .unwrap_or_else(|| panic!("{context} should continue"))
}

fn resolved_state(result: BattleResolutionResult, context: &str) -> BattleState {
    let BattleResolutionResult::Resolved { state } = result else {
        panic!("{context} should resolve")
    };
    state
}

fn opportunity_attack_denied(state: &BattleState) -> bool {
    opportunity_attack_denial_effect(state).prevents_opportunity_attacks()
}

fn opportunity_attack_denial_effect(state: &BattleState) -> BattleOpportunityAttackDenialEffect {
    state
        .skeleton
        .spell_active_effects
        .opportunity_attack_denied
}

fn admission_route() -> Vec<ReducerRouteEvent> {
    vec![
        route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
        route_discover_battle_acts(
            ReducerRouteSubjectFamily::SpellAttack,
            vec![crate::rules::battle_reducer_spine::BattleHoleKind::TargetChoice],
            ReducerRouteOwnerGroup::ActionEconomy,
        ),
        route_resolve_battle_subject(
            ReducerRouteSubjectFamily::SpellAttack,
            ReducerRouteFillKind::TargetChoice,
            vec![crate::rules::battle_reducer_spine::BattleHoleKind::AttackRoll],
            ReducerRouteOwnerGroup::TargetSelection,
        ),
        route_resolve_battle_subject(
            ReducerRouteSubjectFamily::SpellAttack,
            ReducerRouteFillKind::AttackRoll,
            vec![crate::rules::battle_reducer_spine::BattleHoleKind::RolledDice],
            ReducerRouteOwnerGroup::AttackRoll,
        ),
        route_resolve_battle_subject(
            ReducerRouteSubjectFamily::SpellAttack,
            ReducerRouteFillKind::RolledDice,
            Vec::new(),
            ReducerRouteOwnerGroup::HitPoint,
        ),
        route_resolve_battle_subject_without_fill(
            ReducerRouteSubjectFamily::ReactionInterdiction,
            Vec::new(),
            ReducerRouteOwnerGroup::ActiveEffect,
        ),
    ]
}

fn project_route() -> Vec<ReducerRouteEvent> {
    vec![
        route_resolve_battle_subject_without_fill(
            ReducerRouteSubjectFamily::ReactionInterdiction,
            Vec::new(),
            ReducerRouteOwnerGroup::MovementResource,
        ),
        route_resolve_battle_subject_without_fill(
            ReducerRouteSubjectFamily::ReactionInterdiction,
            Vec::new(),
            ReducerRouteOwnerGroup::InterruptStack,
        ),
    ]
}

fn expire_and_cleanup_route() -> Vec<ReducerRouteEvent> {
    vec![
        route_resolve_battle_subject_without_fill(
            ReducerRouteSubjectFamily::ReactionInterdiction,
            Vec::new(),
            ReducerRouteOwnerGroup::TurnBoundary,
        ),
        route_resolve_battle_subject_without_fill(
            ReducerRouteSubjectFamily::ReactionInterdiction,
            Vec::new(),
            ReducerRouteOwnerGroup::ActiveEffect,
        ),
    ]
}

fn admission_facts() -> Vec<ReactionInterdictionRouteFact> {
    vec![
        ReactionInterdictionRouteFact::SourceHostAttackHitDamageOutcome,
        ReactionInterdictionRouteFact::CarrierHostOutcomeTargetCarrier,
        ReactionInterdictionRouteFact::ScopeAffectedTargetReactionsOnly,
        ReactionInterdictionRouteFact::TriggerFamilyOpportunityAttack,
        ReactionInterdictionRouteFact::ExpirationUntilAffectedTargetNextTurnStarts,
        ReactionInterdictionRouteFact::Admitted,
    ]
}

fn projection_facts() -> Vec<ReactionInterdictionRouteFact> {
    vec![
        ReactionInterdictionRouteFact::ProjectedToMovementDiscovery,
        ReactionInterdictionRouteFact::ProjectedToReactionDiscovery,
    ]
}

fn expiry_facts() -> Vec<ReactionInterdictionRouteFact> {
    vec![
        ReactionInterdictionRouteFact::Expired,
        ReactionInterdictionRouteFact::CleanedUp,
    ]
}

fn admission_witness() -> OpportunityAttackDenialRouteWitness {
    OpportunityAttackDenialRouteWitness {
        target_hit_points_after_damage: 8,
        active_after_attack_hit: true,
        active_after_admission: true,
        projected_to_movement_discovery: false,
        projected_to_reaction_discovery: false,
        active_after_projection: true,
        expired_at_boundary: false,
        active_after_cleanup: true,
        facts: admission_facts(),
    }
}

fn projected_witness() -> OpportunityAttackDenialRouteWitness {
    let mut witness = admission_witness();
    witness.projected_to_movement_discovery = true;
    witness.projected_to_reaction_discovery = true;
    witness.facts.extend(projection_facts());
    witness
}

fn expired_witness(projected: bool) -> OpportunityAttackDenialRouteWitness {
    let mut witness = if projected {
        projected_witness()
    } else {
        admission_witness()
    };
    witness.expired_at_boundary = true;
    witness.active_after_cleanup = false;
    witness.facts.extend(expiry_facts());
    witness
}

fn facts_payload(facts: &[ReactionInterdictionRouteFact]) -> String {
    facts.iter().map(fact_ref).collect::<Vec<_>>().join(",")
}

fn fact_ref(fact: &ReactionInterdictionRouteFact) -> &'static str {
    match fact {
        ReactionInterdictionRouteFact::SourceHostAttackHitDamageOutcome => {
            "RouteReactionInterdictionSource.HostAttackHitDamageOutcome"
        }
        ReactionInterdictionRouteFact::CarrierHostOutcomeTargetCarrier => {
            "RouteReactionInterdictionCarrier.HostOutcomeTargetCarrier"
        }
        ReactionInterdictionRouteFact::ScopeAffectedTargetReactionsOnly => {
            "RouteReactionInterdictionScope.AffectedTargetReactionsOnly"
        }
        ReactionInterdictionRouteFact::TriggerFamilyOpportunityAttack => {
            "RouteReactionInterdictionTriggerFamily.OpportunityAttackTriggerFamily"
        }
        ReactionInterdictionRouteFact::ExpirationUntilAffectedTargetNextTurnStarts => {
            "RouteReactionInterdictionExpirationBoundary.UntilAffectedTargetNextTurnStarts"
        }
        ReactionInterdictionRouteFact::Admitted => "RouteReactionInterdictionAdmitted",
        ReactionInterdictionRouteFact::ProjectedToMovementDiscovery => {
            "RouteReactionInterdictionProjectedToMovementDiscovery"
        }
        ReactionInterdictionRouteFact::ProjectedToReactionDiscovery => {
            "RouteReactionInterdictionProjectedToReactionDiscovery"
        }
        ReactionInterdictionRouteFact::Expired => "RouteReactionInterdictionExpired",
        ReactionInterdictionRouteFact::CleanedUp => "RouteReactionInterdictionCleanedUp",
    }
}
