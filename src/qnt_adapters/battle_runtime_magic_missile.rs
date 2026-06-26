use crate::rules::battle_reducer_spine::{
    combatant_is_dead, discover_battle_acts, resolve_battle_subject_test_fill,
    slot_spell_holes_from_battle, start_fighter_skeleton_battle,
    stat_block_multiattack_dispatches_available, Actor, BattleFill, BattleHoleKind,
    BattleResolutionResult, BattleSlotSpellFill, BattleSlotSpellHole, BattleState,
    BattleSubjectKind,
};
use crate::rules::magic_missile::{
    MagicMissileHole, MagicMissileProtocolResult, MagicMissileState,
};

use super::battle_runtime_reducer_route::{
    route_discover_battle_acts, route_resolve_battle_subject, route_start_battle,
    ReducerRouteEvent, ReducerRouteFillKind, ReducerRouteOwnerGroup, ReducerRouteSubjectFamily,
};

pub const BRANCH_ACTIONS: [&str; 2] = ["doFillMagicMissileAllocation", "doFillMagicMissileDamage"];

pub const DAMAGE_SAMPLE_DART_ROLL_TOTALS: [i16; 2] = [3, 12];

pub fn replay_observed_action(observed_action_taken: &str) -> MagicMissileState {
    match observed_action_taken {
        "doFillMagicMissileAllocation" => {
            magic_missile_state_from_battle(&slot_spell_targets_resolved())
        }
        "doFillMagicMissileDamage" => replay_magic_missile_damage(3),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn replay_magic_missile_damage(dart_roll_total: i16) -> MagicMissileState {
    magic_missile_state_from_battle(&slot_spell_damage_resolved(dart_roll_total))
}

pub fn expected_witness(observed_action_taken: &str) -> MagicMissileState {
    replay_observed_action(observed_action_taken)
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doFillMagicMissileAllocation" => slot_spell_targets_resolved_with_route().1,
        "doFillMagicMissileDamage" => slot_spell_damage_resolved_with_route(3).1,
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    let mut route = initial_magic_missile_route();
    match observed_action_taken {
        "doFillMagicMissileAllocation" => {
            route.push(route_resolve_battle_subject(
                ReducerRouteSubjectFamily::SlotSpell,
                ReducerRouteFillKind::SpellTargetAllocation,
                vec![BattleHoleKind::RolledDice],
                ReducerRouteOwnerGroup::HoleFrontier,
            ));
            route
        }
        "doFillMagicMissileDamage" => {
            route.push(route_resolve_battle_subject(
                ReducerRouteSubjectFamily::SlotSpell,
                ReducerRouteFillKind::SpellTargetAllocation,
                vec![BattleHoleKind::RolledDice],
                ReducerRouteOwnerGroup::HoleFrontier,
            ));
            route.push(route_resolve_battle_subject(
                ReducerRouteSubjectFamily::SlotSpell,
                ReducerRouteFillKind::RolledDice,
                Vec::new(),
                ReducerRouteOwnerGroup::HitPoint,
            ));
            route
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn projection_payload(state: &MagicMissileState) -> String {
    [
        format!("qSkeletonHp={}", state.skeleton_hit_points),
        format!("qSkeletonDead={}", state.skeleton_dead),
        format!("qActionAvailable={}", state.action_available),
        format!(
            "qMultiattackDispatchesAvailable={}",
            state.multiattack_dispatches_available
        ),
        format!(
            "qSneakAttackUsedThisTurn={}",
            state.sneak_attack_used_this_turn
        ),
        format!(
            "protocolResult={}",
            protocol_result_ref(state.protocol_result)
        ),
        format!("protocolHoles={}", joined_holes(&state.protocol_holes)),
    ]
    .join("\n")
}

fn protocol_result_ref(result: MagicMissileProtocolResult) -> &'static str {
    match result {
        MagicMissileProtocolResult::Init => "init",
        MagicMissileProtocolResult::NeedsHoles => "needsHoles",
        MagicMissileProtocolResult::Resolved => "resolved",
    }
}

fn joined_holes(holes: &[MagicMissileHole]) -> String {
    if holes.is_empty() {
        return "none".to_string();
    }

    holes
        .iter()
        .map(|hole| match hole {
            MagicMissileHole::SpellTargetAllocation => "SpellTargetAllocation",
            MagicMissileHole::SpellDamageRoll => "SpellDamageRoll",
        })
        .collect::<Vec<_>>()
        .join(",")
}

fn slot_spell_targets_resolved() -> BattleState {
    slot_spell_targets_resolved_with_route().0
}

fn slot_spell_targets_resolved_with_route() -> (BattleState, Vec<ReducerRouteEvent>) {
    let state = start_fighter_skeleton_battle();
    let mut route = vec![route_start_battle(ReducerRouteOwnerGroup::ActionEconomy)];
    let act = discovered_slot_spell_act(&state);
    route.push(route_discover_battle_acts(
        ReducerRouteSubjectFamily::SlotSpell,
        act.holes.clone(),
        ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
    ));
    match resolve_battle_subject_test_fill(
        state,
        act.subject,
        BattleFill::SlotSpell(BattleSlotSpellFill::TargetAllocation(Actor::Skeleton)),
    ) {
        BattleResolutionResult::NeedsHoles { state, holes, .. } => {
            route.push(route_resolve_battle_subject(
                ReducerRouteSubjectFamily::SlotSpell,
                ReducerRouteFillKind::SpellTargetAllocation,
                holes,
                ReducerRouteOwnerGroup::HoleFrontier,
            ));
            (state, route)
        }
        other => panic!("Magic Missile target allocation should need damage holes, got {other:?}"),
    }
}

fn slot_spell_damage_resolved(dart_roll_total: i16) -> BattleState {
    slot_spell_damage_resolved_with_route(dart_roll_total).0
}

fn slot_spell_damage_resolved_with_route(
    dart_roll_total: i16,
) -> (BattleState, Vec<ReducerRouteEvent>) {
    let (state, mut route) = slot_spell_targets_resolved_with_route();
    let act = discovered_slot_spell_subject_from_state(&state);
    match resolve_battle_subject_test_fill(
        state,
        act,
        BattleFill::SlotSpell(BattleSlotSpellFill::DamageRoll(dart_roll_total)),
    ) {
        BattleResolutionResult::Resolved { state } => {
            route.push(route_resolve_battle_subject(
                ReducerRouteSubjectFamily::SlotSpell,
                ReducerRouteFillKind::RolledDice,
                Vec::new(),
                ReducerRouteOwnerGroup::HitPoint,
            ));
            (state, route)
        }
        other => panic!("Magic Missile damage should resolve through reducer, got {other:?}"),
    }
}

fn initial_magic_missile_route() -> Vec<ReducerRouteEvent> {
    vec![
        route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
        route_discover_battle_acts(
            ReducerRouteSubjectFamily::SlotSpell,
            vec![BattleHoleKind::SpellTargetAllocation],
            ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
        ),
    ]
}

fn discovered_slot_spell_act(
    state: &BattleState,
) -> crate::rules::battle_reducer_spine::AvailableBattleAct {
    discover_battle_acts(state)
        .into_iter()
        .find(|act| act.subject.kind == BattleSubjectKind::SlotSpell)
        .expect("Magic Missile reducer route should discover one slot-spell act")
}

fn discovered_slot_spell_subject_from_state(
    state: &BattleState,
) -> crate::rules::battle_reducer_spine::BattleSubject {
    let active = match state.slot_spell_procedure {
        crate::rules::battle_reducer_spine::BattleSlotSpellProcedure::Active(subject) => subject,
        crate::rules::battle_reducer_spine::BattleSlotSpellProcedure::Inactive => {
            panic!("Magic Missile reducer route should have an active slot-spell subject")
        }
    };
    crate::rules::battle_reducer_spine::BattleSubject {
        kind: BattleSubjectKind::SlotSpell,
        actor: active.actor,
        target: active.target,
        stage: crate::rules::weapon_attack_ordering::WeaponAttackFrontierStage::Resolved,
        damage_modifier: 0,
    }
}

fn magic_missile_state_from_battle(state: &BattleState) -> MagicMissileState {
    MagicMissileState {
        skeleton_hit_points: state.skeleton.hp,
        skeleton_dead: combatant_is_dead(state.skeleton),
        action_available: state.action_available,
        multiattack_dispatches_available: stat_block_multiattack_dispatches_available(state) as u8,
        sneak_attack_used_this_turn: state.rogue.sneak_attack_used_this_turn,
        protocol_holes: slot_spell_holes_from_battle(state)
            .into_iter()
            .map(magic_missile_hole_from_reducer)
            .collect(),
        protocol_result: if slot_spell_holes_from_battle(state).is_empty() {
            MagicMissileProtocolResult::Resolved
        } else {
            MagicMissileProtocolResult::NeedsHoles
        },
    }
}

fn magic_missile_hole_from_reducer(hole: BattleSlotSpellHole) -> MagicMissileHole {
    match hole {
        BattleSlotSpellHole::SpellTargetAllocation => MagicMissileHole::SpellTargetAllocation,
        BattleSlotSpellHole::RolledDice => MagicMissileHole::SpellDamageRoll,
    }
}
