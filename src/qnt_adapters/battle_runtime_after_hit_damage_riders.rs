use super::battle_runtime_reducer_route::{
    observed_reducer_route, route_discover_battle_acts_from_route_holes,
    route_resolve_battle_subject_from_route_result,
    route_resolve_battle_subject_without_fill_from_route_result, route_start_battle,
    ReducerRouteEvent, ReducerRouteFillKind, ReducerRouteHoleKind, ReducerRouteOwnerGroup,
    ReducerRouteResolutionOutcome, ReducerRouteSubjectFamily,
};
use crate::rules::battle_reducer_spine::{
    discover_generic_route_subject_observed, resolve_battle_subject_observed,
    start_battle_observed, BattleEntrypointTrace, BattleGenericRouteFill, BattleResolutionRequest,
    BattleSetup, BattleSubjectKind,
};

pub const BRANCH_ACTIONS: [&str; 26] = [
    "doDiscoverWeaponHit",
    "doFillTargetChoice",
    "doFillHitAttackRoll",
    "doChooseDivineSmiteSlot",
    "doChooseDivineSmiteFreeCast",
    "doChooseEnsnaringStrike",
    "doFillEnsnaringFailedSave",
    "doFillEnsnaringSuccessfulSave",
    "doChooseSearingSmite",
    "doChooseShiningSmite",
    "doFillDivineSmiteDamage",
    "doFillEnsnaringWeaponDamage",
    "doFillSearingSmiteDamage",
    "doFillShiningSmiteDamage",
    "doDiscoverEnsnaringStartTurnDamage",
    "doFillEnsnaringStartTurnDamage",
    "doFillEnsnaringEscapeCheck",
    "doDiscoverSearingStartTurnDamageAndSave",
    "doFillSearingStartTurnDamageAndSave",
    "doBreakShiningConcentration",
    "doStartDivineSmiteFreeCast",
    "doStartEnsnaringFailedSave",
    "doStartEnsnaringSuccessfulSave",
    "doStartSearingSmite",
    "doStartShiningSmite",
    "doFinish",
];

pub const ACCEPTED_ROUTE_BRANCH_ACTIONS: [&str; 15] = [
    "doFillHitAttackRoll",
    "doChooseDivineSmiteSlot",
    "doChooseDivineSmiteFreeCast",
    "doChooseEnsnaringStrike",
    "doFillEnsnaringFailedSave",
    "doFillEnsnaringSuccessfulSave",
    "doChooseSearingSmite",
    "doFillDivineSmiteDamage",
    "doFillEnsnaringWeaponDamage",
    "doFillSearingSmiteDamage",
    "doDiscoverEnsnaringStartTurnDamage",
    "doFillEnsnaringStartTurnDamage",
    "doFillEnsnaringEscapeCheck",
    "doDiscoverSearingStartTurnDamageAndSave",
    "doFillSearingStartTurnDamageAndSave",
];

