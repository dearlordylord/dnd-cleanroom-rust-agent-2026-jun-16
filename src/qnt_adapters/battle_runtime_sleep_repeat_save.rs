use crate::rules::battle_reducer_spine::{BattleGenericRouteFill, BattleSubjectKind};
use crate::rules::sleep_repeat_save::{
    break_sleep_concentration_before_repeat, discover_sleep_repeat_save,
    end_caster_turn_after_sleep_concentration_break, end_caster_turn_with_sleep,
    end_target_turn_after_sleep_concentration_break, fill_sleep_initial_save_failure,
    fill_sleep_repeat_save_failure, fill_sleep_repeat_save_success, SleepRepeatSaveHole,
    SleepRepeatSaveProtocol, SleepRepeatSaveState, SleepRepeatSaveTurnRole,
};

use super::battle_runtime_reducer_route::{
    replay_generic_battle_route, GenericBattleRouteStep, ReducerRouteEvent,
};

pub const BRANCH_ACTIONS: [&str; 8] = [
    "doFillInitialSaveFailure",
    "doBreakConcentrationBeforeRepeat",
    "doEndCasterTurn",
    "doEndCasterTurnAfterConcentrationBreak",
    "doEndTargetTurnAfterConcentrationBreak",
    "doDiscoverRepeatSave",
    "doFillRepeatSaveSuccess",
    "doFillRepeatSaveFailure",
];

