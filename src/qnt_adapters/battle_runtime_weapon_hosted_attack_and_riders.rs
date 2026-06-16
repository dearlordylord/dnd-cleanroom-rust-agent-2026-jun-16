use crate::rules::weapon_hosted_attack_and_riders::{
    cast_divine_favor, cast_shillelagh, clean_divine_favor_duration, clean_magic_weapon_duration,
    clean_shillelagh_after_let_go, discover_divine_favor_attack, discover_magic_weapon_target_item,
    discover_shillelagh_attack, discover_true_strike_hosted_attack, fill_divine_favor_damage_high,
    fill_divine_favor_damage_low, fill_divine_favor_hit, fill_divine_favor_target,
    fill_magic_weapon_target_item, fill_shillelagh_damage_high, fill_shillelagh_damage_low,
    fill_shillelagh_hit, fill_shillelagh_target, fill_true_strike_damage_high,
    fill_true_strike_damage_low, fill_true_strike_hit, fill_true_strike_radiant_target,
    finish_weapon_hosted_attack_and_riders, start_divine_favor_scenario,
    start_magic_weapon_scenario, start_shillelagh_scenario, WeaponHostedAttackAndRidersState,
    WeaponHostedHole, WeaponHostedPhase, WeaponHostedProtocol, WeaponHostedScenario,
};

pub const BRANCH_ACTIONS: [&str; 23] = [
    "doDiscoverTrueStrike",
    "doFillTrueStrikeRadiantTarget",
    "doFillTrueStrikeHit",
    "doFillTrueStrikeDamage",
    "doStartShillelagh",
    "doCastShillelagh",
    "doDiscoverShillelaghAttack",
    "doFillShillelaghTarget",
    "doFillShillelaghHit",
    "doFillShillelaghDamage",
    "doCleanShillelaghLetGo",
    "doStartDivineFavor",
    "doCastDivineFavor",
    "doDiscoverDivineFavorAttack",
    "doFillDivineFavorTarget",
    "doFillDivineFavorHit",
    "doFillDivineFavorDamage",
    "doCleanDivineFavorDuration",
    "doStartMagicWeapon",
    "doDiscoverMagicWeapon",
    "doFillMagicWeaponTarget",
    "doCleanMagicWeaponDuration",
    "doFinish",
];

