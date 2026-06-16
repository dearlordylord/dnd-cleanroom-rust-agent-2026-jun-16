#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreatureAttackActor {
    CreatureA,
    CreatureB,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreatureAttackFills {
    pub damage: i16,
    pub hit: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreatureAttackState {
    pub creature_a_hit_points: i16,
    pub creature_b_hit_points: i16,
}

pub const CREATURE_ATTACK_INITIAL_HIT_POINTS: i16 = 20;
pub const CREATURE_ATTACK_MAX_DAMAGE_ROLL: i16 = 6;

#[must_use]
pub fn creature_attack_initial_state() -> CreatureAttackState {
    CreatureAttackState {
        creature_a_hit_points: CREATURE_ATTACK_INITIAL_HIT_POINTS,
        creature_b_hit_points: CREATURE_ATTACK_INITIAL_HIT_POINTS,
    }
}

#[must_use]
pub fn resolve_creature_attack(
    state: CreatureAttackState,
    attacker: CreatureAttackActor,
    fills: CreatureAttackFills,
) -> CreatureAttackState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md
    // "Attack Rolls", "Making an Attack", "Hit Points", and "Damage Rolls";
    // Monsters/Overview.md "Attack Notation" and "Damage Notation".
    // QNT: cleanroom-input/qnt/battle-runtime/creature-attack.qnt.
    if !fills.hit {
        return state;
    }

    match attacker {
        CreatureAttackActor::CreatureA => CreatureAttackState {
            creature_b_hit_points: apply_damage_to_creature(
                state.creature_b_hit_points,
                fills.damage,
            ),
            ..state
        },
        CreatureAttackActor::CreatureB => CreatureAttackState {
            creature_a_hit_points: apply_damage_to_creature(
                state.creature_a_hit_points,
                fills.damage,
            ),
            ..state
        },
    }
}

#[must_use]
pub fn apply_damage_to_creature(current_hit_points: i16, damage: i16) -> i16 {
    (current_hit_points - damage.max(0)).max(0)
}
