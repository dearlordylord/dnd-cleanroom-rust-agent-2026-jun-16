use crate::rules::battle_reducer_spine::{
    discover_generic_route_subject_observed, generic_route_subject,
    resolve_battle_subject_observed, start_battle_observed, Actor, BattleEntrypointTrace,
    BattleGenericRouteFill, BattleResolutionRequest, BattleSetup, BattleSubjectKind,
};

use super::battle_runtime_reducer_route::{
    observed_reducer_route, route_discover_battle_acts_from_route_holes,
    route_resolve_battle_interrupt_from_route_holes, route_resolve_battle_subject_from_route_holes,
    route_resolve_battle_subject_without_fill_from_route_holes, route_start_battle,
    ReducerRouteEvent, ReducerRouteFillKind, ReducerRouteHoleKind, ReducerRouteOwnerGroup,
    ReducerRouteSubjectFamily,
};

pub const BRANCH_ACTIONS: [&str; 5] = [
    "doRouteReactionArmorClassEffect",
    "doRouteAfterDamageSaveDamage",
    "doRouteSpellInterruptionEnded",
    "doRouteSpellInterruptionResumed",
    "doRouteFallMitigation",
];

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doRouteReactionArmorClassEffect" => replay_generic_route(
            BattleSubjectKind::ReactionArmorClassEffectSlotCommit,
            &[
                (
                    BattleSubjectKind::ReactionArmorClassEffectSlotCommit,
                    BattleGenericRouteFill::InterruptDecision,
                ),
                (
                    BattleSubjectKind::ReactionArmorClassEffectActiveEffect,
                    BattleGenericRouteFill::InterruptDecision,
                ),
                (
                    BattleSubjectKind::ReactionArmorClassEffectArmorClass,
                    BattleGenericRouteFill::WithoutFill,
                ),
                (
                    BattleSubjectKind::ReactionArmorClassEffectClosedContinuation,
                    BattleGenericRouteFill::WithoutFill,
                ),
            ],
            &[ReducerRouteSubjectFamily::ReactionArmorClassEffect],
        ),
        "doRouteAfterDamageSaveDamage" => replay_generic_route(
            BattleSubjectKind::ReactionAfterDamageEffectInterruptDecision,
            &[
                (
                    BattleSubjectKind::ReactionAfterDamageEffectInterruptDecision,
                    BattleGenericRouteFill::InterruptDecision,
                ),
                (
                    BattleSubjectKind::ReactionAfterDamageEffectSavingThrow,
                    BattleGenericRouteFill::SavingThrowOutcome,
                ),
                (
                    BattleSubjectKind::ReactionAfterDamageEffectDamage,
                    BattleGenericRouteFill::RolledDice,
                ),
                (
                    BattleSubjectKind::ReactionAfterDamageEffectSlotCommit,
                    BattleGenericRouteFill::WithoutFill,
                ),
            ],
            &[ReducerRouteSubjectFamily::ReactionAfterDamageEffect],
        ),
        "doRouteSpellInterruptionEnded" => replay_generic_route(
            BattleSubjectKind::ReactionSpellInterruptionInterruptDecision,
            &[
                (
                    BattleSubjectKind::ReactionSpellInterruptionInterruptDecision,
                    BattleGenericRouteFill::InterruptDecision,
                ),
                (
                    BattleSubjectKind::ReactionSpellInterruptionSavingThrowEnded,
                    BattleGenericRouteFill::SavingThrowOutcome,
                ),
                (
                    BattleSubjectKind::ReactionSpellInterruptionClosedContinuation,
                    BattleGenericRouteFill::WithoutFill,
                ),
            ],
            &[ReducerRouteSubjectFamily::ReactionSpellInterruption],
        ),
        "doRouteSpellInterruptionResumed" => replay_generic_route(
            BattleSubjectKind::ReactionSpellInterruptionInterruptDecision,
            &[
                (
                    BattleSubjectKind::ReactionSpellInterruptionInterruptDecision,
                    BattleGenericRouteFill::InterruptDecision,
                ),
                (
                    BattleSubjectKind::ReactionSpellInterruptionSavingThrowResumed,
                    BattleGenericRouteFill::SavingThrowOutcome,
                ),
                (
                    BattleSubjectKind::SlotSpellDamageContinuation,
                    BattleGenericRouteFill::RolledDice,
                ),
                (
                    BattleSubjectKind::ReactionSpellInterruptionClosedContinuation,
                    BattleGenericRouteFill::WithoutFill,
                ),
            ],
            &[
                ReducerRouteSubjectFamily::ReactionSpellInterruption,
                ReducerRouteSubjectFamily::SlotSpell,
            ],
        ),
        "doRouteFallMitigation" => replay_generic_route(
            BattleSubjectKind::ReactionFallMitigationSlotCommit,
            &[
                (
                    BattleSubjectKind::ReactionFallMitigationSlotCommit,
                    BattleGenericRouteFill::InterruptDecision,
                ),
                (
                    BattleSubjectKind::ReactionFallMitigationActiveEffect,
                    BattleGenericRouteFill::WithoutFill,
                ),
                (
                    BattleSubjectKind::ReactionFallMitigationMovement,
                    BattleGenericRouteFill::WithoutFill,
                ),
                (
                    BattleSubjectKind::ReactionFallMitigationHitPoint,
                    BattleGenericRouteFill::WithoutFill,
                ),
            ],
            &[ReducerRouteSubjectFamily::ReactionFallMitigation],
        ),
        action => panic!("unsupported route mbt::actionTaken {action}"),
    }
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doRouteReactionArmorClassEffect" => reaction_armor_class_effect_route(),
        "doRouteAfterDamageSaveDamage" => after_damage_save_damage_route(),
        "doRouteSpellInterruptionEnded" => spell_interruption_ended_route(),
        "doRouteSpellInterruptionResumed" => spell_interruption_resumed_route(),
        "doRouteFallMitigation" => fall_mitigation_route(),
        action => panic!("unsupported route mbt::actionTaken {action}"),
    }
}

