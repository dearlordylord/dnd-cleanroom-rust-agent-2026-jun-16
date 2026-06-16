use crate::rules::rule_core_reactions::{
    break_reactor_concentration_after_large_damage, decline_reaction_opportunity_attack,
    decline_readied_movement_reaction, hold_reactor_concentration_after_small_damage,
    offer_reaction_opportunity_attack, offer_readied_movement_reaction,
    ready_reaction_movement_fixture, reject_readied_movement_zero, start_reactor_concentration,
    take_readied_movement_fill, take_readied_movement_short, RuleCorePendingTrigger,
    RuleCoreReactionHole, RuleCoreReactionInvalidReason, RuleCoreReactionProtocol,
    RuleCoreReactionState, RuleCoreReactionWindow,
};

pub const BRANCH_ACTIONS: [&str; 11] = [
    "doBreakReactorConcentrationAfterLargeDamage",
    "doDeclineOpportunityAttack",
    "doDeclineReadiedMovement",
    "doHoldReactorConcentrationAfterSmallDamage",
    "doOfferOpportunityAttack",
    "doOfferReadiedMovement",
    "doReadyMovementFixture",
    "doRejectReadiedMovementZero",
    "doStartReactorConcentrationFixture",
    "doTakeReadiedMovementFill",
    "doTakeReadiedMovementShort",
];

pub fn replay_observed_action(observed_action_taken: &str) -> RuleCoreReactionState {
    match observed_action_taken {
        "doBreakReactorConcentrationAfterLargeDamage" => {
            break_reactor_concentration_after_large_damage()
        }
        "doDeclineOpportunityAttack" => decline_reaction_opportunity_attack(),
        "doDeclineReadiedMovement" => decline_readied_movement_reaction(),
        "doHoldReactorConcentrationAfterSmallDamage" => {
            hold_reactor_concentration_after_small_damage()
        }
        "doOfferOpportunityAttack" => offer_reaction_opportunity_attack(),
        "doOfferReadiedMovement" => offer_readied_movement_reaction(),
        "doReadyMovementFixture" => ready_reaction_movement_fixture(),
        "doRejectReadiedMovementZero" => reject_readied_movement_zero(),
        "doStartReactorConcentrationFixture" => start_reactor_concentration(),
        "doTakeReadiedMovementFill" => take_readied_movement_fill(),
        "doTakeReadiedMovementShort" => take_readied_movement_short(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> RuleCoreReactionState {
    replay_observed_action(observed_action_taken)
}

pub fn projection_payload(state: &RuleCoreReactionState) -> String {
    let protocol_holes = protocol_holes(&state.protocol);
    [
        format!(
            "qInterruptedMovementSpentFeet={}",
            state.interrupted_movement_spent_feet
        ),
        format!(
            "qReactorReactionAvailable={}",
            state.reactor_reaction_available
        ),
        format!(
            "qReactorReadiedMovementHeld={}",
            state.reactor_readied_movement_held
        ),
        format!(
            "qReactorReadiedSpellHeld={}",
            state.reactor_readied_spell_held
        ),
        format!(
            "qReactorMovementSpentFeet={}",
            state.reactor_movement_spent_feet
        ),
        format!(
            "qInterruptedConcentration={}",
            state.interrupted_concentration
        ),
        format!("qReactorConcentration={}", state.reactor_concentration),
        format!(
            "qReactionWindow={}",
            reaction_window_ref(state.reaction_window)
        ),
        format!(
            "qPendingTrigger={}",
            pending_trigger_ref(state.pending_trigger)
        ),
        format!("qHoles={}", joined_or_none(&protocol_holes)),
        format!(
            "qLastConcentrationSaveDc={}",
            state.last_concentration_save_dc
        ),
        format!("protocolResult={}", protocol_result_ref(&state.protocol)),
        format!(
            "protocolInvalidReason={}",
            protocol_invalid_reason_ref(&state.protocol)
        ),
        format!("protocolHoles={}", joined_or_none(&protocol_holes)),
    ]
    .join("\n")
}

fn reaction_window_ref(window: RuleCoreReactionWindow) -> &'static str {
    match window {
        RuleCoreReactionWindow::NoReactionWindow => "NoReactionWindow",
        RuleCoreReactionWindow::OfferedOpportunityAttackWindow => "OfferedOpportunityAttackWindow",
        RuleCoreReactionWindow::OfferedReadiedMovementWindow => "OfferedReadiedMovementWindow",
    }
}

fn pending_trigger_ref(trigger: RuleCorePendingTrigger) -> &'static str {
    match trigger {
        RuleCorePendingTrigger::None => "none",
        RuleCorePendingTrigger::OpportunityAttack => "opportunityAttack",
        RuleCorePendingTrigger::AttackHit => "attackHit",
    }
}

fn protocol_result_ref(protocol: &RuleCoreReactionProtocol) -> &'static str {
    match protocol {
        RuleCoreReactionProtocol::Init => "init",
        RuleCoreReactionProtocol::NeedsHoles(_) => "needsHoles",
        RuleCoreReactionProtocol::Resolved => "resolved",
        RuleCoreReactionProtocol::Invalid { .. } => "invalid",
    }
}

fn protocol_invalid_reason_ref(protocol: &RuleCoreReactionProtocol) -> &'static str {
    match protocol {
        RuleCoreReactionProtocol::Invalid {
            reason: RuleCoreReactionInvalidReason::InvalidFill,
            ..
        } => "WInvalidFill",
        RuleCoreReactionProtocol::Init
        | RuleCoreReactionProtocol::NeedsHoles(_)
        | RuleCoreReactionProtocol::Resolved => "",
    }
}

fn protocol_holes(protocol: &RuleCoreReactionProtocol) -> Vec<&'static str> {
    match protocol {
        RuleCoreReactionProtocol::NeedsHoles(holes) => holes.iter().map(hole_ref).collect(),
        RuleCoreReactionProtocol::Invalid { holes, .. } => holes.iter().map(hole_ref).collect(),
        RuleCoreReactionProtocol::Init | RuleCoreReactionProtocol::Resolved => Vec::new(),
    }
}

fn hole_ref(hole: &RuleCoreReactionHole) -> &'static str {
    match hole {
        RuleCoreReactionHole::ReactionDecision => "ReactionDecision",
        RuleCoreReactionHole::DamageRoll => "DamageRoll",
    }
}

fn joined_or_none(values: &[&str]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}
