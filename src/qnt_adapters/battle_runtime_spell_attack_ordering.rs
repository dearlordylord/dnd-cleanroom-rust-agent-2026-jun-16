use crate::rules::battle_reducer_spine::{
    discover_battle_acts, discover_single_target_spell_attack_battle,
    discover_typed_spell_attack_battle, resolve_battle_subject,
    spell_attack_ordering_projection_from_battle, start_spell_attack_ordering_battle, Actor,
    AttackRollFacts, BattleHoleKind, BattleResolutionRequest, BattleResolutionResult,
    BattleSpellAttackFill, BattleState, BattleSubject, BattleSubjectKind,
};
use crate::rules::spell_attack_ordering::{
    discover_single_target_spell_attack, discover_typed_spell_attack, fill_attack_roll_hit,
    fill_attack_roll_miss, fill_damage_dice, fill_damage_type_after_target_choice,
    fill_damage_type_before_target_choice, fill_target_choice,
    fill_target_choice_after_damage_type, fill_target_choice_before_damage_type,
    submit_attack_roll_before_target_choice, submit_damage_before_attack_roll,
    SpellAttackFillOrderingError, SpellAttackFrontierStage, SpellAttackHoleKind,
    SpellAttackOrderingProtocol, SpellAttackOrderingState,
};

use super::battle_runtime_reducer_route::{
    battle_resolution_continuation, route_discover_battle_acts,
    route_resolve_battle_subject_from_result, route_start_battle, ReducerRouteEvent,
    ReducerRouteFillKind, ReducerRouteOwnerGroup, ReducerRouteResolveConnector,
    ReducerRouteResolveFill, ReducerRouteSubjectFamily,
};

pub const BRANCH_ACTIONS: [&str; 12] = [
    "doDiscoverSingleTargetSpellAttack",
    "doSubmitAttackRollBeforeTargetChoice",
    "doFillTargetChoice",
    "doSubmitDamageBeforeAttackRoll",
    "doFillAttackRollMiss",
    "doFillAttackRollHit",
    "doFillDamageDice",
    "doDiscoverTypedSpellAttack",
    "doFillDamageTypeBeforeTargetChoice",
    "doFillTargetChoiceBeforeDamageType",
    "doFillDamageTypeAfterTargetChoice",
    "doFillTargetChoiceAfterDamageType",
];

pub fn replay_observed_action(observed_action_taken: &str) -> SpellAttackOrderingState {
    spell_attack_ordering_projection_from_battle(&replay_observed_battle_state(
        observed_action_taken,
    ))
}

