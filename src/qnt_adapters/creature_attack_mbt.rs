use crate::rules::creature_attack::{
    creature_attack_initial_state, resolve_creature_attack, CreatureAttackActor,
    CreatureAttackFills, CreatureAttackState, CREATURE_ATTACK_MAX_DAMAGE_ROLL,
};

pub const BRANCH_ACTIONS: [&str; 2] = ["doAttackerAAttacks", "doAttackerBAttacks"];
pub const REPLAY_DAMAGE_SAMPLE: i16 = CREATURE_ATTACK_MAX_DAMAGE_ROLL;
pub const REPLAY_HIT_SAMPLE: bool = true;

pub fn replay_observed_action(observed_action_taken: &str) -> CreatureAttackState {
    match observed_action_taken {
        "doAttackerAAttacks" => replay_attack(CreatureAttackActor::CreatureA),
        "doAttackerBAttacks" => replay_attack(CreatureAttackActor::CreatureB),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> CreatureAttackState {
    replay_observed_action(observed_action_taken)
}

pub fn projection_payload(state: &CreatureAttackState) -> String {
    [
        format!("qCreatureAHp={}", state.creature_a_hit_points),
        format!("qCreatureBHp={}", state.creature_b_hit_points),
    ]
    .join("\n")
}

pub fn replay_sampled_inputs() -> CreatureAttackFills {
    CreatureAttackFills {
        damage: REPLAY_DAMAGE_SAMPLE,
        hit: REPLAY_HIT_SAMPLE,
    }
}

fn replay_attack(attacker: CreatureAttackActor) -> CreatureAttackState {
    resolve_creature_attack(
        creature_attack_initial_state(),
        attacker,
        replay_sampled_inputs(),
    )
}
