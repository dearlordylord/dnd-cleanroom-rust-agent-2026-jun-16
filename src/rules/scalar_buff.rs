#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScalarBuffTargetHole {
    TargetChoice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScalarBuffTargetInvalidReason {
    StaleSubject,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScalarBuffTargetProtocol {
    Init,
    Resolved,
    Invalid(ScalarBuffTargetInvalidReason),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScalarBuffTargetState {
    pub fighter_speed_feet: i16,
    pub goblin_speed_feet: i16,
    pub action_available: bool,
    pub protocol: ScalarBuffTargetProtocol,
    pub protocol_holes: Vec<ScalarBuffTargetHole>,
}

#[must_use]
pub fn scalar_buff_target_initial_state() -> ScalarBuffTargetState {
    ScalarBuffTargetState {
        fighter_speed_feet: 30,
        goblin_speed_feet: 30,
        action_available: true,
        protocol: ScalarBuffTargetProtocol::Init,
        protocol_holes: vec![ScalarBuffTargetHole::TargetChoice],
    }
}

#[must_use]
pub fn fill_longstrider_target_choice() -> ScalarBuffTargetState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-E-L.md
    // "Longstrider"; QNT: battle-runtime-scalar-buff.mbt.qnt.
    ScalarBuffTargetState {
        fighter_speed_feet: 30,
        goblin_speed_feet: 40,
        action_available: false,
        protocol: ScalarBuffTargetProtocol::Resolved,
        protocol_holes: Vec::new(),
    }
}

#[must_use]
pub fn reject_stale_scalar_buff_after_resolved() -> ScalarBuffTargetState {
    let mut state = fill_longstrider_target_choice();
    state.protocol = ScalarBuffTargetProtocol::Invalid(ScalarBuffTargetInvalidReason::StaleSubject);
    state
}
