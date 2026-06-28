use crate::rules::battle_reducer_spine::{
    discover_save_gated_damage_subject_observed, discover_spell_attack_damage_subject_observed,
    resolve_battle_subject_observed, start_battle_observed, Actor, AttackRollFacts,
    BattleEntrypointTrace, BattleResolutionRequest, BattleResolutionResult,
    BattleSaveGatedDamageApplication, BattleSaveGatedSpellFill, BattleSetup,
    BattleSpellActiveEffectKind, BattleSpellActiveEffects, BattleSpellAttackFill, BattleState,
};

use super::battle_runtime_reducer_route::{
    observed_reducer_route, route_discover_battle_acts, route_resolve_battle_subject,
    route_start_battle, ReducerRouteEvent, ReducerRouteFillKind, ReducerRouteOwnerGroup,
    ReducerRouteSubjectFamily,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttackSpellShapeWitness {
    pub target_hit_points: i16,
    pub spell_slot_spent_this_turn: bool,
    pub level_one_slots_remaining: i16,
    pub active_effect_kind: &'static str,
    pub active_effect_count: usize,
    pub scenario_outcome: &'static str,
    pub protocol_result: &'static str,
    pub invalid_reason: &'static str,
    pub protocol_hole_count: usize,
}

pub const BRANCH_ACTIONS: [&str; 6] = [
    "doChillTouchHitPointRegainPrevention",
    "doFireBoltHit",
    "doGuidingBoltNextAttackAdvantage",
    "doInflictWoundsFailedSave",
    "doInflictWoundsSuccessfulSave",
    "doShockingGraspOpportunityAttackDenied",
];

pub fn replay_observed_action(observed_action_taken: &str) -> AttackSpellShapeWitness {
    let (state, _route) = replay_observed_state_and_route(observed_action_taken);
    witness_from_battle_state(&state)
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    replay_observed_state_and_route(observed_action_taken).1
}

fn replay_observed_state_and_route(
    observed_action_taken: &str,
) -> (BattleState, Vec<ReducerRouteEvent>) {
    match observed_action_taken {
        "doChillTouchHitPointRegainPrevention" => spell_attack_route(
            false,
            BattleSpellActiveEffectKind::HitPointRegainPrevented,
            "ChillTouch",
        ),
        "doFireBoltHit" => spell_attack_route(false, BattleSpellActiveEffectKind::None, "FireBolt"),
        "doGuidingBoltNextAttackAdvantage" => spell_attack_route(
            true,
            BattleSpellActiveEffectKind::NextAttackRollAgainstSelfAdvantage,
            "GuidingBolt",
        ),
        "doInflictWoundsFailedSave" => save_gated_route(
            BattleSaveGatedDamageApplication {
                primary_target: Actor::Skeleton,
                primary_damage: 6,
                primary_effect: BattleSpellActiveEffectKind::None,
                secondary_target: None,
                secondary_damage: 0,
                spend_first_level_slot: true,
            },
            "InflictWoundsFailure",
        ),
        "doInflictWoundsSuccessfulSave" => save_gated_route(
            BattleSaveGatedDamageApplication {
                primary_target: Actor::Skeleton,
                primary_damage: 3,
                primary_effect: BattleSpellActiveEffectKind::None,
                secondary_target: None,
                secondary_damage: 0,
                spend_first_level_slot: true,
            },
            "InflictWoundsSuccess",
        ),
        "doShockingGraspOpportunityAttackDenied" => spell_attack_route(
            false,
            BattleSpellActiveEffectKind::OpportunityAttackDenied,
            "ShockingGrasp",
        ),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> AttackSpellShapeWitness {
    match observed_action_taken {
        "doChillTouchHitPointRegainPrevention" => {
            expected(8, false, 2, "hitPointRegainPrevented", 1, "ChillTouch")
        }
        "doFireBoltHit" => expected(8, false, 2, "none", 0, "FireBolt"),
        "doGuidingBoltNextAttackAdvantage" => {
            expected(8, true, 1, "nextAttackRollAgainstSelf", 1, "GuidingBolt")
        }
        "doInflictWoundsFailedSave" => expected(6, true, 1, "none", 0, "InflictWoundsFailure"),
        "doInflictWoundsSuccessfulSave" => expected(9, true, 1, "none", 0, "InflictWoundsSuccess"),
        "doShockingGraspOpportunityAttackDenied" => {
            expected(8, false, 2, "opportunityAttackDenied", 1, "ShockingGrasp")
        }
        action => panic!("unsupported expected mbt::actionTaken {action}"),
    }
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doChillTouchHitPointRegainPrevention"
        | "doFireBoltHit"
        | "doGuidingBoltNextAttackAdvantage"
        | "doShockingGraspOpportunityAttackDenied" => expected_spell_attack_route(),
        "doInflictWoundsFailedSave" | "doInflictWoundsSuccessfulSave" => {
            expected_save_gated_route()
        }
        action => panic!("unsupported expected route mbt::actionTaken {action}"),
    }
}

pub fn projection_payload(witness: &AttackSpellShapeWitness) -> String {
    [
        format!("targetHp={}", witness.target_hit_points),
        format!(
            "spellSlotSpentThisTurn={}",
            witness.spell_slot_spent_this_turn
        ),
        format!("level1SlotsRemaining={}", witness.level_one_slots_remaining),
        format!("activeEffectKind={}", witness.active_effect_kind),
        format!("activeEffectCount={}", witness.active_effect_count),
        format!("scenarioOutcome={}", witness.scenario_outcome),
        format!("protocolResult={}", witness.protocol_result),
        format!("invalidReason={}", witness.invalid_reason),
        format!("protocolHoleCount={}", witness.protocol_hole_count),
    ]
    .join("\n")
}

fn spell_attack_route(
    requires_spell_slot: bool,
    effect: BattleSpellActiveEffectKind,
    scenario_outcome: &'static str,
) -> (BattleState, Vec<ReducerRouteEvent>) {
    let mut observer = BattleEntrypointTrace::default();
    let setup = attack_spell_shape_setup();
    let state = start_battle_observed(setup, &mut observer).state;
    let (state, subject) = discover_spell_attack_damage_subject_observed(
        state,
        requires_spell_slot,
        effect,
        &mut observer,
    );
    let result = resolve_battle_subject_observed(
        state,
        BattleResolutionRequest::spell_attack(
            subject,
            BattleSpellAttackFill::TargetChoice(Actor::Skeleton),
        )
        .expect("single-target spell attack subject should accept target choice"),
        &mut observer,
    );
    let (state, subject) = continuation(result, "target choice");
    let result = resolve_battle_subject_observed(
        state,
        BattleResolutionRequest::spell_attack(
            subject,
            BattleSpellAttackFill::AttackRoll(AttackRollFacts {
                total: 20,
                natural_d20: 15,
            }),
        )
        .expect("single-target spell attack subject should accept attack roll"),
        &mut observer,
    );
    let (state, subject) = continuation(result, "attack roll");
    let result = resolve_battle_subject_observed(
        state,
        BattleResolutionRequest::spell_attack(subject, BattleSpellAttackFill::DamageRoll(4))
            .expect("single-target spell attack subject should accept damage roll"),
        &mut observer,
    );
    let state = resolved_state(result, scenario_outcome);
    (
        state,
        observed_reducer_route(&observer, &[ReducerRouteSubjectFamily::SpellAttack]),
    )
}

fn save_gated_route(
    application: BattleSaveGatedDamageApplication,
    scenario_outcome: &'static str,
) -> (BattleState, Vec<ReducerRouteEvent>) {
    let mut observer = BattleEntrypointTrace::default();
    let state = start_battle_observed(attack_spell_shape_setup(), &mut observer).state;
    let (state, subject) =
        discover_save_gated_damage_subject_observed(state, application, &mut observer);
    let result = resolve_battle_subject_observed(
        state,
        BattleResolutionRequest::save_gated_spell(
            subject,
            BattleSaveGatedSpellFill::SavingThrowOutcome,
        )
        .expect("save-gated spell subject should accept saving throw"),
        &mut observer,
    );
    let (state, subject) = continuation(result, "saving throw");
    let result = resolve_battle_subject_observed(
        state,
        BattleResolutionRequest::save_gated_spell(subject, BattleSaveGatedSpellFill::DamageRoll)
            .expect("save-gated spell subject should accept damage roll"),
        &mut observer,
    );
    let state = resolved_state(result, scenario_outcome);
    (
        state,
        observed_reducer_route(&observer, &[ReducerRouteSubjectFamily::SaveGatedSpell]),
    )
}

fn attack_spell_shape_setup() -> BattleSetup {
    let mut setup = BattleSetup::standard();
    setup.initiative.still_to_act.waiting = vec![Actor::Skeleton];
    setup.skeleton.hp = 12;
    setup.skeleton.max_hp = 12;
    setup.skeleton.armor_class = 10;
    setup
}

fn continuation(
    result: BattleResolutionResult,
    context: &str,
) -> (
    BattleState,
    crate::rules::battle_reducer_spine::BattleSubject,
) {
    result
        .into_needs_holes()
        .map(|needs| (needs.state, needs.subject))
        .unwrap_or_else(|| panic!("{context} should continue"))
}

fn resolved_state(result: BattleResolutionResult, scenario_outcome: &'static str) -> BattleState {
    let BattleResolutionResult::Resolved { state } = result else {
        panic!("{scenario_outcome} should resolve")
    };
    state
}

fn witness_from_battle_state(state: &BattleState) -> AttackSpellShapeWitness {
    let effects = state.skeleton.spell_active_effects;
    AttackSpellShapeWitness {
        target_hit_points: state.skeleton.hp,
        spell_slot_spent_this_turn: state.fighter.spell_slots.first_level_expended > 0,
        level_one_slots_remaining: 2 - state.fighter.spell_slots.first_level_expended,
        active_effect_kind: active_effect_kind(effects),
        active_effect_count: effects.count(),
        scenario_outcome: scenario_from_battle_state(state),
        protocol_result: "resolved",
        invalid_reason: "none",
        protocol_hole_count: 0,
    }
}

fn expected(
    target_hit_points: i16,
    spell_slot_spent_this_turn: bool,
    level_one_slots_remaining: i16,
    active_effect_kind: &'static str,
    active_effect_count: usize,
    scenario_outcome: &'static str,
) -> AttackSpellShapeWitness {
    AttackSpellShapeWitness {
        target_hit_points,
        spell_slot_spent_this_turn,
        level_one_slots_remaining,
        active_effect_kind,
        active_effect_count,
        scenario_outcome,
        protocol_result: "resolved",
        invalid_reason: "none",
        protocol_hole_count: 0,
    }
}

fn active_effect_kind(effects: BattleSpellActiveEffects) -> &'static str {
    if effects
        .hit_point_regain_prevention
        .prevents_hit_point_regain()
    {
        "hitPointRegainPrevented"
    } else if effects.next_attack_roll_against_self_advantage {
        "nextAttackRollAgainstSelf"
    } else if effects.opportunity_attack_denied {
        "opportunityAttackDenied"
    } else {
        "none"
    }
}

fn scenario_from_battle_state(state: &BattleState) -> &'static str {
    match (
        state.skeleton.hp,
        state.fighter.spell_slots.first_level_expended,
        active_effect_kind(state.skeleton.spell_active_effects),
    ) {
        (8, 0, "hitPointRegainPrevented") => "ChillTouch",
        (8, 0, "none") => "FireBolt",
        (8, 1, "nextAttackRollAgainstSelf") => "GuidingBolt",
        (6, 1, "none") => "InflictWoundsFailure",
        (9, 1, "none") => "InflictWoundsSuccess",
        (8, 0, "opportunityAttackDenied") => "ShockingGrasp",
        _ => "Unknown",
    }
}

