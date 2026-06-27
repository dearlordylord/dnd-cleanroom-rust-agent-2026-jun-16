use crate::rules::species_passive_traits::{
    project_dragonborn_damage_resistance, project_dwarven_resilience,
    project_goliath_powerful_build, project_halfling_brave, SpeciesPassiveProtocol,
    SpeciesPassiveRollMode, SpeciesPassiveScenarioResult, SpeciesPassiveTraitState,
};

pub const BRANCH_ACTIONS: [&str; 4] = [
    "doDragonbornDamageResistance",
    "doDwarvenResilience",
    "doHalflingBrave",
    "doGoliathPowerfulBuild",
];

pub fn replay_observed_action(observed_action_taken: &str) -> SpeciesPassiveTraitState {
    match observed_action_taken {
        "doDragonbornDamageResistance" => project_dragonborn_damage_resistance(),
        "doDwarvenResilience" => project_dwarven_resilience(),
        "doHalflingBrave" => project_halfling_brave(),
        "doGoliathPowerfulBuild" => project_goliath_powerful_build(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> SpeciesPassiveTraitState {
    match observed_action_taken {
        "doDragonbornDamageResistance" => SpeciesPassiveTraitState {
            dragonborn_fire_damage_after: 4,
            dragonborn_cold_damage_after: 9,
            scenario_result: SpeciesPassiveScenarioResult::DragonbornDamageResistance,
            protocol: SpeciesPassiveProtocol::Resolved,
            ..initial_expected()
        },
        "doDwarvenResilience" => SpeciesPassiveTraitState {
            dwarf_poison_damage_after: 4,
            dwarf_fire_damage_after: 9,
            dwarf_poisoned_save_advantage: true,
            dwarf_charmed_save_advantage: false,
            scenario_result: SpeciesPassiveScenarioResult::DwarvenResilience,
            protocol: SpeciesPassiveProtocol::Resolved,
            ..initial_expected()
        },
        "doHalflingBrave" => SpeciesPassiveTraitState {
            halfling_frightened_avoid_save_advantage: true,
            halfling_frightened_end_save_advantage: true,
            halfling_poisoned_save_advantage: false,
            scenario_result: SpeciesPassiveScenarioResult::HalflingBrave,
            protocol: SpeciesPassiveProtocol::Resolved,
            ..initial_expected()
        },
        "doGoliathPowerfulBuild" => SpeciesPassiveTraitState {
            goliath_escape_roll_mode: SpeciesPassiveRollMode::Advantage,
            goliath_poisoned_escape_roll_mode: SpeciesPassiveRollMode::Normal,
            scenario_result: SpeciesPassiveScenarioResult::GoliathPowerfulBuild,
            protocol: SpeciesPassiveProtocol::Resolved,
            ..initial_expected()
        },
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn projection_payload(state: &SpeciesPassiveTraitState) -> String {
    [
        format!(
            "dragonbornFireDamageAfter={}",
            state.dragonborn_fire_damage_after
        ),
        format!(
            "dragonbornColdDamageAfter={}",
            state.dragonborn_cold_damage_after
        ),
        format!("dwarfPoisonDamageAfter={}", state.dwarf_poison_damage_after),
        format!("dwarfFireDamageAfter={}", state.dwarf_fire_damage_after),
        format!(
            "dwarfPoisonedSaveAdvantage={}",
            state.dwarf_poisoned_save_advantage
        ),
        format!(
            "dwarfCharmedSaveAdvantage={}",
            state.dwarf_charmed_save_advantage
        ),
        format!(
            "halflingFrightenedAvoidSaveAdvantage={}",
            state.halfling_frightened_avoid_save_advantage
        ),
        format!(
            "halflingFrightenedEndSaveAdvantage={}",
            state.halfling_frightened_end_save_advantage
        ),
        format!(
            "halflingPoisonedSaveAdvantage={}",
            state.halfling_poisoned_save_advantage
        ),
        format!(
            "goliathEscapeRollMode={}",
            roll_mode_ref(state.goliath_escape_roll_mode)
        ),
        format!(
            "goliathPoisonedEscapeRollMode={}",
            roll_mode_ref(state.goliath_poisoned_escape_roll_mode)
        ),
        format!("scenarioResult={}", scenario_ref(state.scenario_result)),
        format!("protocolResult={}", protocol_ref(state.protocol)),
        "protocolHoles=none".to_string(),
    ]
    .join("\n")
}

fn roll_mode_ref(mode: SpeciesPassiveRollMode) -> &'static str {
    match mode {
        SpeciesPassiveRollMode::Normal => "normal",
        SpeciesPassiveRollMode::Advantage => "advantage",
    }
}

fn scenario_ref(result: SpeciesPassiveScenarioResult) -> &'static str {
    match result {
        SpeciesPassiveScenarioResult::Init => "init",
        SpeciesPassiveScenarioResult::DragonbornDamageResistance => "dragonbornDamageResistance",
        SpeciesPassiveScenarioResult::DwarvenResilience => "dwarvenResilience",
        SpeciesPassiveScenarioResult::HalflingBrave => "halflingBrave",
        SpeciesPassiveScenarioResult::GoliathPowerfulBuild => "goliathPowerfulBuild",
    }
}

fn protocol_ref(protocol: SpeciesPassiveProtocol) -> &'static str {
    match protocol {
        SpeciesPassiveProtocol::Init => "init",
        SpeciesPassiveProtocol::Resolved => "resolved",
    }
}

fn initial_expected() -> SpeciesPassiveTraitState {
    SpeciesPassiveTraitState {
        dragonborn_fire_damage_after: 9,
        dragonborn_cold_damage_after: 9,
        dwarf_poison_damage_after: 9,
        dwarf_fire_damage_after: 9,
        dwarf_poisoned_save_advantage: false,
        dwarf_charmed_save_advantage: false,
        halfling_frightened_avoid_save_advantage: false,
        halfling_frightened_end_save_advantage: false,
        halfling_poisoned_save_advantage: false,
        goliath_escape_roll_mode: SpeciesPassiveRollMode::Normal,
        goliath_poisoned_escape_roll_mode: SpeciesPassiveRollMode::Normal,
        scenario_result: SpeciesPassiveScenarioResult::Init,
        protocol: SpeciesPassiveProtocol::Init,
    }
}
