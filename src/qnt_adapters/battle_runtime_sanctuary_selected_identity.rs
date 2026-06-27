use crate::rules::battle_reducer_spine::{BattleGenericRouteFill, BattleSubjectKind};
use crate::rules::sanctuary_selected_identity::{
    cast_sanctuary_ward_creation, end_ward_on_warded_attack_roll, end_ward_on_warded_damage_dealt,
    end_ward_on_warded_spell_cast, exclude_area_effect_from_interdiction,
    interdict_direct_attack_failed_save_loss, interdict_direct_spell_successful_save_pass_through,
    reject_illegal_replacement_target, retarget_direct_attack_to_legal_replacement,
    SanctuarySelectedIdentityProtocol, SanctuarySelectedIdentityScenarioOutcome,
    SanctuarySelectedIdentityState,
};

use super::battle_runtime_reducer_route::{
    replay_generic_battle_route, route_discover_battle_acts_from_route_holes,
    route_resolve_battle_subject_from_route_result,
    route_resolve_battle_subject_without_fill_from_route_result, route_start_battle,
    GenericBattleRouteStep, ReducerRouteEvent, ReducerRouteFillKind, ReducerRouteHoleKind,
    ReducerRouteOwnerGroup, ReducerRouteResolutionOutcome, ReducerRouteSubjectFamily,
};

pub const BRANCH_ACTIONS: [&str; 9] = [
    "doCastSanctuaryWardCreation",
    "doInterdictDirectAttackFailedSaveLoss",
    "doInterdictDirectSpellSuccessfulSavePassThrough",
    "doRetargetDirectAttackToLegalReplacement",
    "doRejectIllegalReplacementTarget",
    "doExcludeAreaEffectFromInterdiction",
    "doEndWardOnWardedAttackRoll",
    "doEndWardOnWardedSpellCast",
    "doEndWardOnWardedDamageDealt",
];

