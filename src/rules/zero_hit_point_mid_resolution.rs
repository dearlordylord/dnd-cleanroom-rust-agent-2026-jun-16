#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZeroHitPointMidResolutionScenario {
    Init,
    SpellAttackSequenceResolved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZeroHitPointMidResolutionHole {
    ZeroHitPointMidResolution,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZeroHitPointMidResolutionProtocol {
    Init(Vec<ZeroHitPointMidResolutionHole>),
    Resolved,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZeroHitPointMidResolutionState {
    pub scenario: ZeroHitPointMidResolutionScenario,
    pub source_hit_points: i16,
    pub source_unconscious: bool,
    pub source_concentrating: bool,
    pub shield_of_faith_present: bool,
    pub protected_target_hit_points: i16,
    pub source_damage: i16,
    pub protected_target_damage: i16,
    pub protected_target_damage_if_shield_of_faith_remained: i16,
    pub zero_hit_points_applied_before_second_beam: bool,
    pub teardown_before_second_beam: bool,
    pub remainder_used_post_teardown_state: bool,
    pub protocol: ZeroHitPointMidResolutionProtocol,
}

pub const ZERO_HP_SOURCE_INITIAL_HIT_POINTS: i16 = 4;
pub const ZERO_HP_PROTECTED_TARGET_INITIAL_HIT_POINTS: i16 = 12;
pub const ZERO_HP_SOURCE_DAMAGE: i16 = 6;
pub const ZERO_HP_PROTECTED_TARGET_DAMAGE_AFTER_TEARDOWN: i16 = 4;

#[must_use]
pub fn zero_hit_point_mid_resolution_initial_state() -> ZeroHitPointMidResolutionState {
    ZeroHitPointMidResolutionState {
        scenario: ZeroHitPointMidResolutionScenario::Init,
        source_hit_points: ZERO_HP_SOURCE_INITIAL_HIT_POINTS,
        source_unconscious: false,
        source_concentrating: true,
        shield_of_faith_present: true,
        protected_target_hit_points: ZERO_HP_PROTECTED_TARGET_INITIAL_HIT_POINTS,
        source_damage: 0,
        protected_target_damage: 0,
        protected_target_damage_if_shield_of_faith_remained: 0,
        zero_hit_points_applied_before_second_beam: false,
        teardown_before_second_beam: false,
        remainder_used_post_teardown_state: false,
        protocol: ZeroHitPointMidResolutionProtocol::Init(vec![
            ZeroHitPointMidResolutionHole::ZeroHitPointMidResolution,
        ]),
    }
}

#[must_use]
pub fn resolve_eldritch_blast_zero_hit_point_mid_resolution() -> ZeroHitPointMidResolutionState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-E-L.md
    // "Eldritch Blast"; Playing-the-Game.md "Dropping to 0 Hit Points";
    // Rules-Glossary.md "Unconscious", "Incapacitated", and
    // "Concentration"; Spells/Descriptions-S-Z.md "Shield of Faith".
    // QNT: battle-runtime-zero-hit-point-mid-resolution.mbt.qnt.
    ZeroHitPointMidResolutionState {
        scenario: ZeroHitPointMidResolutionScenario::SpellAttackSequenceResolved,
        source_hit_points: hit_points_after_damage(
            ZERO_HP_SOURCE_INITIAL_HIT_POINTS,
            ZERO_HP_SOURCE_DAMAGE,
        ),
        source_unconscious: true,
        source_concentrating: false,
        shield_of_faith_present: false,
        protected_target_hit_points: hit_points_after_damage(
            ZERO_HP_PROTECTED_TARGET_INITIAL_HIT_POINTS,
            ZERO_HP_PROTECTED_TARGET_DAMAGE_AFTER_TEARDOWN,
        ),
        source_damage: ZERO_HP_SOURCE_DAMAGE,
        protected_target_damage: ZERO_HP_PROTECTED_TARGET_DAMAGE_AFTER_TEARDOWN,
        protected_target_damage_if_shield_of_faith_remained: 0,
        zero_hit_points_applied_before_second_beam: true,
        teardown_before_second_beam: true,
        remainder_used_post_teardown_state: true,
        protocol: ZeroHitPointMidResolutionProtocol::Resolved,
    }
}

#[must_use]
pub fn hit_points_after_damage(hit_points: i16, damage: i16) -> i16 {
    (hit_points - damage).max(0)
}
