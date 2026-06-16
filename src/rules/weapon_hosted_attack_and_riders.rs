#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeaponHostedScenario {
    TrueStrikeRadiantHit,
    ShillelaghHeldWeaponOverride,
    DivineFavorWeaponDamageRider,
    MagicWeaponEnhancement,
    Done,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeaponHostedPhase {
    Fresh,
    SpellChoiceNeeded,
    AttackRollNeeded,
    AttackDamageNeeded,
    ActiveEffectApplied,
    WeaponTargetNeeded,
    AfterWeaponDamage,
    Cleaned,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeaponHostedHole {
    DamageTypeChoice,
    TargetChoice,
    AttackRoll,
    AttackDamageRoll,
    MagicWeaponTargetItem,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeaponHostedProtocol {
    Init(Vec<WeaponHostedHole>),
    NeedsHoles(Vec<WeaponHostedHole>),
    Resolved,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeaponHostedAttackAndRidersState {
    pub scenario: WeaponHostedScenario,
    pub phase: WeaponHostedPhase,
    pub target_hit_points: i16,
    pub bonus_action_available: bool,
    pub slot_expended: bool,
    pub active_effect_present: bool,
    pub attack_bonus: i16,
    pub damage_type_choice_applied: bool,
    pub damage_rider_present: bool,
    pub weapon_enhancement_bonus: i16,
    pub protocol: WeaponHostedProtocol,
}

pub const WEAPON_HOSTED_TARGET_INITIAL_HIT_POINTS: i16 = 20;
pub const WEAPON_HOSTED_SPELLCASTING_ABILITY_MODIFIER: i16 = 3;
pub const WEAPON_HOSTED_PROFICIENCY_BONUS: i16 = 2;
pub const MAGIC_WEAPON_LEVEL_TWO_BONUS: i16 = 1;

#[must_use]
pub fn weapon_hosted_attack_and_riders_initial_state() -> WeaponHostedAttackAndRidersState {
    projection(WeaponHostedProjectionFacts {
        scenario: WeaponHostedScenario::TrueStrikeRadiantHit,
        phase: WeaponHostedPhase::Fresh,
        target_hit_points: WEAPON_HOSTED_TARGET_INITIAL_HIT_POINTS,
        bonus_action_available: true,
        slot_expended: false,
        active_effect_present: false,
        attack_bonus: 0,
        damage_type_choice_applied: false,
        damage_rider_present: false,
        weapon_enhancement_bonus: 0,
        holes: Vec::new(),
        protocol_result: WeaponHostedProtocolResult::Init,
    })
}

#[must_use]
pub fn discover_true_strike_hosted_attack() -> WeaponHostedAttackAndRidersState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-S-Z.md
    // "True Strike"; QNT:
    // battle-runtime-weapon-hosted-attack-and-riders.mbt.qnt.
    let current = weapon_hosted_attack_and_riders_initial_state();
    projection(WeaponHostedProjectionFacts {
        scenario: WeaponHostedScenario::TrueStrikeRadiantHit,
        phase: WeaponHostedPhase::SpellChoiceNeeded,
        holes: vec![
            WeaponHostedHole::DamageTypeChoice,
            WeaponHostedHole::TargetChoice,
        ],
        protocol_result: WeaponHostedProtocolResult::NeedsHoles,
        ..facts_from(&current)
    })
}

#[must_use]
pub fn fill_true_strike_radiant_target() -> WeaponHostedAttackAndRidersState {
    let current = discover_true_strike_hosted_attack();
    projection(WeaponHostedProjectionFacts {
        phase: WeaponHostedPhase::AttackRollNeeded,
        attack_bonus: hosted_spell_attack_bonus(),
        damage_type_choice_applied: true,
        holes: vec![WeaponHostedHole::AttackRoll],
        protocol_result: WeaponHostedProtocolResult::NeedsHoles,
        ..facts_from(&current)
    })
}

#[must_use]
pub fn fill_true_strike_hit() -> WeaponHostedAttackAndRidersState {
    let current = fill_true_strike_radiant_target();
    projection(WeaponHostedProjectionFacts {
        phase: WeaponHostedPhase::AttackDamageNeeded,
        damage_rider_present: true,
        holes: vec![WeaponHostedHole::AttackDamageRoll],
        protocol_result: WeaponHostedProtocolResult::NeedsHoles,
        ..facts_from(&current)
    })
}

#[must_use]
pub fn fill_true_strike_damage_low() -> WeaponHostedAttackAndRidersState {
    fill_true_strike_damage(1, 1)
}

#[must_use]
pub fn fill_true_strike_damage_high() -> WeaponHostedAttackAndRidersState {
    fill_true_strike_damage(4, 5)
}

#[must_use]
pub fn start_shillelagh_scenario() -> WeaponHostedAttackAndRidersState {
    start_scenario(WeaponHostedScenario::ShillelaghHeldWeaponOverride)
}

#[must_use]
pub fn cast_shillelagh() -> WeaponHostedAttackAndRidersState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-S-Z.md
    // "Shillelagh"; QNT:
    // battle-runtime-weapon-hosted-attack-and-riders.mbt.qnt.
    let current = start_shillelagh_scenario();
    projection(WeaponHostedProjectionFacts {
        phase: WeaponHostedPhase::ActiveEffectApplied,
        bonus_action_available: false,
        active_effect_present: true,
        attack_bonus: hosted_spell_attack_bonus(),
        damage_type_choice_applied: true,
        ..facts_from(&current)
    })
}

#[must_use]
pub fn discover_shillelagh_attack() -> WeaponHostedAttackAndRidersState {
    let current = cast_shillelagh();
    projection(WeaponHostedProjectionFacts {
        phase: WeaponHostedPhase::WeaponTargetNeeded,
        holes: vec![WeaponHostedHole::TargetChoice],
        protocol_result: WeaponHostedProtocolResult::NeedsHoles,
        ..facts_from(&current)
    })
}

#[must_use]
pub fn fill_shillelagh_target() -> WeaponHostedAttackAndRidersState {
    let current = discover_shillelagh_attack();
    projection(WeaponHostedProjectionFacts {
        phase: WeaponHostedPhase::AttackRollNeeded,
        holes: vec![WeaponHostedHole::AttackRoll],
        protocol_result: WeaponHostedProtocolResult::NeedsHoles,
        ..facts_from(&current)
    })
}

#[must_use]
pub fn fill_shillelagh_hit() -> WeaponHostedAttackAndRidersState {
    let current = fill_shillelagh_target();
    projection(WeaponHostedProjectionFacts {
        phase: WeaponHostedPhase::AttackDamageNeeded,
        holes: vec![WeaponHostedHole::AttackDamageRoll],
        protocol_result: WeaponHostedProtocolResult::NeedsHoles,
        ..facts_from(&current)
    })
}

#[must_use]
pub fn fill_shillelagh_damage_low() -> WeaponHostedAttackAndRidersState {
    fill_shillelagh_damage(1)
}

#[must_use]
pub fn fill_shillelagh_damage_high() -> WeaponHostedAttackAndRidersState {
    fill_shillelagh_damage(4)
}

#[must_use]
pub fn clean_shillelagh_after_let_go() -> WeaponHostedAttackAndRidersState {
    clean_active_effect(&fill_shillelagh_damage_high())
}

#[must_use]
pub fn start_divine_favor_scenario() -> WeaponHostedAttackAndRidersState {
    start_scenario(WeaponHostedScenario::DivineFavorWeaponDamageRider)
}

#[must_use]
pub fn cast_divine_favor() -> WeaponHostedAttackAndRidersState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-A-D.md
    // "Divine Favor"; QNT:
    // battle-runtime-weapon-hosted-attack-and-riders.mbt.qnt.
    let current = start_divine_favor_scenario();
    projection(WeaponHostedProjectionFacts {
        phase: WeaponHostedPhase::ActiveEffectApplied,
        bonus_action_available: false,
        slot_expended: true,
        active_effect_present: true,
        ..facts_from(&current)
    })
}

