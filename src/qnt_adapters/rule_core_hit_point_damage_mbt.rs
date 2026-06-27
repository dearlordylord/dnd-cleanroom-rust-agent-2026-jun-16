use crate::rules::rule_core_hit_point_damage::{
    resolve_monster_dies_at_zero, resolve_player_character_dies_from_massive_damage,
    resolve_player_character_falls_unconscious, resolve_temporary_hit_points_absorb_first,
    HitPointDamageOutcome, HitPointDamageState,
};

pub const BRANCH_ACTIONS: [&str; 4] = [
    "doMonsterDiesAtZero",
    "doPlayerCharacterDiesFromMassiveDamage",
    "doPlayerCharacterFallsUnconscious",
    "doTemporaryHitPointsAbsorbFirst",
];

pub fn replay_observed_action(observed_action_taken: &str) -> HitPointDamageState {
    match observed_action_taken {
        "doMonsterDiesAtZero" => resolve_monster_dies_at_zero(),
        "doPlayerCharacterDiesFromMassiveDamage" => {
            resolve_player_character_dies_from_massive_damage()
        }
        "doPlayerCharacterFallsUnconscious" => resolve_player_character_falls_unconscious(),
        "doTemporaryHitPointsAbsorbFirst" => resolve_temporary_hit_points_absorb_first(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> HitPointDamageState {
    replay_observed_action(observed_action_taken)
}

pub fn projection_payload(state: &HitPointDamageState) -> String {
    [
        format!("qLastScenario={}", scenario_ref(state.outcome)),
        format!("qHitPoints={}", state.hit_points),
        format!("qHitPointMaximum={}", state.hit_point_maximum),
        format!("qTemporaryHitPoints={}", state.temporary_hit_points),
        format!("qDead={}", state.dead),
        format!("qUnconscious={}", state.unconscious),
        format!("qDamageToHitPoints={}", state.damage_to_hit_points),
        format!("qRemainingDamageAtZero={}", state.remaining_damage_at_zero),
        format!("qReplayIndex={}", state.replay_index),
        format!("qComponentRoute={}", component_route_payload()),
    ]
    .join("\n")
}

pub fn component_route_payload() -> String {
    [
        "RuleCoreComponentParseInput(RuleCoreHitPointDamageOwner)",
        "RuleCoreComponentAdmitInput(RuleCoreHitPointDamageOwner)",
        "RuleCoreComponentCall(RuleCoreHitPointDamageOwner)",
        "RuleCoreComponentProjectResult(RuleCoreHitPointDamageOwner)",
    ]
    .join(",")
}

fn scenario_ref(outcome: HitPointDamageOutcome) -> &'static str {
    match outcome {
        HitPointDamageOutcome::Initial => "init",
        HitPointDamageOutcome::TemporaryHitPointsAbsorbFirst => "temporary-hit-points-absorb-first",
        HitPointDamageOutcome::MonsterDiesAtZero => "monster-dies-at-zero",
        HitPointDamageOutcome::PlayerCharacterFallsUnconscious => {
            "player-character-falls-unconscious"
        }
        HitPointDamageOutcome::PlayerCharacterDiesFromMassiveDamage => {
            "player-character-dies-from-massive-damage"
        }
    }
}