pub fn replay_observed_action(observed_action_taken: &str) -> WeaponHostedAttackAndRidersState {
    match observed_action_taken {
        "doDiscoverTrueStrike" => discover_true_strike_hosted_attack(),
        "doFillTrueStrikeRadiantTarget" => fill_true_strike_radiant_target(),
        "doFillTrueStrikeHit" => fill_true_strike_hit(),
        "doFillTrueStrikeDamage" => fill_true_strike_damage_high(),
        "doStartShillelagh" => start_shillelagh_scenario(),
        "doCastShillelagh" => cast_shillelagh(),
        "doDiscoverShillelaghAttack" => discover_shillelagh_attack(),
        "doFillShillelaghTarget" => fill_shillelagh_target(),
        "doFillShillelaghHit" => fill_shillelagh_hit(),
        "doFillShillelaghDamage" => fill_shillelagh_damage_high(),
        "doCleanShillelaghLetGo" => clean_shillelagh_after_let_go(),
        "doStartDivineFavor" => start_divine_favor_scenario(),
        "doCastDivineFavor" => cast_divine_favor(),
        "doDiscoverDivineFavorAttack" => discover_divine_favor_attack(),
        "doFillDivineFavorTarget" => fill_divine_favor_target(),
        "doFillDivineFavorHit" => fill_divine_favor_hit(),
        "doFillDivineFavorDamage" => fill_divine_favor_damage_high(),
        "doCleanDivineFavorDuration" => clean_divine_favor_duration(),
        "doStartMagicWeapon" => start_magic_weapon_scenario(),
        "doDiscoverMagicWeapon" => discover_magic_weapon_target_item(),
        "doFillMagicWeaponTarget" => fill_magic_weapon_target_item(),
        "doCleanMagicWeaponDuration" => clean_magic_weapon_duration(),
        "doFinish" => finish_weapon_hosted_attack_and_riders(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> WeaponHostedAttackAndRidersState {
    replay_observed_action(observed_action_taken)
}

pub fn replay_true_strike_damage_sample(
    weapon_die_pip: i16,
    rider_die_pip: i16,
) -> WeaponHostedAttackAndRidersState {
    if weapon_die_pip == 1 && rider_die_pip == 1 {
        fill_true_strike_damage_low()
    } else {
        fill_true_strike_damage_high()
    }
}

pub fn replay_shillelagh_damage_sample(damage_die_pip: i16) -> WeaponHostedAttackAndRidersState {
    if damage_die_pip == 1 {
        fill_shillelagh_damage_low()
    } else {
        fill_shillelagh_damage_high()
    }
}

pub fn replay_divine_favor_damage_sample(
    weapon_die_pip: i16,
    rider_die_pip: i16,
) -> WeaponHostedAttackAndRidersState {
    if weapon_die_pip == 1 && rider_die_pip == 1 {
        fill_divine_favor_damage_low()
    } else {
        fill_divine_favor_damage_high()
    }
}

pub fn projection_payload(state: &WeaponHostedAttackAndRidersState) -> String {
    let protocol_holes = protocol_holes(&state.protocol);
    [
        format!("qScenario={}", scenario_ref(state.scenario)),
        format!("qPhase={}", phase_ref(state.phase)),
        format!("qTargetHp={}", state.target_hit_points),
        format!("qBonusActionAvailable={}", state.bonus_action_available),
        format!("qSlotExpended={}", state.slot_expended),
        format!("qActiveEffectPresent={}", state.active_effect_present),
        format!("qAttackBonus={}", state.attack_bonus),
        format!(
            "qDamageTypeChoiceApplied={}",
            state.damage_type_choice_applied
        ),
        format!("qDamageRiderPresent={}", state.damage_rider_present),
        format!("qWeaponEnhancementBonus={}", state.weapon_enhancement_bonus),
        format!("qHoles={}", joined_or_none(&protocol_holes)),
        format!("protocolResult={}", protocol_result_ref(&state.protocol)),
        format!("protocolHoles={}", joined_or_none(&protocol_holes)),
    ]
    .join("\n")
}

fn scenario_ref(scenario: WeaponHostedScenario) -> &'static str {
    match scenario {
        WeaponHostedScenario::TrueStrikeRadiantHit => "TrueStrikeRadiantHit",
        WeaponHostedScenario::ShillelaghHeldWeaponOverride => "ShillelaghHeldWeaponOverride",
        WeaponHostedScenario::DivineFavorWeaponDamageRider => "DivineFavorWeaponDamageRider",
        WeaponHostedScenario::MagicWeaponEnhancement => "MagicWeaponEnhancement",
        WeaponHostedScenario::Done => "Done",
    }
}

fn phase_ref(phase: WeaponHostedPhase) -> &'static str {
    match phase {
        WeaponHostedPhase::Fresh => "Fresh",
        WeaponHostedPhase::SpellChoiceNeeded => "SpellChoiceNeeded",
        WeaponHostedPhase::AttackRollNeeded => "AttackRollNeeded",
        WeaponHostedPhase::AttackDamageNeeded => "AttackDamageNeeded",
        WeaponHostedPhase::ActiveEffectApplied => "ActiveEffectApplied",
        WeaponHostedPhase::WeaponTargetNeeded => "WeaponTargetNeeded",
        WeaponHostedPhase::AfterWeaponDamage => "AfterWeaponDamage",
        WeaponHostedPhase::Cleaned => "Cleaned",
    }
}

fn protocol_result_ref(protocol: &WeaponHostedProtocol) -> &'static str {
    match protocol {
        WeaponHostedProtocol::Init(_) => "init",
        WeaponHostedProtocol::NeedsHoles(_) => "needsHoles",
        WeaponHostedProtocol::Resolved => "resolved",
    }
}

fn protocol_holes(protocol: &WeaponHostedProtocol) -> Vec<&'static str> {
    match protocol {
        WeaponHostedProtocol::Init(holes) | WeaponHostedProtocol::NeedsHoles(holes) => {
            holes.iter().map(hole_ref).collect()
        }
        WeaponHostedProtocol::Resolved => Vec::new(),
    }
}

fn hole_ref(hole: &WeaponHostedHole) -> &'static str {
    match hole {
        WeaponHostedHole::DamageTypeChoice => "DamageTypeChoice",
        WeaponHostedHole::TargetChoice => "TargetChoice",
        WeaponHostedHole::AttackRoll => "AttackRoll",
        WeaponHostedHole::AttackDamageRoll => "AttackDamageRoll",
        WeaponHostedHole::MagicWeaponTargetItem => "MagicWeaponTargetItem",
    }
}

fn joined_or_none(values: &[&str]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}
