use crate::rules::battle_reducer_spine::{
    BattleHoleKind, BattleResolutionOutcome, BattleResolutionResult, BattleState, BattleSubject,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReducerRouteHoleKind {
    AttackRoll,
    ConcentrationSavingThrow,
    ConditionChoice,
    DamageTypeChoice,
    DeathSavingThrow,
    HitPointHealingDistribution,
    RolledDice,
    SavingThrowOutcome,
    SpellTargetAllocation,
    SpellTargetList,
    StatBlockRechargeRoll,
    TargetChoice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReducerRouteFillKind {
    AttackRoll,
    ConcentrationSavingThrow,
    ConditionChoice,
    DamageTypeChoice,
    DeathSavingThrow,
    HitPointHealingDistribution,
    RolledDice,
    SavingThrowOutcome,
    SpellTargetAllocation,
    SpellTargetList,
    StatBlockRechargeRoll,
    TargetChoice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReducerRouteResolveFill {
    Fill(ReducerRouteFillKind),
    WithoutFill,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReducerRouteSubjectFamily {
    ConcentrationTeardown,
    CreatureAttack,
    BattleAction,
    DeathSavingThrow,
    HitPointRestoration,
    SaveGatedSpell,
    SlotSpell,
    SpellAttack,
    StatBlockAction,
    WeaponAttack,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReducerRouteOwnerGroup {
    ActionEconomy,
    AttackRoll,
    Concentration,
    ConditionLifecycle,
    HitPointAndZeroHpLifecycle,
    HitPoint,
    HoleFrontier,
    SpellSlotAndActionEconomy,
    StatBlockAction,
    TargetSelection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReducerRouteResolveConnector {
    pub subject: ReducerRouteSubjectFamily,
    pub fill: ReducerRouteResolveFill,
    pub owner: ReducerRouteOwnerGroup,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReducerRouteEvent {
    StartBattle {
        owner: ReducerRouteOwnerGroup,
    },
    DiscoverBattleActs {
        subject: ReducerRouteSubjectFamily,
        holes: Vec<ReducerRouteHoleKind>,
        owner: ReducerRouteOwnerGroup,
    },
    ResolveBattleSubject {
        subject: ReducerRouteSubjectFamily,
        fill: ReducerRouteFillKind,
        holes: Vec<ReducerRouteHoleKind>,
        owner: ReducerRouteOwnerGroup,
    },
    ResolveBattleSubjectWithoutFill {
        subject: ReducerRouteSubjectFamily,
        holes: Vec<ReducerRouteHoleKind>,
        owner: ReducerRouteOwnerGroup,
    },
}

#[must_use]
pub fn route_start_battle(owner: ReducerRouteOwnerGroup) -> ReducerRouteEvent {
    ReducerRouteEvent::StartBattle { owner }
}

#[must_use]
pub fn route_discover_battle_acts(
    subject: ReducerRouteSubjectFamily,
    holes: Vec<BattleHoleKind>,
    owner: ReducerRouteOwnerGroup,
) -> ReducerRouteEvent {
    route_discover_battle_acts_from_route_holes(subject, reducer_route_holes(holes), owner)
}

#[must_use]
pub fn route_discover_battle_acts_from_route_holes(
    subject: ReducerRouteSubjectFamily,
    holes: Vec<ReducerRouteHoleKind>,
    owner: ReducerRouteOwnerGroup,
) -> ReducerRouteEvent {
    ReducerRouteEvent::DiscoverBattleActs {
        subject,
        holes: sorted_route_holes(holes),
        owner,
    }
}

#[must_use]
pub fn route_discover_battle_acts_from_result(
    subject: ReducerRouteSubjectFamily,
    result: &BattleResolutionResult,
    owner: ReducerRouteOwnerGroup,
) -> ReducerRouteEvent {
    route_discover_battle_acts_from_route_holes(subject, reducer_route_result_holes(result), owner)
}

#[must_use]
pub fn route_resolve_battle_subject(
    subject: ReducerRouteSubjectFamily,
    fill: ReducerRouteFillKind,
    holes: Vec<BattleHoleKind>,
    owner: ReducerRouteOwnerGroup,
) -> ReducerRouteEvent {
    route_resolve_battle_subject_from_route_holes(subject, fill, reducer_route_holes(holes), owner)
}

#[must_use]
pub fn route_resolve_battle_subject_from_route_holes(
    subject: ReducerRouteSubjectFamily,
    fill: ReducerRouteFillKind,
    holes: Vec<ReducerRouteHoleKind>,
    owner: ReducerRouteOwnerGroup,
) -> ReducerRouteEvent {
    ReducerRouteEvent::ResolveBattleSubject {
        subject,
        fill,
        holes: sorted_route_holes(holes),
        owner,
    }
}

#[must_use]
pub fn route_resolve_battle_subject_without_fill(
    subject: ReducerRouteSubjectFamily,
    holes: Vec<BattleHoleKind>,
    owner: ReducerRouteOwnerGroup,
) -> ReducerRouteEvent {
    route_resolve_battle_subject_without_fill_from_route_holes(
        subject,
        reducer_route_holes(holes),
        owner,
    )
}

#[must_use]
pub fn route_resolve_battle_subject_without_fill_from_route_holes(
    subject: ReducerRouteSubjectFamily,
    holes: Vec<ReducerRouteHoleKind>,
    owner: ReducerRouteOwnerGroup,
) -> ReducerRouteEvent {
    ReducerRouteEvent::ResolveBattleSubjectWithoutFill {
        subject,
        holes: sorted_route_holes(holes),
        owner,
    }
}

#[must_use]
pub fn route_resolve_battle_subject_from_result(
    connector: ReducerRouteResolveConnector,
    result: &BattleResolutionResult,
) -> ReducerRouteEvent {
    let holes = reducer_route_result_holes(result);
    route_resolve_battle_subject_from_result_holes(connector, holes)
}

fn route_resolve_battle_subject_from_result_holes(
    connector: ReducerRouteResolveConnector,
    holes: Vec<ReducerRouteHoleKind>,
) -> ReducerRouteEvent {
    match connector.fill {
        ReducerRouteResolveFill::Fill(fill) => ReducerRouteEvent::ResolveBattleSubject {
            subject: connector.subject,
            fill,
            holes,
            owner: connector.owner,
        },
        ReducerRouteResolveFill::WithoutFill => {
            ReducerRouteEvent::ResolveBattleSubjectWithoutFill {
                subject: connector.subject,
                holes,
                owner: connector.owner,
            }
        }
    }
}

#[must_use]
pub fn battle_resolution_continuation(
    result: BattleResolutionResult,
    context: &str,
) -> (BattleState, BattleSubject) {
    let outcome = result.outcome();
    let continuing_subject = result
        .continuing_subject()
        .unwrap_or_else(|| panic!("{context} should continue from NeedsHoles, got {outcome:?}"));
    let needs_holes = result
        .into_needs_holes()
        .unwrap_or_else(|| panic!("{context} should continue from NeedsHoles, got {outcome:?}"));
    assert_eq!(
        needs_holes.subject, continuing_subject,
        "{context} continuing_subject() and into_needs_holes() disagreed"
    );
    (needs_holes.state, needs_holes.subject)
}

#[must_use]
pub fn reducer_route_payload(route: &[ReducerRouteEvent]) -> String {
    route
        .iter()
        .map(route_event_ref)
        .collect::<Vec<_>>()
        .join("\n")
}

fn reducer_route_result_holes(result: &BattleResolutionResult) -> Vec<ReducerRouteHoleKind> {
    let holes = match result.outcome() {
        BattleResolutionOutcome::NeedsHoles | BattleResolutionOutcome::Invalid(_) => result
            .requested_holes()
            .expect("non-resolved battle resolution result should carry requested holes"),
        BattleResolutionOutcome::Resolved => &[],
    };
    reducer_route_holes_from_slice(holes)
}

fn reducer_route_holes(holes: Vec<BattleHoleKind>) -> Vec<ReducerRouteHoleKind> {
    reducer_route_holes_from_slice(&holes)
}

fn reducer_route_holes_from_slice(holes: &[BattleHoleKind]) -> Vec<ReducerRouteHoleKind> {
    sorted_route_holes(
        holes
            .iter()
            .copied()
            .map(reducer_route_hole)
            .collect::<Vec<_>>(),
    )
}

fn sorted_route_holes(mut route_holes: Vec<ReducerRouteHoleKind>) -> Vec<ReducerRouteHoleKind> {
    route_holes.sort();
    route_holes
}

fn reducer_route_hole(hole: BattleHoleKind) -> ReducerRouteHoleKind {
    match hole {
        BattleHoleKind::ConcentrationSavingThrow => ReducerRouteHoleKind::ConcentrationSavingThrow,
        BattleHoleKind::ConditionChoice => ReducerRouteHoleKind::ConditionChoice,
        BattleHoleKind::DeathSavingThrow => ReducerRouteHoleKind::DeathSavingThrow,
        BattleHoleKind::DamageTypeChoice => ReducerRouteHoleKind::DamageTypeChoice,
        BattleHoleKind::HitPointHealingDistribution => {
            ReducerRouteHoleKind::HitPointHealingDistribution
        }
        BattleHoleKind::RolledDice => ReducerRouteHoleKind::RolledDice,
        BattleHoleKind::SavingThrowOutcome => ReducerRouteHoleKind::SavingThrowOutcome,
        BattleHoleKind::SpellTargetAllocation => ReducerRouteHoleKind::SpellTargetAllocation,
        BattleHoleKind::SpellTargetList => ReducerRouteHoleKind::SpellTargetList,
        BattleHoleKind::StatBlockRechargeRoll => ReducerRouteHoleKind::StatBlockRechargeRoll,
        BattleHoleKind::TargetChoice => ReducerRouteHoleKind::TargetChoice,
        BattleHoleKind::AttackRoll => ReducerRouteHoleKind::AttackRoll,
    }
}

fn route_event_ref(event: &ReducerRouteEvent) -> String {
    match event {
        ReducerRouteEvent::StartBattle { owner } => {
            format!("start_battle owner={}", owner_ref(*owner))
        }
        ReducerRouteEvent::DiscoverBattleActs {
            subject,
            holes,
            owner,
        } => format!(
            "discover_battle_acts subject={} holes={} owner={}",
            subject_ref(*subject),
            joined_holes(holes),
            owner_ref(*owner)
        ),
        ReducerRouteEvent::ResolveBattleSubject {
            subject,
            fill,
            holes,
            owner,
        } => format!(
            "resolve_battle_subject subject={} fill={} holes={} owner={}",
            subject_ref(*subject),
            fill_ref(*fill),
            joined_holes(holes),
            owner_ref(*owner)
        ),
        ReducerRouteEvent::ResolveBattleSubjectWithoutFill {
            subject,
            holes,
            owner,
        } => format!(
            "resolve_battle_subject_without_fill subject={} holes={} owner={}",
            subject_ref(*subject),
            joined_holes(holes),
            owner_ref(*owner)
        ),
    }
}

fn joined_holes(holes: &[ReducerRouteHoleKind]) -> String {
    if holes.is_empty() {
        return "none".to_string();
    }
    holes
        .iter()
        .map(|hole| hole_ref(*hole))
        .collect::<Vec<_>>()
        .join(",")
}

fn hole_ref(hole: ReducerRouteHoleKind) -> &'static str {
    match hole {
        ReducerRouteHoleKind::AttackRoll => "AttackRollHoleKind",
        ReducerRouteHoleKind::ConcentrationSavingThrow => "ConcentrationSavingThrowHoleKind",
        ReducerRouteHoleKind::ConditionChoice => "ConditionChoiceHoleKind",
        ReducerRouteHoleKind::DamageTypeChoice => "DamageTypeChoiceHoleKind",
        ReducerRouteHoleKind::DeathSavingThrow => "DeathSavingThrowHoleKind",
        ReducerRouteHoleKind::HitPointHealingDistribution => "HitPointHealingDistributionHoleKind",
        ReducerRouteHoleKind::RolledDice => "RolledDiceHoleKind",
        ReducerRouteHoleKind::SavingThrowOutcome => "SavingThrowOutcomeHoleKind",
        ReducerRouteHoleKind::SpellTargetAllocation => "SpellTargetAllocationHoleKind",
        ReducerRouteHoleKind::SpellTargetList => "SpellTargetListHoleKind",
        ReducerRouteHoleKind::StatBlockRechargeRoll => "StatBlockRechargeRollHoleKind",
        ReducerRouteHoleKind::TargetChoice => "TargetChoiceHoleKind",
    }
}

fn fill_ref(fill: ReducerRouteFillKind) -> &'static str {
    match fill {
        ReducerRouteFillKind::AttackRoll => "AttackRollFillKind",
        ReducerRouteFillKind::ConcentrationSavingThrow => "ConcentrationSavingThrowFillKind",
        ReducerRouteFillKind::ConditionChoice => "ConditionChoiceFillKind",
        ReducerRouteFillKind::DamageTypeChoice => "DamageTypeChoiceFillKind",
        ReducerRouteFillKind::DeathSavingThrow => "DeathSavingThrowFillKind",
        ReducerRouteFillKind::HitPointHealingDistribution => "HitPointHealingDistributionFillKind",
        ReducerRouteFillKind::RolledDice => "RolledDiceFillKind",
        ReducerRouteFillKind::SavingThrowOutcome => "SavingThrowOutcomeFillKind",
        ReducerRouteFillKind::SpellTargetAllocation => "SpellTargetAllocationFillKind",
        ReducerRouteFillKind::SpellTargetList => "SpellTargetListFillKind",
        ReducerRouteFillKind::StatBlockRechargeRoll => "StatBlockRechargeRollFillKind",
        ReducerRouteFillKind::TargetChoice => "TargetChoiceFillKind",
    }
}

fn owner_ref(owner: ReducerRouteOwnerGroup) -> &'static str {
    match owner {
        ReducerRouteOwnerGroup::ActionEconomy => "BattleActionEconomyOwner",
        ReducerRouteOwnerGroup::AttackRoll => "BattleAttackRollOwner",
        ReducerRouteOwnerGroup::Concentration => "BattleConcentrationOwner",
        ReducerRouteOwnerGroup::ConditionLifecycle => "BattleConditionLifecycleOwner",
        ReducerRouteOwnerGroup::HitPointAndZeroHpLifecycle => {
            "BattleHitPointAndZeroHpLifecycleOwner"
        }
        ReducerRouteOwnerGroup::HitPoint => "BattleHitPointOwner",
        ReducerRouteOwnerGroup::HoleFrontier => "BattleHoleFrontierOwner",
        ReducerRouteOwnerGroup::SpellSlotAndActionEconomy => "BattleSpellSlotAndActionEconomyOwner",
        ReducerRouteOwnerGroup::StatBlockAction => "BattleStatBlockActionOwner",
        ReducerRouteOwnerGroup::TargetSelection => "BattleTargetSelectionOwner",
    }
}

fn subject_ref(subject: ReducerRouteSubjectFamily) -> &'static str {
    match subject {
        ReducerRouteSubjectFamily::BattleAction => "BattleActionRouteSubject",
        ReducerRouteSubjectFamily::ConcentrationTeardown => "ConcentrationTeardownRouteSubject",
        ReducerRouteSubjectFamily::CreatureAttack => "CreatureAttackRouteSubject",
        ReducerRouteSubjectFamily::DeathSavingThrow => "DeathSavingThrowRouteSubject",
        ReducerRouteSubjectFamily::HitPointRestoration => "HitPointRestorationRouteSubject",
        ReducerRouteSubjectFamily::SaveGatedSpell => "SaveGatedSpellRouteSubject",
        ReducerRouteSubjectFamily::SlotSpell => "SlotSpellRouteSubject",
        ReducerRouteSubjectFamily::SpellAttack => "SpellAttackRouteSubject",
        ReducerRouteSubjectFamily::StatBlockAction => "StatBlockActionRouteSubject",
        ReducerRouteSubjectFamily::WeaponAttack => "WeaponAttackRouteSubject",
    }
}

#[cfg(test)]
mod tests {
    use super::{
        battle_resolution_continuation, route_discover_battle_acts_from_result,
        route_resolve_battle_subject_from_result, ReducerRouteEvent, ReducerRouteFillKind,
        ReducerRouteHoleKind, ReducerRouteOwnerGroup, ReducerRouteResolveConnector,
        ReducerRouteResolveFill, ReducerRouteSubjectFamily,
    };
    use crate::rules::battle_reducer_spine::{
        discover_rolled_stat_block_attack_control, start_fighter_skeleton_battle,
        start_stat_block_actor_battle, Actor, BattleHoleKind, BattleResolutionInvalidReason,
        BattleResolutionResult, BattleSubject, BattleSubjectKind,
    };
    use crate::rules::weapon_attack_ordering::WeaponAttackFrontierStage;

    #[test]
    fn resolve_route_event_reads_holes_from_resolution_result() {
        let result = BattleResolutionResult::NeedsHoles {
            state: start_fighter_skeleton_battle(),
            subject: route_test_subject(BattleSubjectKind::SlotSpell),
            holes: vec![BattleHoleKind::RolledDice],
        };

        let event = route_resolve_battle_subject_from_result(
            ReducerRouteResolveConnector {
                subject: ReducerRouteSubjectFamily::SlotSpell,
                fill: ReducerRouteResolveFill::Fill(ReducerRouteFillKind::SpellTargetAllocation),
                owner: ReducerRouteOwnerGroup::HoleFrontier,
            },
            &result,
        );

        assert_eq!(
            event,
            ReducerRouteEvent::ResolveBattleSubject {
                subject: ReducerRouteSubjectFamily::SlotSpell,
                fill: ReducerRouteFillKind::SpellTargetAllocation,
                holes: vec![ReducerRouteHoleKind::RolledDice],
                owner: ReducerRouteOwnerGroup::HoleFrontier,
            }
        );
    }

    #[test]
    fn resolved_route_event_uses_empty_holes_from_result_outcome() {
        let result = BattleResolutionResult::Resolved {
            state: start_fighter_skeleton_battle(),
        };

        let event = route_resolve_battle_subject_from_result(
            ReducerRouteResolveConnector {
                subject: ReducerRouteSubjectFamily::SaveGatedSpell,
                fill: ReducerRouteResolveFill::Fill(ReducerRouteFillKind::RolledDice),
                owner: ReducerRouteOwnerGroup::HitPoint,
            },
            &result,
        );

        assert_eq!(
            event,
            ReducerRouteEvent::ResolveBattleSubject {
                subject: ReducerRouteSubjectFamily::SaveGatedSpell,
                fill: ReducerRouteFillKind::RolledDice,
                holes: Vec::new(),
                owner: ReducerRouteOwnerGroup::HitPoint,
            }
        );
    }

    #[test]
    fn no_fill_route_event_reads_invalid_result_holes() {
        let result = BattleResolutionResult::Invalid {
            state: start_fighter_skeleton_battle(),
            reason: BattleResolutionInvalidReason::WrongActor,
            holes: vec![BattleHoleKind::ConcentrationSavingThrow],
        };

        let event = route_resolve_battle_subject_from_result(
            ReducerRouteResolveConnector {
                subject: ReducerRouteSubjectFamily::ConcentrationTeardown,
                fill: ReducerRouteResolveFill::WithoutFill,
                owner: ReducerRouteOwnerGroup::Concentration,
            },
            &result,
        );

        assert_eq!(
            event,
            ReducerRouteEvent::ResolveBattleSubjectWithoutFill {
                subject: ReducerRouteSubjectFamily::ConcentrationTeardown,
                holes: vec![ReducerRouteHoleKind::ConcentrationSavingThrow],
                owner: ReducerRouteOwnerGroup::Concentration,
            }
        );
    }

    #[test]
    fn discovery_route_event_reads_stat_block_holes_from_resolution_result() {
        let result = discover_rolled_stat_block_attack_control(
            start_stat_block_actor_battle(Actor::Goblin),
            Actor::Goblin,
        );

        let event = route_discover_battle_acts_from_result(
            ReducerRouteSubjectFamily::StatBlockAction,
            &result,
            ReducerRouteOwnerGroup::StatBlockAction,
        );

        assert_eq!(
            event,
            ReducerRouteEvent::DiscoverBattleActs {
                subject: ReducerRouteSubjectFamily::StatBlockAction,
                holes: vec![ReducerRouteHoleKind::TargetChoice],
                owner: ReducerRouteOwnerGroup::StatBlockAction,
            }
        );
    }

    #[test]
    fn continuation_helper_consumes_stat_block_needs_holes_result() {
        let result = discover_rolled_stat_block_attack_control(
            start_stat_block_actor_battle(Actor::Goblin),
            Actor::Goblin,
        );
        let borrowed = result
            .needs_holes()
            .expect("stat-block needs-holes result should expose generic holes");
        let continuing_subject = result
            .continuing_subject()
            .expect("stat-block needs-holes result should expose a continuing subject");

        assert_eq!(borrowed.holes(), &[BattleHoleKind::TargetChoice]);
        assert_eq!(borrowed.subject(), continuing_subject);

        let (_, subject) = battle_resolution_continuation(result, "stat-block test continuation");
        assert_eq!(subject, continuing_subject);
    }

    fn route_test_subject(kind: BattleSubjectKind) -> BattleSubject {
        BattleSubject {
            kind,
            actor: Actor::Fighter,
            target: Some(Actor::Skeleton),
            stage: WeaponAttackFrontierStage::ActSelection,
            damage_modifier: 0,
        }
    }
}
