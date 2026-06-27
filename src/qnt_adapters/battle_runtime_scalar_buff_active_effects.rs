use crate::rules::battle_reducer_spine::{BattleGenericRouteFill, BattleSubjectKind};
use crate::rules::scalar_buff_active_effects::{
    resolve_aid_scalar_buff, resolve_false_life_scalar_buff, resolve_longstrider_scalar_buff,
    resolve_shield_of_faith_scalar_buff, resolve_spider_climb_scalar_buff,
    stutter_scalar_buff_active_effect, ScalarBuffActiveEffectsState, ScalarBuffProtocol,
    ScalarBuffScenarioResult,
};

use super::battle_runtime_reducer_route::{
    replay_generic_battle_route, route_discover_battle_acts_from_route_holes,
    route_resolve_battle_subject_without_fill_from_route_holes, route_start_battle,
    GenericBattleRouteStep, ReducerRouteEvent, ReducerRouteOwnerGroup, ReducerRouteSubjectFamily,
};

pub const BRANCH_ACTIONS: [&str; 6] = [
    "doCastShieldOfFaith",
    "doCastLongstrider",
    "doCastSpiderClimb",
    "doCastAid",
    "doCastFalseLife",
    "doStutter",
];

pub fn replay_observed_action(observed_action_taken: &str) -> ScalarBuffActiveEffectsState {
    match observed_action_taken {
        "doCastShieldOfFaith" => resolve_shield_of_faith_scalar_buff(),
        "doCastLongstrider" => resolve_longstrider_scalar_buff(),
        "doCastSpiderClimb" => resolve_spider_climb_scalar_buff(),
        "doCastAid" => resolve_aid_scalar_buff(),
        "doCastFalseLife" => resolve_false_life_scalar_buff(),
        "doStutter" => stutter_scalar_buff_active_effect(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> ScalarBuffActiveEffectsState {
    replay_observed_action(observed_action_taken)
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doCastShieldOfFaith" => shield_of_faith_route(),
        "doCastLongstrider" => longstrider_route(),
        "doCastSpiderClimb" => spider_climb_route(),
        "doCastAid" => aid_route(),
        "doCastFalseLife" | "doStutter" => false_life_route(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doCastShieldOfFaith" => expected_shield_of_faith_route(),
        "doCastLongstrider" => expected_longstrider_route(),
        "doCastSpiderClimb" => expected_spider_climb_route(),
        "doCastAid" => expected_aid_route(),
        "doCastFalseLife" | "doStutter" => expected_false_life_route(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn projection_payload(state: &ScalarBuffActiveEffectsState) -> String {
    [
        format!("affectedArmorClass={}", state.affected_armor_class),
        format!("affectedSpeedFeet={}", state.affected_speed_feet),
        format!("affectedClimbSpeedFeet={}", state.affected_climb_speed_feet),
        format!(
            "affectedHitPointMaximum={}",
            state.affected_hit_point_maximum
        ),
        format!("affectedHitPoints={}", state.affected_hit_points),
        format!(
            "affectedTemporaryHitPoints={}",
            state.affected_temporary_hit_points
        ),
        format!("armorClassBonusActive={}", state.armor_class_bonus_active),
        format!("speedDeltaActive={}", state.speed_delta_active),
        format!(
            "specialSpeedGrantActive={}",
            state.special_speed_grant_active
        ),
        format!(
            "hitPointMaximumIncreaseActive={}",
            state.hit_point_maximum_increase_active
        ),
        format!("casterConcentrating={}", state.caster_concentrating),
        format!("scenarioResult={}", scenario_ref(state.scenario_result)),
        format!("protocolResult={}", protocol_ref(state.protocol)),
        "protocolHoles=none".to_string(),
    ]
    .join("\n")
}

fn scenario_ref(result: ScalarBuffScenarioResult) -> &'static str {
    match result {
        ScalarBuffScenarioResult::Init => "init",
        ScalarBuffScenarioResult::ShieldOfFaith => "shieldOfFaith",
        ScalarBuffScenarioResult::Longstrider => "longstrider",
        ScalarBuffScenarioResult::SpiderClimb => "spiderClimb",
        ScalarBuffScenarioResult::Aid => "aid",
        ScalarBuffScenarioResult::FalseLife => "falseLife",
    }
}

fn protocol_ref(protocol: ScalarBuffProtocol) -> &'static str {
    match protocol {
        ScalarBuffProtocol::Init => "init",
        ScalarBuffProtocol::Resolved => "resolved",
    }
}

fn shield_of_faith_route() -> Vec<ReducerRouteEvent> {
    replay_generic_battle_route(&[
        discover(BattleSubjectKind::ScalarBuffEffectActiveEffect),
        resolve(BattleSubjectKind::ScalarBuffEffectActiveEffect),
        resolve(BattleSubjectKind::ScalarBuffEffectConcentration),
    ])
}

fn longstrider_route() -> Vec<ReducerRouteEvent> {
    let mut route = shield_of_faith_route();
    route.extend(
        replay_generic_battle_route(&[
            discover(BattleSubjectKind::ScalarBuffEffectSpeedDelta),
            resolve(BattleSubjectKind::ScalarBuffEffectActiveEffect),
            resolve(BattleSubjectKind::ScalarBuffEffectSpeedDelta),
        ])
        .into_iter()
        .skip(1),
    );
    route
}

fn spider_climb_route() -> Vec<ReducerRouteEvent> {
    let mut route = longstrider_route();
    route.extend(
        replay_generic_battle_route(&[
            discover(BattleSubjectKind::ScalarBuffEffectSpeedDelta),
            resolve(BattleSubjectKind::ScalarBuffEffectActiveEffect),
            resolve(BattleSubjectKind::ScalarBuffEffectSpeedDelta),
            resolve(BattleSubjectKind::ScalarBuffEffectConcentration),
        ])
        .into_iter()
        .skip(1),
    );
    route
}

fn aid_route() -> Vec<ReducerRouteEvent> {
    let mut route = spider_climb_route();
    route.extend(
        replay_generic_battle_route(&[
            discover(BattleSubjectKind::ScalarBuffEffectHitPoint),
            resolve(BattleSubjectKind::ScalarBuffEffectActiveEffect),
            resolve(BattleSubjectKind::ScalarBuffEffectHitPoint),
        ])
        .into_iter()
        .skip(1),
    );
    route
}

fn false_life_route() -> Vec<ReducerRouteEvent> {
    let mut route = aid_route();
    route.extend(
        replay_generic_battle_route(&[
            discover(BattleSubjectKind::ScalarBuffEffectTemporaryHitPoint),
            resolve(BattleSubjectKind::ScalarBuffEffectTemporaryHitPoint),
        ])
        .into_iter()
        .skip(1),
    );
    route
}

fn discover(kind: BattleSubjectKind) -> GenericBattleRouteStep {
    GenericBattleRouteStep::Discover(kind)
}

fn resolve(kind: BattleSubjectKind) -> GenericBattleRouteStep {
    GenericBattleRouteStep::Resolve {
        kind,
        fill: BattleGenericRouteFill::WithoutFill,
    }
}

fn expected_shield_of_faith_route() -> Vec<ReducerRouteEvent> {
    vec![
        expected_start(),
        expected_discover(ReducerRouteOwnerGroup::SpellSlotAndActionEconomy),
        expected_resolve(ReducerRouteOwnerGroup::ActiveEffect),
        expected_resolve(ReducerRouteOwnerGroup::Concentration),
    ]
}

fn expected_longstrider_route() -> Vec<ReducerRouteEvent> {
    let mut route = expected_shield_of_faith_route();
    route.extend([
        expected_discover(ReducerRouteOwnerGroup::SpellSlotAndActionEconomy),
        expected_resolve(ReducerRouteOwnerGroup::ActiveEffect),
        expected_resolve(ReducerRouteOwnerGroup::MovementResource),
    ]);
    route
}

fn expected_spider_climb_route() -> Vec<ReducerRouteEvent> {
    let mut route = expected_longstrider_route();
    route.extend([
        expected_discover(ReducerRouteOwnerGroup::SpellSlotAndActionEconomy),
        expected_resolve(ReducerRouteOwnerGroup::ActiveEffect),
        expected_resolve(ReducerRouteOwnerGroup::MovementResource),
        expected_resolve(ReducerRouteOwnerGroup::Concentration),
    ]);
    route
}

fn expected_aid_route() -> Vec<ReducerRouteEvent> {
    let mut route = expected_spider_climb_route();
    route.extend([
        expected_discover(ReducerRouteOwnerGroup::SpellSlotAndActionEconomy),
        expected_resolve(ReducerRouteOwnerGroup::ActiveEffect),
        expected_resolve(ReducerRouteOwnerGroup::HitPoint),
    ]);
    route
}

fn expected_false_life_route() -> Vec<ReducerRouteEvent> {
    let mut route = expected_aid_route();
    route.extend([
        expected_discover(ReducerRouteOwnerGroup::SpellSlotAndActionEconomy),
        expected_resolve(ReducerRouteOwnerGroup::TemporaryHitPoint),
    ]);
    route
}

fn expected_start() -> ReducerRouteEvent {
    route_start_battle(ReducerRouteOwnerGroup::ActionEconomy)
}

fn expected_discover(owner: ReducerRouteOwnerGroup) -> ReducerRouteEvent {
    route_discover_battle_acts_from_route_holes(
        ReducerRouteSubjectFamily::ScalarBuffEffect,
        Vec::new(),
        owner,
    )
}

fn expected_resolve(owner: ReducerRouteOwnerGroup) -> ReducerRouteEvent {
    route_resolve_battle_subject_without_fill_from_route_holes(
        ReducerRouteSubjectFamily::ScalarBuffEffect,
        Vec::new(),
        owner,
    )
}
