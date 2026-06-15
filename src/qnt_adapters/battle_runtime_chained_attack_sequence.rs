use crate::rules::chained_spell_attacks::{
    choose_chromatic_orb_damage_type, choose_chromatic_orb_initial_target,
    choose_chromatic_orb_leap_target, chromatic_orb_initial_state,
    resolve_chromatic_orb_attack_hit, resolve_chromatic_orb_damage, start_chromatic_orb_cast,
    ChainedAttackHole, ChainedAttackProtocol, ChainedAttackRollFacts, ChainedAttackScenarioOutcome,
    ChainedAttackStep, ChainedDamageRollFacts, ChainedDamageTypeChoice, ChainedTarget,
    ChromaticOrbSequenceState,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChainedAttackSequenceWitness {
    pub damage_type: &'static str,
    pub slot_level: i16,
    pub targeted: Vec<&'static str>,
    pub previous_target: &'static str,
    pub leaps_used: i16,
    pub step_index: i16,
    pub first_target_hit_points: i16,
    pub second_target_hit_points: i16,
    pub third_target_hit_points: i16,
    pub step0_attack_total: i16,
    pub step0_natural_d20: i16,
    pub step0_damage_total: i16,
    pub step0_damage_has_duplicate: bool,
    pub step1_attack_total: i16,
    pub step1_natural_d20: i16,
    pub step1_damage_total: i16,
    pub step1_damage_has_duplicate: bool,
    pub scenario_outcome: &'static str,
    pub protocol_result: &'static str,
    pub protocol_holes: Vec<&'static str>,
}

pub const BRANCH_ACTIONS: [&str; 10] = [
    "doStartCast",
    "doChooseDamageType",
    "doChooseInitialTarget",
    "doResolveStep0AttackHit",
    "doResolveStep0DamageNoDuplicate",
    "doResolveStep0DamageDuplicate",
    "doChooseFirstLeapTarget",
    "doResolveStep1AttackHit",
    "doResolveStep1DuplicateDamageSlot1Limit",
    "doResolveStep1DuplicateDamageSlot2AllowsLeap",
];

pub fn replay_observed_action(observed_action_taken: &str) -> ChainedAttackSequenceWitness {
    match observed_action_taken {
        "doStartCast" => witness_from_state(started_cast(1)),
        "doChooseDamageType" => witness_from_state(chose_damage_type(1)),
        "doChooseInitialTarget" => witness_from_state(chose_initial_target(1)),
        "doResolveStep0AttackHit" => witness_from_state(resolved_step0_attack(1)),
        "doResolveStep0DamageNoDuplicate" => witness_from_state(resolved_step0_damage(1, 6, false)),
        "doResolveStep0DamageDuplicate" => witness_from_state(resolved_step0_damage(1, 9, true)),
        "doChooseFirstLeapTarget" => witness_from_state(chose_first_leap_target(1, 9)),
        "doResolveStep1AttackHit" => witness_from_state(resolved_step1_attack(1, 9)),
        "doResolveStep1DuplicateDamageSlot1Limit" => {
            witness_from_state(resolved_step1_damage(1, 9, 3))
        }
        "doResolveStep1DuplicateDamageSlot2AllowsLeap" => {
            witness_from_state(resolved_step1_damage(2, 10, 4))
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> ChainedAttackSequenceWitness {
    replay_observed_action(observed_action_taken)
}

pub fn projection_payload(witness: &ChainedAttackSequenceWitness) -> String {
    [
        format!("damageType={}", witness.damage_type),
        format!("slotLevel={}", witness.slot_level),
        format!("targeted={}", joined_or_none(&witness.targeted)),
        format!("previousTarget={}", witness.previous_target),
        format!("leapsUsed={}", witness.leaps_used),
        format!("stepIndex={}", witness.step_index),
        format!("firstTargetHp={}", witness.first_target_hit_points),
        format!("secondTargetHp={}", witness.second_target_hit_points),
        format!("thirdTargetHp={}", witness.third_target_hit_points),
        format!("step0AttackTotal={}", witness.step0_attack_total),
        format!("step0NaturalD20={}", witness.step0_natural_d20),
        format!("step0DamageTotal={}", witness.step0_damage_total),
        format!(
            "step0DamageHasDuplicate={}",
            witness.step0_damage_has_duplicate
        ),
        format!("step1AttackTotal={}", witness.step1_attack_total),
        format!("step1NaturalD20={}", witness.step1_natural_d20),
        format!("step1DamageTotal={}", witness.step1_damage_total),
        format!(
            "step1DamageHasDuplicate={}",
            witness.step1_damage_has_duplicate
        ),
        format!("scenarioOutcome={}", witness.scenario_outcome),
        format!("protocolResult={}", witness.protocol_result),
        format!("protocolHoles={}", joined_or_none(&witness.protocol_holes)),
    ]
    .join("\n")
}

fn started_cast(slot_level: i16) -> ChromaticOrbSequenceState {
    start_chromatic_orb_cast(chromatic_orb_initial_state(), slot_level)
        .expect("QNT sampled slot level is legal for Chromatic Orb")
}

fn chose_damage_type(slot_level: i16) -> ChromaticOrbSequenceState {
    choose_chromatic_orb_damage_type(started_cast(slot_level), ChainedDamageTypeChoice::Fire)
        .expect("QNT damage type choice is a Chromatic Orb legal damage type")
}

fn chose_initial_target(slot_level: i16) -> ChromaticOrbSequenceState {
    choose_chromatic_orb_initial_target(chose_damage_type(slot_level), ChainedTarget::First)
        .expect("QNT initial target is accepted")
}

fn resolved_step0_attack(slot_level: i16) -> ChromaticOrbSequenceState {
    resolve_chromatic_orb_attack_hit(
        chose_initial_target(slot_level),
        ChainedAttackStep::Step0,
        attack_hit_facts(),
    )
    .expect("QNT step 0 attack roll resolves as a hit")
}

fn resolved_step0_damage(
    slot_level: i16,
    damage_total: i16,
    has_duplicate: bool,
) -> ChromaticOrbSequenceState {
    resolve_chromatic_orb_damage(
        resolved_step0_attack(slot_level),
        ChainedAttackStep::Step0,
        ChainedDamageRollFacts {
            damage_total,
            has_duplicate,
        },
    )
    .expect("QNT step 0 damage roll is accepted")
}

fn chose_first_leap_target(slot_level: i16, step0_damage_total: i16) -> ChromaticOrbSequenceState {
    choose_chromatic_orb_leap_target(
        resolved_step0_damage(slot_level, step0_damage_total, true),
        ChainedTarget::Second,
        true,
    )
    .expect("QNT first leap target is within 30 feet and not already targeted")
}

fn resolved_step1_attack(slot_level: i16, step0_damage_total: i16) -> ChromaticOrbSequenceState {
    resolve_chromatic_orb_attack_hit(
        chose_first_leap_target(slot_level, step0_damage_total),
        ChainedAttackStep::Step1,
        attack_hit_facts(),
    )
    .expect("QNT step 1 attack roll resolves as a hit")
}

fn resolved_step1_damage(
    slot_level: i16,
    step0_damage_total: i16,
    step1_damage_total: i16,
) -> ChromaticOrbSequenceState {
    resolve_chromatic_orb_damage(
        resolved_step1_attack(slot_level, step0_damage_total),
        ChainedAttackStep::Step1,
        ChainedDamageRollFacts {
            damage_total: step1_damage_total,
            has_duplicate: true,
        },
    )
    .expect("QNT step 1 duplicate damage roll is accepted")
}

fn attack_hit_facts() -> ChainedAttackRollFacts {
    ChainedAttackRollFacts {
        attack_total: 18,
        natural_d20: 12,
    }
}

fn witness_from_state(state: ChromaticOrbSequenceState) -> ChainedAttackSequenceWitness {
    ChainedAttackSequenceWitness {
        damage_type: damage_type_ref(state.damage_type),
        slot_level: state.slot_level,
        targeted: state.targeted.iter().copied().map(target_ref).collect(),
        previous_target: state.previous_target.map_or("none", target_ref),
        leaps_used: state.leaps_used,
        step_index: state.step_index,
        first_target_hit_points: state.first_target_hit_points,
        second_target_hit_points: state.second_target_hit_points,
        third_target_hit_points: state.third_target_hit_points,
        step0_attack_total: state.step0_attack.map_or(0, |facts| facts.attack_total),
        step0_natural_d20: state.step0_attack.map_or(0, |facts| facts.natural_d20),
        step0_damage_total: state.step0_damage.map_or(0, |facts| facts.damage_total),
        step0_damage_has_duplicate: state.step0_damage.is_some_and(|facts| facts.has_duplicate),
        step1_attack_total: state.step1_attack.map_or(0, |facts| facts.attack_total),
        step1_natural_d20: state.step1_attack.map_or(0, |facts| facts.natural_d20),
        step1_damage_total: state.step1_damage.map_or(0, |facts| facts.damage_total),
        step1_damage_has_duplicate: state.step1_damage.is_some_and(|facts| facts.has_duplicate),
        scenario_outcome: scenario_outcome_ref(state.outcome),
        protocol_result: protocol_result_ref(state.protocol),
        protocol_holes: protocol_holes(state.protocol),
    }
}

fn damage_type_ref(damage_type: Option<ChainedDamageTypeChoice>) -> &'static str {
    match damage_type {
        Some(ChainedDamageTypeChoice::Acid) => "acid",
        Some(ChainedDamageTypeChoice::Cold) => "cold",
        Some(ChainedDamageTypeChoice::Fire) => "fire",
        Some(ChainedDamageTypeChoice::Lightning) => "lightning",
        Some(ChainedDamageTypeChoice::Poison) => "poison",
        Some(ChainedDamageTypeChoice::Thunder) => "thunder",
        None => "none",
    }
}

fn target_ref(target: ChainedTarget) -> &'static str {
    match target {
        ChainedTarget::First => "first",
        ChainedTarget::Second => "second",
        ChainedTarget::Third => "third",
    }
}

fn scenario_outcome_ref(outcome: ChainedAttackScenarioOutcome) -> &'static str {
    match outcome {
        ChainedAttackScenarioOutcome::Init => "Init",
        ChainedAttackScenarioOutcome::AwaitingDamageType => "AwaitingDamageType",
        ChainedAttackScenarioOutcome::AwaitingInitialTarget => "AwaitingInitialTarget",
        ChainedAttackScenarioOutcome::AwaitingStep0Attack => "AwaitingStep0Attack",
        ChainedAttackScenarioOutcome::AwaitingStep0Damage => "AwaitingStep0Damage",
        ChainedAttackScenarioOutcome::Step0NoLeapComplete => "Step0NoLeapComplete",
        ChainedAttackScenarioOutcome::AwaitingFirstLeapTarget => "AwaitingFirstLeapTarget",
        ChainedAttackScenarioOutcome::AwaitingStep1Attack => "AwaitingStep1Attack",
        ChainedAttackScenarioOutcome::AwaitingStep1Damage => "AwaitingStep1Damage",
        ChainedAttackScenarioOutcome::Slot1LeapLimitComplete => "Slot1LeapLimitComplete",
        ChainedAttackScenarioOutcome::AwaitingSecondLeapTarget => "AwaitingSecondLeapTarget",
        ChainedAttackScenarioOutcome::Step1Complete => "Step1Complete",
    }
}

fn protocol_result_ref(protocol: ChainedAttackProtocol) -> &'static str {
    match protocol {
        ChainedAttackProtocol::Init(_) => "init",
        ChainedAttackProtocol::NeedsHole(_) => "needsHoles",
        ChainedAttackProtocol::Resolved => "resolved",
    }
}

fn protocol_holes(protocol: ChainedAttackProtocol) -> Vec<&'static str> {
    match protocol {
        ChainedAttackProtocol::Init(hole) | ChainedAttackProtocol::NeedsHole(hole) => {
            vec![hole_ref(hole)]
        }
        ChainedAttackProtocol::Resolved => vec![],
    }
}

fn hole_ref(hole: ChainedAttackHole) -> &'static str {
    match hole {
        ChainedAttackHole::DamageTypeChoice => "DamageTypeChoice",
        ChainedAttackHole::TargetChoice0 => "TargetChoice0",
        ChainedAttackHole::AttackRoll0 => "AttackRoll0",
        ChainedAttackHole::DamageRoll0 => "DamageRoll0",
        ChainedAttackHole::TargetChoice1 => "TargetChoice1",
        ChainedAttackHole::AttackRoll1 => "AttackRoll1",
        ChainedAttackHole::DamageRoll1 => "DamageRoll1",
        ChainedAttackHole::TargetChoice2 => "TargetChoice2",
    }
}

fn joined_or_none(values: &[&'static str]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}