pub const BLOCKED_ROUTE_BRANCH_ACTIONS: [(&str, &str); 11] = [
    (
        "doDiscoverWeaponHit",
        "pre-hit weapon target discovery is outside the after-hit rider route substrate",
    ),
    (
        "doFillTargetChoice",
        "pre-hit weapon target fill is outside the after-hit rider route substrate",
    ),
    (
        "doChooseShiningSmite",
        "Shining Smite rows are out of scope in the copied level-1 inventory",
    ),
    (
        "doFillShiningSmiteDamage",
        "Shining Smite rows are out of scope in the copied level-1 inventory",
    ),
    (
        "doBreakShiningConcentration",
        "Shining Smite rows are out of scope in the copied level-1 inventory",
    ),
    (
        "doStartDivineSmiteFreeCast",
        "scenario transition has no reducer route surface",
    ),
    (
        "doStartEnsnaringFailedSave",
        "scenario transition has no reducer route surface",
    ),
    (
        "doStartEnsnaringSuccessfulSave",
        "scenario transition has no reducer route surface",
    ),
    (
        "doStartSearingSmite",
        "scenario transition has no reducer route surface",
    ),
    (
        "doStartShiningSmite",
        "Shining Smite rows are out of scope in the copied level-1 inventory",
    ),
    (
        "doFinish",
        "scenario transition has no reducer route surface",
    ),
];

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doFillHitAttackRoll" => replay_generic_route_sequence(&[(
            BattleSubjectKind::AfterHitDamageRiderInterruptDecision,
            BattleGenericRouteFill::InterruptDecision,
        )]),
        "doChooseDivineSmiteSlot" | "doChooseSearingSmite" => replay_generic_route_sequence(&[(
            BattleSubjectKind::AfterHitDamageRiderSlotSpend,
            BattleGenericRouteFill::InterruptDecision,
        )]),
        "doChooseDivineSmiteFreeCast" => replay_generic_route_sequence(&[(
            BattleSubjectKind::AfterHitDamageRiderFreeCastSpend,
            BattleGenericRouteFill::InterruptDecision,
        )]),
        "doChooseEnsnaringStrike" => replay_generic_route_sequence(&[(
            BattleSubjectKind::AfterHitDamageRiderSaveGatedInterruptDecision,
            BattleGenericRouteFill::InterruptDecision,
        )]),
        "doFillEnsnaringFailedSave" => replay_generic_route_sequence(&[
            (
                BattleSubjectKind::AfterHitDamageRiderSaveGatedSlotAndActionEconomySpend,
                BattleGenericRouteFill::SavingThrowOutcome,
            ),
            (
                BattleSubjectKind::AfterHitDamageRiderSaveGatedCondition,
                BattleGenericRouteFill::SavingThrowOutcome,
            ),
            (
                BattleSubjectKind::AfterHitDamageRiderSaveGatedConcentration,
                BattleGenericRouteFill::SavingThrowOutcome,
            ),
        ]),
        "doFillEnsnaringSuccessfulSave" => replay_generic_route_sequence(&[(
            BattleSubjectKind::AfterHitDamageRiderSaveGatedSlotAndActionEconomySpend,
            BattleGenericRouteFill::SavingThrowOutcome,
        )]),
        "doFillDivineSmiteDamage" | "doFillEnsnaringWeaponDamage" | "doFillSearingSmiteDamage" => {
            replay_generic_route_sequence(&[(
                BattleSubjectKind::AfterHitDamageRiderAttackDamage,
                BattleGenericRouteFill::RolledDice,
            )])
        }
        "doDiscoverEnsnaringStartTurnDamage" | "doFillEnsnaringStartTurnDamage" => {
            replay_generic_route_sequence(&[(
                BattleSubjectKind::AfterHitDamageRiderTurnStartDamage,
                BattleGenericRouteFill::RolledDice,
            )])
        }
        "doFillEnsnaringEscapeCheck" => replay_generic_route_sequence(&[
            (
                BattleSubjectKind::AfterHitDamageRiderEscapeCheck,
                BattleGenericRouteFill::AbilityCheck,
            ),
            (
                BattleSubjectKind::AfterHitDamageRiderEscapeConditionCleanup,
                BattleGenericRouteFill::AbilityCheck,
            ),
            (
                BattleSubjectKind::AfterHitDamageRiderEscapeConcentrationCleanup,
                BattleGenericRouteFill::AbilityCheck,
            ),
        ]),
        "doDiscoverSearingStartTurnDamageAndSave" | "doFillSearingStartTurnDamageAndSave" => {
            replay_generic_route_sequence(&[
                (
                    BattleSubjectKind::AfterHitDamageRiderTurnStartSaveCleanup,
                    BattleGenericRouteFill::RolledDice,
                ),
                (
                    BattleSubjectKind::AfterHitDamageRiderTurnStartSaveCleanup,
                    BattleGenericRouteFill::SavingThrowOutcome,
                ),
            ])
        }
        action if blocker_reason(action).is_some() => {
            panic!("{}: {}", action, blocker_reason(action).unwrap())
        }
        action => panic!("unsupported route mbt::actionTaken {action}"),
    }
}

