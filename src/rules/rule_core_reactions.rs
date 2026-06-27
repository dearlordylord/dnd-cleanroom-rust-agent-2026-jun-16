use crate::rules::concentration::concentration_save_dc_for_damage;
use crate::rules::rule_core_movement::{MOVEMENT_FILL_COST_FEET, MOVEMENT_SHORT_COST_FEET};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleCoreReactionWindow {
    NoReactionWindow,
    OfferedOpportunityAttackWindow,
    OfferedReadiedMovementWindow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleCorePendingTrigger {
    None,
    OpportunityAttack,
    AttackHit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleCoreReactionHole {
    ReactionDecision,
    DamageRoll,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleCoreReactionInvalidReason {
    InvalidFill,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuleCoreReactionProtocol {
    Init,
    NeedsHoles(Vec<RuleCoreReactionHole>),
    Resolved,
    Invalid {
        holes: Vec<RuleCoreReactionHole>,
        reason: RuleCoreReactionInvalidReason,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuleCoreReactionState {
    pub interrupted_movement_spent_feet: i16,
    pub reactor_reaction_available: bool,
    pub reactor_readied_movement_held: bool,
    pub reactor_readied_spell_held: bool,
    pub reactor_movement_spent_feet: i16,
    pub interrupted_concentration: bool,
    pub reactor_concentration: bool,
    pub reaction_window: RuleCoreReactionWindow,
    pub pending_trigger: RuleCorePendingTrigger,
    pub protocol: RuleCoreReactionProtocol,
    pub last_concentration_save_dc: i16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleCoreReactionSubject {
    OfferOpportunityAttack,
    ReactionDecision(RuleCoreReactionDecision),
    ReadyMovement,
    OfferReadiedMovement,
    StartReactorConcentration,
    ResolveReactorConcentrationAfterDamage {
        damage_taken: i16,
        save_succeeded: bool,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleCoreReactionDecision {
    Decline,
    TakeReadiedMovement { distance_feet: i16 },
}

#[must_use]
pub fn rule_core_reactions_initial_state() -> RuleCoreReactionState {
    RuleCoreReactionState {
        interrupted_movement_spent_feet: 0,
        reactor_reaction_available: true,
        reactor_readied_movement_held: false,
        reactor_readied_spell_held: false,
        reactor_movement_spent_feet: 0,
        interrupted_concentration: false,
        reactor_concentration: false,
        reaction_window: RuleCoreReactionWindow::NoReactionWindow,
        pending_trigger: RuleCorePendingTrigger::None,
        protocol: RuleCoreReactionProtocol::Init,
        last_concentration_save_dc: 0,
    }
}

#[must_use]
pub fn resolve_rule_core_reaction_subject(
    state: RuleCoreReactionState,
    subject: RuleCoreReactionSubject,
) -> RuleCoreReactionState {
    // QNT: cleanroom-input/qnt/shared-algebras/proofs/rule-core/
    // reactions-continuations-concentration.qnt; MBT projection:
    // cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt.
    match subject {
        RuleCoreReactionSubject::OfferOpportunityAttack => offer_opportunity_attack_from(state),
        RuleCoreReactionSubject::ReactionDecision(RuleCoreReactionDecision::Decline) => {
            decline_reaction_from(state)
        }
        RuleCoreReactionSubject::ReactionDecision(
            RuleCoreReactionDecision::TakeReadiedMovement { distance_feet },
        ) => take_readied_movement_from(state, distance_feet),
        RuleCoreReactionSubject::ReadyMovement => ready_movement_from(state),
        RuleCoreReactionSubject::OfferReadiedMovement => offer_readied_movement_from(state),
        RuleCoreReactionSubject::StartReactorConcentration => {
            start_reactor_concentration_from(state)
        }
        RuleCoreReactionSubject::ResolveReactorConcentrationAfterDamage {
            damage_taken,
            save_succeeded,
        } => resolve_reactor_concentration_after_damage_from(state, damage_taken, save_succeeded),
    }
}

#[must_use]
pub fn offer_reaction_opportunity_attack() -> RuleCoreReactionState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md
    // "Reactions" and "Opportunity Attacks"; Rules-Glossary.md "Reaction".
    // QNT: rule-core-reactions.mbt.qnt and
    // shared-algebras/proofs/rule-core/reactions-continuations-concentration.qnt.
    offer_opportunity_attack_from(rule_core_reactions_initial_state())
}

#[must_use]
pub fn decline_reaction_opportunity_attack() -> RuleCoreReactionState {
    decline_reaction_from(offer_reaction_opportunity_attack())
}

#[must_use]
pub fn ready_movement_reaction() -> RuleCoreReactionState {
    ready_movement_from(rule_core_reactions_initial_state())
}

#[must_use]
pub fn offer_readied_movement_reaction() -> RuleCoreReactionState {
    offer_readied_movement_from(ready_movement_reaction())
}

#[must_use]
pub fn decline_readied_movement_reaction() -> RuleCoreReactionState {
    decline_reaction_from(offer_readied_movement_reaction())
}

#[must_use]
pub fn take_readied_movement_short() -> RuleCoreReactionState {
    spend_readied_movement(MOVEMENT_SHORT_COST_FEET)
}

#[must_use]
pub fn take_readied_movement_fill() -> RuleCoreReactionState {
    spend_readied_movement(MOVEMENT_FILL_COST_FEET)
}

#[must_use]
pub fn reject_readied_movement_zero() -> RuleCoreReactionState {
    take_readied_movement_from(offer_readied_movement_reaction(), 0)
}

#[must_use]
pub fn start_reactor_concentration() -> RuleCoreReactionState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Rules-Glossary.md "Concentration".
    start_reactor_concentration_from(rule_core_reactions_initial_state())
}

#[must_use]
pub fn hold_reactor_concentration_after_small_damage() -> RuleCoreReactionState {
    resolve_reactor_concentration_after_damage(8, true)
}

#[must_use]
pub fn break_reactor_concentration_after_large_damage() -> RuleCoreReactionState {
    resolve_reactor_concentration_after_damage(22, false)
}

fn spend_readied_movement(distance_feet: i16) -> RuleCoreReactionState {
    take_readied_movement_from(offer_readied_movement_reaction(), distance_feet)
}

fn offer_opportunity_attack_from(state: RuleCoreReactionState) -> RuleCoreReactionState {
    RuleCoreReactionState {
        reaction_window: RuleCoreReactionWindow::OfferedOpportunityAttackWindow,
        pending_trigger: RuleCorePendingTrigger::OpportunityAttack,
        protocol: RuleCoreReactionProtocol::NeedsHoles(vec![
            RuleCoreReactionHole::ReactionDecision,
        ]),
        ..state
    }
}

fn ready_movement_from(state: RuleCoreReactionState) -> RuleCoreReactionState {
    RuleCoreReactionState {
        reactor_readied_movement_held: true,
        protocol: RuleCoreReactionProtocol::Resolved,
        ..state
    }
}

fn offer_readied_movement_from(state: RuleCoreReactionState) -> RuleCoreReactionState {
    RuleCoreReactionState {
        reaction_window: RuleCoreReactionWindow::OfferedReadiedMovementWindow,
        pending_trigger: RuleCorePendingTrigger::AttackHit,
        protocol: RuleCoreReactionProtocol::NeedsHoles(vec![
            RuleCoreReactionHole::ReactionDecision,
        ]),
        ..state
    }
}

fn decline_reaction_from(state: RuleCoreReactionState) -> RuleCoreReactionState {
    match (state.pending_trigger, state.reaction_window) {
        (
            RuleCorePendingTrigger::OpportunityAttack,
            RuleCoreReactionWindow::OfferedOpportunityAttackWindow,
        ) => RuleCoreReactionState {
            interrupted_movement_spent_feet: state.interrupted_movement_spent_feet
                + MOVEMENT_FILL_COST_FEET,
            reaction_window: RuleCoreReactionWindow::NoReactionWindow,
            pending_trigger: RuleCorePendingTrigger::None,
            protocol: RuleCoreReactionProtocol::Resolved,
            ..state
        },
        (
            RuleCorePendingTrigger::AttackHit,
            RuleCoreReactionWindow::OfferedReadiedMovementWindow,
        ) => RuleCoreReactionState {
            reaction_window: RuleCoreReactionWindow::NoReactionWindow,
            pending_trigger: RuleCorePendingTrigger::None,
            protocol: RuleCoreReactionProtocol::NeedsHoles(vec![RuleCoreReactionHole::DamageRoll]),
            ..state
        },
        _ => invalid_reaction_decision(state),
    }
}

fn take_readied_movement_from(
    state: RuleCoreReactionState,
    distance_feet: i16,
) -> RuleCoreReactionState {
    if distance_feet <= 0
        || state.pending_trigger != RuleCorePendingTrigger::AttackHit
        || state.reaction_window != RuleCoreReactionWindow::OfferedReadiedMovementWindow
    {
        return invalid_reaction_decision(state);
    }

    RuleCoreReactionState {
        reactor_reaction_available: false,
        reactor_readied_movement_held: false,
        reactor_movement_spent_feet: distance_feet,
        reaction_window: RuleCoreReactionWindow::NoReactionWindow,
        pending_trigger: RuleCorePendingTrigger::None,
        protocol: RuleCoreReactionProtocol::NeedsHoles(vec![RuleCoreReactionHole::DamageRoll]),
        ..state
    }
}

fn invalid_reaction_decision(state: RuleCoreReactionState) -> RuleCoreReactionState {
    RuleCoreReactionState {
        protocol: RuleCoreReactionProtocol::Invalid {
            holes: vec![RuleCoreReactionHole::ReactionDecision],
            reason: RuleCoreReactionInvalidReason::InvalidFill,
        },
        ..state
    }
}

fn start_reactor_concentration_from(state: RuleCoreReactionState) -> RuleCoreReactionState {
    RuleCoreReactionState {
        reactor_concentration: true,
        protocol: RuleCoreReactionProtocol::Resolved,
        ..state
    }
}

fn resolve_reactor_concentration_after_damage_from(
    state: RuleCoreReactionState,
    damage_taken: i16,
    save_succeeded: bool,
) -> RuleCoreReactionState {
    let resolved = resolve_reactor_concentration_after_damage(damage_taken, save_succeeded);
    RuleCoreReactionState {
        reactor_concentration: resolved.reactor_concentration,
        protocol: resolved.protocol,
        last_concentration_save_dc: resolved.last_concentration_save_dc,
        ..state
    }
}

fn resolve_reactor_concentration_after_damage(
    damage_taken: i16,
    save_succeeded: bool,
) -> RuleCoreReactionState {
    RuleCoreReactionState {
        reactor_concentration: save_succeeded,
        protocol: RuleCoreReactionProtocol::Resolved,
        last_concentration_save_dc: concentration_save_dc_for_damage(damage_taken),
        ..rule_core_reactions_initial_state()
    }
}
