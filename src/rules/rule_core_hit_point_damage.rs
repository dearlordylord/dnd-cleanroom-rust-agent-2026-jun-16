use crate::rules::attack_damage_disposition::{
    apply_resolved_damage_to_positive_hit_points, CreatureKind, CreatureVitals,
    PositiveHitPointDamageResult,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HitPointDamageOutcome {
    Initial,
    TemporaryHitPointsAbsorbFirst,
    MonsterDiesAtZero,
    PlayerCharacterFallsUnconscious,
    PlayerCharacterDiesFromMassiveDamage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HitPointDamageState {
    pub outcome: HitPointDamageOutcome,
    pub hit_points: i16,
    pub hit_point_maximum: i16,
    pub temporary_hit_points: i16,
    pub dead: bool,
    pub unconscious: bool,
    pub damage_to_hit_points: i16,
    pub remaining_damage_at_zero: i16,
    pub replay_index: i16,
}

#[must_use]
pub fn hit_point_damage_initial_state() -> HitPointDamageState {
    HitPointDamageState {
        outcome: HitPointDamageOutcome::Initial,
        hit_points: 0,
        hit_point_maximum: 1,
        temporary_hit_points: 0,
        dead: false,
        unconscious: false,
        damage_to_hit_points: 0,
        remaining_damage_at_zero: 0,
        replay_index: 0,
    }
}

#[must_use]
pub fn resolve_temporary_hit_points_absorb_first() -> HitPointDamageState {
    state_from_damage_result(
        HitPointDamageOutcome::TemporaryHitPointsAbsorbFirst,
        apply_resolved_damage_to_positive_hit_points(
            CreatureVitals {
                kind: CreatureKind::PlayerCharacter,
                hit_points: 8,
                hit_point_maximum: 10,
                temporary_hit_points: 3,
                dead: false,
                unconscious: false,
            },
            5,
        ),
        1,
    )
}

#[must_use]
pub fn resolve_monster_dies_at_zero() -> HitPointDamageState {
    state_from_damage_result(
        HitPointDamageOutcome::MonsterDiesAtZero,
        apply_resolved_damage_to_positive_hit_points(
            CreatureVitals {
                kind: CreatureKind::Monster,
                hit_points: 4,
                hit_point_maximum: 10,
                temporary_hit_points: 0,
                dead: false,
                unconscious: false,
            },
            4,
        ),
        2,
    )
}

#[must_use]
pub fn resolve_player_character_falls_unconscious() -> HitPointDamageState {
    state_from_damage_result(
        HitPointDamageOutcome::PlayerCharacterFallsUnconscious,
        apply_resolved_damage_to_positive_hit_points(
            CreatureVitals {
                kind: CreatureKind::PlayerCharacter,
                hit_points: 4,
                hit_point_maximum: 12,
                temporary_hit_points: 0,
                dead: false,
                unconscious: false,
            },
            7,
        ),
        3,
    )
}

#[must_use]
pub fn resolve_player_character_dies_from_massive_damage() -> HitPointDamageState {
    state_from_damage_result(
        HitPointDamageOutcome::PlayerCharacterDiesFromMassiveDamage,
        apply_resolved_damage_to_positive_hit_points(
            CreatureVitals {
                kind: CreatureKind::PlayerCharacter,
                hit_points: 6,
                hit_point_maximum: 12,
                temporary_hit_points: 0,
                dead: false,
                unconscious: false,
            },
            18,
        ),
        4,
    )
}

fn state_from_damage_result(
    outcome: HitPointDamageOutcome,
    result: PositiveHitPointDamageResult,
    replay_index: i16,
) -> HitPointDamageState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md
    // "Hit Points", "Dropping to 0 Hit Points", and
    // "Temporary Hit Points". QNT:
    // rule-core-hit-point-damage.mbt.qnt and
    // shared-algebras/proofs/rule-core/hit-point-damage.qnt.
    HitPointDamageState {
        outcome,
        hit_points: result.vitals.hit_points,
        hit_point_maximum: result.vitals.hit_point_maximum,
        temporary_hit_points: result.vitals.temporary_hit_points,
        dead: result.vitals.dead,
        unconscious: result.vitals.unconscious,
        damage_to_hit_points: result.damage_to_hit_points,
        remaining_damage_at_zero: result.remaining_damage_at_zero,
        replay_index,
    }
}