pub fn replay_observed_battle_state(observed_action_taken: &str) -> BattleState {
    match observed_action_taken {
        "doDiscoverSingleTargetSpellAttack" => single_target_discovered(),
        "doSubmitAttackRollBeforeTargetChoice" => resolve_spell_attack_fill_state(
            single_target_discovered_route().0,
            single_target_discovered_route().1,
            BattleSpellAttackFill::AttackRoll(hit_attack_roll()),
        ),
        "doFillTargetChoice" => resolve_spell_attack_fill_state(
            single_target_discovered_route().0,
            single_target_discovered_route().1,
            target_choice(),
        ),
        "doSubmitDamageBeforeAttackRoll" => resolve_spell_attack_fill_state(
            single_target_target_choice_route().0,
            single_target_target_choice_route().1,
            BattleSpellAttackFill::DamageRoll(4),
        ),
        "doFillAttackRollMiss" => resolve_spell_attack_fill_state(
            single_target_target_choice_route().0,
            single_target_target_choice_route().1,
            BattleSpellAttackFill::AttackRoll(miss_attack_roll()),
        ),
        "doFillAttackRollHit" => resolve_spell_attack_fill_state(
            single_target_target_choice_route().0,
            single_target_target_choice_route().1,
            BattleSpellAttackFill::AttackRoll(hit_attack_roll()),
        ),
        "doFillDamageDice" => resolve_spell_attack_fill_state(
            single_target_attack_hit_route().0,
            single_target_attack_hit_route().1,
            BattleSpellAttackFill::DamageRoll(4),
        ),
        "doDiscoverTypedSpellAttack" => typed_spell_attack_discovered(),
        "doFillDamageTypeBeforeTargetChoice" => resolve_spell_attack_fill_state(
            typed_spell_attack_discovered_route().0,
            typed_spell_attack_discovered_route().1,
            BattleSpellAttackFill::DamageTypeChoice,
        ),
        "doFillTargetChoiceBeforeDamageType" => resolve_spell_attack_fill_state(
            typed_spell_attack_discovered_route().0,
            typed_spell_attack_discovered_route().1,
            target_choice(),
        ),
        "doFillDamageTypeAfterTargetChoice" => resolve_spell_attack_fill_state(
            typed_target_choice_route().0,
            typed_target_choice_route().1,
            BattleSpellAttackFill::DamageTypeChoice,
        ),
        "doFillTargetChoiceAfterDamageType" => resolve_spell_attack_fill_state(
            typed_damage_type_route().0,
            typed_damage_type_route().1,
            target_choice(),
        ),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> SpellAttackOrderingState {
    match observed_action_taken {
        "doDiscoverSingleTargetSpellAttack" => discover_single_target_spell_attack(),
        "doSubmitAttackRollBeforeTargetChoice" => submit_attack_roll_before_target_choice(),
        "doFillTargetChoice" => fill_target_choice(),
        "doSubmitDamageBeforeAttackRoll" => submit_damage_before_attack_roll(),
        "doFillAttackRollMiss" => fill_attack_roll_miss(),
        "doFillAttackRollHit" => fill_attack_roll_hit(),
        "doFillDamageDice" => fill_damage_dice(),
        "doDiscoverTypedSpellAttack" => discover_typed_spell_attack(),
        "doFillDamageTypeBeforeTargetChoice" => fill_damage_type_before_target_choice(),
        "doFillTargetChoiceBeforeDamageType" => fill_target_choice_before_damage_type(),
        "doFillDamageTypeAfterTargetChoice" => fill_damage_type_after_target_choice(),
        "doFillTargetChoiceAfterDamageType" => fill_target_choice_after_damage_type(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doDiscoverSingleTargetSpellAttack" => single_target_discovery_route().2,
        "doSubmitAttackRollBeforeTargetChoice" => {
            let (state, subject, route) = single_target_discovery_route();
            let (_result, route) = resolve_spell_attack_route(
                state,
                subject,
                BattleSpellAttackFill::AttackRoll(hit_attack_roll()),
                ReducerRouteFillKind::AttackRoll,
                ReducerRouteOwnerGroup::HoleFrontier,
                route,
            );
            route
        }
        "doFillTargetChoice" => {
            let (_state, _subject, route) = single_target_target_choice_route();
            route
        }
        "doSubmitDamageBeforeAttackRoll" => {
            let (state, subject, route) = single_target_target_choice_route();
            let (_result, route) = resolve_spell_attack_route(
                state,
                subject,
                BattleSpellAttackFill::DamageRoll(4),
                ReducerRouteFillKind::RolledDice,
                ReducerRouteOwnerGroup::HoleFrontier,
                route,
            );
            route
        }
        "doFillAttackRollMiss" => {
            let (state, subject, route) = single_target_target_choice_route();
            let (_result, route) = resolve_spell_attack_route(
                state,
                subject,
                BattleSpellAttackFill::AttackRoll(miss_attack_roll()),
                ReducerRouteFillKind::AttackRoll,
                ReducerRouteOwnerGroup::AttackRoll,
                route,
            );
            route
        }
        "doFillAttackRollHit" => {
            let (_state, _subject, route) = single_target_attack_hit_route();
            route
        }
        "doFillDamageDice" => {
            let (state, subject, route) = single_target_attack_hit_route();
            let (_result, route) = resolve_spell_attack_route(
                state,
                subject,
                BattleSpellAttackFill::DamageRoll(4),
                ReducerRouteFillKind::RolledDice,
                ReducerRouteOwnerGroup::HitPoint,
                route,
            );
            route
        }
        "doDiscoverTypedSpellAttack" => typed_spell_attack_discovery_route().2,
        "doFillDamageTypeBeforeTargetChoice" => {
            let (state, subject, route) = typed_spell_attack_discovery_route();
            let (_result, route) = resolve_spell_attack_route(
                state,
                subject,
                BattleSpellAttackFill::DamageTypeChoice,
                ReducerRouteFillKind::DamageTypeChoice,
                ReducerRouteOwnerGroup::HoleFrontier,
                route,
            );
            route
        }
        "doFillTargetChoiceBeforeDamageType" => {
            let (state, subject, route) = typed_spell_attack_discovery_route();
            let (_result, route) = resolve_spell_attack_route(
                state,
                subject,
                target_choice(),
                ReducerRouteFillKind::TargetChoice,
                ReducerRouteOwnerGroup::TargetSelection,
                route,
            );
            route
        }
        "doFillDamageTypeAfterTargetChoice" => {
            let (state, subject, route) = typed_target_choice_route();
            let (_result, route) = resolve_spell_attack_route(
                state,
                subject,
                BattleSpellAttackFill::DamageTypeChoice,
                ReducerRouteFillKind::DamageTypeChoice,
                ReducerRouteOwnerGroup::HoleFrontier,
                route,
            );
            route
        }
        "doFillTargetChoiceAfterDamageType" => {
            let (state, subject, route) = typed_damage_type_route();
            let (_result, route) = resolve_spell_attack_route(
                state,
                subject,
                target_choice(),
                ReducerRouteFillKind::TargetChoice,
                ReducerRouteOwnerGroup::TargetSelection,
                route,
            );
            route
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doDiscoverSingleTargetSpellAttack" => {
            expected_spell_discovery_route(vec![BattleHoleKind::TargetChoice])
        }
        "doSubmitAttackRollBeforeTargetChoice" => expected_single_target_route(&[(
            ReducerRouteFillKind::AttackRoll,
            vec![BattleHoleKind::TargetChoice],
            ReducerRouteOwnerGroup::HoleFrontier,
        )]),
        "doFillTargetChoice" => expected_single_target_route(&[(
            ReducerRouteFillKind::TargetChoice,
            vec![BattleHoleKind::AttackRoll],
            ReducerRouteOwnerGroup::TargetSelection,
        )]),
        "doSubmitDamageBeforeAttackRoll" => expected_single_target_route(&[
            (
                ReducerRouteFillKind::TargetChoice,
                vec![BattleHoleKind::AttackRoll],
                ReducerRouteOwnerGroup::TargetSelection,
            ),
            (
                ReducerRouteFillKind::RolledDice,
                vec![BattleHoleKind::AttackRoll],
                ReducerRouteOwnerGroup::HoleFrontier,
            ),
        ]),
        "doFillAttackRollMiss" => expected_single_target_route(&[
            (
                ReducerRouteFillKind::TargetChoice,
                vec![BattleHoleKind::AttackRoll],
                ReducerRouteOwnerGroup::TargetSelection,
            ),
            (
                ReducerRouteFillKind::AttackRoll,
                Vec::new(),
                ReducerRouteOwnerGroup::AttackRoll,
            ),
        ]),
        "doFillAttackRollHit" => expected_single_target_route(&[
            (
                ReducerRouteFillKind::TargetChoice,
                vec![BattleHoleKind::AttackRoll],
                ReducerRouteOwnerGroup::TargetSelection,
            ),
            (
                ReducerRouteFillKind::AttackRoll,
                vec![BattleHoleKind::RolledDice],
                ReducerRouteOwnerGroup::AttackRoll,
            ),
        ]),
        "doFillDamageDice" => expected_single_target_route(&[
            (
                ReducerRouteFillKind::TargetChoice,
                vec![BattleHoleKind::AttackRoll],
                ReducerRouteOwnerGroup::TargetSelection,
            ),
            (
                ReducerRouteFillKind::AttackRoll,
                vec![BattleHoleKind::RolledDice],
                ReducerRouteOwnerGroup::AttackRoll,
            ),
            (
                ReducerRouteFillKind::RolledDice,
                Vec::new(),
                ReducerRouteOwnerGroup::HitPoint,
            ),
        ]),
        "doDiscoverTypedSpellAttack" => expected_typed_discovery_route(),
        "doFillDamageTypeBeforeTargetChoice" => expected_typed_route(&[(
            ReducerRouteFillKind::DamageTypeChoice,
            vec![BattleHoleKind::TargetChoice],
            ReducerRouteOwnerGroup::HoleFrontier,
        )]),
        "doFillTargetChoiceBeforeDamageType" => expected_typed_route(&[(
            ReducerRouteFillKind::TargetChoice,
            vec![BattleHoleKind::DamageTypeChoice],
            ReducerRouteOwnerGroup::TargetSelection,
        )]),
        "doFillDamageTypeAfterTargetChoice" => expected_typed_route(&[
            (
                ReducerRouteFillKind::TargetChoice,
                vec![BattleHoleKind::DamageTypeChoice],
                ReducerRouteOwnerGroup::TargetSelection,
            ),
            (
                ReducerRouteFillKind::DamageTypeChoice,
                vec![BattleHoleKind::AttackRoll],
                ReducerRouteOwnerGroup::HoleFrontier,
            ),
        ]),
        "doFillTargetChoiceAfterDamageType" => expected_typed_route(&[
            (
                ReducerRouteFillKind::DamageTypeChoice,
                vec![BattleHoleKind::TargetChoice],
                ReducerRouteOwnerGroup::HoleFrontier,
            ),
            (
                ReducerRouteFillKind::TargetChoice,
                vec![BattleHoleKind::AttackRoll],
                ReducerRouteOwnerGroup::TargetSelection,
            ),
        ]),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

fn expected_single_target_route(
    steps: &[(
        ReducerRouteFillKind,
        Vec<BattleHoleKind>,
        ReducerRouteOwnerGroup,
    )],
) -> Vec<ReducerRouteEvent> {
    expected_spell_route(vec![BattleHoleKind::TargetChoice], steps)
}

fn expected_typed_discovery_route() -> Vec<ReducerRouteEvent> {
    expected_spell_discovery_route(vec![
        BattleHoleKind::DamageTypeChoice,
        BattleHoleKind::TargetChoice,
    ])
}

fn expected_typed_route(
    steps: &[(
        ReducerRouteFillKind,
        Vec<BattleHoleKind>,
        ReducerRouteOwnerGroup,
    )],
) -> Vec<ReducerRouteEvent> {
    expected_spell_route(
        vec![
            BattleHoleKind::DamageTypeChoice,
            BattleHoleKind::TargetChoice,
        ],
        steps,
    )
}

fn expected_spell_route(
    discovery_holes: Vec<BattleHoleKind>,
    steps: &[(
        ReducerRouteFillKind,
        Vec<BattleHoleKind>,
        ReducerRouteOwnerGroup,
    )],
) -> Vec<ReducerRouteEvent> {
    let mut route = expected_spell_discovery_route(discovery_holes);
    for (fill, holes, owner) in steps {
        route.push(
            super::battle_runtime_reducer_route::route_resolve_battle_subject(
                ReducerRouteSubjectFamily::SpellAttack,
                *fill,
                holes.clone(),
                *owner,
            ),
        );
    }
    route
}

fn expected_spell_discovery_route(discovery_holes: Vec<BattleHoleKind>) -> Vec<ReducerRouteEvent> {
    vec![
        route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
        route_discover_battle_acts(
            ReducerRouteSubjectFamily::SpellAttack,
            discovery_holes,
            ReducerRouteOwnerGroup::ActionEconomy,
        ),
    ]
}

pub fn projection_payload(state: &SpellAttackOrderingState) -> String {
    let protocol_holes = protocol_holes(&state.protocol);
    [
        format!("qStage={}", stage_ref(state.stage)),
        format!("qHoles={}", joined_or_none(&protocol_holes)),
        format!(
            "qLastOrderingError={}",
            ordering_error_ref(state.last_ordering_error)
        ),
        format!("protocolResult={}", protocol_result_ref(&state.protocol)),
        format!("protocolHoles={}", joined_or_none(&protocol_holes)),
    ]
    .join("\n")
}

fn stage_ref(stage: SpellAttackFrontierStage) -> &'static str {
    match stage {
        SpellAttackFrontierStage::ActSelection => "SpellAttackActSelectionStage",
        SpellAttackFrontierStage::TargetChoice => "SpellAttackTargetChoiceStage",
        SpellAttackFrontierStage::TypedTargetChoice => "SpellAttackTypedTargetChoiceStage",
        SpellAttackFrontierStage::TargetList => "SpellAttackTargetListStage",
        SpellAttackFrontierStage::TargetAllocation => "SpellAttackTargetAllocationStage",
        SpellAttackFrontierStage::DamageTypeAndTargetChoice => {
            "SpellAttackDamageTypeAndTargetChoiceStage"
        }
        SpellAttackFrontierStage::DamageTypeChoice => "SpellAttackDamageTypeChoiceStage",
        SpellAttackFrontierStage::AttackRoll => "SpellAttackAttackRollStage",
        SpellAttackFrontierStage::DamageDice => "SpellAttackDamageDiceStage",
        SpellAttackFrontierStage::Resolved => "SpellAttackResolvedStage",
    }
}

fn protocol_result_ref(protocol: &SpellAttackOrderingProtocol) -> &'static str {
    match protocol {
        SpellAttackOrderingProtocol::Init => "init",
        SpellAttackOrderingProtocol::NeedsHoles(_) => "needsHoles",
        SpellAttackOrderingProtocol::Resolved => "resolved",
    }
}

fn protocol_holes(protocol: &SpellAttackOrderingProtocol) -> Vec<&'static str> {
    match protocol {
        SpellAttackOrderingProtocol::NeedsHoles(holes) => holes.iter().map(hole_ref).collect(),
        SpellAttackOrderingProtocol::Init | SpellAttackOrderingProtocol::Resolved => Vec::new(),
    }
}

fn hole_ref(hole: &SpellAttackHoleKind) -> &'static str {
    match hole {
        SpellAttackHoleKind::TargetChoice => "TargetChoiceHoleKind",
        SpellAttackHoleKind::SpellTargetList => "SpellTargetListHoleKind",
        SpellAttackHoleKind::SpellTargetAllocation => "SpellTargetAllocationHoleKind",
        SpellAttackHoleKind::DamageTypeChoice => "DamageTypeChoiceHoleKind",
        SpellAttackHoleKind::AttackRoll => "AttackRollHoleKind",
        SpellAttackHoleKind::RolledDice => "RolledDiceHoleKind",
    }
}

fn ordering_error_ref(error: Option<SpellAttackFillOrderingError>) -> &'static str {
    match error {
        Some(SpellAttackFillOrderingError::TargetRequired) => "targetRequired",
        Some(SpellAttackFillOrderingError::DamageTypeRequired) => "damageTypeRequired",
        Some(SpellAttackFillOrderingError::TargetOrDamageTypeRequired) => {
            "targetOrDamageTypeRequired"
        }
        Some(SpellAttackFillOrderingError::AttackRollRequired) => "attackRollRequired",
        None => "",
    }
}

