use super::battle_runtime_reducer_route::{
    observed_reducer_route, route_discover_battle_acts, route_resolve_battle_subject,
    route_resolve_battle_subject_without_fill_from_route_result, route_start_battle,
    ReducerRouteEvent, ReducerRouteFillKind, ReducerRouteHoleKind, ReducerRouteOwnerGroup,
    ReducerRouteResolutionOutcome, ReducerRouteSubjectFamily,
};
use crate::rules::battle_reducer_spine::{
    discover_generic_route_subject_observed, generic_route_subject_from_battle,
    resolve_battle_subject_observed, start_battle_observed, BattleGenericRouteFill, BattleHoleKind,
    BattleResolutionInvalidReason, BattleResolutionRequest, BattleSetup, BattleSubjectKind,
};
use crate::rules::spell_shapes::{
    eldritch_blast_initial_state, fill_eldritch_blast_attack, fill_eldritch_blast_damage,
    fill_eldritch_blast_targets, reject_eldritch_blast_stale_after_resolved,
    EldritchBlastAttackFacts, EldritchBlastDamageFacts, EldritchBlastInvalidReason,
    EldritchBlastProtocol, EldritchBlastState,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EldritchBlastWitness {
    pub action_available: bool,
    pub target_hp: i16,
    pub resolved_beams: i16,
    pub protocol_result: &'static str,
    pub protocol_invalid_reason: &'static str,
    pub protocol_holes: Vec<&'static str>,
}

pub const BRANCH_ACTIONS: [&str; 8] = [
    "doFillFirstAttackHit",
    "doFillFirstAttackMiss",
    "doFillFirstDamageLow",
    "doFillSecondAttackHit",
    "doFillSecondAttackMiss",
    "doFillSecondDamageLow",
    "doFillTwoCreatureTargets",
    "doRejectStaleAfterResolved",
];

pub fn replay_observed_action(observed_action_taken: &str) -> EldritchBlastWitness {
    match observed_action_taken {
        "doFillFirstAttackHit" => witness_from_state(first_attack_hit()),
        "doFillFirstAttackMiss" => witness_from_state(first_attack_miss()),
        "doFillFirstDamageLow" => witness_from_state(first_damage_low()),
        "doFillSecondAttackHit" => witness_from_state(second_attack_hit()),
        "doFillSecondAttackMiss" => witness_from_state(second_attack_miss()),
        "doFillSecondDamageLow" => witness_from_state(second_damage_low()),
        "doFillTwoCreatureTargets" => witness_from_state(two_creature_targets()),
        "doRejectStaleAfterResolved" => witness_from_state(reject_stale_after_resolved()),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> EldritchBlastWitness {
    replay_observed_action(observed_action_taken)
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    let mut trace = crate::rules::battle_reducer_spine::BattleEntrypointTrace::default();
    let state = start_battle_observed(BattleSetup::standard(), &mut trace).state;
    let (state, _) = discover_generic_route_subject_observed(
        state,
        BattleSubjectKind::IndependentSpellAttackSequenceTargetChoice,
        &mut trace,
    );
    let (mut state, _) = discover_generic_route_subject_observed(
        state,
        BattleSubjectKind::IndependentSpellAttackSequenceObjectBoundary,
        &mut trace,
    );
    for (kind, fill) in route_fills_for_action(observed_action_taken) {
        let subject = generic_route_subject_from_battle(&state, kind);
        state = resolve_battle_subject_observed(
            state,
            BattleResolutionRequest::generic_route(subject, fill)
                .expect("independent spell attack route subject accepts generic fills"),
            &mut trace,
        )
        .into_state();
    }
    observed_reducer_route(&trace, &[ReducerRouteSubjectFamily::SpellAttack])
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    let mut route = vec![route_start_battle(ReducerRouteOwnerGroup::ActionEconomy)];
    route.push(route_discover_battle_acts(
        ReducerRouteSubjectFamily::SpellAttack,
        vec![BattleHoleKind::TargetChoice],
        ReducerRouteOwnerGroup::ActionEconomy,
    ));
    route.push(route_discover_battle_acts(
        ReducerRouteSubjectFamily::SpellAttack,
        Vec::new(),
        ReducerRouteOwnerGroup::ObjectBoundary,
    ));
    for (kind, fill) in route_fills_for_action(observed_action_taken) {
        append_expected_route_fill(&mut route, kind, fill);
    }
    route
}

pub fn projection_payload(witness: &EldritchBlastWitness) -> String {
    [
        format!("qActionAvailable={}", witness.action_available),
        format!("qTargetHp={}", witness.target_hp),
        format!("qResolvedBeams={}", witness.resolved_beams),
        format!("protocolResult={}", witness.protocol_result),
        format!("protocolInvalidReason={}", witness.protocol_invalid_reason),
        format!("protocolHoles={}", joined_or_none(&witness.protocol_holes)),
    ]
    .join("\n")
}

fn route_fills_for_action(
    observed_action_taken: &str,
) -> Vec<(BattleSubjectKind, BattleGenericRouteFill)> {
    use BattleGenericRouteFill::{AttackRoll, RolledDice, TargetChoice, WithoutFill};
    use BattleSubjectKind::{
        IndependentSpellAttackSequenceAttackContinues, IndependentSpellAttackSequenceAttackHit,
        IndependentSpellAttackSequenceAttackResolved,
        IndependentSpellAttackSequenceDamageContinues,
        IndependentSpellAttackSequenceDamageResolved, IndependentSpellAttackSequenceStaleReplay,
        IndependentSpellAttackSequenceTargetChoice,
    };

    match observed_action_taken {
        "doFillTwoCreatureTargets" => {
            vec![(IndependentSpellAttackSequenceTargetChoice, TargetChoice)]
        }
        "doFillFirstAttackMiss" => vec![
            (IndependentSpellAttackSequenceTargetChoice, TargetChoice),
            (IndependentSpellAttackSequenceAttackContinues, AttackRoll),
        ],
        "doFillFirstAttackHit" => vec![
            (IndependentSpellAttackSequenceTargetChoice, TargetChoice),
            (IndependentSpellAttackSequenceAttackHit, AttackRoll),
        ],
        "doFillFirstDamageLow" => vec![
            (IndependentSpellAttackSequenceTargetChoice, TargetChoice),
            (IndependentSpellAttackSequenceAttackHit, AttackRoll),
            (IndependentSpellAttackSequenceDamageContinues, RolledDice),
        ],
        "doFillSecondAttackMiss" => vec![
            (IndependentSpellAttackSequenceTargetChoice, TargetChoice),
            (IndependentSpellAttackSequenceAttackHit, AttackRoll),
            (IndependentSpellAttackSequenceDamageContinues, RolledDice),
            (IndependentSpellAttackSequenceAttackResolved, AttackRoll),
        ],
        "doFillSecondAttackHit" => vec![
            (IndependentSpellAttackSequenceTargetChoice, TargetChoice),
            (IndependentSpellAttackSequenceAttackHit, AttackRoll),
            (IndependentSpellAttackSequenceDamageContinues, RolledDice),
            (IndependentSpellAttackSequenceAttackHit, AttackRoll),
        ],
        "doFillSecondDamageLow" => vec![
            (IndependentSpellAttackSequenceTargetChoice, TargetChoice),
            (IndependentSpellAttackSequenceAttackHit, AttackRoll),
            (IndependentSpellAttackSequenceDamageContinues, RolledDice),
            (IndependentSpellAttackSequenceAttackHit, AttackRoll),
            (IndependentSpellAttackSequenceDamageResolved, RolledDice),
        ],
        "doRejectStaleAfterResolved" => vec![
            (IndependentSpellAttackSequenceTargetChoice, TargetChoice),
            (IndependentSpellAttackSequenceAttackHit, AttackRoll),
            (IndependentSpellAttackSequenceDamageContinues, RolledDice),
            (IndependentSpellAttackSequenceAttackHit, AttackRoll),
            (IndependentSpellAttackSequenceDamageResolved, RolledDice),
            (IndependentSpellAttackSequenceStaleReplay, WithoutFill),
        ],
        action => panic!("unsupported route mbt::actionTaken {action}"),
    }
}

fn append_expected_route_fill(
    route: &mut Vec<ReducerRouteEvent>,
    kind: BattleSubjectKind,
    fill: BattleGenericRouteFill,
) {
    let (fill_kind, holes, owners) = expected_route_fill(kind, fill);
    match fill_kind {
        Some(fill_kind) => {
            for owner in owners {
                route.push(route_resolve_battle_subject(
                    ReducerRouteSubjectFamily::SpellAttack,
                    fill_kind,
                    holes.clone(),
                    owner,
                ));
            }
        }
        None => route.push(route_resolve_battle_subject_without_fill_from_route_result(
            ReducerRouteSubjectFamily::SpellAttack,
            ReducerRouteResolutionOutcome::Invalid(BattleResolutionInvalidReason::StaleSubject),
            Vec::<ReducerRouteHoleKind>::new(),
            ReducerRouteOwnerGroup::HoleFrontier,
        )),
    }
}

fn expected_route_fill(
    kind: BattleSubjectKind,
    fill: BattleGenericRouteFill,
) -> (
    Option<ReducerRouteFillKind>,
    Vec<BattleHoleKind>,
    Vec<ReducerRouteOwnerGroup>,
) {
    use BattleGenericRouteFill::{AttackRoll, RolledDice, TargetChoice, WithoutFill};
    use BattleSubjectKind::{
        IndependentSpellAttackSequenceAttackContinues, IndependentSpellAttackSequenceAttackHit,
        IndependentSpellAttackSequenceAttackResolved,
        IndependentSpellAttackSequenceDamageContinues,
        IndependentSpellAttackSequenceDamageResolved, IndependentSpellAttackSequenceStaleReplay,
        IndependentSpellAttackSequenceTargetChoice,
    };

    match (kind, fill) {
        (IndependentSpellAttackSequenceTargetChoice, TargetChoice) => (
            Some(ReducerRouteFillKind::TargetChoice),
            vec![BattleHoleKind::AttackRoll],
            vec![
                ReducerRouteOwnerGroup::TargetSelection,
                ReducerRouteOwnerGroup::SpellAttackProcedure,
            ],
        ),
        (IndependentSpellAttackSequenceAttackHit, AttackRoll) => (
            Some(ReducerRouteFillKind::AttackRoll),
            vec![BattleHoleKind::RolledDice],
            vec![
                ReducerRouteOwnerGroup::AttackRoll,
                ReducerRouteOwnerGroup::SpellAttackProcedure,
            ],
        ),
        (IndependentSpellAttackSequenceAttackContinues, AttackRoll) => (
            Some(ReducerRouteFillKind::AttackRoll),
            vec![BattleHoleKind::AttackRoll],
            vec![
                ReducerRouteOwnerGroup::AttackRoll,
                ReducerRouteOwnerGroup::SpellAttackProcedure,
            ],
        ),
        (IndependentSpellAttackSequenceAttackResolved, AttackRoll) => (
            Some(ReducerRouteFillKind::AttackRoll),
            Vec::new(),
            vec![
                ReducerRouteOwnerGroup::AttackRoll,
                ReducerRouteOwnerGroup::SpellAttackProcedure,
            ],
        ),
        (IndependentSpellAttackSequenceDamageContinues, RolledDice) => (
            Some(ReducerRouteFillKind::RolledDice),
            vec![BattleHoleKind::AttackRoll],
            vec![
                ReducerRouteOwnerGroup::HitPoint,
                ReducerRouteOwnerGroup::SpellAttackProcedure,
            ],
        ),
        (IndependentSpellAttackSequenceDamageResolved, RolledDice) => (
            Some(ReducerRouteFillKind::RolledDice),
            Vec::new(),
            vec![
                ReducerRouteOwnerGroup::HitPoint,
                ReducerRouteOwnerGroup::SpellAttackProcedure,
            ],
        ),
        (IndependentSpellAttackSequenceStaleReplay, WithoutFill) => {
            (None, Vec::new(), vec![ReducerRouteOwnerGroup::HoleFrontier])
        }
        _ => panic!("unsupported independent spell attack route fill"),
    }
}

fn two_creature_targets() -> EldritchBlastState {
    fill_eldritch_blast_targets(eldritch_blast_initial_state())
}

fn first_attack_miss() -> EldritchBlastState {
    fill_eldritch_blast_attack(
        two_creature_targets(),
        EldritchBlastAttackFacts { hit: false },
    )
}

fn first_attack_hit() -> EldritchBlastState {
    fill_eldritch_blast_attack(
        two_creature_targets(),
        EldritchBlastAttackFacts { hit: true },
    )
}

fn first_damage_low() -> EldritchBlastState {
    fill_eldritch_blast_damage(
        first_attack_hit(),
        EldritchBlastDamageFacts { damage_roll: 4 },
    )
}

fn second_attack_miss() -> EldritchBlastState {
    fill_eldritch_blast_attack(first_damage_low(), EldritchBlastAttackFacts { hit: false })
}

fn second_attack_hit() -> EldritchBlastState {
    fill_eldritch_blast_attack(first_damage_low(), EldritchBlastAttackFacts { hit: true })
}

fn second_damage_low() -> EldritchBlastState {
    fill_eldritch_blast_damage(
        second_attack_hit(),
        EldritchBlastDamageFacts { damage_roll: 4 },
    )
}

fn reject_stale_after_resolved() -> EldritchBlastState {
    reject_eldritch_blast_stale_after_resolved(second_damage_low())
}

fn witness_from_state(state: EldritchBlastState) -> EldritchBlastWitness {
    EldritchBlastWitness {
        action_available: state.action_available,
        target_hp: state.target_hit_points,
        resolved_beams: state.resolved_beams,
        protocol_result: protocol_result_ref(state.protocol),
        protocol_invalid_reason: protocol_invalid_reason_ref(state.protocol),
        protocol_holes: protocol_holes(state.protocol),
    }
}

fn protocol_result_ref(protocol: EldritchBlastProtocol) -> &'static str {
    match protocol {
        EldritchBlastProtocol::NeedsTargets | EldritchBlastProtocol::NeedsAttackRoll => {
            "needsHoles"
        }
        EldritchBlastProtocol::NeedsDamageRoll => "needsHoles",
        EldritchBlastProtocol::Resolved => "resolved",
        EldritchBlastProtocol::Invalid(_) => "invalid",
    }
}

fn protocol_invalid_reason_ref(protocol: EldritchBlastProtocol) -> &'static str {
    match protocol {
        EldritchBlastProtocol::Invalid(EldritchBlastInvalidReason::StaleSubject) => "WStaleSubject",
        EldritchBlastProtocol::Invalid(EldritchBlastInvalidReason::InvalidFill) => "WInvalidFill",
        EldritchBlastProtocol::NeedsTargets
        | EldritchBlastProtocol::NeedsAttackRoll
        | EldritchBlastProtocol::NeedsDamageRoll
        | EldritchBlastProtocol::Resolved => "",
    }
}

fn protocol_holes(protocol: EldritchBlastProtocol) -> Vec<&'static str> {
    match protocol {
        EldritchBlastProtocol::NeedsTargets => vec!["ObjectTargetChoice", "TargetChoice"],
        EldritchBlastProtocol::NeedsAttackRoll => vec!["AttackRoll"],
        EldritchBlastProtocol::NeedsDamageRoll => vec!["SpellDamageRoll"],
        EldritchBlastProtocol::Resolved | EldritchBlastProtocol::Invalid(_) => vec![],
    }
}

fn joined_or_none(values: &[&'static str]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}
