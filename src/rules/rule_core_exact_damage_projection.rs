use crate::rules::attack_damage_disposition::{
    apply_resolved_damage_to_positive_hit_points, CreatureKind, CreatureVitals,
};
use crate::rules::rule_core_components::{
    rule_core_component_route, RuleCoreComponentOwner, RuleCoreComponentRouteEvent,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleDamageType {
    Acid,
    Bludgeoning,
    Cold,
    Fire,
    Force,
    Lightning,
    Necrotic,
    Piercing,
    Poison,
    Psychic,
    Radiant,
    Slashing,
    Thunder,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpellSaveSuccessDamagePolicy {
    NoDamage,
    HalfDamage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExactDamageProjectionScenario {
    Initial,
    DirectInstanceDamageProjection,
    AttackCriticalDamageProjection,
    SaveSuccessHalfDamageProjection,
    SaveSuccessNoDamageProjection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AttackRollProcedureFacts {
    pub natural_d20: i16,
    pub total: i16,
    pub armor_class: i16,
    pub critical_threshold: i16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AttackRollOutcome {
    pub hits: bool,
    pub critical: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DirectSpellDamageFacts {
    pub damage_type: RuleDamageType,
    pub instance_count: i16,
    pub damage_per_instance: i16,
    pub minimum_instance_count: i16,
    pub maximum_instance_count: i16,
    pub minimum_damage_per_instance: i16,
    pub maximum_damage_per_instance: i16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpellAttackDamageBranchFacts {
    pub attack_roll: AttackRollProcedureFacts,
    pub damage_type: RuleDamageType,
    pub base_damage_dice: i16,
    pub maximum_base_damage_dice: i16,
    pub rolled_damage_dice_count: i16,
    pub damage_die_size: i16,
    pub damage_roll: i16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpellSaveDamageBranchFacts {
    pub damage_type: RuleDamageType,
    pub success_policy: SpellSaveSuccessDamagePolicy,
    pub saving_throw_failed: bool,
    pub damage_roll: i16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResolvedDamage {
    pub damage_amount: i16,
    pub damage_type: RuleDamageType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExactDamageProjectionState {
    pub scenario: ExactDamageProjectionScenario,
    pub damage_type: RuleDamageType,
    pub damage_amount: i16,
    pub instance_count: i16,
    pub damage_per_instance: i16,
    pub base_damage_dice: i16,
    pub rolled_damage_dice_count: i16,
    pub damage_die_size: i16,
    pub critical: bool,
    pub success_policy: SpellSaveSuccessDamagePolicy,
    pub saving_throw_failed: bool,
    pub target_initial_hp: i16,
    pub target_hp: i16,
    pub damage_to_hit_points: i16,
    pub replay_index: i16,
    pub component_route: Vec<RuleCoreComponentRouteEvent>,
}

pub const EXACT_DAMAGE_TARGET_INITIAL_HP: i16 = 20;

#[must_use]
pub fn exact_damage_projection_initial_state() -> ExactDamageProjectionState {
    project_exact_damage(
        ExactDamageProjectionScenario::Initial,
        ResolvedDamage {
            damage_type: RuleDamageType::Force,
            damage_amount: 0,
        },
        ExactDamageProjectionFacts::default(),
    )
}

#[must_use]
pub fn resolve_direct_instance_damage_projection() -> ExactDamageProjectionState {
    let facts = DirectSpellDamageFacts {
        damage_type: RuleDamageType::Force,
        instance_count: 3,
        damage_per_instance: 4,
        minimum_instance_count: 1,
        maximum_instance_count: 5,
        minimum_damage_per_instance: 2,
        maximum_damage_per_instance: 5,
    };
    let damage = resolve_direct_spell_damage(facts);
    project_exact_damage(
        ExactDamageProjectionScenario::DirectInstanceDamageProjection,
        damage,
        ExactDamageProjectionFacts {
            instance_count: facts.instance_count,
            damage_per_instance: facts.damage_per_instance,
            replay_index: 1,
            ..ExactDamageProjectionFacts::default()
        },
    )
}

#[must_use]
pub fn resolve_attack_critical_damage_projection() -> ExactDamageProjectionState {
    let facts = SpellAttackDamageBranchFacts {
        attack_roll: AttackRollProcedureFacts {
            natural_d20: 20,
            total: 20,
            armor_class: 13,
            critical_threshold: 20,
        },
        damage_type: RuleDamageType::Cold,
        base_damage_dice: 1,
        maximum_base_damage_dice: 4,
        rolled_damage_dice_count: 2,
        damage_die_size: 8,
        damage_roll: 7,
    };
    let attack = resolve_attack_roll(facts.attack_roll);
    let damage = resolve_spell_attack_damage_branch(facts);
    project_exact_damage(
        ExactDamageProjectionScenario::AttackCriticalDamageProjection,
        damage,
        ExactDamageProjectionFacts {
            base_damage_dice: facts.base_damage_dice,
            rolled_damage_dice_count: facts.rolled_damage_dice_count,
            damage_die_size: facts.damage_die_size,
            critical: attack.critical,
            replay_index: 2,
            ..ExactDamageProjectionFacts::default()
        },
    )
}

#[must_use]
pub fn resolve_save_success_half_damage_projection() -> ExactDamageProjectionState {
    let facts = SpellSaveDamageBranchFacts {
        damage_type: RuleDamageType::Fire,
        success_policy: SpellSaveSuccessDamagePolicy::HalfDamage,
        saving_throw_failed: false,
        damage_roll: 9,
    };
    let damage = resolve_spell_save_damage_branch(facts);
    project_exact_damage(
        ExactDamageProjectionScenario::SaveSuccessHalfDamageProjection,
        damage,
        ExactDamageProjectionFacts {
            success_policy: facts.success_policy,
            saving_throw_failed: facts.saving_throw_failed,
            replay_index: 3,
            ..ExactDamageProjectionFacts::default()
        },
    )
}

#[must_use]
pub fn resolve_save_success_no_damage_projection() -> ExactDamageProjectionState {
    let facts = SpellSaveDamageBranchFacts {
        damage_type: RuleDamageType::Acid,
        success_policy: SpellSaveSuccessDamagePolicy::NoDamage,
        saving_throw_failed: false,
        damage_roll: 6,
    };
    let damage = resolve_spell_save_damage_branch(facts);
    project_exact_damage(
        ExactDamageProjectionScenario::SaveSuccessNoDamageProjection,
        damage,
        ExactDamageProjectionFacts {
            success_policy: facts.success_policy,
            saving_throw_failed: facts.saving_throw_failed,
            replay_index: 4,
            ..ExactDamageProjectionFacts::default()
        },
    )
}

#[must_use]
pub fn resolve_direct_spell_damage(facts: DirectSpellDamageFacts) -> ResolvedDamage {
    ResolvedDamage {
        damage_type: facts.damage_type,
        damage_amount: if legal_direct_spell_damage_facts(facts) {
            facts.instance_count * facts.damage_per_instance
        } else {
            0
        },
    }
}

#[must_use]
pub fn resolve_spell_attack_damage_branch(facts: SpellAttackDamageBranchFacts) -> ResolvedDamage {
    let outcome = resolve_attack_roll(facts.attack_roll);
    ResolvedDamage {
        damage_type: facts.damage_type,
        damage_amount: if legal_spell_attack_damage_branch_facts(facts) && outcome.hits {
            facts.damage_roll.max(0)
        } else {
            0
        },
    }
}

#[must_use]
pub fn resolve_spell_save_damage_branch(facts: SpellSaveDamageBranchFacts) -> ResolvedDamage {
    ResolvedDamage {
        damage_type: facts.damage_type,
        damage_amount: if !legal_spell_save_damage_branch_facts(facts) {
            0
        } else if facts.saving_throw_failed {
            facts.damage_roll.max(0)
        } else {
            spell_save_success_damage_amount(facts.success_policy, facts.damage_roll)
        },
    }
}

#[must_use]
pub fn resolve_attack_roll(facts: AttackRollProcedureFacts) -> AttackRollOutcome {
    let critical = facts.natural_d20 >= facts.critical_threshold;
    AttackRollOutcome {
        hits: if facts.natural_d20 == 1 {
            false
        } else {
            critical || facts.total >= facts.armor_class
        },
        critical,
    }
}

#[must_use]
pub fn critical_damage_dice_count(base_dice: i16, critical: bool) -> i16 {
    if critical {
        base_dice * 2
    } else {
        base_dice
    }
}

#[must_use]
pub fn exact_damage_component_route() -> Vec<RuleCoreComponentRouteEvent> {
    let mut route = rule_core_component_route(RuleCoreComponentOwner::SpellProcedureProfile);
    route.extend(rule_core_component_route(
        RuleCoreComponentOwner::HitPointDamage,
    ));
    route
}

fn legal_direct_spell_damage_facts(facts: DirectSpellDamageFacts) -> bool {
    facts.minimum_instance_count >= 1
        && facts.maximum_instance_count >= facts.minimum_instance_count
        && facts.instance_count >= facts.minimum_instance_count
        && facts.instance_count <= facts.maximum_instance_count
        && facts.minimum_damage_per_instance >= 0
        && facts.maximum_damage_per_instance >= facts.minimum_damage_per_instance
        && facts.damage_per_instance >= facts.minimum_damage_per_instance
        && facts.damage_per_instance <= facts.maximum_damage_per_instance
}

fn legal_attack_roll_procedure_facts(facts: AttackRollProcedureFacts) -> bool {
    (1..=20).contains(&facts.natural_d20)
        && (0..=40).contains(&facts.total)
        && (1..=30).contains(&facts.armor_class)
        && (facts.critical_threshold == 19 || facts.critical_threshold == 20)
}

fn legal_spell_attack_damage_branch_facts(facts: SpellAttackDamageBranchFacts) -> bool {
    legal_attack_roll_procedure_facts(facts.attack_roll)
        && facts.base_damage_dice >= 1
        && facts.maximum_base_damage_dice >= 1
        && facts.maximum_base_damage_dice <= 10
        && facts.base_damage_dice <= facts.maximum_base_damage_dice
        && (1..=12).contains(&facts.damage_die_size)
        && facts.rolled_damage_dice_count
            == critical_damage_dice_count(
                facts.base_damage_dice,
                resolve_attack_roll(facts.attack_roll).critical,
            )
        && facts.damage_roll >= facts.rolled_damage_dice_count
        && facts.damage_roll <= facts.rolled_damage_dice_count * facts.damage_die_size
}

fn legal_spell_save_damage_branch_facts(facts: SpellSaveDamageBranchFacts) -> bool {
    facts.damage_roll >= 0
}

fn spell_save_success_damage_amount(policy: SpellSaveSuccessDamagePolicy, damage_roll: i16) -> i16 {
    match policy {
        SpellSaveSuccessDamagePolicy::NoDamage => 0,
        SpellSaveSuccessDamagePolicy::HalfDamage => damage_roll.max(0) / 2,
    }
}

fn project_exact_damage(
    scenario: ExactDamageProjectionScenario,
    damage: ResolvedDamage,
    facts: ExactDamageProjectionFacts,
) -> ExactDamageProjectionState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md "Damage
    // Rolls", "Critical Hits", "Saving Throws and Damage", "Half Damage",
    // "Damage Types", and "Hit Points". QNT:
    // rule-core-exact-damage-projection.mbt.qnt plus rule-core
    // spell-damage-projection-core.qnt and hit-point-damage.qnt.
    let applied = apply_resolved_damage_to_positive_hit_points(
        exact_damage_target_vitals(EXACT_DAMAGE_TARGET_INITIAL_HP),
        damage.damage_amount,
    );
    ExactDamageProjectionState {
        scenario,
        damage_type: damage.damage_type,
        damage_amount: damage.damage_amount,
        instance_count: facts.instance_count,
        damage_per_instance: facts.damage_per_instance,
        base_damage_dice: facts.base_damage_dice,
        rolled_damage_dice_count: facts.rolled_damage_dice_count,
        damage_die_size: facts.damage_die_size,
        critical: facts.critical,
        success_policy: facts.success_policy,
        saving_throw_failed: facts.saving_throw_failed,
        target_initial_hp: EXACT_DAMAGE_TARGET_INITIAL_HP,
        target_hp: applied.vitals.hit_points,
        damage_to_hit_points: applied.damage_to_hit_points,
        replay_index: facts.replay_index,
        component_route: exact_damage_component_route(),
    }
}

fn exact_damage_target_vitals(hit_points: i16) -> CreatureVitals {
    CreatureVitals {
        kind: CreatureKind::PlayerCharacter,
        hit_points,
        hit_point_maximum: EXACT_DAMAGE_TARGET_INITIAL_HP,
        temporary_hit_points: 0,
        dead: false,
        unconscious: false,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ExactDamageProjectionFacts {
    instance_count: i16,
    damage_per_instance: i16,
    base_damage_dice: i16,
    rolled_damage_dice_count: i16,
    damage_die_size: i16,
    critical: bool,
    success_policy: SpellSaveSuccessDamagePolicy,
    saving_throw_failed: bool,
    replay_index: i16,
}

impl Default for ExactDamageProjectionFacts {
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
            replay_index: 0,
        }
    }
}