fn joined_or_none(values: &[&str]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}

fn single_target_discovered() -> BattleState {
    single_target_discovered_route().0
}

fn initial_route() -> Vec<ReducerRouteEvent> {
    vec![route_start_battle(ReducerRouteOwnerGroup::ActionEconomy)]
}

fn single_target_discovered_route() -> (BattleState, BattleSubject) {
    let state = start_spell_attack_ordering_battle();
    let subject =
        spell_attack_discovery_subject(&state, BattleSubjectKind::SingleTargetSpellAttack);
    (discover_single_target_spell_attack_battle(state), subject)
}

fn single_target_discovery_route() -> (BattleState, BattleSubject, Vec<ReducerRouteEvent>) {
    let (state, subject) = single_target_discovered_route();
    let mut route = initial_route();
    route.push(route_discover_battle_acts(
        ReducerRouteSubjectFamily::SpellAttack,
        spell_attack_discovery_holes(&start_spell_attack_ordering_battle(), subject),
        ReducerRouteOwnerGroup::ActionEconomy,
    ));
    (state, subject, route)
}

fn single_target_target_choice_route() -> (BattleState, BattleSubject, Vec<ReducerRouteEvent>) {
    let (state, subject, route) = single_target_discovery_route();
    let (result, route) = resolve_spell_attack_route(
        state,
        subject,
        target_choice(),
        ReducerRouteFillKind::TargetChoice,
        ReducerRouteOwnerGroup::TargetSelection,
        route,
    );
    let (state, subject) =
        battle_resolution_continuation(result, "single target spell attack target choice");
    (state, subject, route)
}

