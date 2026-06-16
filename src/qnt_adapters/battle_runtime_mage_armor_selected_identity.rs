use crate::rules::level_one_armor_spells::{
    discover_mage_armor_unarmored_self_target, expire_mage_armor_duration,
    reject_mage_armor_armored_target, resolve_mage_armor_base_armor_class, MageArmorProtocol,
    MageArmorScenarioOutcome, MageArmorState,
};

pub const BRANCH_ACTIONS: [&str; 4] = [
    "doDiscoverMageArmorUnarmoredSelfTarget",
    "doExpireMageArmorDuration",
    "doRejectMageArmorArmoredTarget",
    "doResolveMageArmorBaseArmorClassProjection",
];

pub fn replay_observed_action(observed_action_taken: &str) -> MageArmorState {
    match observed_action_taken {
        "doDiscoverMageArmorUnarmoredSelfTarget" => discover_mage_armor_unarmored_self_target(),
        "doExpireMageArmorDuration" => expire_mage_armor_duration(),
        "doRejectMageArmorArmoredTarget" => reject_mage_armor_armored_target(),
        "doResolveMageArmorBaseArmorClassProjection" => resolve_mage_armor_base_armor_class(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> MageArmorState {
    replay_observed_action(observed_action_taken)
}

pub fn projection_payload(state: &MageArmorState) -> String {
    [
        format!("qSelfTargetAdmitted={}", state.self_target_admitted),
        format!("qArmoredTargetRejected={}", state.armored_target_rejected),
        format!(
            "qMageArmorEffectPresent={}",
            state.mage_armor_effect_present
        ),
        format!(
            "qStoredArmorBaseStillUnarmored={}",
            state.stored_armor_base_still_unarmored
        ),
        format!(
            "qProjectedBaseIsMageArmor={}",
            state.projected_base_is_mage_armor
        ),
        format!("qArmorClass={}", state.armor_class),
        format!(
            "qMageArmorDurationTicks={}",
            state.mage_armor_duration_ticks
        ),
        format!("qLevel1SlotsExpended={}", state.level_one_slots_expended),
        format!("qActionAvailable={}", state.action_available),
        format!(
            "qScenarioOutcome={}",
            scenario_outcome_ref(state.scenario_outcome)
        ),
        format!("protocolResult={}", protocol_ref(state.protocol)),
        "protocolHoles=none".to_string(),
    ]
    .join("\n")
}

fn scenario_outcome_ref(outcome: MageArmorScenarioOutcome) -> &'static str {
    match outcome {
        MageArmorScenarioOutcome::Init => "Init",
        MageArmorScenarioOutcome::Discovered => "Discovered",
        MageArmorScenarioOutcome::ArmoredRejected => "ArmoredRejected",
        MageArmorScenarioOutcome::Resolved => "Resolved",
        MageArmorScenarioOutcome::DurationExpired => "DurationExpired",
    }
}

fn protocol_ref(protocol: MageArmorProtocol) -> &'static str {
    match protocol {
        MageArmorProtocol::Init => "init",
        MageArmorProtocol::Resolved => "resolved",
    }
}
