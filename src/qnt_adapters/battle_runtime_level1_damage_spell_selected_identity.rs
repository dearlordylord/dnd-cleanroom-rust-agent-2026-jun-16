use crate::rules::level_one_damage_spells::{
    project_burning_hands_mixed_cone_saving_throws, project_chromatic_orb_duplicate_damage_leap,
    project_ice_knife_hit_attack_damage_and_burst_saving_throws,
    project_ice_knife_miss_burst_saving_throws, project_poison_spray_spell_attack_damage,
    project_ray_of_sickness_spell_attack_damage_and_poisoned,
    project_sacred_flame_dexterity_saving_throw_radiant_damage,
    project_sorcerous_burst_spell_attack_damage,
    project_starry_wisp_object_spell_attack_damage_and_dim_light,
    project_vicious_mockery_wisdom_saving_throw_psychic_damage_and_next_attack_disadvantage,
    LevelOneDamageSpellProtocol, LevelOneDamageSpellScenarioOutcome, LevelOneDamageSpellState,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LevelOneDamageSpellWitness {
    pub action_available: bool,
    pub spell_slot_spent_this_turn: bool,
    pub level1_slots_remaining: u8,
    pub primary_target_hp: i16,
    pub primary_target_poisoned: bool,
    pub primary_target_next_attack_roll_disadvantage: bool,
    pub secondary_target_hp: i16,
    pub scenario_outcome: &'static str,
    pub protocol_result: &'static str,
    pub protocol_holes: Vec<&'static str>,
}

pub const BRANCH_ACTIONS: [&str; 10] = [
    "doResolveBurningHandsMixedConeSavingThrows",
    "doResolveChromaticOrbDuplicateDamageLeap",
    "doResolveIceKnifeHitAttackDamageAndBurstSavingThrows",
    "doResolveIceKnifeMissBurstSavingThrows",
    "doResolvePoisonSpraySpellAttackDamage",
    "doResolveRayOfSicknessSpellAttackDamageAndPoisoned",
    "doResolveSacredFlameDexteritySavingThrowRadiantDamage",
    "doResolveSorcerousBurstSpellAttackDamage",
    "doResolveStarryWispObjectSpellAttackDamageAndDimLight",
    "doResolveViciousMockeryWisdomSavingThrowPsychicDamageAndNextAttackDisadvantage",
];

pub fn replay_observed_action(observed_action_taken: &str) -> LevelOneDamageSpellWitness {
    match observed_action_taken {
        "doResolveBurningHandsMixedConeSavingThrows" => {
            witness_from_state(project_burning_hands_mixed_cone_saving_throws())
        }
        "doResolveChromaticOrbDuplicateDamageLeap" => {
            witness_from_state(project_chromatic_orb_duplicate_damage_leap())
        }
        "doResolveIceKnifeHitAttackDamageAndBurstSavingThrows" => {
            witness_from_state(project_ice_knife_hit_attack_damage_and_burst_saving_throws())
        }
        "doResolveIceKnifeMissBurstSavingThrows" => {
            witness_from_state(project_ice_knife_miss_burst_saving_throws())
        }
        "doResolvePoisonSpraySpellAttackDamage" => {
            witness_from_state(project_poison_spray_spell_attack_damage())
        }
        "doResolveRayOfSicknessSpellAttackDamageAndPoisoned" => {
            witness_from_state(project_ray_of_sickness_spell_attack_damage_and_poisoned())
        }
        "doResolveSacredFlameDexteritySavingThrowRadiantDamage" => {
            witness_from_state(project_sacred_flame_dexterity_saving_throw_radiant_damage())
        }
        "doResolveSorcerousBurstSpellAttackDamage" => {
            witness_from_state(project_sorcerous_burst_spell_attack_damage())
        }
        "doResolveStarryWispObjectSpellAttackDamageAndDimLight" => {
            witness_from_state(project_starry_wisp_object_spell_attack_damage_and_dim_light())
        }
        "doResolveViciousMockeryWisdomSavingThrowPsychicDamageAndNextAttackDisadvantage" => {
            witness_from_state(
                project_vicious_mockery_wisdom_saving_throw_psychic_damage_and_next_attack_disadvantage(),
            )
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> LevelOneDamageSpellWitness {
    replay_observed_action(observed_action_taken)
}

pub fn projection_payload(witness: &LevelOneDamageSpellWitness) -> String {
    [
        format!("qActionAvailable={}", witness.action_available),
        format!(
            "qSpellSlotSpentThisTurn={}",
            witness.spell_slot_spent_this_turn
        ),
        format!("qLevel1SlotsRemaining={}", witness.level1_slots_remaining),
        format!("qPrimaryTargetHp={}", witness.primary_target_hp),
        format!("qPrimaryTargetPoisoned={}", witness.primary_target_poisoned),
        format!(
            "qPrimaryTargetNextAttackRollDisadvantage={}",
            witness.primary_target_next_attack_roll_disadvantage
        ),
        format!("qSecondaryTargetHp={}", witness.secondary_target_hp),
        format!("qScenarioOutcome={}", witness.scenario_outcome),
        format!("protocolResult={}", witness.protocol_result),
        format!("protocolHoles={}", joined_or_none(&witness.protocol_holes)),
    ]
    .join("\n")
}

fn witness_from_state(state: LevelOneDamageSpellState) -> LevelOneDamageSpellWitness {
    LevelOneDamageSpellWitness {
        action_available: state.action_available,
        spell_slot_spent_this_turn: state.spell_slot_spent_this_turn,
        level1_slots_remaining: state.level_one_slots_remaining,
        primary_target_hp: state.primary_target_hit_points,
        primary_target_poisoned: state.primary_target_poisoned,
        primary_target_next_attack_roll_disadvantage: state
            .primary_target_next_attack_roll_disadvantage,
        secondary_target_hp: state.secondary_target_hit_points,
        scenario_outcome: scenario_outcome_ref(state.scenario_outcome),
        protocol_result: protocol_ref(state.protocol),
        protocol_holes: Vec::new(),
    }
}

fn scenario_outcome_ref(outcome: LevelOneDamageSpellScenarioOutcome) -> &'static str {
    match outcome {
        LevelOneDamageSpellScenarioOutcome::Init => "Init",
        LevelOneDamageSpellScenarioOutcome::BurningHandsMixedConeSavingThrows => {
            "BurningHandsMixedConeSavingThrows"
        }
        LevelOneDamageSpellScenarioOutcome::ChromaticOrbDuplicateDamageLeap => {
            "ChromaticOrbDuplicateDamageLeap"
        }
        LevelOneDamageSpellScenarioOutcome::IceKnifeHitAttackDamageAndBurstSavingThrows => {
            "IceKnifeHitAttackDamageAndBurstSavingThrows"
        }
        LevelOneDamageSpellScenarioOutcome::IceKnifeMissBurstSavingThrows => {
            "IceKnifeMissBurstSavingThrows"
        }
        LevelOneDamageSpellScenarioOutcome::PoisonSpraySpellAttackDamage => {
            "PoisonSpraySpellAttackDamage"
        }
        LevelOneDamageSpellScenarioOutcome::RayOfSicknessSpellAttackDamageAndPoisoned => {
            "RayOfSicknessSpellAttackDamageAndPoisoned"
        }
        LevelOneDamageSpellScenarioOutcome::SacredFlameDexteritySavingThrowRadiantDamage => {
            "SacredFlameDexteritySavingThrowRadiantDamage"
        }
        LevelOneDamageSpellScenarioOutcome::SorcerousBurstSpellAttackDamage => {
            "SorcerousBurstSpellAttackDamage"
        }
        LevelOneDamageSpellScenarioOutcome::StarryWispObjectSpellAttackDamageAndDimLight => {
            "StarryWispObjectSpellAttackDamageAndDimLight"
        }
        LevelOneDamageSpellScenarioOutcome::ViciousMockeryWisdomSavingThrowPsychicDamageAndNextAttackDisadvantage => {
            "ViciousMockeryWisdomSavingThrowPsychicDamageAndNextAttackDisadvantage"
        }
    }
}

fn protocol_ref(protocol: LevelOneDamageSpellProtocol) -> &'static str {
    match protocol {
        LevelOneDamageSpellProtocol::Init => "init",
        LevelOneDamageSpellProtocol::Resolved => "resolved",
    }
}

fn joined_or_none(values: &[&'static str]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}
