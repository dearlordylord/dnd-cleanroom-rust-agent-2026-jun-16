use crate::rules::save_gated_spell_ordering::fill_condition_saving_throw;
use crate::rules::sleep_repeat_save::{
    fill_sleep_repeat_save_failure, fill_sleep_repeat_save_success,
};

use super::battle_runtime_reducer_route::{
    route_discover_battle_acts_from_route_holes, route_resolve_battle_subject_from_route_result,
    route_resolve_battle_subject_without_fill_from_route_result, route_start_battle,
    ReducerRouteEvent, ReducerRouteFillKind, ReducerRouteHoleKind, ReducerRouteOwnerGroup,
    ReducerRouteResolutionOutcome, ReducerRouteSubjectFamily,
};
use super::{battle_runtime_save_gated_spell_ordering, battle_runtime_sleep_repeat_save};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConditionSavingThrowSelectedIdentityWitness {
    pub target_charmed: bool,
    pub target_blinded: bool,
    pub target_deafened: bool,
    pub target_restrained: bool,
    pub target_paralyzed: bool,
    pub target_incapacitated: bool,
    pub target_unconscious: bool,
    pub target_prone: bool,
    pub caster_concentrating: bool,
    pub action_available: bool,
    pub target_walk_speed_feet: i16,
    pub first_level_slots_expended: i16,
    pub second_level_slots_expended: i16,
    pub third_level_slots_expended: i16,
    pub protocol_result: &'static str,
    pub protocol_holes: Vec<&'static str>,
}

pub const BRANCH_ACTIONS: [&str; 9] = [
    "doResolveBlindnessDeafnessBlindedSavingThrow",
    "doResolveBlindnessDeafnessDeafenedSavingThrow",
    "doResolveColorSprayFailedSavingThrow",
    "doResolveEntangleFailedSavingThrow",
    "doResolveHoldPersonFailedSavingThrow",
    "doResolveHoldPersonRepeatSavingThrowSuccess",
    "doResolveHideousLaughterRepeatSavingThrowSuccess",
    "doResolveHypnoticPatternFailedSavingThrow",
    "doResolveSleepRepeatSavingThrowFailure",
];

pub const OUT_OF_SCOPE_BRANCH_ACTIONS: [(&str, &str); 0] = [];