fn single_target_attack_hit_route() -> (BattleState, BattleSubject, Vec<ReducerRouteEvent>) {
    let (state, subject, route) = single_target_target_choice_route();
    let (result, route) = resolve_spell_attack_route(
        state,
        subject,
        BattleSpellAttackFill::AttackRoll(hit_attack_roll()),
        ReducerRouteFillKind::AttackRoll,
        ReducerRouteOwnerGroup::AttackRoll,
        route,
    );
    let (state, subject) =
        battle_resolution_continuation(result, "single target spell attack roll hit");
    (state, subject, route)
}

fn typed_spell_attack_discovered() -> BattleState {
    typed_spell_attack_discovered_route().0
}

fn typed_spell_attack_discovered_route() -> (BattleState, BattleSubject) {
    let state = start_spell_attack_ordering_battle();
    let subject = spell_attack_discovery_subject(&state, BattleSubjectKind::TypedSpellAttack);
    (discover_typed_spell_attack_battle(state), subject)
}

fn typed_spell_attack_discovery_route() -> (BattleState, BattleSubject, Vec<ReducerRouteEvent>) {
    let (state, subject) = typed_spell_attack_discovered_route();
    let mut route = initial_route();
    route.push(route_discover_battle_acts(
        ReducerRouteSubjectFamily::SpellAttack,
        spell_attack_discovery_holes(&start_spell_attack_ordering_battle(), subject),
        ReducerRouteOwnerGroup::ActionEconomy,
    ));
    (state, subject, route)
}