fn replay_generic_route(
    discovery_subject: BattleSubjectKind,
    steps: &[(BattleSubjectKind, BattleGenericRouteFill)],
    route_subjects: &[ReducerRouteSubjectFamily],
) -> Vec<ReducerRouteEvent> {
    let mut trace = BattleEntrypointTrace::default();
    let state = start_battle_observed(BattleSetup::standard(), &mut trace).state;
    let (mut state, _) =
        discover_generic_route_subject_observed(state, discovery_subject, &mut trace);
    for (subject_kind, fill) in steps {
        let request = BattleResolutionRequest::generic_route(
            generic_route_subject(*subject_kind, Actor::Fighter),
            *fill,
        )
        .expect("generic route subject should accept generic route fills");
        state = resolve_battle_subject_observed(state, request, &mut trace).into_state();
    }
    observed_reducer_route(&trace, route_subjects)
}

fn pending_reaction_payload_route(subject: ReducerRouteSubjectFamily) -> Vec<ReducerRouteEvent> {
    vec![
        route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
        route_discover_battle_acts_from_route_holes(
            subject,
            vec![ReducerRouteHoleKind::InterruptDecision],
            ReducerRouteOwnerGroup::InterruptStack,
        ),
    ]
}

fn reaction_armor_class_effect_route() -> Vec<ReducerRouteEvent> {
    let subject = ReducerRouteSubjectFamily::ReactionArmorClassEffect;
    let mut route = pending_reaction_payload_route(subject);
    route.push(route_resolve_battle_interrupt_from_route_holes(
        subject,
        ReducerRouteFillKind::InterruptDecision,
        Vec::new(),
        ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
    ));
    route.push(route_resolve_battle_interrupt_from_route_holes(
        subject,
        ReducerRouteFillKind::InterruptDecision,
        Vec::new(),
        ReducerRouteOwnerGroup::ActiveEffect,
    ));
    route.push(route_resolve_battle_subject_without_fill_from_route_holes(
        subject,
        Vec::new(),
        ReducerRouteOwnerGroup::ArmorClass,
    ));
    route.push(route_resolve_battle_subject_without_fill_from_route_holes(
        subject,
        Vec::new(),
        ReducerRouteOwnerGroup::InterruptStack,
    ));
    route
}