fn replay_generic_route_sequence(
    steps: &[(BattleSubjectKind, BattleGenericRouteFill)],
) -> Vec<ReducerRouteEvent> {
    let mut trace = BattleEntrypointTrace::default();
    let mut state = start_battle_observed(BattleSetup::standard(), &mut trace).state;
    for (subject_kind, fill) in steps {
        let discovered = discover_generic_route_subject_observed(state, *subject_kind, &mut trace);
        state = resolve_battle_subject_observed(
            discovered.0,
            BattleResolutionRequest::generic_route(discovered.1, *fill)
                .expect("generic route subject should accept generic route fills"),
            &mut trace,
        )
        .into_state();
    }
    observed_reducer_route(&trace, &[ReducerRouteSubjectFamily::AfterHitDamageRider])
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doFillHitAttackRoll" => expected_interrupt_route(
            ReducerRouteOwnerGroup::InterruptStack,
            ReducerRouteFillKind::InterruptDecision,
            vec![ReducerRouteHoleKind::RolledDice],
            ReducerRouteOwnerGroup::InterruptStack,
        ),
        "doChooseDivineSmiteSlot" | "doChooseSearingSmite" => expected_interrupt_route(
            ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
            ReducerRouteFillKind::InterruptDecision,
            vec![ReducerRouteHoleKind::RolledDice],
            ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
        ),
        "doChooseDivineSmiteFreeCast" => expected_interrupt_route(
            ReducerRouteOwnerGroup::FeatureResource,
            ReducerRouteFillKind::InterruptDecision,
            vec![ReducerRouteHoleKind::RolledDice],
            ReducerRouteOwnerGroup::FeatureResource,
        ),
        "doChooseEnsnaringStrike" => expected_interrupt_route(
            ReducerRouteOwnerGroup::InterruptStack,
            ReducerRouteFillKind::InterruptDecision,
            vec![ReducerRouteHoleKind::SavingThrowOutcome],
            ReducerRouteOwnerGroup::InterruptStack,
        ),
        "doFillEnsnaringFailedSave" => expected_sequence(&[
            expected_save_gated_route(ReducerRouteOwnerGroup::SpellSlotAndActionEconomy),
            expected_save_gated_route(ReducerRouteOwnerGroup::ConditionLifecycle),
            expected_save_gated_route(ReducerRouteOwnerGroup::Concentration),
        ]),
        "doFillEnsnaringSuccessfulSave" => {
            expected_save_gated_route(ReducerRouteOwnerGroup::SpellSlotAndActionEconomy)
        }
        "doFillDivineSmiteDamage" | "doFillEnsnaringWeaponDamage" | "doFillSearingSmiteDamage" => {
            expected_damage_route(
                ReducerRouteOwnerGroup::HitPoint,
                ReducerRouteOwnerGroup::HitPoint,
            )
        }
        "doDiscoverEnsnaringStartTurnDamage" | "doFillEnsnaringStartTurnDamage" => {
            expected_single_fill_route(
                vec![ReducerRouteHoleKind::RolledDice],
                ReducerRouteOwnerGroup::ActiveEffect,
                ReducerRouteFillKind::RolledDice,
                ReducerRouteResolutionOutcome::NeedsHoles,
                vec![ReducerRouteHoleKind::AbilityCheck],
                ReducerRouteOwnerGroup::HitPoint,
            )
        }
        "doFillEnsnaringEscapeCheck" => expected_sequence(&[
            expected_ability_check_route(ReducerRouteOwnerGroup::AbilityCheckRollMode),
            expected_ability_check_route(ReducerRouteOwnerGroup::ConditionLifecycle),
            expected_ability_check_route(ReducerRouteOwnerGroup::Concentration),
        ]),
        "doDiscoverSearingStartTurnDamageAndSave" | "doFillSearingStartTurnDamageAndSave" => {
            expected_sequence(&[
                expected_single_fill_route(
                    vec![
                        ReducerRouteHoleKind::RolledDice,
                        ReducerRouteHoleKind::SavingThrowOutcome,
                    ],
                    ReducerRouteOwnerGroup::ActiveEffect,
                    ReducerRouteFillKind::RolledDice,
                    ReducerRouteResolutionOutcome::NeedsHoles,
                    vec![ReducerRouteHoleKind::SavingThrowOutcome],
                    ReducerRouteOwnerGroup::HitPoint,
                ),
                expected_single_fill_route(
                    vec![
                        ReducerRouteHoleKind::RolledDice,
                        ReducerRouteHoleKind::SavingThrowOutcome,
                    ],
                    ReducerRouteOwnerGroup::ActiveEffect,
                    ReducerRouteFillKind::SavingThrowOutcome,
                    ReducerRouteResolutionOutcome::Resolved,
                    Vec::new(),
                    ReducerRouteOwnerGroup::ActiveEffect,
                ),
            ])
        }
        action if blocker_reason(action).is_some() => {
            panic!("{}: {}", action, blocker_reason(action).unwrap())
        }
        action => panic!("unsupported expected route mbt::actionTaken {action}"),
    }
}