fn typed_target_choice_route() -> (BattleState, BattleSubject, Vec<ReducerRouteEvent>) {
    let (state, subject, route) = typed_spell_attack_discovery_route();
    let (result, route) = resolve_spell_attack_route(
        state,
        subject,
        target_choice(),
        ReducerRouteFillKind::TargetChoice,
        ReducerRouteOwnerGroup::TargetSelection,
        route,
    );
    let (state, subject) =
        battle_resolution_continuation(result, "typed spell attack target choice");
    (state, subject, route)
}

fn typed_damage_type_route() -> (BattleState, BattleSubject, Vec<ReducerRouteEvent>) {
    let (state, subject, route) = typed_spell_attack_discovery_route();
    let (result, route) = resolve_spell_attack_route(
        state,
        subject,
        BattleSpellAttackFill::DamageTypeChoice,
        ReducerRouteFillKind::DamageTypeChoice,
        ReducerRouteOwnerGroup::HoleFrontier,
        route,
    );
    let (state, subject) = battle_resolution_continuation(result, "typed spell damage type choice");
    (state, subject, route)
}

fn resolve_spell_attack_fill_state(
    state: BattleState,
    subject: BattleSubject,
    fill: BattleSpellAttackFill,
) -> BattleState {
    let result = resolve_spell_attack_result(state, subject, fill);
    if let BattleResolutionResult::Invalid { reason, .. } = &result {
        panic!("spell attack replay unexpectedly produced invalid result {reason:?}");
    }
    result.into_state()
}

