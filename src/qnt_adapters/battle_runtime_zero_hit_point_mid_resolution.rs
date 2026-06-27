use crate::rules::battle_reducer_spine::{BattleGenericRouteFill, BattleSubjectKind};
use crate::rules::zero_hit_point_mid_resolution::{
    resolve_eldritch_blast_zero_hit_point_mid_resolution, ZeroHitPointMidResolutionHole,
    ZeroHitPointMidResolutionProtocol, ZeroHitPointMidResolutionScenario,
    ZeroHitPointMidResolutionState,
};

use super::battle_runtime_reducer_route::{
    replay_generic_battle_route, route_discover_battle_acts_from_route_holes,
    route_resolve_battle_subject_from_route_holes,
    route_resolve_battle_subject_without_fill_from_route_holes, route_start_battle,
    GenericBattleRouteStep, ReducerRouteEvent, ReducerRouteFillKind, ReducerRouteHoleKind,
    ReducerRouteOwnerGroup, ReducerRouteSubjectFamily,
};

pub const BRANCH_ACTIONS: [&str; 1] = ["doResolveEldritchBlast"];

pub fn replay_observed_action(observed_action_taken: &str) -> ZeroHitPointMidResolutionState {
    match observed_action_taken {
        "doResolveEldritchBlast" => resolve_eldritch_blast_zero_hit_point_mid_resolution(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> ZeroHitPointMidResolutionState {
    replay_observed_action(observed_action_taken)
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doResolveEldritchBlast" => replay_generic_battle_route(&[
            discover(BattleSubjectKind::SpellAttackProcedureInitialTargetChoice),
            resolve(
                BattleSubjectKind::SpellAttackProcedureInitialTargetChoice,
                BattleGenericRouteFill::TargetChoice,
            ),
            resolve(
                BattleSubjectKind::SpellAttackProcedureSecondTargetChoice,
                BattleGenericRouteFill::TargetChoice,
            ),
            resolve(
                BattleSubjectKind::SpellAttackProcedureAttackRoll,
                BattleGenericRouteFill::AttackRoll,
            ),
            resolve(
                BattleSubjectKind::SpellAttackProcedureDamageToZeroHitPoints,
                BattleGenericRouteFill::RolledDice,
            ),
            resolve(
                BattleSubjectKind::SpellAttackProcedureConcentrationSave,
                BattleGenericRouteFill::ConcentrationSavingThrow,
            ),
            resolve(
                BattleSubjectKind::ZeroHitPointSpellEffectTeardownConditionLifecycle,
                BattleGenericRouteFill::WithoutFill,
            ),
            resolve(
                BattleSubjectKind::ZeroHitPointSpellEffectTeardownConcentration,
                BattleGenericRouteFill::WithoutFill,
            ),
            resolve(
                BattleSubjectKind::ZeroHitPointSpellEffectTeardownActiveEffect,
                BattleGenericRouteFill::WithoutFill,
            ),
            resolve(
                BattleSubjectKind::SpellAttackProcedureAttackRoll,
                BattleGenericRouteFill::AttackRoll,
            ),
            resolve(
                BattleSubjectKind::SpellAttackProcedureRemainderDamage,
                BattleGenericRouteFill::RolledDice,
            ),
        ]),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doResolveEldritchBlast" => expected_eldritch_blast_route(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn projection_payload(state: &ZeroHitPointMidResolutionState) -> String {
    let protocol_holes = protocol_holes(&state.protocol);
    [
        format!("qScenario={}", scenario_ref(state.scenario)),
        format!("qSourceHp={}", state.source_hit_points),
        format!("qSourceUnconscious={}", state.source_unconscious),
        format!("qSourceConcentrating={}", state.source_concentrating),
        format!("qShieldOfFaithPresent={}", state.shield_of_faith_present),
        format!("qProtectedTargetHp={}", state.protected_target_hit_points),
        format!("qSourceDamage={}", state.source_damage),
        format!("qProtectedTargetDamage={}", state.protected_target_damage),
        format!(
            "qProtectedTargetDamageIfShieldOfFaithRemained={}",
            state.protected_target_damage_if_shield_of_faith_remained
        ),
        format!(
            "qZeroHpAppliedBeforeSecondBeam={}",
            state.zero_hit_points_applied_before_second_beam
        ),
        format!(
            "qTeardownBeforeSecondBeam={}",
            state.teardown_before_second_beam
        ),
        format!(
            "qRemainderUsedPostTeardownState={}",
            state.remainder_used_post_teardown_state
        ),
        format!("qHoles={}", joined_or_none(&protocol_holes)),
        format!("protocolResult={}", protocol_result_ref(&state.protocol)),
        format!("protocolHoles={}", joined_or_none(&protocol_holes)),
    ]
    .join("\n")
}

fn scenario_ref(scenario: ZeroHitPointMidResolutionScenario) -> &'static str {
    match scenario {
        ZeroHitPointMidResolutionScenario::Init => "Init",
        ZeroHitPointMidResolutionScenario::SpellAttackSequenceResolved => {
            "SpellAttackSequenceResolved"
        }
    }
}

