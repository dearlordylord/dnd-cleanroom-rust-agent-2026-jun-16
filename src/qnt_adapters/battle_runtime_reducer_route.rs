use crate::rules::battle_reducer_spine::BattleHoleKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReducerRouteHoleKind {
    ConcentrationSavingThrow,
    ConditionChoice,
    DeathSavingThrow,
    HitPointHealingDistribution,
    RolledDice,
    SavingThrowOutcome,
    SpellTargetAllocation,
    SpellTargetList,
    TargetChoice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReducerRouteFillKind {
    ConcentrationSavingThrow,
    ConditionChoice,
    DeathSavingThrow,
    HitPointHealingDistribution,
    RolledDice,
    SavingThrowOutcome,
    SpellTargetAllocation,
    SpellTargetList,
    TargetChoice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReducerRouteSubjectFamily {
    ConcentrationTeardown,
    DeathSavingThrow,
    HitPointRestoration,
    SaveGatedSpell,
    SlotSpell,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReducerRouteOwnerGroup {
    ActionEconomy,
    Concentration,
    HitPointAndZeroHpLifecycle,
    HitPoint,
    HoleFrontier,
    SpellSlotAndActionEconomy,
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
    ReducerRouteEvent::DiscoverBattleActs {
        subject,
        holes: reducer_route_holes(holes),
        owner,
    }
}

#[must_use]
pub fn route_resolve_battle_subject(
    subject: ReducerRouteSubjectFamily,
    fill: ReducerRouteFillKind,
    holes: Vec<BattleHoleKind>,
    owner: ReducerRouteOwnerGroup,
) -> ReducerRouteEvent {
    ReducerRouteEvent::ResolveBattleSubject {
        subject,
        fill,
        holes: reducer_route_holes(holes),
        owner,
    }
}

#[must_use]
pub fn route_resolve_battle_subject_without_fill(
    subject: ReducerRouteSubjectFamily,
    holes: Vec<BattleHoleKind>,
    owner: ReducerRouteOwnerGroup,
) -> ReducerRouteEvent {
    ReducerRouteEvent::ResolveBattleSubjectWithoutFill {
        subject,
        holes: reducer_route_holes(holes),
        owner,
    }
}

#[must_use]
pub fn reducer_route_payload(route: &[ReducerRouteEvent]) -> String {
    route
        .iter()
        .map(route_event_ref)
        .collect::<Vec<_>>()
        .join("\n")
}

fn reducer_route_holes(holes: Vec<BattleHoleKind>) -> Vec<ReducerRouteHoleKind> {
    let mut route_holes = holes
        .into_iter()
        .map(reducer_route_hole)
        .collect::<Vec<_>>();
    route_holes.sort();
    route_holes
}

fn reducer_route_hole(hole: BattleHoleKind) -> ReducerRouteHoleKind {
    match hole {
        BattleHoleKind::ConcentrationSavingThrow => ReducerRouteHoleKind::ConcentrationSavingThrow,
        BattleHoleKind::ConditionChoice => ReducerRouteHoleKind::ConditionChoice,
        BattleHoleKind::DeathSavingThrow => ReducerRouteHoleKind::DeathSavingThrow,
        BattleHoleKind::HitPointHealingDistribution => {
            ReducerRouteHoleKind::HitPointHealingDistribution
        }
        BattleHoleKind::RolledDice => ReducerRouteHoleKind::RolledDice,
        BattleHoleKind::SavingThrowOutcome => ReducerRouteHoleKind::SavingThrowOutcome,
        BattleHoleKind::SpellTargetAllocation => ReducerRouteHoleKind::SpellTargetAllocation,
        BattleHoleKind::SpellTargetList => ReducerRouteHoleKind::SpellTargetList,
        BattleHoleKind::TargetChoice => ReducerRouteHoleKind::TargetChoice,
        BattleHoleKind::AttackRoll => {
            panic!("route connector batch does not model attack-roll holes")
        }
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
        ReducerRouteHoleKind::ConcentrationSavingThrow => "ConcentrationSavingThrowHoleKind",
        ReducerRouteHoleKind::ConditionChoice => "ConditionChoiceHoleKind",
        ReducerRouteHoleKind::DeathSavingThrow => "DeathSavingThrowHoleKind",
        ReducerRouteHoleKind::HitPointHealingDistribution => "HitPointHealingDistributionHoleKind",
        ReducerRouteHoleKind::RolledDice => "RolledDiceHoleKind",
        ReducerRouteHoleKind::SavingThrowOutcome => "SavingThrowOutcomeHoleKind",
        ReducerRouteHoleKind::SpellTargetAllocation => "SpellTargetAllocationHoleKind",
        ReducerRouteHoleKind::SpellTargetList => "SpellTargetListHoleKind",
        ReducerRouteHoleKind::TargetChoice => "TargetChoiceHoleKind",
    }
}

fn fill_ref(fill: ReducerRouteFillKind) -> &'static str {
    match fill {
        ReducerRouteFillKind::ConcentrationSavingThrow => "ConcentrationSavingThrowFillKind",
        ReducerRouteFillKind::ConditionChoice => "ConditionChoiceFillKind",
        ReducerRouteFillKind::DeathSavingThrow => "DeathSavingThrowFillKind",
        ReducerRouteFillKind::HitPointHealingDistribution => "HitPointHealingDistributionFillKind",
        ReducerRouteFillKind::RolledDice => "RolledDiceFillKind",
        ReducerRouteFillKind::SavingThrowOutcome => "SavingThrowOutcomeFillKind",
        ReducerRouteFillKind::SpellTargetAllocation => "SpellTargetAllocationFillKind",
        ReducerRouteFillKind::SpellTargetList => "SpellTargetListFillKind",
        ReducerRouteFillKind::TargetChoice => "TargetChoiceFillKind",
    }
}

fn owner_ref(owner: ReducerRouteOwnerGroup) -> &'static str {
    match owner {
        ReducerRouteOwnerGroup::ActionEconomy => "BattleActionEconomyOwner",
        ReducerRouteOwnerGroup::Concentration => "BattleConcentrationOwner",
        ReducerRouteOwnerGroup::HitPointAndZeroHpLifecycle => {
            "BattleHitPointAndZeroHpLifecycleOwner"
        }
        ReducerRouteOwnerGroup::HitPoint => "BattleHitPointOwner",
        ReducerRouteOwnerGroup::HoleFrontier => "BattleHoleFrontierOwner",
        ReducerRouteOwnerGroup::SpellSlotAndActionEconomy => "BattleSpellSlotAndActionEconomyOwner",
    }
}

fn subject_ref(subject: ReducerRouteSubjectFamily) -> &'static str {
    match subject {
        ReducerRouteSubjectFamily::ConcentrationTeardown => "ConcentrationTeardownRouteSubject",
        ReducerRouteSubjectFamily::DeathSavingThrow => "DeathSavingThrowRouteSubject",
        ReducerRouteSubjectFamily::HitPointRestoration => "HitPointRestorationRouteSubject",
        ReducerRouteSubjectFamily::SaveGatedSpell => "SaveGatedSpellRouteSubject",
        ReducerRouteSubjectFamily::SlotSpell => "SlotSpellRouteSubject",
    }
}
