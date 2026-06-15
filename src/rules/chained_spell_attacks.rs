#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChainedDamageTypeChoice {
    Acid,
    Cold,
    Fire,
    Lightning,
    Poison,
    Thunder,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChainedTarget {
    First,
    Second,
    Third,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChainedAttackStep {
    Step0,
    Step1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChainedAttackHole {
    DamageTypeChoice,
    TargetChoice0,
    AttackRoll0,
    DamageRoll0,
    TargetChoice1,
    AttackRoll1,
    DamageRoll1,
    TargetChoice2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChainedAttackProtocol {
    Init(ChainedAttackHole),
    NeedsHole(ChainedAttackHole),
    Resolved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChainedAttackScenarioOutcome {
    Init,
    AwaitingDamageType,
    AwaitingInitialTarget,
    AwaitingStep0Attack,
    AwaitingStep0Damage,
    Step0NoLeapComplete,
    AwaitingFirstLeapTarget,
    AwaitingStep1Attack,
    AwaitingStep1Damage,
    Slot1LeapLimitComplete,
    AwaitingSecondLeapTarget,
    Step1Complete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChainedAttackSequenceError {
    IllegalSlotLevel,
    WrongProtocol,
    TargetAlreadyTargeted,
    LeapTargetNotWithinPreviousTarget30Feet,
    MissingPreviousTarget,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChainedAttackRollFacts {
    pub attack_total: i16,
    pub natural_d20: i16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChainedDamageRollFacts {
    pub damage_total: i16,
    pub has_duplicate: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChromaticOrbSequenceState {
    pub damage_type: Option<ChainedDamageTypeChoice>,
    pub slot_level: i16,
    pub targeted: Vec<ChainedTarget>,
    pub previous_target: Option<ChainedTarget>,
    pub leaps_used: i16,
    pub step_index: i16,
    pub first_target_hit_points: i16,
    pub second_target_hit_points: i16,
    pub third_target_hit_points: i16,
    pub step0_attack: Option<ChainedAttackRollFacts>,
    pub step0_damage: Option<ChainedDamageRollFacts>,
    pub step1_attack: Option<ChainedAttackRollFacts>,
    pub step1_damage: Option<ChainedDamageRollFacts>,
    pub outcome: ChainedAttackScenarioOutcome,
    pub protocol: ChainedAttackProtocol,
}

#[must_use]
pub fn chromatic_orb_initial_state() -> ChromaticOrbSequenceState {
    ChromaticOrbSequenceState {
        damage_type: None,
        slot_level: 0,
        targeted: vec![],
        previous_target: None,
        leaps_used: 0,
        step_index: 0,
        first_target_hit_points: 12,
        second_target_hit_points: 12,
        third_target_hit_points: 12,
        step0_attack: None,
        step0_damage: None,
        step1_attack: None,
        step1_damage: None,
        outcome: ChainedAttackScenarioOutcome::Init,
        protocol: ChainedAttackProtocol::Init(ChainedAttackHole::DamageTypeChoice),
    }
}

pub fn start_chromatic_orb_cast(
    mut state: ChromaticOrbSequenceState,
    slot_level: i16,
) -> Result<ChromaticOrbSequenceState, ChainedAttackSequenceError> {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-A-D.md
    // "Chromatic Orb"; QNT: battle-runtime-chained-attack-sequence.mbt.qnt.
    if slot_level < chromatic_orb_base_spell_level() {
        return Err(ChainedAttackSequenceError::IllegalSlotLevel);
    }
    require_protocol(
        state.protocol,
        ChainedAttackProtocol::Init(ChainedAttackHole::DamageTypeChoice),
    )?;

    state.slot_level = slot_level;
    state.outcome = ChainedAttackScenarioOutcome::AwaitingDamageType;
    state.protocol = ChainedAttackProtocol::NeedsHole(ChainedAttackHole::DamageTypeChoice);
    Ok(state)
}

pub fn choose_chromatic_orb_damage_type(
    mut state: ChromaticOrbSequenceState,
    damage_type: ChainedDamageTypeChoice,
) -> Result<ChromaticOrbSequenceState, ChainedAttackSequenceError> {
    require_protocol(
        state.protocol,
        ChainedAttackProtocol::NeedsHole(ChainedAttackHole::DamageTypeChoice),
    )?;

    state.damage_type = Some(damage_type);
    state.outcome = ChainedAttackScenarioOutcome::AwaitingInitialTarget;
    state.protocol = ChainedAttackProtocol::NeedsHole(ChainedAttackHole::TargetChoice0);
    Ok(state)
}

pub fn choose_chromatic_orb_initial_target(
    mut state: ChromaticOrbSequenceState,
    target: ChainedTarget,
) -> Result<ChromaticOrbSequenceState, ChainedAttackSequenceError> {
    require_protocol(
        state.protocol,
        ChainedAttackProtocol::NeedsHole(ChainedAttackHole::TargetChoice0),
    )?;
    add_unique_target(&mut state, target)?;

    state.previous_target = Some(target);
    state.outcome = ChainedAttackScenarioOutcome::AwaitingStep0Attack;
    state.protocol = ChainedAttackProtocol::NeedsHole(ChainedAttackHole::AttackRoll0);
    Ok(state)
}

pub fn resolve_chromatic_orb_attack_hit(
    mut state: ChromaticOrbSequenceState,
    step: ChainedAttackStep,
    facts: ChainedAttackRollFacts,
) -> Result<ChromaticOrbSequenceState, ChainedAttackSequenceError> {
    match step {
        ChainedAttackStep::Step0 => {
            require_protocol(
                state.protocol,
                ChainedAttackProtocol::NeedsHole(ChainedAttackHole::AttackRoll0),
            )?;
            state.step_index = 0;
            state.step0_attack = Some(facts);
            state.outcome = ChainedAttackScenarioOutcome::AwaitingStep0Damage;
            state.protocol = ChainedAttackProtocol::NeedsHole(ChainedAttackHole::DamageRoll0);
        }
        ChainedAttackStep::Step1 => {
            require_protocol(
                state.protocol,
                ChainedAttackProtocol::NeedsHole(ChainedAttackHole::AttackRoll1),
            )?;
            state.step_index = 1;
            state.step1_attack = Some(facts);
            state.outcome = ChainedAttackScenarioOutcome::AwaitingStep1Damage;
            state.protocol = ChainedAttackProtocol::NeedsHole(ChainedAttackHole::DamageRoll1);
        }
    }
    Ok(state)
}

pub fn resolve_chromatic_orb_damage(
    mut state: ChromaticOrbSequenceState,
    step: ChainedAttackStep,
    facts: ChainedDamageRollFacts,
) -> Result<ChromaticOrbSequenceState, ChainedAttackSequenceError> {
    match step {
        ChainedAttackStep::Step0 => {
            require_protocol(
                state.protocol,
                ChainedAttackProtocol::NeedsHole(ChainedAttackHole::DamageRoll0),
            )?;
            state.first_target_hit_points =
                apply_damage(state.first_target_hit_points, facts.damage_total);
            state.step_index = 0;
            state.step0_damage = Some(facts);
            if chromatic_orb_can_leap(state.slot_level, state.leaps_used, facts.has_duplicate) {
                state.outcome = ChainedAttackScenarioOutcome::AwaitingFirstLeapTarget;
                state.protocol = ChainedAttackProtocol::NeedsHole(ChainedAttackHole::TargetChoice1);
            } else {
                state.outcome = ChainedAttackScenarioOutcome::Step0NoLeapComplete;
                state.protocol = ChainedAttackProtocol::Resolved;
            }
        }
        ChainedAttackStep::Step1 => {
            require_protocol(
                state.protocol,
                ChainedAttackProtocol::NeedsHole(ChainedAttackHole::DamageRoll1),
            )?;
            let target = state
                .previous_target
                .ok_or(ChainedAttackSequenceError::MissingPreviousTarget)?;
            apply_damage_to_target(&mut state, target, facts.damage_total);
            state.step_index = 1;
            state.step1_damage = Some(facts);
            if chromatic_orb_can_leap(state.slot_level, state.leaps_used, facts.has_duplicate) {
                state.outcome = ChainedAttackScenarioOutcome::AwaitingSecondLeapTarget;
                state.protocol = ChainedAttackProtocol::NeedsHole(ChainedAttackHole::TargetChoice2);
            } else if facts.has_duplicate {
                state.outcome = ChainedAttackScenarioOutcome::Slot1LeapLimitComplete;
                state.protocol = ChainedAttackProtocol::Resolved;
            } else {
                state.outcome = ChainedAttackScenarioOutcome::Step1Complete;
                state.protocol = ChainedAttackProtocol::Resolved;
            }
        }
    }
    Ok(state)
}

pub fn choose_chromatic_orb_leap_target(
    mut state: ChromaticOrbSequenceState,
    target: ChainedTarget,
    target_within_previous_target_30_feet: bool,
) -> Result<ChromaticOrbSequenceState, ChainedAttackSequenceError> {
    require_protocol(
        state.protocol,
        ChainedAttackProtocol::NeedsHole(ChainedAttackHole::TargetChoice1),
    )?;
    if state.previous_target.is_none() {
        return Err(ChainedAttackSequenceError::MissingPreviousTarget);
    }
    if !target_within_previous_target_30_feet {
        return Err(ChainedAttackSequenceError::LeapTargetNotWithinPreviousTarget30Feet);
    }
    if !chromatic_orb_can_leap(state.slot_level, state.leaps_used, true) {
        return Err(ChainedAttackSequenceError::WrongProtocol);
    }
    add_unique_target(&mut state, target)?;

    state.previous_target = Some(target);
    state.leaps_used += 1;
    state.step_index = state.leaps_used;
    state.outcome = ChainedAttackScenarioOutcome::AwaitingStep1Attack;
    state.protocol = ChainedAttackProtocol::NeedsHole(ChainedAttackHole::AttackRoll1);
    Ok(state)
}

#[must_use]
pub fn chromatic_orb_base_spell_level() -> i16 {
    1
}

#[must_use]
pub fn chromatic_orb_damage_die_size() -> i16 {
    8
}

#[must_use]
pub fn chromatic_orb_damage_die_count(slot_level: i16) -> i16 {
    // QNT: cleanroom-input/qnt/shared-algebras/proofs/rule-core/
    // spell-chained-attack-damage-projection-core.qnt.
    if slot_level < chromatic_orb_base_spell_level() {
        0
    } else {
        3 + (slot_level - chromatic_orb_base_spell_level())
    }
}

#[must_use]
pub fn chromatic_orb_can_leap(slot_level: i16, leaps_used: i16, has_duplicate: bool) -> bool {
    slot_level >= chromatic_orb_base_spell_level()
        && leaps_used >= 0
        && leaps_used < slot_level
        && has_duplicate
}

fn require_protocol(
    actual: ChainedAttackProtocol,
    expected: ChainedAttackProtocol,
) -> Result<(), ChainedAttackSequenceError> {
    if actual == expected {
        Ok(())
    } else {
        Err(ChainedAttackSequenceError::WrongProtocol)
    }
}

fn add_unique_target(
    state: &mut ChromaticOrbSequenceState,
    target: ChainedTarget,
) -> Result<(), ChainedAttackSequenceError> {
    if state.targeted.contains(&target) {
        return Err(ChainedAttackSequenceError::TargetAlreadyTargeted);
    }
    state.targeted.push(target);
    Ok(())
}

fn apply_damage_to_target(
    state: &mut ChromaticOrbSequenceState,
    target: ChainedTarget,
    damage: i16,
) {
    match target {
        ChainedTarget::First => {
            state.first_target_hit_points = apply_damage(state.first_target_hit_points, damage);
        }
        ChainedTarget::Second => {
            state.second_target_hit_points = apply_damage(state.second_target_hit_points, damage);
        }
        ChainedTarget::Third => {
            state.third_target_hit_points = apply_damage(state.third_target_hit_points, damage);
        }
    }
}

fn apply_damage(hit_points: i16, damage: i16) -> i16 {
    hit_points.saturating_sub(damage.max(0)).max(0)
}