pub fn replay_observed_action(observed_action_taken: &str) -> SleepRepeatSaveState {
    match observed_action_taken {
        "doFillInitialSaveFailure" => fill_sleep_initial_save_failure(),
        "doBreakConcentrationBeforeRepeat" => break_sleep_concentration_before_repeat(),
        "doEndCasterTurn" => end_caster_turn_with_sleep(),
        "doEndCasterTurnAfterConcentrationBreak" => {
            end_caster_turn_after_sleep_concentration_break()
        }
        "doEndTargetTurnAfterConcentrationBreak" => {
            end_target_turn_after_sleep_concentration_break()
        }
        "doDiscoverRepeatSave" => discover_sleep_repeat_save(),
        "doFillRepeatSaveSuccess" => fill_sleep_repeat_save_success(),
        "doFillRepeatSaveFailure" => fill_sleep_repeat_save_failure(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> SleepRepeatSaveState {
    match observed_action_taken {
        "doFillInitialSaveFailure" => SleepRepeatSaveState {
            current_turn_role: SleepRepeatSaveTurnRole::Caster,
            target_incapacitated: true,
            target_unconscious: false,
            target_prone: false,
            caster_concentrating: true,
            action_available: false,
            protocol: SleepRepeatSaveProtocol::Resolved,
            protocol_holes: Vec::new(),
        },
        "doBreakConcentrationBeforeRepeat" => SleepRepeatSaveState {
            target_incapacitated: false,
            caster_concentrating: false,
            ..expected_witness("doFillInitialSaveFailure")
        },
        "doEndCasterTurn" => SleepRepeatSaveState {
            current_turn_role: SleepRepeatSaveTurnRole::Target,
            action_available: true,
            ..expected_witness("doFillInitialSaveFailure")
        },
        "doEndCasterTurnAfterConcentrationBreak" => SleepRepeatSaveState {
            current_turn_role: SleepRepeatSaveTurnRole::Target,
            action_available: true,
            ..expected_witness("doBreakConcentrationBeforeRepeat")
        },
        "doEndTargetTurnAfterConcentrationBreak" => SleepRepeatSaveState {
            current_turn_role: SleepRepeatSaveTurnRole::Caster,
            ..expected_witness("doEndCasterTurnAfterConcentrationBreak")
        },
        "doDiscoverRepeatSave" => SleepRepeatSaveState {
            protocol: SleepRepeatSaveProtocol::NeedsHoles,
            protocol_holes: vec![SleepRepeatSaveHole::SavingThrowOutcome],
            ..expected_witness("doEndCasterTurn")
        },
        "doFillRepeatSaveSuccess" => SleepRepeatSaveState {
            current_turn_role: SleepRepeatSaveTurnRole::Caster,
            target_incapacitated: false,
            action_available: true,
            ..expected_witness("doFillInitialSaveFailure")
        },
        "doFillRepeatSaveFailure" => SleepRepeatSaveState {
            current_turn_role: SleepRepeatSaveTurnRole::Caster,
            target_unconscious: true,
            target_prone: true,
            action_available: true,
            ..expected_witness("doFillInitialSaveFailure")
        },
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doFillInitialSaveFailure" => initial_save_failure_route(),
        "doBreakConcentrationBeforeRepeat" => {
            let mut route = initial_save_failure_route();
            append_concentration_break_before_repeat_route(&mut route);
            route
        }
        "doEndCasterTurn" => {
            let mut route = initial_save_failure_route();
            append_end_caster_turn_route(&mut route);
            route
        }
        "doEndCasterTurnAfterConcentrationBreak" => {
            let mut route = replay_observed_route("doBreakConcentrationBeforeRepeat");
            append_end_caster_turn_route(&mut route);
            route
        }
        "doEndTargetTurnAfterConcentrationBreak" => {
            let mut route = replay_observed_route("doEndCasterTurnAfterConcentrationBreak");
            append_turn_boundary_route(&mut route);
            route
        }
        "doDiscoverRepeatSave" => {
            let mut route = replay_observed_route("doEndCasterTurn");
            route.extend(
                replay_generic_battle_route(&[discover(
                    BattleSubjectKind::RepeatSaveConditionEffectRepeatSave,
                )])
                .into_iter()
                .skip(1),
            );
            route
        }
        "doFillRepeatSaveSuccess" => {
            let mut route = replay_observed_route("doDiscoverRepeatSave");
            route.extend(
                replay_generic_battle_route(&[
                    resolve(
                        BattleSubjectKind::RepeatSaveConditionEffectRepeatSave,
                        BattleGenericRouteFill::SavingThrowOutcome,
                    ),
                    resolve(
                        BattleSubjectKind::RepeatSaveConditionEffectActiveEffect,
                        BattleGenericRouteFill::WithoutFill,
                    ),
                ])
                .into_iter()
                .skip(1),
            );
            route
        }
        "doFillRepeatSaveFailure" => {
            let mut route = replay_observed_route("doDiscoverRepeatSave");
            route.extend(
                replay_generic_battle_route(&[resolve(
                    BattleSubjectKind::RepeatSaveConditionEffectRepeatSave,
                    BattleGenericRouteFill::SavingThrowOutcome,
                )])
                .into_iter()
                .skip(1),
            );
            route
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    replay_observed_route(observed_action_taken)
}

pub fn projection_payload(state: &SleepRepeatSaveState) -> String {
    [
        format!("currentTurnRole={}", turn_role_ref(state.current_turn_role)),
        format!("targetIncapacitated={}", state.target_incapacitated),
        format!("targetUnconscious={}", state.target_unconscious),
        format!("targetProne={}", state.target_prone),
        format!("casterConcentrating={}", state.caster_concentrating),
        format!("actionAvailable={}", state.action_available),
        format!("protocolResult={}", protocol_ref(state.protocol)),
        format!("protocolHoles={}", joined_holes(&state.protocol_holes)),
    ]
    .join("\n")
}

fn turn_role_ref(role: SleepRepeatSaveTurnRole) -> &'static str {
    match role {
        SleepRepeatSaveTurnRole::Caster => "caster",
        SleepRepeatSaveTurnRole::Target => "target",
    }
}

fn protocol_ref(protocol: SleepRepeatSaveProtocol) -> &'static str {
    match protocol {
        SleepRepeatSaveProtocol::Init => "init",
        SleepRepeatSaveProtocol::NeedsHoles => "needsHoles",
        SleepRepeatSaveProtocol::Resolved => "resolved",
    }
}

fn joined_holes(holes: &[SleepRepeatSaveHole]) -> String {
    if holes.is_empty() {
        return "none".to_string();
    }

    holes
        .iter()
        .map(|hole| match hole {
            SleepRepeatSaveHole::SavingThrowOutcome => "SavingThrowOutcome",
        })
        .collect::<Vec<_>>()
        .join(",")
}

fn initial_save_failure_route() -> Vec<ReducerRouteEvent> {
    replay_generic_battle_route(&[
        discover(BattleSubjectKind::RepeatSaveConditionEffectInitialSave),
        resolve(
            BattleSubjectKind::RepeatSaveConditionEffectInitialSave,
            BattleGenericRouteFill::SavingThrowOutcome,
        ),
        resolve(
            BattleSubjectKind::RepeatSaveConditionEffectActiveEffect,
            BattleGenericRouteFill::WithoutFill,
        ),
        resolve(
            BattleSubjectKind::RepeatSaveConditionEffectConcentration,
            BattleGenericRouteFill::WithoutFill,
        ),
    ])
}

fn append_concentration_break_before_repeat_route(route: &mut Vec<ReducerRouteEvent>) {
    route.extend(
        replay_generic_battle_route(&[
            resolve(
                BattleSubjectKind::RepeatSaveConditionEffectConcentration,
                BattleGenericRouteFill::WithoutFill,
            ),
            resolve(
                BattleSubjectKind::RepeatSaveConditionEffectActiveEffect,
                BattleGenericRouteFill::WithoutFill,
            ),
            resolve(
                BattleSubjectKind::RepeatSaveConditionEffectConditionLifecycle,
                BattleGenericRouteFill::WithoutFill,
            ),
        ])
        .into_iter()
        .skip(1),
    );
}

fn append_end_caster_turn_route(route: &mut Vec<ReducerRouteEvent>) {
    append_turn_boundary_route(route);
}

fn append_turn_boundary_route(route: &mut Vec<ReducerRouteEvent>) {
    route.extend(
        replay_generic_battle_route(&[resolve(
            BattleSubjectKind::RepeatSaveConditionEffectTurnBoundary,
            BattleGenericRouteFill::WithoutFill,
        )])
        .into_iter()
        .skip(1),
    );
}

fn discover(kind: BattleSubjectKind) -> GenericBattleRouteStep {
    GenericBattleRouteStep::Discover(kind)
}

fn resolve(kind: BattleSubjectKind, fill: BattleGenericRouteFill) -> GenericBattleRouteStep {
    GenericBattleRouteStep::Resolve { kind, fill }
}