fn expected_sequence(routes: &[Vec<ReducerRouteEvent>]) -> Vec<ReducerRouteEvent> {
    let mut combined = Vec::new();
    for (index, route) in routes.iter().enumerate() {
        combined.extend(route.iter().skip(usize::from(index > 0)).cloned());
    }
    combined
}

fn expected_interrupt_route(
    owner: ReducerRouteOwnerGroup,
    fill: ReducerRouteFillKind,
    next_holes: Vec<ReducerRouteHoleKind>,
    resolution_owner: ReducerRouteOwnerGroup,
) -> Vec<ReducerRouteEvent> {
    expected_single_fill_route(
        vec![ReducerRouteHoleKind::InterruptDecision],
        owner,
        fill,
        ReducerRouteResolutionOutcome::NeedsHoles,
        next_holes,
        resolution_owner,
    )
}

fn expected_save_gated_route(owner: ReducerRouteOwnerGroup) -> Vec<ReducerRouteEvent> {
    expected_single_fill_route(
        vec![ReducerRouteHoleKind::SavingThrowOutcome],
        owner,
        ReducerRouteFillKind::SavingThrowOutcome,
        ReducerRouteResolutionOutcome::NeedsHoles,
        vec![ReducerRouteHoleKind::RolledDice],
        owner,
    )
}

fn expected_ability_check_route(owner: ReducerRouteOwnerGroup) -> Vec<ReducerRouteEvent> {
    expected_single_fill_route(
        vec![ReducerRouteHoleKind::AbilityCheck],
        owner,
        ReducerRouteFillKind::AbilityCheck,
        ReducerRouteResolutionOutcome::Resolved,
        Vec::new(),
        owner,
    )
}

fn expected_damage_route(
    discovery_owner: ReducerRouteOwnerGroup,
    resolution_owner: ReducerRouteOwnerGroup,
) -> Vec<ReducerRouteEvent> {
    expected_single_fill_route(
        vec![ReducerRouteHoleKind::RolledDice],
        discovery_owner,
        ReducerRouteFillKind::RolledDice,
        ReducerRouteResolutionOutcome::Resolved,
        Vec::new(),
        resolution_owner,
    )
}

fn expected_single_fill_route(
    holes: Vec<ReducerRouteHoleKind>,
    discovery_owner: ReducerRouteOwnerGroup,
    fill: ReducerRouteFillKind,
    outcome: ReducerRouteResolutionOutcome,
    next_holes: Vec<ReducerRouteHoleKind>,
    resolution_owner: ReducerRouteOwnerGroup,
) -> Vec<ReducerRouteEvent> {
    vec![
        route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
        route_discover_battle_acts_from_route_holes(
            ReducerRouteSubjectFamily::AfterHitDamageRider,
            holes,
            discovery_owner,
        ),
        route_resolve_battle_subject_from_route_result(
            ReducerRouteSubjectFamily::AfterHitDamageRider,
            fill,
            outcome,
            next_holes,
            resolution_owner,
        ),
    ]
}

#[allow(dead_code)]
fn expected_without_fill_route(
    discovery_owner: ReducerRouteOwnerGroup,
    resolution_owner: ReducerRouteOwnerGroup,
) -> Vec<ReducerRouteEvent> {
    vec![
        route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
        route_discover_battle_acts_from_route_holes(
            ReducerRouteSubjectFamily::AfterHitDamageRider,
            Vec::new(),
            discovery_owner,
        ),
        route_resolve_battle_subject_without_fill_from_route_result(
            ReducerRouteSubjectFamily::AfterHitDamageRider,
            ReducerRouteResolutionOutcome::Resolved,
            Vec::new(),
            resolution_owner,
        ),
    ]
}

#[must_use]
pub fn blocker_reason(observed_action_taken: &str) -> Option<&'static str> {
    BLOCKED_ROUTE_BRANCH_ACTIONS
        .iter()
        .find_map(|(action, reason)| (*action == observed_action_taken).then_some(*reason))
}