#[must_use]
pub fn discover_divine_favor_attack() -> WeaponHostedAttackAndRidersState {
    let current = cast_divine_favor();
    projection(WeaponHostedProjectionFacts {
        phase: WeaponHostedPhase::WeaponTargetNeeded,
        holes: vec![WeaponHostedHole::TargetChoice],
        protocol_result: WeaponHostedProtocolResult::NeedsHoles,
        ..facts_from(&current)
    })
}

#[must_use]
pub fn fill_divine_favor_target() -> WeaponHostedAttackAndRidersState {
    let current = discover_divine_favor_attack();
    projection(WeaponHostedProjectionFacts {
        phase: WeaponHostedPhase::AttackRollNeeded,
        holes: vec![WeaponHostedHole::AttackRoll],
        protocol_result: WeaponHostedProtocolResult::NeedsHoles,
        ..facts_from(&current)
    })
}

#[must_use]
pub fn fill_divine_favor_hit() -> WeaponHostedAttackAndRidersState {
    let current = fill_divine_favor_target();
    projection(WeaponHostedProjectionFacts {
        phase: WeaponHostedPhase::AttackDamageNeeded,
        damage_rider_present: true,
        holes: vec![WeaponHostedHole::AttackDamageRoll],
        protocol_result: WeaponHostedProtocolResult::NeedsHoles,
        ..facts_from(&current)
    })
}

