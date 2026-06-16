pub const MAGIC_MISSILE_SKELETON_INITIAL_HIT_POINTS: i16 = 13;
pub const MAGIC_MISSILE_DART_COUNT: i16 = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MagicMissileHole {
    SpellTargetAllocation,
    SpellDamageRoll,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MagicMissileProtocolResult {
    Init,
    NeedsHoles,
    Resolved,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MagicMissileState {
    pub skeleton_hit_points: i16,
    pub skeleton_dead: bool,
    pub action_available: bool,
    pub multiattack_dispatches_available: u8,
    pub sneak_attack_used_this_turn: bool,
    pub protocol_holes: Vec<MagicMissileHole>,
    pub protocol_result: MagicMissileProtocolResult,
}

#[must_use]
pub fn magic_missile_initial_state() -> MagicMissileState {
    MagicMissileState {
        skeleton_hit_points: MAGIC_MISSILE_SKELETON_INITIAL_HIT_POINTS,
        skeleton_dead: false,
        action_available: true,
        multiattack_dispatches_available: 0,
        sneak_attack_used_this_turn: false,
        protocol_holes: vec![MagicMissileHole::SpellTargetAllocation],
        protocol_result: MagicMissileProtocolResult::Init,
    }
}

#[must_use]
pub fn project_magic_missile_target_allocation() -> MagicMissileState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-M-P.md
    // "Magic Missile"; QNT: battle-runtime-magic-missile.mbt.qnt.
    MagicMissileState {
        protocol_holes: vec![MagicMissileHole::SpellDamageRoll],
        protocol_result: MagicMissileProtocolResult::NeedsHoles,
        ..magic_missile_initial_state()
    }
}

#[must_use]
pub fn project_magic_missile_damage_fill(dart_pips_sum: i16) -> MagicMissileState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-M-P.md
    // "Magic Missile"; QNT: battle-runtime-magic-missile.mbt.qnt.
    let skeleton_hit_points = skeleton_hit_points_after_magic_missile_damage(
        MAGIC_MISSILE_SKELETON_INITIAL_HIT_POINTS,
        dart_pips_sum,
    );

    MagicMissileState {
        skeleton_hit_points,
        skeleton_dead: skeleton_hit_points == 0,
        action_available: false,
        protocol_holes: Vec::new(),
        protocol_result: MagicMissileProtocolResult::Resolved,
        ..project_magic_missile_target_allocation()
    }
}

#[must_use]
pub fn magic_missile_force_damage(dart_pips_sum: i16) -> i16 {
    (dart_pips_sum + MAGIC_MISSILE_DART_COUNT).max(0)
}

#[must_use]
pub fn skeleton_hit_points_after_magic_missile_damage(hit_points: i16, dart_pips_sum: i16) -> i16 {
    (hit_points - magic_missile_force_damage(dart_pips_sum)).max(0)
}
