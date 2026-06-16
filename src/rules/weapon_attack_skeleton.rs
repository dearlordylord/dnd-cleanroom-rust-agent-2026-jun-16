#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeaponAttackSkeletonHole {
    TargetChoice,
    AttackRoll,
    DamageRoll,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeaponAttackSkeletonInvalidReason {
    InvalidFill,
    StaleSubject,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeaponAttackSkeletonProtocol {
    Init(Vec<WeaponAttackSkeletonHole>),
    NeedsHoles(Vec<WeaponAttackSkeletonHole>),
    Resolved,
    Invalid {
        holes: Vec<WeaponAttackSkeletonHole>,
        reason: WeaponAttackSkeletonInvalidReason,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeaponAttackSkeletonState {
    pub skeleton_hp: i16,
    pub skeleton_dead: bool,
    pub action_available: bool,
    pub multiattack_dispatches_available: i16,
    pub sneak_attack_used_this_turn: bool,
    pub protocol: WeaponAttackSkeletonProtocol,
}

pub const SKELETON_INITIAL_HP: i16 = 13;
pub const ROGUE_ATTACK_MODIFIER: i16 = 3;

#[must_use]
pub fn weapon_attack_skeleton_initial_state() -> WeaponAttackSkeletonState {
    WeaponAttackSkeletonState {
        skeleton_hp: SKELETON_INITIAL_HP,
        skeleton_dead: false,
        action_available: true,
        multiattack_dispatches_available: 0,
        sneak_attack_used_this_turn: false,
        protocol: WeaponAttackSkeletonProtocol::Init(vec![WeaponAttackSkeletonHole::TargetChoice]),
    }
}

#[must_use]
pub fn discover_skeleton_weapon_attack() -> WeaponAttackSkeletonState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md
    // "Making an Attack"; QNT:
    // battle-runtime-weapon-attack-skeleton.mbt.qnt.
    with_protocol(
        weapon_attack_skeleton_initial_state(),
        WeaponAttackSkeletonProtocol::NeedsHoles(vec![WeaponAttackSkeletonHole::TargetChoice]),
    )
}

#[must_use]
pub fn fill_skeleton_weapon_attack_target() -> WeaponAttackSkeletonState {
    with_protocol(
        discover_skeleton_weapon_attack(),
        WeaponAttackSkeletonProtocol::NeedsHoles(vec![WeaponAttackSkeletonHole::AttackRoll]),
    )
}

#[must_use]
pub fn reject_skeleton_weapon_attack_wrong_target() -> WeaponAttackSkeletonState {
    invalid_from(
        discover_skeleton_weapon_attack(),
        vec![WeaponAttackSkeletonHole::TargetChoice],
        WeaponAttackSkeletonInvalidReason::InvalidFill,
    )
}

#[must_use]
pub fn fill_skeleton_weapon_attack_roll_miss() -> WeaponAttackSkeletonState {
    let current = fill_skeleton_weapon_attack_target();
    with_protocol(
        WeaponAttackSkeletonState {
            action_available: false,
            ..current
        },
        WeaponAttackSkeletonProtocol::Resolved,
    )
}

#[must_use]
pub fn fill_skeleton_weapon_attack_roll_hit() -> WeaponAttackSkeletonState {
    with_protocol(
        fill_skeleton_weapon_attack_target(),
        WeaponAttackSkeletonProtocol::NeedsHoles(vec![WeaponAttackSkeletonHole::DamageRoll]),
    )
}

#[must_use]
pub fn fill_skeleton_weapon_damage_low() -> WeaponAttackSkeletonState {
    fill_skeleton_weapon_damage(2, false)
}

#[must_use]
pub fn fill_skeleton_weapon_damage_high() -> WeaponAttackSkeletonState {
    fill_skeleton_weapon_damage(4, false)
}

#[must_use]
pub fn fill_skeleton_weapon_damage_low_sneak_attack() -> WeaponAttackSkeletonState {
    fill_skeleton_weapon_damage(4, true)
}

#[must_use]
pub fn fill_skeleton_weapon_damage_high_sneak_attack() -> WeaponAttackSkeletonState {
    fill_skeleton_weapon_damage(8, true)
}

#[must_use]
pub fn reject_skeleton_weapon_attack_stale_after_resolved() -> WeaponAttackSkeletonState {
    invalid_from(
        fill_skeleton_weapon_damage_high_sneak_attack(),
        Vec::new(),
        WeaponAttackSkeletonInvalidReason::StaleSubject,
    )
}

#[must_use]
pub fn start_skeleton_turn() -> WeaponAttackSkeletonState {
    with_protocol(
        weapon_attack_skeleton_initial_state(),
        WeaponAttackSkeletonProtocol::Resolved,
    )
}

#[must_use]
pub fn resolve_skeleton_multiattack() -> WeaponAttackSkeletonState {
    let current = start_skeleton_turn();
    with_protocol(
        WeaponAttackSkeletonState {
            action_available: false,
            multiattack_dispatches_available: 1,
            ..current
        },
        WeaponAttackSkeletonProtocol::Resolved,
    )
}

#[must_use]
pub fn reject_recursive_skeleton_multiattack() -> WeaponAttackSkeletonState {
    let current = resolve_skeleton_multiattack();
    invalid_from(
        current,
        Vec::new(),
        WeaponAttackSkeletonInvalidReason::StaleSubject,
    )
}

#[must_use]
pub fn spend_skeleton_multiattack_dispatch() -> WeaponAttackSkeletonState {
    let current = resolve_skeleton_multiattack();
    with_protocol(
        WeaponAttackSkeletonState {
            multiattack_dispatches_available: current.multiattack_dispatches_available - 1,
            ..current
        },
        WeaponAttackSkeletonProtocol::Resolved,
    )
}

#[must_use]
pub fn skeleton_hp_after_piercing_weapon_damage(hp: i16, damage_roll: i16) -> i16 {
    let damage = damage_roll + ROGUE_ATTACK_MODIFIER;
    (hp - damage).max(0)
}

fn fill_skeleton_weapon_damage(
    damage_roll: i16,
    use_sneak_attack: bool,
) -> WeaponAttackSkeletonState {
    let current = fill_skeleton_weapon_attack_roll_hit();
    let skeleton_hp = skeleton_hp_after_piercing_weapon_damage(current.skeleton_hp, damage_roll);
    with_protocol(
        WeaponAttackSkeletonState {
            skeleton_hp,
            skeleton_dead: skeleton_hp == 0,
            action_available: false,
            sneak_attack_used_this_turn: use_sneak_attack,
            ..current
        },
        WeaponAttackSkeletonProtocol::Resolved,
    )
}

fn invalid_from(
    state: WeaponAttackSkeletonState,
    holes: Vec<WeaponAttackSkeletonHole>,
    reason: WeaponAttackSkeletonInvalidReason,
) -> WeaponAttackSkeletonState {
    with_protocol(
        state,
        WeaponAttackSkeletonProtocol::Invalid { holes, reason },
    )
}

fn with_protocol(
    state: WeaponAttackSkeletonState,
    protocol: WeaponAttackSkeletonProtocol,
) -> WeaponAttackSkeletonState {
    WeaponAttackSkeletonState { protocol, ..state }
}