fn protocol_result_ref(protocol: &ZeroHitPointMidResolutionProtocol) -> &'static str {
    match protocol {
        ZeroHitPointMidResolutionProtocol::Init(_) => "init",
        ZeroHitPointMidResolutionProtocol::Resolved => "resolved",
    }
}

fn protocol_holes(protocol: &ZeroHitPointMidResolutionProtocol) -> Vec<&'static str> {
    match protocol {
        ZeroHitPointMidResolutionProtocol::Init(holes) => holes.iter().map(hole_ref).collect(),
        ZeroHitPointMidResolutionProtocol::Resolved => Vec::new(),
    }
}

fn hole_ref(hole: &ZeroHitPointMidResolutionHole) -> &'static str {
    match hole {
        ZeroHitPointMidResolutionHole::ZeroHitPointMidResolution => "ZeroHitPointMidResolution",
    }
}

fn joined_or_none(values: &[&str]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}

fn discover(kind: BattleSubjectKind) -> GenericBattleRouteStep {
    GenericBattleRouteStep::Discover(kind)
}

fn resolve(kind: BattleSubjectKind, fill: BattleGenericRouteFill) -> GenericBattleRouteStep {
    GenericBattleRouteStep::Resolve { kind, fill }
}

fn expected_eldritch_blast_route() -> Vec<ReducerRouteEvent> {
    vec![
        route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
        expected_spell_attack_discover(vec![
            ReducerRouteHoleKind::TargetChoice,
            ReducerRouteHoleKind::AttackRoll,
            ReducerRouteHoleKind::RolledDice,
        ]),
        expected_spell_attack_resolve(
            ReducerRouteFillKind::TargetChoice,
            vec![
                ReducerRouteHoleKind::TargetChoice,
                ReducerRouteHoleKind::AttackRoll,
                ReducerRouteHoleKind::RolledDice,
            ],
            ReducerRouteOwnerGroup::TargetSelection,
        ),
        expected_spell_attack_resolve(
            ReducerRouteFillKind::TargetChoice,
            vec![
                ReducerRouteHoleKind::AttackRoll,
                ReducerRouteHoleKind::RolledDice,
            ],
            ReducerRouteOwnerGroup::TargetSelection,
        ),
        expected_spell_attack_resolve(
            ReducerRouteFillKind::AttackRoll,
            vec![ReducerRouteHoleKind::RolledDice],
            ReducerRouteOwnerGroup::AttackRoll,
        ),
        expected_spell_attack_resolve(
            ReducerRouteFillKind::RolledDice,
            vec![ReducerRouteHoleKind::ConcentrationSavingThrow],
            ReducerRouteOwnerGroup::HitPointAndZeroHpLifecycle,
        ),
        expected_spell_attack_resolve(
            ReducerRouteFillKind::ConcentrationSavingThrow,
            vec![
                ReducerRouteHoleKind::AttackRoll,
                ReducerRouteHoleKind::RolledDice,
            ],
            ReducerRouteOwnerGroup::Concentration,
        ),
        expected_teardown_resolve(ReducerRouteOwnerGroup::ConditionLifecycle),
        expected_teardown_resolve(ReducerRouteOwnerGroup::Concentration),
        expected_teardown_resolve(ReducerRouteOwnerGroup::ActiveEffect),
        expected_spell_attack_resolve(
            ReducerRouteFillKind::AttackRoll,
            vec![ReducerRouteHoleKind::RolledDice],
            ReducerRouteOwnerGroup::AttackRoll,
        ),
        expected_spell_attack_resolve(
            ReducerRouteFillKind::RolledDice,
            Vec::new(),
            ReducerRouteOwnerGroup::HitPoint,
        ),
    ]
}

fn expected_spell_attack_discover(holes: Vec<ReducerRouteHoleKind>) -> ReducerRouteEvent {
    route_discover_battle_acts_from_route_holes(
        ReducerRouteSubjectFamily::SpellAttackProcedure,
        holes,
        ReducerRouteOwnerGroup::SpellAttackProcedure,
    )
}

fn expected_spell_attack_resolve(
    fill: ReducerRouteFillKind,
    holes: Vec<ReducerRouteHoleKind>,
    owner: ReducerRouteOwnerGroup,
) -> ReducerRouteEvent {
    route_resolve_battle_subject_from_route_holes(
        ReducerRouteSubjectFamily::SpellAttackProcedure,
        fill,
        holes,
        owner,
    )
}

fn expected_teardown_resolve(owner: ReducerRouteOwnerGroup) -> ReducerRouteEvent {
    route_resolve_battle_subject_without_fill_from_route_holes(
        ReducerRouteSubjectFamily::ZeroHitPointSpellEffectTeardown,
        Vec::new(),
        owner,
    )
}
