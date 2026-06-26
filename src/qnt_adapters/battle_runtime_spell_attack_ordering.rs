use crate::rules::battle_reducer_spine::{
    discover_single_target_spell_attack_battle, discover_typed_spell_attack_battle,
    resolve_spell_attack_subject, spell_attack_ordering_projection_from_battle,
    start_spell_attack_ordering_battle, Actor, AttackRollFacts, BattleSpellAttackFill, BattleState,
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
    route_discover_battle_acts_from_route_holes, route_resolve_battle_subject_from_route_holes,
    route_start_battle, ReducerRouteEvent, ReducerRouteFillKind, ReducerRouteHoleKind,
    ReducerRouteOwnerGroup, ReducerRouteSubjectFamily,
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
        "doSubmitAttackRollBeforeTargetChoice" => resolve_spell_attack_subject(
            single_target_discovered(),
            BattleSpellAttackFill::AttackRoll(hit_attack_roll()),
        ),
        "doFillTargetChoice" => {
            resolve_spell_attack_subject(single_target_discovered(), target_choice())
        }
        "doSubmitDamageBeforeAttackRoll" => resolve_spell_attack_subject(
            single_target_target_filled(),
            BattleSpellAttackFill::DamageRoll(4),
        ),
        "doFillAttackRollMiss" => resolve_spell_attack_subject(
            single_target_target_filled(),
            BattleSpellAttackFill::AttackRoll(miss_attack_roll()),
        ),
        "doFillAttackRollHit" => resolve_spell_attack_subject(
            single_target_target_filled(),
            BattleSpellAttackFill::AttackRoll(hit_attack_roll()),
        ),
        "doFillDamageDice" => {
            resolve_spell_attack_subject(single_target_hit(), BattleSpellAttackFill::DamageRoll(4))
        }
        "doDiscoverTypedSpellAttack" => typed_spell_attack_discovered(),
        "doFillDamageTypeBeforeTargetChoice" => resolve_spell_attack_subject(
            typed_spell_attack_discovered(),
            BattleSpellAttackFill::DamageTypeChoice,
        ),
        "doFillTargetChoiceBeforeDamageType" => {
            resolve_spell_attack_subject(typed_spell_attack_discovered(), target_choice())
        }
        "doFillDamageTypeAfterTargetChoice" => resolve_spell_attack_subject(
            typed_target_filled(),
            BattleSpellAttackFill::DamageTypeChoice,
        ),
        "doFillTargetChoiceAfterDamageType" => {
            resolve_spell_attack_subject(typed_damage_type_filled(), target_choice())
        }
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
        "doDiscoverSingleTargetSpellAttack" => single_target_discovery_route().1,
        "doSubmitAttackRollBeforeTargetChoice" => {
            let (state, mut route) = single_target_discovery_route();
            let state = resolve_spell_attack_subject(
                state,
                BattleSpellAttackFill::AttackRoll(hit_attack_roll()),
            );
            route.push(spell_attack_route_event(
                ReducerRouteFillKind::AttackRoll,
                &state,
                ReducerRouteOwnerGroup::HoleFrontier,
            ));
            route
        }
        "doFillTargetChoice" => {
            let (_state, route) = single_target_target_choice_route();
            route
        }
        "doSubmitDamageBeforeAttackRoll" => {
            let (state, mut route) = single_target_target_choice_route();
            let state = resolve_spell_attack_subject(state, BattleSpellAttackFill::DamageRoll(4));
            route.push(spell_attack_route_event(
                ReducerRouteFillKind::RolledDice,
                &state,
                ReducerRouteOwnerGroup::HoleFrontier,
            ));
            route
        }
        "doFillAttackRollMiss" => {
            let (state, mut route) = single_target_target_choice_route();
            let state = resolve_spell_attack_subject(
                state,
                BattleSpellAttackFill::AttackRoll(miss_attack_roll()),
            );
            route.push(spell_attack_route_event(
                ReducerRouteFillKind::AttackRoll,
                &state,
                ReducerRouteOwnerGroup::AttackRoll,
            ));
            route
        }
        "doFillAttackRollHit" => {
            let (_state, route) = single_target_attack_hit_route();
            route
        }
        "doFillDamageDice" => {
            let (state, mut route) = single_target_attack_hit_route();
            let state = resolve_spell_attack_subject(state, BattleSpellAttackFill::DamageRoll(4));
            route.push(spell_attack_route_event(
                ReducerRouteFillKind::RolledDice,
                &state,
                ReducerRouteOwnerGroup::HitPoint,
            ));
            route
        }
        "doDiscoverTypedSpellAttack" => typed_spell_attack_discovery_route().1,
        "doFillDamageTypeBeforeTargetChoice" => {
            let (state, mut route) = typed_spell_attack_discovery_route();
            let state =
                resolve_spell_attack_subject(state, BattleSpellAttackFill::DamageTypeChoice);
            route.push(spell_attack_route_event(
                ReducerRouteFillKind::DamageTypeChoice,
                &state,
                ReducerRouteOwnerGroup::HoleFrontier,
            ));
            route
        }
        "doFillTargetChoiceBeforeDamageType" => {
            let (state, mut route) = typed_spell_attack_discovery_route();
            let state = resolve_spell_attack_subject(state, target_choice());
            route.push(spell_attack_route_event(
                ReducerRouteFillKind::TargetChoice,
                &state,
                ReducerRouteOwnerGroup::TargetSelection,
            ));
            route
        }
        "doFillDamageTypeAfterTargetChoice" => {
            let (state, mut route) = typed_target_choice_route();
            let state =
                resolve_spell_attack_subject(state, BattleSpellAttackFill::DamageTypeChoice);
            route.push(spell_attack_route_event(
                ReducerRouteFillKind::DamageTypeChoice,
                &state,
                ReducerRouteOwnerGroup::HoleFrontier,
            ));
            route
        }
        "doFillTargetChoiceAfterDamageType" => {
            let (state, mut route) = typed_damage_type_route();
            let state = resolve_spell_attack_subject(state, target_choice());
            route.push(spell_attack_route_event(
                ReducerRouteFillKind::TargetChoice,
                &state,
                ReducerRouteOwnerGroup::TargetSelection,
            ));
            route
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    replay_observed_route(observed_action_taken)
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
    discover_single_target_spell_attack_battle(start_spell_attack_ordering_battle())
}

fn initial_route() -> Vec<ReducerRouteEvent> {
    vec![route_start_battle(ReducerRouteOwnerGroup::ActionEconomy)]
}

fn single_target_discovery_route() -> (BattleState, Vec<ReducerRouteEvent>) {
    let state = single_target_discovered();
    let mut route = initial_route();
    route.push(route_discover_battle_acts_from_route_holes(
        ReducerRouteSubjectFamily::SpellAttack,
        spell_attack_route_holes(&state),
        ReducerRouteOwnerGroup::ActionEconomy,
    ));
    (state, route)
}

fn single_target_target_filled() -> BattleState {
    resolve_spell_attack_subject(single_target_discovered(), target_choice())
}

fn single_target_target_choice_route() -> (BattleState, Vec<ReducerRouteEvent>) {
    let (state, mut route) = single_target_discovery_route();
    let state = resolve_spell_attack_subject(state, target_choice());
    route.push(spell_attack_route_event(
        ReducerRouteFillKind::TargetChoice,
        &state,
        ReducerRouteOwnerGroup::TargetSelection,
    ));
    (state, route)
}

fn single_target_hit() -> BattleState {
    resolve_spell_attack_subject(
        single_target_target_filled(),
        BattleSpellAttackFill::AttackRoll(hit_attack_roll()),
    )
}

fn single_target_attack_hit_route() -> (BattleState, Vec<ReducerRouteEvent>) {
    let (state, mut route) = single_target_target_choice_route();
    let state =
        resolve_spell_attack_subject(state, BattleSpellAttackFill::AttackRoll(hit_attack_roll()));
    route.push(spell_attack_route_event(
        ReducerRouteFillKind::AttackRoll,
        &state,
        ReducerRouteOwnerGroup::AttackRoll,
    ));
    (state, route)
}

fn typed_spell_attack_discovered() -> BattleState {
    discover_typed_spell_attack_battle(start_spell_attack_ordering_battle())
}

fn typed_spell_attack_discovery_route() -> (BattleState, Vec<ReducerRouteEvent>) {
    let state = typed_spell_attack_discovered();
    let mut route = initial_route();
    route.push(route_discover_battle_acts_from_route_holes(
        ReducerRouteSubjectFamily::SpellAttack,
        spell_attack_route_holes(&state),
        ReducerRouteOwnerGroup::ActionEconomy,
    ));
    (state, route)
}

fn typed_target_filled() -> BattleState {
    resolve_spell_attack_subject(typed_spell_attack_discovered(), target_choice())
}

fn typed_target_choice_route() -> (BattleState, Vec<ReducerRouteEvent>) {
    let (state, mut route) = typed_spell_attack_discovery_route();
    let state = resolve_spell_attack_subject(state, target_choice());
    route.push(spell_attack_route_event(
        ReducerRouteFillKind::TargetChoice,
        &state,
        ReducerRouteOwnerGroup::TargetSelection,
    ));
    (state, route)
}

fn typed_damage_type_filled() -> BattleState {
    resolve_spell_attack_subject(
        typed_spell_attack_discovered(),
        BattleSpellAttackFill::DamageTypeChoice,
    )
}

fn typed_damage_type_route() -> (BattleState, Vec<ReducerRouteEvent>) {
    let (state, mut route) = typed_spell_attack_discovery_route();
    let state = resolve_spell_attack_subject(state, BattleSpellAttackFill::DamageTypeChoice);
    route.push(spell_attack_route_event(
        ReducerRouteFillKind::DamageTypeChoice,
        &state,
        ReducerRouteOwnerGroup::HoleFrontier,
    ));
    (state, route)
}

fn spell_attack_route_event(
    fill: ReducerRouteFillKind,
    state: &BattleState,
    owner: ReducerRouteOwnerGroup,
) -> ReducerRouteEvent {
    route_resolve_battle_subject_from_route_holes(
        ReducerRouteSubjectFamily::SpellAttack,
        fill,
        spell_attack_route_holes(state),
        owner,
    )
}

fn spell_attack_route_holes(state: &BattleState) -> Vec<ReducerRouteHoleKind> {
    match spell_attack_ordering_projection_from_battle(state).stage {
        SpellAttackFrontierStage::ActSelection | SpellAttackFrontierStage::Resolved => Vec::new(),
        SpellAttackFrontierStage::TargetChoice | SpellAttackFrontierStage::TypedTargetChoice => {
            vec![ReducerRouteHoleKind::TargetChoice]
        }
        SpellAttackFrontierStage::TargetList => vec![ReducerRouteHoleKind::SpellTargetList],
        SpellAttackFrontierStage::TargetAllocation => {
            vec![ReducerRouteHoleKind::SpellTargetAllocation]
        }
        SpellAttackFrontierStage::DamageTypeAndTargetChoice => vec![
            ReducerRouteHoleKind::DamageTypeChoice,
            ReducerRouteHoleKind::TargetChoice,
        ],
        SpellAttackFrontierStage::DamageTypeChoice => {
            vec![ReducerRouteHoleKind::DamageTypeChoice]
        }
        SpellAttackFrontierStage::AttackRoll => vec![ReducerRouteHoleKind::AttackRoll],
        SpellAttackFrontierStage::DamageDice => vec![ReducerRouteHoleKind::RolledDice],
    }
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