#[must_use]
pub fn fill_divine_favor_damage_low() -> WeaponHostedAttackAndRidersState {
    fill_divine_favor_damage(1, 1)
}

#[must_use]
pub fn fill_divine_favor_damage_high() -> WeaponHostedAttackAndRidersState {
    fill_divine_favor_damage(4, 3)
}

#[must_use]
pub fn clean_divine_favor_duration() -> WeaponHostedAttackAndRidersState {
    clean_active_effect(&fill_divine_favor_damage_high())
}

#[must_use]
pub fn start_magic_weapon_scenario() -> WeaponHostedAttackAndRidersState {
    start_scenario(WeaponHostedScenario::MagicWeaponEnhancement)
}

#[must_use]
pub fn discover_magic_weapon_target_item() -> WeaponHostedAttackAndRidersState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-M-P.md
    // "Magic Weapon"; QNT:
    // battle-runtime-weapon-hosted-attack-and-riders.mbt.qnt.
    let current = start_magic_weapon_scenario();
    projection(WeaponHostedProjectionFacts {
        phase: WeaponHostedPhase::WeaponTargetNeeded,
        holes: vec![WeaponHostedHole::MagicWeaponTargetItem],
        protocol_result: WeaponHostedProtocolResult::NeedsHoles,
        ..facts_from(&current)
    })
}

#[must_use]
pub fn fill_magic_weapon_target_item() -> WeaponHostedAttackAndRidersState {
    let current = discover_magic_weapon_target_item();
    projection(WeaponHostedProjectionFacts {
        phase: WeaponHostedPhase::ActiveEffectApplied,
        bonus_action_available: false,
        slot_expended: true,
        active_effect_present: true,
        weapon_enhancement_bonus: MAGIC_WEAPON_LEVEL_TWO_BONUS,
        holes: Vec::new(),
        protocol_result: WeaponHostedProtocolResult::Resolved,
        ..facts_from(&current)
    })
}

#[must_use]
pub fn clean_magic_weapon_duration() -> WeaponHostedAttackAndRidersState {
    clean_active_effect(&fill_magic_weapon_target_item())
}

#[must_use]
pub fn finish_weapon_hosted_attack_and_riders() -> WeaponHostedAttackAndRidersState {
    let current = clean_magic_weapon_duration();
    projection(WeaponHostedProjectionFacts {
        scenario: WeaponHostedScenario::Done,
        active_effect_present: false,
        attack_bonus: 0,
        damage_type_choice_applied: false,
        damage_rider_present: false,
        weapon_enhancement_bonus: 0,
        ..facts_from(&current)
    })
}

#[must_use]
pub fn hosted_spell_attack_bonus() -> i16 {
    WEAPON_HOSTED_SPELLCASTING_ABILITY_MODIFIER + WEAPON_HOSTED_PROFICIENCY_BONUS
}

#[must_use]
pub fn damage_after(hit_points: i16, damage: i16) -> i16 {
    (hit_points - damage).max(0)
}

fn fill_true_strike_damage(
    weapon_roll_sample: i16,
    rider_roll_sample: i16,
) -> WeaponHostedAttackAndRidersState {
    let current = fill_true_strike_hit();
    let damage =
        weapon_roll_sample + WEAPON_HOSTED_SPELLCASTING_ABILITY_MODIFIER + rider_roll_sample;
    projection(WeaponHostedProjectionFacts {
        phase: WeaponHostedPhase::Cleaned,
        target_hit_points: damage_after(current.target_hit_points, damage),
        holes: Vec::new(),
        protocol_result: WeaponHostedProtocolResult::Resolved,
        ..facts_from(&current)
    })
}

fn fill_shillelagh_damage(damage_roll_sample: i16) -> WeaponHostedAttackAndRidersState {
    let current = fill_shillelagh_hit();
    let damage = (damage_roll_sample * 2) + WEAPON_HOSTED_SPELLCASTING_ABILITY_MODIFIER;
    projection(WeaponHostedProjectionFacts {
        phase: WeaponHostedPhase::AfterWeaponDamage,
        target_hit_points: damage_after(current.target_hit_points, damage),
        holes: Vec::new(),
        protocol_result: WeaponHostedProtocolResult::Resolved,
        ..facts_from(&current)
    })
}

