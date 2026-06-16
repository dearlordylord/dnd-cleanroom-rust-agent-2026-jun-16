#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MovementActor {
    Fighter,
    GrappledTarget,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MovementHole {
    Movement,
    ReactionDecision,
    TargetChoice,
    GrappleOutcome,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MovementInvalidReason {
    InvalidFill,
    StaleSubject,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MovementProtocol {
    Init,
    NeedsHoles(Vec<MovementHole>),
    Resolved,
    Invalid {
        holes: Vec<MovementHole>,
        reason: MovementInvalidReason,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MovementState {
    pub current_actor: MovementActor,
    pub movement_spent_feet: i16,
    pub dash_bonus_feet: i16,
    pub prone: bool,
    pub disengaged: bool,
    pub action_available: bool,
    pub grapple_active: bool,
    pub grapple_escape_dc: i16,
    pub protocol: MovementProtocol,
    pub pending_opportunity_attack: bool,
}

pub const MOVEMENT_SPEED_FEET: i16 = 30;
pub const MOVEMENT_SHORT_COST_FEET: i16 = 5;
pub const MOVEMENT_FILL_COST_FEET: i16 = 10;
pub const MOVEMENT_FULL_COST_FEET: i16 = 30;
pub const MOVEMENT_GRAPPLE_ESCAPE_DC: i16 = 10;

#[must_use]
pub fn movement_initial_state() -> MovementState {
    MovementState {
        current_actor: MovementActor::Fighter,
        movement_spent_feet: 0,
        dash_bonus_feet: 0,
        prone: true,
        disengaged: false,
        action_available: true,
        grapple_active: false,
        grapple_escape_dc: 0,
        protocol: MovementProtocol::Init,
        pending_opportunity_attack: false,
    }
}

#[must_use]
pub fn discover_movement() -> MovementState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md
    // "Movement and Position", "Actions", and "Opportunity Attacks";
    // Rules-Glossary.md "Dash", "Disengage", "Grappling", and
    // "Opportunity Attacks". QNT: rule-core-movement.mbt.qnt and
    // shared-algebras/proofs/rule-core/movement-spatial-grapple.qnt.
    with_protocol(
        movement_initial_state(),
        MovementProtocol::NeedsHoles(vec![MovementHole::Movement]),
    )
}

#[must_use]
pub fn spend_movement() -> MovementState {
    resolved_after_movement_spend(MOVEMENT_FILL_COST_FEET, false, true)
}

#[must_use]
pub fn spend_short_movement() -> MovementState {
    resolved_after_movement_spend(MOVEMENT_SHORT_COST_FEET, false, true)
}

#[must_use]
pub fn spend_full_movement() -> MovementState {
    resolved_after_movement_spend(MOVEMENT_FULL_COST_FEET, false, true)
}

#[must_use]
pub fn move_provokes_opportunity_attack() -> MovementState {
    MovementState {
        protocol: MovementProtocol::NeedsHoles(vec![MovementHole::ReactionDecision]),
        pending_opportunity_attack: true,
        ..movement_initial_state()
    }
}

#[must_use]
pub fn move_threat_suppressed_by_disengage() -> MovementState {
    resolved_after_movement_spend(MOVEMENT_FILL_COST_FEET, true, false)
}

#[must_use]
pub fn decline_opportunity_attack() -> MovementState {
    resolved_after_movement_spend(MOVEMENT_FILL_COST_FEET, false, true)
}

#[must_use]
pub fn reject_movement_overspend() -> MovementState {
    MovementState {
        protocol: MovementProtocol::Invalid {
            holes: vec![MovementHole::Movement],
            reason: MovementInvalidReason::InvalidFill,
        },
        ..movement_initial_state()
    }
}

#[must_use]
pub fn dash() -> MovementState {
    MovementState {
        dash_bonus_feet: MOVEMENT_SPEED_FEET,
        action_available: false,
        protocol: MovementProtocol::Resolved,
        ..movement_initial_state()
    }
}

#[must_use]
pub fn disengage() -> MovementState {
    MovementState {
        disengaged: true,
        action_available: false,
        protocol: MovementProtocol::Resolved,
        ..movement_initial_state()
    }
}

#[must_use]
pub fn reject_dash_after_action_spent() -> MovementState {
    MovementState {
        dash_bonus_feet: MOVEMENT_SPEED_FEET,
        action_available: false,
        protocol: MovementProtocol::Invalid {
            holes: Vec::new(),
            reason: MovementInvalidReason::StaleSubject,
        },
        ..movement_initial_state()
    }
}

#[must_use]
pub fn stand_from_prone() -> MovementState {
    MovementState {
        movement_spent_feet: MOVEMENT_SPEED_FEET / 2,
        prone: false,
        protocol: MovementProtocol::Resolved,
        ..movement_initial_state()
    }
}

#[must_use]
pub fn discover_grapple() -> MovementState {
    with_protocol(
        movement_initial_state(),
        MovementProtocol::NeedsHoles(vec![MovementHole::TargetChoice]),
    )
}

#[must_use]
pub fn select_grapple_target() -> MovementState {
    with_protocol(
        movement_initial_state(),
        MovementProtocol::NeedsHoles(vec![MovementHole::GrappleOutcome]),
    )
}

#[must_use]
pub fn resolve_grapple_success() -> MovementState {
    grapple_state(
        true,
        MOVEMENT_GRAPPLE_ESCAPE_DC,
        false,
        MovementActor::Fighter,
    )
}

#[must_use]
pub fn resolve_grapple_failure() -> MovementState {
    grapple_state(false, 0, false, MovementActor::Fighter)
}

#[must_use]
pub fn start_grappled_target_turn() -> MovementState {
    grapple_state(
        true,
        MOVEMENT_GRAPPLE_ESCAPE_DC,
        true,
        MovementActor::GrappledTarget,
    )
}

#[must_use]
pub fn discover_escape_grapple() -> MovementState {
    MovementState {
        current_actor: MovementActor::GrappledTarget,
        action_available: true,
        grapple_active: true,
        grapple_escape_dc: MOVEMENT_GRAPPLE_ESCAPE_DC,
        protocol: MovementProtocol::NeedsHoles(vec![MovementHole::GrappleOutcome]),
        ..movement_initial_state()
    }
}

#[must_use]
pub fn resolve_escape_success() -> MovementState {
    grapple_state(false, 0, false, MovementActor::GrappledTarget)
}

#[must_use]
pub fn resolve_escape_failure() -> MovementState {
    grapple_state(
        true,
        MOVEMENT_GRAPPLE_ESCAPE_DC,
        false,
        MovementActor::GrappledTarget,
    )
}

#[must_use]
pub fn release_grapple() -> MovementState {
    grapple_state(false, 0, false, MovementActor::Fighter)
}

fn resolved_after_movement_spend(
    movement_spent_feet: i16,
    disengaged: bool,
    action_available: bool,
) -> MovementState {
    MovementState {
        movement_spent_feet,
        disengaged,
        action_available,
        protocol: MovementProtocol::Resolved,
        ..movement_initial_state()
    }
}

fn grapple_state(
    grapple_active: bool,
    grapple_escape_dc: i16,
    action_available: bool,
    current_actor: MovementActor,
) -> MovementState {
    MovementState {
        current_actor,
        action_available,
        grapple_active,
        grapple_escape_dc,
        protocol: MovementProtocol::Resolved,
        ..movement_initial_state()
    }
}

fn with_protocol(state: MovementState, protocol: MovementProtocol) -> MovementState {
    MovementState { protocol, ..state }
}
