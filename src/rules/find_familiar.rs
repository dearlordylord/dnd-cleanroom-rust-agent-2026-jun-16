use super::battle_reducer_spine::{
    observe_battle_route_discovery, observe_battle_route_resolution,
    observe_battle_route_resolution_without_fill, observe_battle_route_start,
    BattleReducerRouteFillKind, BattleReducerRouteHoleKind, BattleReducerRouteObserver,
    BattleReducerRouteOwnerGroup, BattleReducerRouteSubjectFamily, BattleReducerRouteTrace,
    BattleResolutionOutcome,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FamiliarStatus {
    None,
    Present,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FamiliarSlot {
    None,
    Primary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FamiliarForm {
    None,
    Cat,
    Rat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FamiliarCreatureTypeOverride {
    None,
    Fey,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FindFamiliarCompanionScenarioOutcome {
    Init,
    Created,
    Replaced,
    DismissedAndReappeared,
    SharedSenses,
    TouchDelivered,
    PactAttack,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FindFamiliarCompanionProtocol {
    Init,
    Resolved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FamiliarFormFacts {
    pub form: FamiliarForm,
    pub creature_type_override: FamiliarCreatureTypeOverride,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FindFamiliarCompanionState {
    pub status: FamiliarStatus,
    pub slot: FamiliarSlot,
    pub form: FamiliarForm,
    pub creature_type_override: FamiliarCreatureTypeOverride,
    pub companion_count: i16,
    pub telepathy_available: bool,
    pub shared_senses_active: bool,
    pub bonus_action_available: bool,
    pub owner_action_available: bool,
    pub owner_attack_available: bool,
    pub familiar_reaction_available: bool,
    pub touch_delivery_reaction_spent: bool,
    pub pact_reaction_attack_resolved: bool,
    pub spell_slot_committed: bool,
    pub target_hit_points: i16,
    pub scenario_outcome: FindFamiliarCompanionScenarioOutcome,
    pub protocol: FindFamiliarCompanionProtocol,
}

#[must_use]
pub fn find_familiar_companion_initial_state() -> FindFamiliarCompanionState {
    FindFamiliarCompanionState {
        status: FamiliarStatus::None,
        slot: FamiliarSlot::None,
        form: FamiliarForm::None,
        creature_type_override: FamiliarCreatureTypeOverride::None,
        companion_count: 0,
        telepathy_available: false,
        shared_senses_active: false,
        bonus_action_available: true,
        owner_action_available: true,
        owner_attack_available: true,
        familiar_reaction_available: false,
        touch_delivery_reaction_spent: false,
        pact_reaction_attack_resolved: false,
        spell_slot_committed: false,
        target_hit_points: 12,
        scenario_outcome: FindFamiliarCompanionScenarioOutcome::Init,
        protocol: FindFamiliarCompanionProtocol::Init,
    }
}

#[must_use]
pub fn create_find_familiar_companion(
    state: FindFamiliarCompanionState,
    facts: FamiliarFormFacts,
) -> FindFamiliarCompanionState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-E-L.md
    // "Find Familiar".
    if state.status != FamiliarStatus::None {
        return state;
    }

    FindFamiliarCompanionState {
        status: FamiliarStatus::Present,
        slot: FamiliarSlot::Primary,
        form: facts.form,
        creature_type_override: facts.creature_type_override,
        companion_count: 1,
        telepathy_available: true,
        shared_senses_active: false,
        familiar_reaction_available: true,
        scenario_outcome: FindFamiliarCompanionScenarioOutcome::Created,
        protocol: FindFamiliarCompanionProtocol::Resolved,
        ..state
    }
}

#[must_use]
pub fn replace_find_familiar_companion_form(
    state: FindFamiliarCompanionState,
    form: FamiliarForm,
) -> FindFamiliarCompanionState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-E-L.md
    // "One Familiar Only".
    if state.status != FamiliarStatus::Present {
        return state;
    }

    FindFamiliarCompanionState {
        form,
        companion_count: 1,
        telepathy_available: true,
        familiar_reaction_available: true,
        touch_delivery_reaction_spent: false,
        scenario_outcome: FindFamiliarCompanionScenarioOutcome::Replaced,
        protocol: FindFamiliarCompanionProtocol::Resolved,
        ..state
    }
}

#[must_use]
pub fn recast_find_familiar_companion_replacement(
    state: FindFamiliarCompanionState,
    form: FamiliarForm,
) -> FindFamiliarCompanionState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-E-L.md
    // "One Familiar Only".
    replace_find_familiar_companion_form(state, form)
}

#[must_use]
pub fn dismiss_and_reappear_find_familiar_companion(
    state: FindFamiliarCompanionState,
) -> FindFamiliarCompanionState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-E-L.md
    // "Disappearance of the Familiar".
    if state.status != FamiliarStatus::Present || !state.owner_action_available {
        return state;
    }

    FindFamiliarCompanionState {
        owner_action_available: false,
        familiar_reaction_available: true,
        scenario_outcome: FindFamiliarCompanionScenarioOutcome::DismissedAndReappeared,
        protocol: FindFamiliarCompanionProtocol::Resolved,
        ..state
    }
}

#[must_use]
pub fn share_find_familiar_senses(state: FindFamiliarCompanionState) -> FindFamiliarCompanionState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-E-L.md
    // "Telepathic Connection".
    if state.status != FamiliarStatus::Present
        || !state.telepathy_available
        || !state.bonus_action_available
    {
        return state;
    }

    FindFamiliarCompanionState {
        shared_senses_active: true,
        bonus_action_available: false,
        scenario_outcome: FindFamiliarCompanionScenarioOutcome::SharedSenses,
        protocol: FindFamiliarCompanionProtocol::Resolved,
        ..state
    }
}

#[must_use]
pub fn deliver_find_familiar_touch_spell(
    state: FindFamiliarCompanionState,
) -> FindFamiliarCompanionState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-E-L.md
    // "Find Familiar", touch delivery.
    if state.status != FamiliarStatus::Present
        || !state.telepathy_available
        || !state.owner_action_available
        || !state.owner_attack_available
        || !state.familiar_reaction_available
        || state.spell_slot_committed
    {
        return state;
    }

    FindFamiliarCompanionState {
        owner_action_available: false,
        owner_attack_available: false,
        familiar_reaction_available: false,
        touch_delivery_reaction_spent: true,
        pact_reaction_attack_resolved: false,
        spell_slot_committed: true,
        scenario_outcome: FindFamiliarCompanionScenarioOutcome::TouchDelivered,
        protocol: FindFamiliarCompanionProtocol::Resolved,
        ..state
    }
}

#[must_use]
pub fn resolve_pact_find_familiar_reaction_attack(
    state: FindFamiliarCompanionState,
    damage: i16,
) -> FindFamiliarCompanionState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Classes/Warlock.md
    // "Pact of the Chain".
    if state.status != FamiliarStatus::Present
        || !state.owner_attack_available
        || !state.familiar_reaction_available
    {
        return state;
    }

    FindFamiliarCompanionState {
        owner_attack_available: false,
        familiar_reaction_available: false,
        touch_delivery_reaction_spent: false,
        pact_reaction_attack_resolved: true,
        target_hit_points: apply_companion_damage(state.target_hit_points, damage),
        scenario_outcome: FindFamiliarCompanionScenarioOutcome::PactAttack,
        protocol: FindFamiliarCompanionProtocol::Resolved,
        ..state
    }
}

#[must_use]
pub fn find_familiar_companion_route_observed(
    state: FindFamiliarCompanionState,
) -> BattleReducerRouteTrace {
    let mut trace = BattleReducerRouteTrace::default();
    observe_find_familiar_companion_route(state, &mut trace);
    trace
}

pub fn observe_find_familiar_companion_route(
    state: FindFamiliarCompanionState,
    observer: &mut impl BattleReducerRouteObserver,
) {
    observe_battle_route_start(BattleReducerRouteOwnerGroup::ActionEconomy, observer);
    match state.scenario_outcome {
        FindFamiliarCompanionScenarioOutcome::Created
        | FindFamiliarCompanionScenarioOutcome::Replaced
        | FindFamiliarCompanionScenarioOutcome::DismissedAndReappeared => {
            observe_companion_lifecycle_route(observer);
        }
        FindFamiliarCompanionScenarioOutcome::SharedSenses => {
            observe_companion_shared_senses_route(observer);
        }
        FindFamiliarCompanionScenarioOutcome::TouchDelivered => {
            observe_companion_touch_delivery_route(observer);
        }
        FindFamiliarCompanionScenarioOutcome::PactAttack => {
            observe_companion_reaction_attack_route(observer);
        }
        FindFamiliarCompanionScenarioOutcome::Init => {}
    }
}

fn observe_companion_lifecycle_route(observer: &mut impl BattleReducerRouteObserver) {
    observe_battle_route_discovery(
        BattleReducerRouteSubjectFamily::CompanionLifecycle,
        Vec::new(),
        BattleReducerRouteOwnerGroup::Companion,
        observer,
    );
    observe_battle_route_resolution_without_fill(
        BattleReducerRouteSubjectFamily::CompanionLifecycle,
        BattleResolutionOutcome::Resolved,
        Vec::new(),
        BattleReducerRouteOwnerGroup::Companion,
        observer,
    );
}

fn observe_companion_shared_senses_route(observer: &mut impl BattleReducerRouteObserver) {
    observe_battle_route_discovery(
        BattleReducerRouteSubjectFamily::CompanionSharedSenses,
        Vec::new(),
        BattleReducerRouteOwnerGroup::Companion,
        observer,
    );
    observe_battle_route_resolution_without_fill(
        BattleReducerRouteSubjectFamily::CompanionSharedSenses,
        BattleResolutionOutcome::Resolved,
        Vec::new(),
        BattleReducerRouteOwnerGroup::ActionEconomy,
        observer,
    );
    observe_battle_route_resolution_without_fill(
        BattleReducerRouteSubjectFamily::CompanionSharedSenses,
        BattleResolutionOutcome::Resolved,
        Vec::new(),
        BattleReducerRouteOwnerGroup::ActiveEffect,
        observer,
    );
}

fn observe_companion_touch_delivery_route(observer: &mut impl BattleReducerRouteObserver) {
    observe_battle_route_discovery(
        BattleReducerRouteSubjectFamily::CompanionTouchDelivery,
        vec![BattleReducerRouteHoleKind::TargetChoice],
        BattleReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
        observer,
    );
    observe_battle_route_resolution(
        BattleReducerRouteSubjectFamily::CompanionTouchDelivery,
        BattleReducerRouteFillKind::TargetChoice,
        BattleResolutionOutcome::NeedsHoles,
        vec![BattleReducerRouteHoleKind::RolledDice],
        BattleReducerRouteOwnerGroup::Companion,
        observer,
    );
    observe_battle_route_resolution(
        BattleReducerRouteSubjectFamily::CompanionTouchDelivery,
        BattleReducerRouteFillKind::RolledDice,
        BattleResolutionOutcome::Resolved,
        Vec::new(),
        BattleReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
        observer,
    );
    observe_battle_route_resolution_without_fill(
        BattleReducerRouteSubjectFamily::CompanionTouchDelivery,
        BattleResolutionOutcome::Resolved,
        Vec::new(),
        BattleReducerRouteOwnerGroup::ActionEconomy,
        observer,
    );
}

fn observe_companion_reaction_attack_route(observer: &mut impl BattleReducerRouteObserver) {
    observe_battle_route_discovery(
        BattleReducerRouteSubjectFamily::CompanionReactionAttack,
        vec![BattleReducerRouteHoleKind::TargetChoice],
        BattleReducerRouteOwnerGroup::Companion,
        observer,
    );
    observe_battle_route_resolution_without_fill(
        BattleReducerRouteSubjectFamily::CompanionReactionAttack,
        BattleResolutionOutcome::NeedsHoles,
        vec![BattleReducerRouteHoleKind::TargetChoice],
        BattleReducerRouteOwnerGroup::StatBlockAction,
        observer,
    );
    observe_battle_route_resolution(
        BattleReducerRouteSubjectFamily::CompanionReactionAttack,
        BattleReducerRouteFillKind::TargetChoice,
        BattleResolutionOutcome::NeedsHoles,
        vec![BattleReducerRouteHoleKind::AttackRoll],
        BattleReducerRouteOwnerGroup::TargetSelection,
        observer,
    );
    observe_battle_route_resolution(
        BattleReducerRouteSubjectFamily::CompanionReactionAttack,
        BattleReducerRouteFillKind::AttackRoll,
        BattleResolutionOutcome::NeedsHoles,
        vec![BattleReducerRouteHoleKind::RolledDice],
        BattleReducerRouteOwnerGroup::AttackRoll,
        observer,
    );
    observe_battle_route_resolution(
        BattleReducerRouteSubjectFamily::CompanionReactionAttack,
        BattleReducerRouteFillKind::RolledDice,
        BattleResolutionOutcome::Resolved,
        Vec::new(),
        BattleReducerRouteOwnerGroup::HitPoint,
        observer,
    );
    observe_battle_route_resolution_without_fill(
        BattleReducerRouteSubjectFamily::CompanionReactionAttack,
        BattleResolutionOutcome::Resolved,
        Vec::new(),
        BattleReducerRouteOwnerGroup::ActionEconomy,
        observer,
    );
}

fn apply_companion_damage(hit_points: i16, damage: i16) -> i16 {
    hit_points.saturating_sub(damage.max(0)).max(0)
}
