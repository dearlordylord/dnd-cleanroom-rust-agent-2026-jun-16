#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SleepRepeatSaveTurnRole {
    Caster,
    Target,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SleepRepeatSaveHole {
    SavingThrowOutcome,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SleepRepeatSaveProtocol {
    Init,
    NeedsHoles,
    Resolved,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SleepRepeatSaveState {
    pub current_turn_role: SleepRepeatSaveTurnRole,
    pub target_incapacitated: bool,
    pub target_unconscious: bool,
    pub target_prone: bool,
    pub caster_concentrating: bool,
    pub action_available: bool,
    pub protocol: SleepRepeatSaveProtocol,
    pub protocol_holes: Vec<SleepRepeatSaveHole>,
}

#[must_use]
pub fn sleep_repeat_save_initial_state() -> SleepRepeatSaveState {
    SleepRepeatSaveState {
        current_turn_role: SleepRepeatSaveTurnRole::Caster,
        target_incapacitated: false,
        target_unconscious: false,
        target_prone: false,
        caster_concentrating: false,
        action_available: true,
        protocol: SleepRepeatSaveProtocol::Init,
        protocol_holes: vec![SleepRepeatSaveHole::SavingThrowOutcome],
    }
}

#[must_use]
pub fn fill_sleep_initial_save_failure() -> SleepRepeatSaveState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-S-Z.md
    // "Sleep"; QNT: battle-runtime-sleep-repeat-save.mbt.qnt.
    SleepRepeatSaveState {
        current_turn_role: SleepRepeatSaveTurnRole::Caster,
        target_incapacitated: true,
        target_unconscious: false,
        target_prone: false,
        caster_concentrating: true,
        action_available: false,
        protocol: SleepRepeatSaveProtocol::Resolved,
        protocol_holes: Vec::new(),
    }
}

#[must_use]
pub fn break_sleep_concentration_before_repeat() -> SleepRepeatSaveState {
    let mut state = fill_sleep_initial_save_failure();
    state.target_incapacitated = false;
    state.caster_concentrating = false;
    state
}

#[must_use]
pub fn end_caster_turn_with_sleep() -> SleepRepeatSaveState {
    let mut state = fill_sleep_initial_save_failure();
    state.current_turn_role = SleepRepeatSaveTurnRole::Target;
    state.action_available = true;
    state
}

#[must_use]
pub fn end_caster_turn_after_sleep_concentration_break() -> SleepRepeatSaveState {
    let mut state = break_sleep_concentration_before_repeat();
    state.current_turn_role = SleepRepeatSaveTurnRole::Target;
    state.action_available = true;
    state
}

#[must_use]
pub fn end_target_turn_after_sleep_concentration_break() -> SleepRepeatSaveState {
    let mut state = end_caster_turn_after_sleep_concentration_break();
    state.current_turn_role = SleepRepeatSaveTurnRole::Caster;
    state
}

#[must_use]
pub fn discover_sleep_repeat_save() -> SleepRepeatSaveState {
    let mut state = end_caster_turn_with_sleep();
    state.protocol = SleepRepeatSaveProtocol::NeedsHoles;
    state.protocol_holes = vec![SleepRepeatSaveHole::SavingThrowOutcome];
    state
}

#[must_use]
pub fn fill_sleep_repeat_save_success() -> SleepRepeatSaveState {
    let mut state = end_caster_turn_with_sleep();
    state.current_turn_role = SleepRepeatSaveTurnRole::Caster;
    state.target_incapacitated = false;
    state
}

#[must_use]
pub fn fill_sleep_repeat_save_failure() -> SleepRepeatSaveState {
    let mut state = end_caster_turn_with_sleep();
    state.current_turn_role = SleepRepeatSaveTurnRole::Caster;
    state.target_unconscious = true;
    state.target_prone = true;
    state
}