fn resolve_spell_attack_route(
    state: BattleState,
    subject: BattleSubject,
    fill: BattleSpellAttackFill,
    route_fill: ReducerRouteFillKind,
    owner: ReducerRouteOwnerGroup,
    mut route: Vec<ReducerRouteEvent>,
) -> (BattleResolutionResult, Vec<ReducerRouteEvent>) {
    let result = resolve_spell_attack_result(state, subject, fill);
    route.push(route_resolve_battle_subject_from_result(
        ReducerRouteResolveConnector {
            subject: ReducerRouteSubjectFamily::SpellAttack,
            fill: ReducerRouteResolveFill::Fill(route_fill),
            owner,
        },
        &result,
    ));
    (result, route)
}

fn resolve_spell_attack_result(
    state: BattleState,
    subject: BattleSubject,
    fill: BattleSpellAttackFill,
) -> BattleResolutionResult {
    let request = BattleResolutionRequest::spell_attack(subject, fill)
        .expect("adapter-selected spell attack subject must match fill");
    resolve_battle_subject(state, request)
}

fn spell_attack_discovery_subject(state: &BattleState, kind: BattleSubjectKind) -> BattleSubject {
    discover_battle_acts(state)
        .into_available_acts()
        .into_iter()
        .find(|act| act.subject.kind == kind)
        .unwrap_or_else(|| panic!("spell attack subject {kind:?} should be discoverable"))
        .subject
}

fn spell_attack_discovery_holes(
    state: &BattleState,
    subject: BattleSubject,
) -> Vec<crate::rules::battle_reducer_spine::BattleHoleKind> {
    discover_battle_acts(state)
        .into_available_acts()
        .into_iter()
        .find(|act| act.subject == subject)
        .unwrap_or_else(|| panic!("spell attack subject {subject:?} should be discoverable"))
        .holes
}

fn target_choice() -> BattleSpellAttackFill {
    BattleSpellAttackFill::TargetChoice(Actor::Goblin)
}

fn hit_attack_roll() -> AttackRollFacts {
    AttackRollFacts {
        total: 15,
        natural_d20: 10,
    }
}

fn miss_attack_roll() -> AttackRollFacts {
    AttackRollFacts {
        total: 1,
        natural_d20: 1,
    }
}