pub fn replay_observed_action(observed_action_taken: &str) -> SanctuarySelectedIdentityState {
    match observed_action_taken {
        "doCastSanctuaryWardCreation" => cast_sanctuary_ward_creation(),
        "doInterdictDirectAttackFailedSaveLoss" => interdict_direct_attack_failed_save_loss(),
        "doInterdictDirectSpellSuccessfulSavePassThrough" => {
            interdict_direct_spell_successful_save_pass_through()
        }
        "doRetargetDirectAttackToLegalReplacement" => retarget_direct_attack_to_legal_replacement(),
        "doRejectIllegalReplacementTarget" => reject_illegal_replacement_target(),
        "doExcludeAreaEffectFromInterdiction" => exclude_area_effect_from_interdiction(),
        "doEndWardOnWardedAttackRoll" => end_ward_on_warded_attack_roll(),
        "doEndWardOnWardedSpellCast" => end_ward_on_warded_spell_cast(),
        "doEndWardOnWardedDamageDealt" => end_ward_on_warded_damage_dealt(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> SanctuarySelectedIdentityState {
    replay_observed_action(observed_action_taken)
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    replay_generic_battle_route(&observed_route_steps(observed_action_taken))
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doCastSanctuaryWardCreation" => vec![
            start_route(),
            discover_ward(
                vec![ReducerRouteHoleKind::TargetChoice],
                ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
            ),
            resolve_ward(
                ReducerRouteFillKind::TargetChoice,
                Vec::new(),
                ReducerRouteOwnerGroup::TargetSelection,
            ),
            resolve_ward_without_fill(Vec::new(), ReducerRouteOwnerGroup::ActiveEffect),
        ],
        "doInterdictDirectAttackFailedSaveLoss" => vec![
            start_route(),
            discover_ward(
                vec![ReducerRouteHoleKind::SanctuaryInterdictionOutcome],
                ReducerRouteOwnerGroup::ActiveEffect,
            ),
            resolve_ward(
                ReducerRouteFillKind::SanctuaryInterdictionOutcome,
                Vec::new(),
                ReducerRouteOwnerGroup::SavingThrowOutcome,
            ),
            resolve_ward_without_fill(Vec::new(), ReducerRouteOwnerGroup::ActionEconomy),
        ],
        "doInterdictDirectSpellSuccessfulSavePassThrough" => vec![
            start_route(),
            discover_ward(
                vec![ReducerRouteHoleKind::SanctuaryInterdictionOutcome],
                ReducerRouteOwnerGroup::ActiveEffect,
            ),
            resolve_ward(
                ReducerRouteFillKind::SanctuaryInterdictionOutcome,
                Vec::new(),
                ReducerRouteOwnerGroup::SavingThrowOutcome,
            ),
            resolve_ward_without_fill(Vec::new(), ReducerRouteOwnerGroup::HoleFrontier),
        ],
        "doRetargetDirectAttackToLegalReplacement" => replacement_route(Vec::new()),
        "doRejectIllegalReplacementTarget" => {
            replacement_route(vec![ReducerRouteHoleKind::TargetChoice])
        }
        "doExcludeAreaEffectFromInterdiction" => vec![
            start_route(),
            discover_ward(Vec::new(), ReducerRouteOwnerGroup::AreaShape),
            resolve_ward_without_fill(Vec::new(), ReducerRouteOwnerGroup::AreaShape),
        ],
        "doEndWardOnWardedAttackRoll" => early_end_route(ReducerRouteOwnerGroup::AttackRoll),
        "doEndWardOnWardedSpellCast" => {
            early_end_route(ReducerRouteOwnerGroup::SpellSlotAndActionEconomy)
        }
        "doEndWardOnWardedDamageDealt" => early_end_route(ReducerRouteOwnerGroup::HitPoint),
        action => panic!("unsupported route mbt::actionTaken {action}"),
    }
}

pub fn projection_payload(state: &SanctuarySelectedIdentityState) -> String {
    [
        format!("qWardPresent={}", state.ward_present),
        format!("qWardSourceIsSanctuary={}", state.ward_source_is_sanctuary),
        format!("qWisdomSaveRequested={}", state.wisdom_save_requested),
        format!("qAttackOrSpellLost={}", state.attack_or_spell_lost),
        format!(
            "qSuccessfulSavePassThrough={}",
            state.successful_save_pass_through
        ),
        format!(
            "qLegalReplacementPassThrough={}",
            state.legal_replacement_pass_through
        ),
        format!(
            "qIllegalReplacementRejected={}",
            state.illegal_replacement_rejected
        ),
        format!(
            "qAreaEffectBypassedInterdiction={}",
            state.area_effect_bypassed_interdiction
        ),
        format!("qWardedHp={}", state.warded_hp),
        format!(
            "qScenarioOutcome={}",
            scenario_outcome_ref(state.scenario_outcome)
        ),
        format!("protocolResult={}", protocol_ref(state.protocol)),
        "protocolHoles=none".to_string(),
    ]
    .join("\n")
}

fn observed_route_steps(observed_action_taken: &str) -> Vec<GenericBattleRouteStep> {
    use BattleGenericRouteFill::{SanctuaryInterdictionOutcome, TargetChoice, WithoutFill};
    use BattleSubjectKind::{
        WardedAreaEffectExclusion, WardedAttackRollEarlyEnd, WardedDamageEarlyEnd,
        WardedInterdictionActionEconomy, WardedInterdictionHoleFrontier, WardedInterdictionOutcome,
        WardedInterdictionOutcomeThenTargetChoice, WardedInterdictionReplacementRejected,
        WardedInterdictionReplacementTarget, WardedSpellCastEarlyEnd,
        WardedTargetCreationActiveEffect, WardedTargetCreationTargetChoice,
    };
    use GenericBattleRouteStep::{Discover, Resolve};

    match observed_action_taken {
        "doCastSanctuaryWardCreation" => vec![
            Discover(WardedTargetCreationTargetChoice),
            Resolve {
                kind: WardedTargetCreationTargetChoice,
                fill: TargetChoice,
            },
            Resolve {
                kind: WardedTargetCreationActiveEffect,
                fill: WithoutFill,
            },
        ],
        "doInterdictDirectAttackFailedSaveLoss" => vec![
            Discover(WardedInterdictionOutcome),
            Resolve {
                kind: WardedInterdictionOutcome,
                fill: SanctuaryInterdictionOutcome,
            },
            Resolve {
                kind: WardedInterdictionActionEconomy,
                fill: WithoutFill,
            },
        ],
        "doInterdictDirectSpellSuccessfulSavePassThrough" => vec![
            Discover(WardedInterdictionOutcome),
            Resolve {
                kind: WardedInterdictionOutcome,
                fill: SanctuaryInterdictionOutcome,
            },
            Resolve {
                kind: WardedInterdictionHoleFrontier,
                fill: WithoutFill,
            },
        ],
        "doRetargetDirectAttackToLegalReplacement" => vec![
            Discover(WardedInterdictionOutcomeThenTargetChoice),
            Resolve {
                kind: WardedInterdictionOutcomeThenTargetChoice,
                fill: SanctuaryInterdictionOutcome,
            },
            Resolve {
                kind: WardedInterdictionReplacementTarget,
                fill: TargetChoice,
            },
        ],
        "doRejectIllegalReplacementTarget" => vec![
            Discover(WardedInterdictionOutcomeThenTargetChoice),
            Resolve {
                kind: WardedInterdictionOutcomeThenTargetChoice,
                fill: SanctuaryInterdictionOutcome,
            },
            Resolve {
                kind: WardedInterdictionReplacementRejected,
                fill: TargetChoice,
            },
        ],
        "doExcludeAreaEffectFromInterdiction" => vec![
            Discover(WardedAreaEffectExclusion),
            Resolve {
                kind: WardedAreaEffectExclusion,
                fill: WithoutFill,
            },
        ],
        "doEndWardOnWardedAttackRoll" => early_end_steps(WardedAttackRollEarlyEnd),
        "doEndWardOnWardedSpellCast" => early_end_steps(WardedSpellCastEarlyEnd),
        "doEndWardOnWardedDamageDealt" => early_end_steps(WardedDamageEarlyEnd),
        action => panic!("unsupported route mbt::actionTaken {action}"),
    }
}

fn early_end_steps(first_resolution: BattleSubjectKind) -> Vec<GenericBattleRouteStep> {
    use BattleGenericRouteFill::WithoutFill;
    use BattleSubjectKind::WardedActiveEffectCleanup;
    use GenericBattleRouteStep::{Discover, Resolve};

    vec![
        Discover(first_resolution),
        Resolve {
            kind: first_resolution,
            fill: WithoutFill,
        },
        Resolve {
            kind: WardedActiveEffectCleanup,
            fill: WithoutFill,
        },
    ]
}

fn replacement_route(final_holes: Vec<ReducerRouteHoleKind>) -> Vec<ReducerRouteEvent> {
    vec![
        start_route(),
        discover_ward(
            vec![
                ReducerRouteHoleKind::SanctuaryInterdictionOutcome,
                ReducerRouteHoleKind::TargetChoice,
            ],
            ReducerRouteOwnerGroup::ActiveEffect,
        ),
        resolve_ward(
            ReducerRouteFillKind::SanctuaryInterdictionOutcome,
            vec![ReducerRouteHoleKind::TargetChoice],
            ReducerRouteOwnerGroup::SavingThrowOutcome,
        ),
        resolve_ward(
            ReducerRouteFillKind::TargetChoice,
            final_holes,
            ReducerRouteOwnerGroup::TargetSelection,
        ),
    ]
}

fn early_end_route(first_owner: ReducerRouteOwnerGroup) -> Vec<ReducerRouteEvent> {
    vec![
        start_route(),
        discover_ward(Vec::new(), ReducerRouteOwnerGroup::ActiveEffect),
        resolve_ward_without_fill(Vec::new(), first_owner),
        resolve_ward_without_fill(Vec::new(), ReducerRouteOwnerGroup::ActiveEffect),
    ]
}

fn start_route() -> ReducerRouteEvent {
    route_start_battle(ReducerRouteOwnerGroup::ActionEconomy)
}

fn discover_ward(
    holes: Vec<ReducerRouteHoleKind>,
    owner: ReducerRouteOwnerGroup,
) -> ReducerRouteEvent {
    route_discover_battle_acts_from_route_holes(
        ReducerRouteSubjectFamily::WardedTargetInterdiction,
        holes,
        owner,
    )
}

fn resolve_ward(
    fill: ReducerRouteFillKind,
    holes: Vec<ReducerRouteHoleKind>,
    owner: ReducerRouteOwnerGroup,
) -> ReducerRouteEvent {
    route_resolve_battle_subject_from_route_result(
        ReducerRouteSubjectFamily::WardedTargetInterdiction,
        fill,
        route_outcome(&holes),
        holes,
        owner,
    )
}

fn resolve_ward_without_fill(
    holes: Vec<ReducerRouteHoleKind>,
    owner: ReducerRouteOwnerGroup,
) -> ReducerRouteEvent {
    route_resolve_battle_subject_without_fill_from_route_result(
        ReducerRouteSubjectFamily::WardedTargetInterdiction,
        route_outcome(&holes),
        holes,
        owner,
    )
}

fn route_outcome(holes: &[ReducerRouteHoleKind]) -> ReducerRouteResolutionOutcome {
    if holes.is_empty() {
        ReducerRouteResolutionOutcome::Resolved
    } else {
        ReducerRouteResolutionOutcome::NeedsHoles
    }
}

fn scenario_outcome_ref(outcome: SanctuarySelectedIdentityScenarioOutcome) -> &'static str {
    match outcome {
        SanctuarySelectedIdentityScenarioOutcome::Init => "Init",
        SanctuarySelectedIdentityScenarioOutcome::WardCreated => "WardCreated",
        SanctuarySelectedIdentityScenarioOutcome::AttackLost => "AttackLost",
        SanctuarySelectedIdentityScenarioOutcome::SpellSaveSucceeded => "SpellSaveSucceeded",
        SanctuarySelectedIdentityScenarioOutcome::ReplacementAdmitted => "ReplacementAdmitted",
        SanctuarySelectedIdentityScenarioOutcome::ReplacementRejected => "ReplacementRejected",
        SanctuarySelectedIdentityScenarioOutcome::AreaEffectExcluded => "AreaEffectExcluded",
        SanctuarySelectedIdentityScenarioOutcome::AttackRollEndedWard => "AttackRollEndedWard",
        SanctuarySelectedIdentityScenarioOutcome::SpellCastEndedWard => "SpellCastEndedWard",
        SanctuarySelectedIdentityScenarioOutcome::DamageEndedWard => "DamageEndedWard",
    }
}

fn protocol_ref(protocol: SanctuarySelectedIdentityProtocol) -> &'static str {
    match protocol {
        SanctuarySelectedIdentityProtocol::Init => "init",
        SanctuarySelectedIdentityProtocol::Resolved => "resolved",
    }
}