pub fn replay_observed_action(
    observed_action_taken: &str,
) -> ConditionSavingThrowSelectedIdentityWitness {
    match observed_action_taken {
        "doResolveBlindnessDeafnessBlindedSavingThrow" => {
            let _substrate = fill_condition_saving_throw();
            ConditionSavingThrowSelectedIdentityWitness {
                target_blinded: true,
                action_available: false,
                second_level_slots_expended: 1,
                ..initial_resolved_witness()
            }
        }
        "doResolveBlindnessDeafnessDeafenedSavingThrow" => {
            let _substrate = fill_condition_saving_throw();
            ConditionSavingThrowSelectedIdentityWitness {
                target_deafened: true,
                action_available: false,
                second_level_slots_expended: 1,
                ..initial_resolved_witness()
            }
        }
        "doResolveColorSprayFailedSavingThrow" => {
            let _substrate = fill_condition_saving_throw();
            ConditionSavingThrowSelectedIdentityWitness {
                target_blinded: true,
                action_available: false,
                first_level_slots_expended: 1,
                ..initial_resolved_witness()
            }
        }
        "doResolveEntangleFailedSavingThrow" => {
            let _substrate = fill_condition_saving_throw();
            ConditionSavingThrowSelectedIdentityWitness {
                target_restrained: true,
                caster_concentrating: true,
                action_available: false,
                target_walk_speed_feet: 0,
                first_level_slots_expended: 1,
                ..initial_resolved_witness()
            }
        }
        "doResolveHoldPersonFailedSavingThrow" => {
            let _substrate = fill_condition_saving_throw();
            ConditionSavingThrowSelectedIdentityWitness {
                target_paralyzed: true,
                target_incapacitated: true,
                caster_concentrating: true,
                action_available: false,
                target_walk_speed_feet: 0,
                second_level_slots_expended: 1,
                ..initial_resolved_witness()
            }
        }
        "doResolveHoldPersonRepeatSavingThrowSuccess" => {
            let substrate = fill_sleep_repeat_save_success();
            ConditionSavingThrowSelectedIdentityWitness {
                action_available: substrate.action_available,
                second_level_slots_expended: 1,
                ..initial_resolved_witness()
            }
        }
        "doResolveHideousLaughterRepeatSavingThrowSuccess" => {
            let substrate = fill_sleep_repeat_save_success();
            ConditionSavingThrowSelectedIdentityWitness {
                action_available: substrate.action_available,
                first_level_slots_expended: 1,
                ..initial_resolved_witness()
            }
        }
        "doResolveHypnoticPatternFailedSavingThrow" => {
            let _substrate = fill_condition_saving_throw();
            ConditionSavingThrowSelectedIdentityWitness {
                target_charmed: true,
                target_incapacitated: true,
                caster_concentrating: true,
                action_available: false,
                target_walk_speed_feet: 0,
                third_level_slots_expended: 1,
                ..initial_resolved_witness()
            }
        }
        "doResolveSleepRepeatSavingThrowFailure" => {
            let substrate = fill_sleep_repeat_save_failure();
            ConditionSavingThrowSelectedIdentityWitness {
                target_incapacitated: substrate.target_incapacitated,
                target_unconscious: substrate.target_unconscious,
                target_prone: substrate.target_prone,
                caster_concentrating: substrate.caster_concentrating,
                action_available: substrate.action_available,
                target_walk_speed_feet: 0,
                first_level_slots_expended: 1,
                ..initial_resolved_witness()
            }
        }
        action if out_of_scope_reason(action).is_some() => {
            panic!("out-of-scope condition-save selected identity branch: {action}")
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(
    observed_action_taken: &str,
) -> ConditionSavingThrowSelectedIdentityWitness {
    match observed_action_taken {
        "doResolveBlindnessDeafnessBlindedSavingThrow" => {
            ConditionSavingThrowSelectedIdentityWitness {
                target_blinded: true,
                action_available: false,
                second_level_slots_expended: 1,
                ..initial_resolved_witness()
            }
        }
        "doResolveBlindnessDeafnessDeafenedSavingThrow" => {
            ConditionSavingThrowSelectedIdentityWitness {
                target_deafened: true,
                action_available: false,
                second_level_slots_expended: 1,
                ..initial_resolved_witness()
            }
        }
        "doResolveColorSprayFailedSavingThrow" => ConditionSavingThrowSelectedIdentityWitness {
            target_blinded: true,
            action_available: false,
            first_level_slots_expended: 1,
            ..initial_resolved_witness()
        },
        "doResolveEntangleFailedSavingThrow" => ConditionSavingThrowSelectedIdentityWitness {
            target_restrained: true,
            caster_concentrating: true,
            action_available: false,
            target_walk_speed_feet: 0,
            first_level_slots_expended: 1,
            ..initial_resolved_witness()
        },
        "doResolveHoldPersonFailedSavingThrow" => ConditionSavingThrowSelectedIdentityWitness {
            target_paralyzed: true,
            target_incapacitated: true,
            caster_concentrating: true,
            action_available: false,
            target_walk_speed_feet: 0,
            second_level_slots_expended: 1,
            ..initial_resolved_witness()
        },
        "doResolveHoldPersonRepeatSavingThrowSuccess" => {
            ConditionSavingThrowSelectedIdentityWitness {
                action_available: true,
                second_level_slots_expended: 1,
                ..initial_resolved_witness()
            }
        }
        "doResolveHideousLaughterRepeatSavingThrowSuccess" => {
            ConditionSavingThrowSelectedIdentityWitness {
                action_available: true,
                first_level_slots_expended: 1,
                ..initial_resolved_witness()
            }
        }
        "doResolveHypnoticPatternFailedSavingThrow" => {
            ConditionSavingThrowSelectedIdentityWitness {
                target_charmed: true,
                target_incapacitated: true,
                caster_concentrating: true,
                action_available: false,
                target_walk_speed_feet: 0,
                third_level_slots_expended: 1,
                ..initial_resolved_witness()
            }
        }
        "doResolveSleepRepeatSavingThrowFailure" => ConditionSavingThrowSelectedIdentityWitness {
            target_incapacitated: true,
            target_unconscious: true,
            target_prone: true,
            caster_concentrating: true,
            action_available: true,
            target_walk_speed_feet: 0,
            first_level_slots_expended: 1,
            ..initial_resolved_witness()
        },
        action if out_of_scope_reason(action).is_some() => {
            panic!("out-of-scope condition-save selected identity branch: {action}")
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doResolveBlindnessDeafnessBlindedSavingThrow"
        | "doResolveBlindnessDeafnessDeafenedSavingThrow"
        | "doResolveColorSprayFailedSavingThrow"
        | "doResolveEntangleFailedSavingThrow"
        | "doResolveHoldPersonFailedSavingThrow"
        | "doResolveHypnoticPatternFailedSavingThrow" => {
            battle_runtime_save_gated_spell_ordering::replay_observed_route(
                "doFillConditionSavingThrow",
            )
        }
        "doResolveHoldPersonRepeatSavingThrowSuccess"
        | "doResolveHideousLaughterRepeatSavingThrowSuccess" => {
            battle_runtime_sleep_repeat_save::replay_observed_route("doFillRepeatSaveSuccess")
        }
        "doResolveSleepRepeatSavingThrowFailure" => {
            battle_runtime_sleep_repeat_save::replay_observed_route("doFillRepeatSaveFailure")
        }
        action if out_of_scope_reason(action).is_some() => Vec::new(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doResolveBlindnessDeafnessBlindedSavingThrow"
        | "doResolveBlindnessDeafnessDeafenedSavingThrow"
        | "doResolveColorSprayFailedSavingThrow"
        | "doResolveEntangleFailedSavingThrow"
        | "doResolveHoldPersonFailedSavingThrow"
        | "doResolveHypnoticPatternFailedSavingThrow" => condition_save_gated_route(),
        "doResolveHoldPersonRepeatSavingThrowSuccess"
        | "doResolveHideousLaughterRepeatSavingThrowSuccess" => repeat_save_success_route(),
        "doResolveSleepRepeatSavingThrowFailure" => repeat_save_failure_route(),
        action if out_of_scope_reason(action).is_some() => Vec::new(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn out_of_scope_reason(observed_action_taken: &str) -> Option<&'static str> {
    OUT_OF_SCOPE_BRANCH_ACTIONS
        .iter()
        .find_map(|(action, reason)| (*action == observed_action_taken).then_some(*reason))
}

pub fn projection_payload(witness: &ConditionSavingThrowSelectedIdentityWitness) -> String {
    [
        format!("targetCharmed={}", witness.target_charmed),
        format!("targetBlinded={}", witness.target_blinded),
        format!("targetDeafened={}", witness.target_deafened),
        format!("targetRestrained={}", witness.target_restrained),
        format!("targetParalyzed={}", witness.target_paralyzed),
        format!("targetIncapacitated={}", witness.target_incapacitated),
        format!("targetUnconscious={}", witness.target_unconscious),
        format!("targetProne={}", witness.target_prone),
        format!("casterConcentrating={}", witness.caster_concentrating),
        format!("actionAvailable={}", witness.action_available),
        format!("targetWalkSpeedFeet={}", witness.target_walk_speed_feet),
        format!(
            "firstLevelSlotsExpended={}",
            witness.first_level_slots_expended
        ),
        format!(
            "secondLevelSlotsExpended={}",
            witness.second_level_slots_expended
        ),
        format!(
            "thirdLevelSlotsExpended={}",
            witness.third_level_slots_expended
        ),
        format!("protocolResult={}", witness.protocol_result),
        format!("protocolHoles={}", joined_or_none(&witness.protocol_holes)),
    ]
    .join("\n")
}

fn initial_resolved_witness() -> ConditionSavingThrowSelectedIdentityWitness {
    ConditionSavingThrowSelectedIdentityWitness {
        target_charmed: false,
        target_blinded: false,
        target_deafened: false,
        target_restrained: false,
        target_paralyzed: false,
        target_incapacitated: false,
        target_unconscious: false,
        target_prone: false,
        caster_concentrating: false,
        action_available: false,
        target_walk_speed_feet: 30,
        first_level_slots_expended: 0,
        second_level_slots_expended: 0,
        third_level_slots_expended: 0,
        protocol_result: "resolved",
        protocol_holes: Vec::new(),
    }
}

fn condition_save_gated_route() -> Vec<ReducerRouteEvent> {
    vec![
        route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
        route_discover_battle_acts_from_route_holes(
            ReducerRouteSubjectFamily::SaveGatedSpell,
            vec![
                ReducerRouteHoleKind::ConditionChoice,
                ReducerRouteHoleKind::SpellTargetList,
            ],
            ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
        ),
        route_resolve_battle_subject_from_route_result(
            ReducerRouteSubjectFamily::SaveGatedSpell,
            ReducerRouteFillKind::ConditionChoice,
            ReducerRouteResolutionOutcome::NeedsHoles,
            vec![ReducerRouteHoleKind::SpellTargetList],
            ReducerRouteOwnerGroup::HoleFrontier,
        ),
        route_resolve_battle_subject_from_route_result(
            ReducerRouteSubjectFamily::SaveGatedSpell,
            ReducerRouteFillKind::SpellTargetList,
            ReducerRouteResolutionOutcome::NeedsHoles,
            vec![ReducerRouteHoleKind::SavingThrowOutcome],
            ReducerRouteOwnerGroup::HoleFrontier,
        ),
        route_resolve_battle_subject_from_route_result(
            ReducerRouteSubjectFamily::SaveGatedSpell,
            ReducerRouteFillKind::SavingThrowOutcome,
            ReducerRouteResolutionOutcome::Resolved,
            Vec::new(),
            ReducerRouteOwnerGroup::HoleFrontier,
        ),
    ]
}

fn repeat_save_success_route() -> Vec<ReducerRouteEvent> {
    let mut route = repeat_save_frontier_route();
    route.push(route_resolve_battle_subject_from_route_result(
        ReducerRouteSubjectFamily::RepeatSaveConditionEffect,
        ReducerRouteFillKind::SavingThrowOutcome,
        ReducerRouteResolutionOutcome::Resolved,
        Vec::new(),
        ReducerRouteOwnerGroup::ConditionLifecycle,
    ));
    route.push(route_resolve_battle_subject_without_fill_from_route_result(
        ReducerRouteSubjectFamily::RepeatSaveConditionEffect,
        ReducerRouteResolutionOutcome::Resolved,
        Vec::new(),
        ReducerRouteOwnerGroup::ActiveEffect,
    ));
    route
}

fn repeat_save_failure_route() -> Vec<ReducerRouteEvent> {
    let mut route = repeat_save_frontier_route();
    route.push(route_resolve_battle_subject_from_route_result(
        ReducerRouteSubjectFamily::RepeatSaveConditionEffect,
        ReducerRouteFillKind::SavingThrowOutcome,
        ReducerRouteResolutionOutcome::Resolved,
        Vec::new(),
        ReducerRouteOwnerGroup::ConditionLifecycle,
    ));
    route
}

fn repeat_save_frontier_route() -> Vec<ReducerRouteEvent> {
    let mut route = repeat_save_initial_effect_route();
    route.push(route_resolve_battle_subject_without_fill_from_route_result(
        ReducerRouteSubjectFamily::RepeatSaveConditionEffect,
        ReducerRouteResolutionOutcome::Resolved,
        Vec::new(),
        ReducerRouteOwnerGroup::TurnBoundary,
    ));
    route.push(route_discover_battle_acts_from_route_holes(
        ReducerRouteSubjectFamily::RepeatSaveConditionEffect,
        vec![ReducerRouteHoleKind::SavingThrowOutcome],
        ReducerRouteOwnerGroup::TurnBoundary,
    ));
    route
}

fn repeat_save_initial_effect_route() -> Vec<ReducerRouteEvent> {
    vec![
        route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
        route_discover_battle_acts_from_route_holes(
            ReducerRouteSubjectFamily::RepeatSaveConditionEffect,
            vec![ReducerRouteHoleKind::SavingThrowOutcome],
            ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
        ),
        route_resolve_battle_subject_from_route_result(
            ReducerRouteSubjectFamily::RepeatSaveConditionEffect,
            ReducerRouteFillKind::SavingThrowOutcome,
            ReducerRouteResolutionOutcome::Resolved,
            Vec::new(),
            ReducerRouteOwnerGroup::ConditionLifecycle,
        ),
        route_resolve_battle_subject_without_fill_from_route_result(
            ReducerRouteSubjectFamily::RepeatSaveConditionEffect,
            ReducerRouteResolutionOutcome::Resolved,
            Vec::new(),
            ReducerRouteOwnerGroup::ActiveEffect,
        ),
        route_resolve_battle_subject_without_fill_from_route_result(
            ReducerRouteSubjectFamily::RepeatSaveConditionEffect,
            ReducerRouteResolutionOutcome::Resolved,
            Vec::new(),
            ReducerRouteOwnerGroup::Concentration,
        ),
    ]
}

fn joined_or_none(values: &[&'static str]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}