fn fill_divine_favor_damage(
    weapon_roll_sample: i16,
    rider_roll_sample: i16,
) -> WeaponHostedAttackAndRidersState {
    let current = fill_divine_favor_hit();
    let damage = weapon_roll_sample + rider_roll_sample;
    projection(WeaponHostedProjectionFacts {
        phase: WeaponHostedPhase::AfterWeaponDamage,
        target_hit_points: damage_after(current.target_hit_points, damage),
        damage_rider_present: false,
        holes: Vec::new(),
        protocol_result: WeaponHostedProtocolResult::Resolved,
        ..facts_from(&current)
    })
}

fn start_scenario(scenario: WeaponHostedScenario) -> WeaponHostedAttackAndRidersState {
    projection(WeaponHostedProjectionFacts {
        scenario,
        phase: WeaponHostedPhase::Fresh,
        target_hit_points: WEAPON_HOSTED_TARGET_INITIAL_HIT_POINTS,
        bonus_action_available: true,
        slot_expended: false,
        active_effect_present: false,
        attack_bonus: 0,
        damage_type_choice_applied: false,
        damage_rider_present: false,
        weapon_enhancement_bonus: 0,
        holes: Vec::new(),
        protocol_result: WeaponHostedProtocolResult::Resolved,
    })
}

fn clean_active_effect(
    current: &WeaponHostedAttackAndRidersState,
) -> WeaponHostedAttackAndRidersState {
    projection(WeaponHostedProjectionFacts {
        phase: WeaponHostedPhase::Cleaned,
        bonus_action_available: true,
        slot_expended: false,
        active_effect_present: false,
        attack_bonus: 0,
        damage_type_choice_applied: false,
        damage_rider_present: false,
        weapon_enhancement_bonus: 0,
        ..facts_from(current)
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct WeaponHostedProjectionFacts {
    scenario: WeaponHostedScenario,
    phase: WeaponHostedPhase,
    target_hit_points: i16,
    bonus_action_available: bool,
    slot_expended: bool,
    active_effect_present: bool,
    attack_bonus: i16,
    damage_type_choice_applied: bool,
    damage_rider_present: bool,
    weapon_enhancement_bonus: i16,
    holes: Vec<WeaponHostedHole>,
    protocol_result: WeaponHostedProtocolResult,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WeaponHostedProtocolResult {
    Init,
    NeedsHoles,
    Resolved,
}

fn facts_from(state: &WeaponHostedAttackAndRidersState) -> WeaponHostedProjectionFacts {
    WeaponHostedProjectionFacts {
        scenario: state.scenario,
        phase: state.phase,
        target_hit_points: state.target_hit_points,
        bonus_action_available: state.bonus_action_available,
        slot_expended: state.slot_expended,
        active_effect_present: state.active_effect_present,
        attack_bonus: state.attack_bonus,
        damage_type_choice_applied: state.damage_type_choice_applied,
        damage_rider_present: state.damage_rider_present,
        weapon_enhancement_bonus: state.weapon_enhancement_bonus,
        holes: protocol_holes(&state.protocol),
        protocol_result: protocol_result(&state.protocol),
    }
}

fn projection(facts: WeaponHostedProjectionFacts) -> WeaponHostedAttackAndRidersState {
    WeaponHostedAttackAndRidersState {
        scenario: facts.scenario,
        phase: facts.phase,
        target_hit_points: facts.target_hit_points,
        bonus_action_available: facts.bonus_action_available,
        slot_expended: facts.slot_expended,
        active_effect_present: facts.active_effect_present,
        attack_bonus: facts.attack_bonus,
        damage_type_choice_applied: facts.damage_type_choice_applied,
        damage_rider_present: facts.damage_rider_present,
        weapon_enhancement_bonus: facts.weapon_enhancement_bonus,
        protocol: match facts.protocol_result {
            WeaponHostedProtocolResult::Init => WeaponHostedProtocol::Init(facts.holes),
            WeaponHostedProtocolResult::NeedsHoles => WeaponHostedProtocol::NeedsHoles(facts.holes),
            WeaponHostedProtocolResult::Resolved => WeaponHostedProtocol::Resolved,
        },
    }
}

fn protocol_holes(protocol: &WeaponHostedProtocol) -> Vec<WeaponHostedHole> {
    match protocol {
        WeaponHostedProtocol::Init(holes) | WeaponHostedProtocol::NeedsHoles(holes) => {
            holes.clone()
        }
        WeaponHostedProtocol::Resolved => Vec::new(),
    }
}

fn protocol_result(protocol: &WeaponHostedProtocol) -> WeaponHostedProtocolResult {
    match protocol {
        WeaponHostedProtocol::Init(_) => WeaponHostedProtocolResult::Init,
        WeaponHostedProtocol::NeedsHoles(_) => WeaponHostedProtocolResult::NeedsHoles,
        WeaponHostedProtocol::Resolved => WeaponHostedProtocolResult::Resolved,
    }
}