fn after_damage_save_damage_route() -> Vec<ReducerRouteEvent> {
    let subject = ReducerRouteSubjectFamily::ReactionAfterDamageEffect;
    let mut route = pending_reaction_payload_route(subject);
    route.push(route_resolve_battle_interrupt_from_route_holes(
        subject,
        ReducerRouteFillKind::InterruptDecision,
        vec![
            ReducerRouteHoleKind::SavingThrowOutcome,
            ReducerRouteHoleKind::RolledDice,
        ],
        ReducerRouteOwnerGroup::InterruptStack,
    ));
    route.push(route_resolve_battle_subject_from_route_holes(
        subject,
        ReducerRouteFillKind::SavingThrowOutcome,
        vec![ReducerRouteHoleKind::RolledDice],
        ReducerRouteOwnerGroup::SavingThrowOutcome,
    ));
    route.push(route_resolve_battle_subject_from_route_holes(
        subject,
        ReducerRouteFillKind::RolledDice,
        Vec::new(),
        ReducerRouteOwnerGroup::HitPoint,
    ));
    route.push(route_resolve_battle_subject_without_fill_from_route_holes(
        subject,
        Vec::new(),
        ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
    ));
    route
}

fn spell_interruption_ended_route() -> Vec<ReducerRouteEvent> {
    let subject = ReducerRouteSubjectFamily::ReactionSpellInterruption;
    let mut route = pending_reaction_payload_route(subject);
    route.push(route_resolve_battle_interrupt_from_route_holes(
        subject,
        ReducerRouteFillKind::InterruptDecision,
        vec![ReducerRouteHoleKind::SavingThrowOutcome],
        ReducerRouteOwnerGroup::InterruptStack,
    ));
    route.push(route_resolve_battle_subject_from_route_holes(
        subject,
        ReducerRouteFillKind::SavingThrowOutcome,
        Vec::new(),
        ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
    ));
    route.push(route_resolve_battle_subject_without_fill_from_route_holes(
        subject,
        Vec::new(),
        ReducerRouteOwnerGroup::InterruptStack,
    ));
    route
}

fn spell_interruption_resumed_route() -> Vec<ReducerRouteEvent> {
    let subject = ReducerRouteSubjectFamily::ReactionSpellInterruption;
    let mut route = pending_reaction_payload_route(subject);
    route.push(route_resolve_battle_interrupt_from_route_holes(
        subject,
        ReducerRouteFillKind::InterruptDecision,
        vec![ReducerRouteHoleKind::SavingThrowOutcome],
        ReducerRouteOwnerGroup::InterruptStack,
    ));
    route.push(route_resolve_battle_subject_from_route_holes(
        subject,
        ReducerRouteFillKind::SavingThrowOutcome,
        vec![ReducerRouteHoleKind::RolledDice],
        ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
    ));
    route.push(route_resolve_battle_subject_from_route_holes(
        ReducerRouteSubjectFamily::SlotSpell,
        ReducerRouteFillKind::RolledDice,
        Vec::new(),
        ReducerRouteOwnerGroup::HitPoint,
    ));
    route.push(route_resolve_battle_subject_without_fill_from_route_holes(
        subject,
        Vec::new(),
        ReducerRouteOwnerGroup::InterruptStack,
    ));
    route
}

fn fall_mitigation_route() -> Vec<ReducerRouteEvent> {
    let subject = ReducerRouteSubjectFamily::ReactionFallMitigation;
    let mut route = pending_reaction_payload_route(subject);
    route.push(route_resolve_battle_interrupt_from_route_holes(
        subject,
        ReducerRouteFillKind::InterruptDecision,
        Vec::new(),
        ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
    ));
    route.push(route_resolve_battle_subject_without_fill_from_route_holes(
        subject,
        Vec::new(),
        ReducerRouteOwnerGroup::ActiveEffect,
    ));
    route.push(route_resolve_battle_subject_without_fill_from_route_holes(
        subject,
        Vec::new(),
        ReducerRouteOwnerGroup::MovementResource,
    ));
    route.push(route_resolve_battle_subject_without_fill_from_route_holes(
        subject,
        Vec::new(),
        ReducerRouteOwnerGroup::HitPoint,
    ));
    route
}