fn expected_spell_attack_route() -> Vec<ReducerRouteEvent> {
    vec![
        route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
        route_discover_battle_acts(
            ReducerRouteSubjectFamily::SpellAttack,
            vec![crate::rules::battle_reducer_spine::BattleHoleKind::TargetChoice],
            ReducerRouteOwnerGroup::ActionEconomy,
        ),
        route_resolve_battle_subject(
            ReducerRouteSubjectFamily::SpellAttack,
            ReducerRouteFillKind::TargetChoice,
            vec![crate::rules::battle_reducer_spine::BattleHoleKind::AttackRoll],
            ReducerRouteOwnerGroup::TargetSelection,
        ),
        route_resolve_battle_subject(
            ReducerRouteSubjectFamily::SpellAttack,
            ReducerRouteFillKind::AttackRoll,
            vec![crate::rules::battle_reducer_spine::BattleHoleKind::RolledDice],
            ReducerRouteOwnerGroup::AttackRoll,
        ),
        route_resolve_battle_subject(
            ReducerRouteSubjectFamily::SpellAttack,
            ReducerRouteFillKind::RolledDice,
            Vec::new(),
            ReducerRouteOwnerGroup::HitPoint,
        ),
    ]
}

fn expected_save_gated_route() -> Vec<ReducerRouteEvent> {
    vec![
        route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
        route_discover_battle_acts(
            ReducerRouteSubjectFamily::SaveGatedSpell,
            vec![crate::rules::battle_reducer_spine::BattleHoleKind::SavingThrowOutcome],
            ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
        ),
        route_resolve_battle_subject(
            ReducerRouteSubjectFamily::SaveGatedSpell,
            ReducerRouteFillKind::SavingThrowOutcome,
            vec![crate::rules::battle_reducer_spine::BattleHoleKind::RolledDice],
            ReducerRouteOwnerGroup::HoleFrontier,
        ),
        route_resolve_battle_subject(
            ReducerRouteSubjectFamily::SaveGatedSpell,
            ReducerRouteFillKind::RolledDice,
            Vec::new(),
            ReducerRouteOwnerGroup::HitPoint,
        ),
    ]
}
