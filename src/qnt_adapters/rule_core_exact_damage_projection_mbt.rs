use crate::rules::rule_core_components::{
    rule_core_component_route_event_ref, RuleCoreComponentRouteEvent,
};
use crate::rules::rule_core_exact_damage_projection::{
    exact_damage_component_route, resolve_attack_critical_damage_projection,
    resolve_direct_instance_damage_projection, resolve_save_success_half_damage_projection,
    resolve_save_success_no_damage_projection, ExactDamageProjectionScenario,
    ExactDamageProjectionState, RuleDamageType, SpellSaveSuccessDamagePolicy,
};

pub const BRANCH_ACTIONS: [&str; 4] = [
    "doAttackCriticalDamage",
    "doDirectInstanceDamage",
    "doSaveSuccessHalfDamage",
    "doSaveSuccessNoDamage",
];

pub fn replay_observed_action(observed_action_taken: &str) -> ExactDamageProjectionState {
    match observed_action_taken {
        "doAttackCriticalDamage" => resolve_attack_critical_damage_projection(),
        "doDirectInstanceDamage" => resolve_direct_instance_damage_projection(),
        "doSaveSuccessHalfDamage" => resolve_save_success_half_damage_projection(),
        "doSaveSuccessNoDamage" => resolve_save_success_no_damage_projection(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> ExactDamageProjectionState {
    match observed_action_taken {
        "doAttackCriticalDamage" => literal_exact_damage_state(
            ExactDamageProjectionScenario::AttackCriticalDamageProjection,
            RuleDamageType::Cold,
            7,
            ExactDamageLiteralFacts {
                base_damage_dice: 1,
                rolled_damage_dice_count: 2,
                damage_die_size: 8,
                critical: true,
                target_hp: 13,
                damage_to_hit_points: 7,
                replay_index: 2,
                ..ExactDamageLiteralFacts::default()
            },
        ),
        "doDirectInstanceDamage" => literal_exact_damage_state(
            ExactDamageProjectionScenario::DirectInstanceDamageProjection,
            RuleDamageType::Force,
            12,
            ExactDamageLiteralFacts {
                instance_count: 3,
                damage_per_instance: 4,
                target_hp: 8,
                damage_to_hit_points: 12,
                replay_index: 1,
                ..ExactDamageLiteralFacts::default()
            },
        ),
        "doSaveSuccessHalfDamage" => literal_exact_damage_state(
            ExactDamageProjectionScenario::SaveSuccessHalfDamageProjection,
            RuleDamageType::Fire,
            4,
            ExactDamageLiteralFacts {
                success_policy: SpellSaveSuccessDamagePolicy::HalfDamage,
                target_hp: 16,
                damage_to_hit_points: 4,
                replay_index: 3,
                ..ExactDamageLiteralFacts::default()
            },
        ),
        "doSaveSuccessNoDamage" => literal_exact_damage_state(
            ExactDamageProjectionScenario::SaveSuccessNoDamageProjection,
            RuleDamageType::Acid,
            0,
            ExactDamageLiteralFacts {
                success_policy: SpellSaveSuccessDamagePolicy::NoDamage,
                target_hp: 20,
                damage_to_hit_points: 0,
                replay_index: 4,
                ..ExactDamageLiteralFacts::default()
            },
        ),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_component_route() -> Vec<RuleCoreComponentRouteEvent> {
    exact_damage_component_route()
}

pub fn projection_payload(state: &ExactDamageProjectionState) -> String {
    [
        format!("qScenario={}", scenario_ref(state.scenario)),
        format!("qDamageType={}", damage_type_ref(state.damage_type)),
        format!("qDamageAmount={}", state.damage_amount),
        format!("qInstanceCount={}", state.instance_count),
        format!("qDamagePerInstance={}", state.damage_per_instance),
        format!("qBaseDamageDice={}", state.base_damage_dice),
        format!("qRolledDamageDiceCount={}", state.rolled_damage_dice_count),
        format!("qDamageDieSize={}", state.damage_die_size),
        format!("qCritical={}", state.critical),
        format!(
            "qSuccessPolicy={}",
            success_policy_ref(state.success_policy)
        ),
        format!("qSavingThrowFailed={}", state.saving_throw_failed),
        format!("qTargetInitialHp={}", state.target_initial_hp),
        format!("qTargetHp={}", state.target_hp),
        format!("qDamageToHitPoints={}", state.damage_to_hit_points),
        format!("qReplayIndex={}", state.replay_index),
        format!(
            "qComponentRoute={}",
            component_route_payload(&state.component_route)
        ),
    ]
    .join("\n")
}

pub fn component_route_payload(route: &[RuleCoreComponentRouteEvent]) -> String {
    route
        .iter()
        .map(rule_core_component_route_event_ref)
        .collect::<Vec<_>>()
        .join(">")
}

fn literal_exact_damage_state(
    scenario: ExactDamageProjectionScenario,
    damage_type: RuleDamageType,
    damage_amount: i16,
    facts: ExactDamageLiteralFacts,
) -> ExactDamageProjectionState {
    ExactDamageProjectionState {
        scenario,
        damage_type,
        damage_amount,
        instance_count: facts.instance_count,
        damage_per_instance: facts.damage_per_instance,
        base_damage_dice: facts.base_damage_dice,
        rolled_damage_dice_count: facts.rolled_damage_dice_count,
        damage_die_size: facts.damage_die_size,
        critical: facts.critical,
        success_policy: facts.success_policy,
        saving_throw_failed: facts.saving_throw_failed,
        target_initial_hp: 20,
        target_hp: facts.target_hp,
        damage_to_hit_points: facts.damage_to_hit_points,
        replay_index: facts.replay_index,
        component_route: exact_damage_component_route(),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ExactDamageLiteralFacts {
    instance_count: i16,
    damage_per_instance: i16,
    base_damage_dice: i16,
    rolled_damage_dice_count: i16,
    damage_die_size: i16,
    critical: bool,
    success_policy: SpellSaveSuccessDamagePolicy,
    saving_throw_failed: bool,
    target_hp: i16,
    damage_to_hit_points: i16,
    replay_index: i16,
}

impl Default for ExactDamageLiteralFacts {
    fn default() -> Self {
        Self {
            instance_count: 0,
            damage_per_instance: 0,
            base_damage_dice: 0,
            rolled_damage_dice_count: 0,
            damage_die_size: 0,
            critical: false,
            success_policy: SpellSaveSuccessDamagePolicy::NoDamage,
            saving_throw_failed: false,
            target_hp: 20,
            damage_to_hit_points: 0,
            replay_index: 0,
        }
    }
}

fn scenario_ref(scenario: ExactDamageProjectionScenario) -> &'static str {
    match scenario {
        ExactDamageProjectionScenario::Initial => "init",
        ExactDamageProjectionScenario::DirectInstanceDamageProjection => {
            "DirectInstanceDamageProjection"
        }
        ExactDamageProjectionScenario::AttackCriticalDamageProjection => {
            "AttackCriticalDamageProjection"
        }
        ExactDamageProjectionScenario::SaveSuccessHalfDamageProjection => {
            "SaveSuccessHalfDamageProjection"
        }
        ExactDamageProjectionScenario::SaveSuccessNoDamageProjection => {
            "SaveSuccessNoDamageProjection"
        }
    }
}

fn damage_type_ref(damage_type: RuleDamageType) -> &'static str {
    match damage_type {
        RuleDamageType::Acid => "AcidDamage",
        RuleDamageType::Bludgeoning => "BludgeoningDamage",
        RuleDamageType::Cold => "ColdDamage",
        RuleDamageType::Fire => "FireDamage",
        RuleDamageType::Force => "ForceDamage",
        RuleDamageType::Lightning => "LightningDamage",
        RuleDamageType::Necrotic => "NecroticDamage",
        RuleDamageType::Piercing => "PiercingDamage",
        RuleDamageType::Poison => "PoisonDamage",
        RuleDamageType::Psychic => "PsychicDamage",
        RuleDamageType::Radiant => "RadiantDamage",
        RuleDamageType::Slashing => "SlashingDamage",
        RuleDamageType::Thunder => "ThunderDamage",
    }
}

fn success_policy_ref(policy: SpellSaveSuccessDamagePolicy) -> &'static str {
    match policy {
        SpellSaveSuccessDamagePolicy::NoDamage => "SpellNoDamageOnSuccessfulSave",
        SpellSaveSuccessDamagePolicy::HalfDamage => "SpellHalfDamageOnSuccessfulSave",
    }
}
