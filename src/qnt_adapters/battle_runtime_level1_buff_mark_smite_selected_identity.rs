use crate::rules::battle_features::SavingThrowAbility;
use crate::rules::battle_reducer_spine::{
    discover_generic_route_subject_observed, resolve_battle_subject_observed,
    start_battle_observed, BattleEntrypointTrace, BattleGenericRouteFill, BattleResolutionRequest,
    BattleSetup, BattleSubjectKind,
};
use crate::rules::level_one_spell_effects::{
    project_divine_favor_weapon_damage_rider, project_divine_smite_after_hit_damage,
    project_ensnaring_strike_restraint_damage_and_escape, project_false_life_temporary_hit_points,
    project_heroism_cleanup_after_effect_end,
    project_heroism_frightened_immunity_and_turn_start_temporary_hit_points,
    project_hex_damage_rider_and_later_turn_transfer,
    project_hunters_mark_damage_rider_and_same_turn_transfer, project_longstrider_speed_increase,
    project_searing_smite_timed_damage_and_save_cleanup, project_shillelagh_weapon_attack_override,
    project_true_strike_spell_hosted_weapon_attack, AbilityCheckSkill, AbilityScore,
    ActiveMarkTransfer, LevelOneCondition, LevelOneEffectTarget, LevelOneSpell,
    LevelOneSpellEffectProtocol, LevelOneSpellEffectScenarioOutcome, LevelOneSpellEffectState,
    MarkRetargetTiming, MarkTransferAvailability, SaveSuccessEnding, SpellHostedWeaponAttackEffect,
    SpellWeapon, SpellWeaponComponent, WeaponAttackOverrideEffect, WeaponAttackPresentation,
};
use crate::rules::spell_shapes::DamageType;

use super::battle_runtime_marked_damage_immunity_active_effects_route::{
    expected_route as expected_marked_damage_immunity_route,
    replay_observed_route as replay_marked_damage_immunity_route,
};
use super::battle_runtime_reducer_route::{
    observed_reducer_route, route_discover_battle_acts_from_route_holes,
    route_resolve_battle_subject_from_route_result,
    route_resolve_battle_subject_without_fill_from_route_result, route_start_battle,
    ReducerRouteEvent, ReducerRouteFillKind, ReducerRouteHoleKind, ReducerRouteOwnerGroup,
    ReducerRouteResolutionOutcome, ReducerRouteSubjectFamily,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LevelOneSpellEffectWitness {
    pub divine_favor_active_rider_count: u8,
    pub target_hp: i16,
    pub longstrider_target_speed_feet: u16,
    pub longstrider_speed_effect_source_spell_id: &'static str,
    pub longstrider_speed_effect_target: &'static str,
    pub longstrider_speed_delta_feet: i16,
    pub caster_temp_hp: i16,
    pub caster_frightened: bool,
    pub spell_slot_spent_this_turn: bool,
    pub level1_slots_remaining: u8,
    pub damage_rider_source_spell_id: &'static str,
    pub damage_rider_damage_type: &'static str,
    pub damage_rider_dice: u8,
    pub damage_rider_die_size: u8,
    pub temporary_hit_points_source_spell_id: &'static str,
    pub temporary_hit_points_dice: u8,
    pub temporary_hit_points_die_size: u8,
    pub temporary_hit_points_flat: i16,
    pub frightened_immunity_source_spell_id: &'static str,
    pub frightened_immunity_condition: &'static str,
    pub turn_start_temporary_hit_points_source_spell_id: &'static str,
    pub turn_start_temporary_hit_points_amount: i16,
    pub ensnaring_strike_restrained_before_escape: bool,
    pub target_restrained: bool,
    pub caster_concentrating: bool,
    pub ensnaring_strike_save_source_spell_id: &'static str,
    pub ensnaring_strike_save_ability: &'static str,
    pub turn_start_damage_source_spell_id: &'static str,
    pub turn_start_damage_damage_type: &'static str,
    pub turn_start_damage_dice: u8,
    pub turn_start_damage_die_size: u8,
    pub escape_check_ability: &'static str,
    pub escape_check_skill: &'static str,
    pub hunters_mark_damage_hole_source_spell_id: &'static str,
    pub hunters_mark_damage_hole_damage_type: &'static str,
    pub hunters_mark_damage_hole_dice: u8,
    pub hunters_mark_damage_hole_die_size: u8,
    pub hunters_mark_active_mark_source_spell_id: &'static str,
    pub hunters_mark_active_mark_target: &'static str,
    pub hunters_mark_concentration_source_spell_id: &'static str,
    pub hunters_mark_transfer_kind_on_drop_turn: &'static str,
    pub hunters_mark_active_mark_transfer_kind: &'static str,
    pub hunters_mark_active_mark_retarget_timing: &'static str,
    pub hunters_mark_transfer_visible_on_drop_turn: bool,
    pub hex_damage_hole_source_spell_id: &'static str,
    pub hex_damage_hole_damage_type: &'static str,
    pub hex_damage_hole_dice: u8,
    pub hex_damage_hole_die_size: u8,
    pub hex_active_mark_source_spell_id: &'static str,
    pub hex_active_mark_target: &'static str,
    pub hex_ability_check_ability: &'static str,
    pub hex_transfer_kind_on_drop_turn: &'static str,
    pub hex_active_mark_transfer_kind: &'static str,
    pub hex_active_mark_retarget_timing: &'static str,
    pub hex_transfer_visible_on_drop_turn: bool,
    pub searing_smite_immediate_damage_source_spell_id: &'static str,
    pub searing_smite_immediate_damage_damage_type: &'static str,
    pub searing_smite_immediate_damage_dice: u8,
    pub searing_smite_immediate_damage_die_size: u8,
    pub searing_smite_active_before_successful_save: bool,
    pub searing_smite_turn_start_damage_source_spell_id: &'static str,
    pub searing_smite_turn_start_damage_damage_type: &'static str,
    pub searing_smite_turn_start_damage_dice: u8,
    pub searing_smite_turn_start_damage_die_size: u8,
    pub searing_smite_turn_start_save_source_spell_id: &'static str,
    pub searing_smite_turn_start_save_ability: &'static str,
    pub searing_smite_turn_start_save_success_ends: &'static str,
    pub searing_smite_active_after_successful_save: bool,
    pub shillelagh_override_source_spell_id: &'static str,
    pub shillelagh_override_weapon_unit_id: &'static str,
    pub shillelagh_spellcasting_ability_modifier: i16,
    pub shillelagh_override_attack_bonus: i16,
    pub shillelagh_override_damage_dice: u8,
    pub shillelagh_override_damage_die_size: u8,
    pub shillelagh_force_attack_name: &'static str,
    pub shillelagh_force_attack_bonus: i16,
    pub shillelagh_force_damage_type: &'static str,
    pub shillelagh_force_damage_dice: u8,
    pub shillelagh_force_damage_die_size: u8,
    pub shillelagh_force_damage_modifier: i16,
    pub true_strike_source_spell_id: &'static str,
    pub true_strike_component_weapon_item_id: &'static str,
    pub true_strike_weapon_unit_id: &'static str,
    pub true_strike_attack_name: &'static str,
    pub true_strike_attack_bonus: i16,
    pub true_strike_damage_type: &'static str,
    pub true_strike_damage_dice: u8,
    pub true_strike_damage_die_size: u8,
    pub true_strike_damage_modifier: i16,
    pub scenario_outcome: &'static str,
    pub protocol_result: &'static str,
    pub protocol_holes: Vec<&'static str>,
}

pub const BRANCH_ACTIONS: [&str; 12] = [
    "doDivineFavorWeaponDamageRider",
    "doDivineSmiteAfterHitDamage",
    "doEnsnaringStrikeAfterHitRestraintTurnStartDamageAndEscape",
    "doFalseLifeTemporaryHitPoints",
    "doHeroismFrightenedImmunityTurnStartTemporaryHitPoints",
    "doHeroismFrightenedImmunityTurnStartTemporaryHitPointsCleanup",
    "doHuntersMarkMarkedDamageRiderConcentrationAndSameTurnTransfer",
    "doHexMarkedDamageRiderAndLaterTurnTransfer",
    "doLongstriderSpeedIncrease",
    "doSearingSmiteAfterHitTimedDamageAndSaveCleanup",
    "doShillelaghWeaponAttackOverride",
    "doTrueStrikeSpellHostedWeaponAttack",
];

pub const ACCEPTED_BRANCH_ACTIONS: [&str; 4] = [
    "doHeroismFrightenedImmunityTurnStartTemporaryHitPoints",
    "doHeroismFrightenedImmunityTurnStartTemporaryHitPointsCleanup",
    "doHuntersMarkMarkedDamageRiderConcentrationAndSameTurnTransfer",
    "doHexMarkedDamageRiderAndLaterTurnTransfer",
];

const SELECTED_ROW_NOT_GENERIC_MARKED_IMMUNITY_FACT: &str = "This lane accepts only selected rows whose public reducer route events match copied generic marked-damage or condition-immunity route facts. This selected row belongs to another active-effect substrate and is not counted by SQNT-07A.";

pub const BLOCKED_BRANCH_ACTIONS: [(&str, &str); 8] = [
    (
        "doDivineFavorWeaponDamageRider",
        SELECTED_ROW_NOT_GENERIC_MARKED_IMMUNITY_FACT,
    ),
    (
        "doDivineSmiteAfterHitDamage",
        SELECTED_ROW_NOT_GENERIC_MARKED_IMMUNITY_FACT,
    ),
    (
        "doEnsnaringStrikeAfterHitRestraintTurnStartDamageAndEscape",
        SELECTED_ROW_NOT_GENERIC_MARKED_IMMUNITY_FACT,
    ),
    (
        "doFalseLifeTemporaryHitPoints",
        SELECTED_ROW_NOT_GENERIC_MARKED_IMMUNITY_FACT,
    ),
    (
        "doLongstriderSpeedIncrease",
        SELECTED_ROW_NOT_GENERIC_MARKED_IMMUNITY_FACT,
    ),
    (
        "doSearingSmiteAfterHitTimedDamageAndSaveCleanup",
        SELECTED_ROW_NOT_GENERIC_MARKED_IMMUNITY_FACT,
    ),
    (
        "doShillelaghWeaponAttackOverride",
        SELECTED_ROW_NOT_GENERIC_MARKED_IMMUNITY_FACT,
    ),
    (
        "doTrueStrikeSpellHostedWeaponAttack",
        SELECTED_ROW_NOT_GENERIC_MARKED_IMMUNITY_FACT,
    ),
];

pub fn replay_observed_action(observed_action_taken: &str) -> LevelOneSpellEffectWitness {
    match observed_action_taken {
        action if blocker_reason(action).is_some() => {
            panic!("{}: {}", action, blocker_reason(action).unwrap())
        }
        "doDivineFavorWeaponDamageRider" => {
            witness_from_state(project_divine_favor_weapon_damage_rider())
        }
        "doDivineSmiteAfterHitDamage" => {
            witness_from_state(project_divine_smite_after_hit_damage())
        }
        "doEnsnaringStrikeAfterHitRestraintTurnStartDamageAndEscape" => {
            witness_from_state(project_ensnaring_strike_restraint_damage_and_escape())
        }
        "doFalseLifeTemporaryHitPoints" => {
            witness_from_state(project_false_life_temporary_hit_points())
        }
        "doHeroismFrightenedImmunityTurnStartTemporaryHitPoints" => witness_from_state(
            project_heroism_frightened_immunity_and_turn_start_temporary_hit_points(),
        ),
        "doHeroismFrightenedImmunityTurnStartTemporaryHitPointsCleanup" => {
            witness_from_state(project_heroism_cleanup_after_effect_end())
        }
        "doHuntersMarkMarkedDamageRiderConcentrationAndSameTurnTransfer" => {
            witness_from_state(project_hunters_mark_damage_rider_and_same_turn_transfer())
        }
        "doHexMarkedDamageRiderAndLaterTurnTransfer" => {
            witness_from_state(project_hex_damage_rider_and_later_turn_transfer())
        }
        "doLongstriderSpeedIncrease" => witness_from_state(project_longstrider_speed_increase()),
        "doSearingSmiteAfterHitTimedDamageAndSaveCleanup" => {
            witness_from_state(project_searing_smite_timed_damage_and_save_cleanup())
        }
        "doShillelaghWeaponAttackOverride" => {
            witness_from_state(project_shillelagh_weapon_attack_override())
        }
        "doTrueStrikeSpellHostedWeaponAttack" => {
            witness_from_state(project_true_strike_spell_hosted_weapon_attack())
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        action if blocker_reason(action).is_some() => {
            panic!("{}: {}", action, blocker_reason(action).unwrap())
        }
        "doDivineFavorWeaponDamageRider" => replay_generic_route(
            BattleSubjectKind::WeaponDamageRiderDamage,
            &[BattleGenericRouteFill::RolledDice],
            &[ReducerRouteSubjectFamily::WeaponDamageRider],
        ),
        "doDivineSmiteAfterHitDamage" => replay_generic_route(
            BattleSubjectKind::AfterHitDamageRiderAttackDamage,
            &[BattleGenericRouteFill::RolledDice],
            &[ReducerRouteSubjectFamily::AfterHitDamageRider],
        ),
        "doEnsnaringStrikeAfterHitRestraintTurnStartDamageAndEscape" => replay_generic_route(
            BattleSubjectKind::AfterHitDamageRiderEscapeConcentrationCleanup,
            &[BattleGenericRouteFill::AbilityCheck],
            &[ReducerRouteSubjectFamily::AfterHitDamageRider],
        ),
        "doFalseLifeTemporaryHitPoints" => replay_generic_route(
            BattleSubjectKind::ScalarBuffEffectTemporaryHitPoint,
            &[BattleGenericRouteFill::WithoutFill],
            &[ReducerRouteSubjectFamily::ScalarBuffEffect],
        ),
        "doHeroismFrightenedImmunityTurnStartTemporaryHitPoints" => {
            replay_marked_damage_immunity_route("doGrantTurnStartTemporaryHitPoints")
        }
        "doHeroismFrightenedImmunityTurnStartTemporaryHitPointsCleanup" => {
            replay_marked_damage_immunity_route("doCleanupConditionImmunityTemporaryHitPoints")
        }
        "doHuntersMarkMarkedDamageRiderConcentrationAndSameTurnTransfer" => {
            replay_marked_damage_immunity_route("doTransferMarkedDamageRiderSameTurn")
        }
        "doHexMarkedDamageRiderAndLaterTurnTransfer" => {
            replay_marked_damage_immunity_route("doTransferMarkedDamageRiderLaterTurn")
        }
        "doLongstriderSpeedIncrease" => replay_generic_route(
            BattleSubjectKind::ScalarBuffEffectSpeedDelta,
            &[BattleGenericRouteFill::WithoutFill],
            &[ReducerRouteSubjectFamily::ScalarBuffEffect],
        ),
        "doSearingSmiteAfterHitTimedDamageAndSaveCleanup" => replay_generic_route(
            BattleSubjectKind::AfterHitDamageRiderTurnStartSaveCleanup,
            &[
                BattleGenericRouteFill::RolledDice,
                BattleGenericRouteFill::SavingThrowOutcome,
            ],
            &[ReducerRouteSubjectFamily::AfterHitDamageRider],
        ),
        "doShillelaghWeaponAttackOverride" => replay_generic_route(
            BattleSubjectKind::HeldWeaponActiveEffectDamage,
            &[BattleGenericRouteFill::RolledDice],
            &[ReducerRouteSubjectFamily::HeldWeaponActiveEffect],
        ),
        "doTrueStrikeSpellHostedWeaponAttack" => replay_generic_route(
            BattleSubjectKind::SpellHostedWeaponAttackDamage,
            &[BattleGenericRouteFill::RolledDice],
            &[ReducerRouteSubjectFamily::SpellHostedWeaponAttack],
        ),
        action => panic!("unsupported route mbt::actionTaken {action}"),
    }
}

fn replay_generic_route(
    subject_kind: BattleSubjectKind,
    fills: &[BattleGenericRouteFill],
    route_subjects: &[ReducerRouteSubjectFamily],
) -> Vec<ReducerRouteEvent> {
    let mut trace = BattleEntrypointTrace::default();
    let state = start_battle_observed(BattleSetup::standard(), &mut trace).state;
    let (mut state, subject) =
        discover_generic_route_subject_observed(state, subject_kind, &mut trace);
    for fill in fills {
        state = resolve_battle_subject_observed(
            state,
            BattleResolutionRequest::generic_route(subject, *fill)
                .expect("generic route subject should accept generic route fills"),
            &mut trace,
        )
        .into_state();
    }
    observed_reducer_route(&trace, route_subjects)
}

pub fn expected_witness(observed_action_taken: &str) -> LevelOneSpellEffectWitness {
    replay_observed_action(observed_action_taken)
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        action if blocker_reason(action).is_some() => {
            panic!("{}: {}", action, blocker_reason(action).unwrap())
        }
        "doDivineFavorWeaponDamageRider" => expected_damage_route(
            ReducerRouteSubjectFamily::WeaponDamageRider,
            ReducerRouteOwnerGroup::ActiveEffect,
            ReducerRouteOwnerGroup::HitPoint,
        ),
        "doDivineSmiteAfterHitDamage" => expected_damage_route(
            ReducerRouteSubjectFamily::AfterHitDamageRider,
            ReducerRouteOwnerGroup::HitPoint,
            ReducerRouteOwnerGroup::HitPoint,
        ),
        "doEnsnaringStrikeAfterHitRestraintTurnStartDamageAndEscape" => vec![
            route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
            route_discover_battle_acts_from_route_holes(
                ReducerRouteSubjectFamily::AfterHitDamageRider,
                vec![ReducerRouteHoleKind::AbilityCheck],
                ReducerRouteOwnerGroup::Concentration,
            ),
            route_resolve_battle_subject_from_route_result(
                ReducerRouteSubjectFamily::AfterHitDamageRider,
                ReducerRouteFillKind::AbilityCheck,
                ReducerRouteResolutionOutcome::Resolved,
                Vec::new(),
                ReducerRouteOwnerGroup::Concentration,
            ),
        ],
        "doFalseLifeTemporaryHitPoints" => expected_without_fill_route(
            ReducerRouteSubjectFamily::ScalarBuffEffect,
            ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
            ReducerRouteOwnerGroup::TemporaryHitPoint,
        ),
        "doHeroismFrightenedImmunityTurnStartTemporaryHitPoints" => {
            expected_marked_damage_immunity_route("doGrantTurnStartTemporaryHitPoints")
        }
        "doHeroismFrightenedImmunityTurnStartTemporaryHitPointsCleanup" => {
            expected_marked_damage_immunity_route("doCleanupConditionImmunityTemporaryHitPoints")
        }
        "doHuntersMarkMarkedDamageRiderConcentrationAndSameTurnTransfer" => {
            expected_marked_damage_immunity_route("doTransferMarkedDamageRiderSameTurn")
        }
        "doHexMarkedDamageRiderAndLaterTurnTransfer" => {
            expected_marked_damage_immunity_route("doTransferMarkedDamageRiderLaterTurn")
        }
        "doLongstriderSpeedIncrease" => expected_without_fill_route(
            ReducerRouteSubjectFamily::ScalarBuffEffect,
            ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
            ReducerRouteOwnerGroup::MovementResource,
        ),
        "doSearingSmiteAfterHitTimedDamageAndSaveCleanup" => vec![
            route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
            route_discover_battle_acts_from_route_holes(
                ReducerRouteSubjectFamily::AfterHitDamageRider,
                vec![
                    ReducerRouteHoleKind::RolledDice,
                    ReducerRouteHoleKind::SavingThrowOutcome,
                ],
                ReducerRouteOwnerGroup::ActiveEffect,
            ),
            route_resolve_battle_subject_from_route_result(
                ReducerRouteSubjectFamily::AfterHitDamageRider,
                ReducerRouteFillKind::RolledDice,
                ReducerRouteResolutionOutcome::NeedsHoles,
                vec![ReducerRouteHoleKind::SavingThrowOutcome],
                ReducerRouteOwnerGroup::HitPoint,
            ),
            route_resolve_battle_subject_from_route_result(
                ReducerRouteSubjectFamily::AfterHitDamageRider,
                ReducerRouteFillKind::SavingThrowOutcome,
                ReducerRouteResolutionOutcome::Resolved,
                Vec::new(),
                ReducerRouteOwnerGroup::ActiveEffect,
            ),
        ],
        "doShillelaghWeaponAttackOverride" => expected_damage_route(
            ReducerRouteSubjectFamily::HeldWeaponActiveEffect,
            ReducerRouteOwnerGroup::ActiveEffect,
            ReducerRouteOwnerGroup::HitPoint,
        ),
        "doTrueStrikeSpellHostedWeaponAttack" => expected_damage_route(
            ReducerRouteSubjectFamily::SpellHostedWeaponAttack,
            ReducerRouteOwnerGroup::HitPoint,
            ReducerRouteOwnerGroup::HitPoint,
        ),
        action => panic!("unsupported expected route mbt::actionTaken {action}"),
    }
}

fn expected_damage_route(
    subject: ReducerRouteSubjectFamily,
    discovery_owner: ReducerRouteOwnerGroup,
    resolution_owner: ReducerRouteOwnerGroup,
) -> Vec<ReducerRouteEvent> {
    vec![
        route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
        route_discover_battle_acts_from_route_holes(
            subject,
            vec![ReducerRouteHoleKind::RolledDice],
            discovery_owner,
        ),
        route_resolve_battle_subject_from_route_result(
            subject,
            ReducerRouteFillKind::RolledDice,
            ReducerRouteResolutionOutcome::Resolved,
            Vec::new(),
            resolution_owner,
        ),
    ]
}

fn expected_without_fill_route(
    subject: ReducerRouteSubjectFamily,
    discovery_owner: ReducerRouteOwnerGroup,
    resolution_owner: ReducerRouteOwnerGroup,
) -> Vec<ReducerRouteEvent> {
    vec![
        route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
        route_discover_battle_acts_from_route_holes(subject, Vec::new(), discovery_owner),
        route_resolve_battle_subject_without_fill_from_route_result(
            subject,
            ReducerRouteResolutionOutcome::Resolved,
            Vec::new(),
            resolution_owner,
        ),
    ]
}

#[must_use]
pub fn blocker_reason(observed_action_taken: &str) -> Option<&'static str> {
    BLOCKED_BRANCH_ACTIONS
        .iter()
        .find_map(|(action, reason)| (*action == observed_action_taken).then_some(*reason))
}

pub fn projection_payload(witness: &LevelOneSpellEffectWitness) -> String {
    [
        format!(
            "qDivineFavorActiveRiderCount={}",
            witness.divine_favor_active_rider_count
        ),
        format!("qTargetHp={}", witness.target_hp),
        format!(
            "qLongstriderTargetSpeedFeet={}",
            witness.longstrider_target_speed_feet
        ),
        format!(
            "qLongstriderSpeedEffectSourceSpellId={}",
            witness.longstrider_speed_effect_source_spell_id
        ),
        format!(
            "qLongstriderSpeedEffectTarget={}",
            witness.longstrider_speed_effect_target
        ),
        format!(
            "qLongstriderSpeedDeltaFeet={}",
            witness.longstrider_speed_delta_feet
        ),
        format!("qCasterTempHp={}", witness.caster_temp_hp),
        format!("qCasterFrightened={}", witness.caster_frightened),
        format!(
            "qSpellSlotSpentThisTurn={}",
            witness.spell_slot_spent_this_turn
        ),
        format!("qLevel1SlotsRemaining={}", witness.level1_slots_remaining),
        format!(
            "qDamageRiderSourceSpellId={}",
            witness.damage_rider_source_spell_id
        ),
        format!(
            "qDamageRiderDamageType={}",
            witness.damage_rider_damage_type
        ),
        format!("qDamageRiderDice={}", witness.damage_rider_dice),
        format!("qDamageRiderDieSize={}", witness.damage_rider_die_size),
        format!(
            "qTemporaryHitPointsSourceSpellId={}",
            witness.temporary_hit_points_source_spell_id
        ),
        format!(
            "qTemporaryHitPointsDice={}",
            witness.temporary_hit_points_dice
        ),
        format!(
            "qTemporaryHitPointsDieSize={}",
            witness.temporary_hit_points_die_size
        ),
        format!(
            "qTemporaryHitPointsFlat={}",
            witness.temporary_hit_points_flat
        ),
        format!(
            "qFrightenedImmunitySourceSpellId={}",
            witness.frightened_immunity_source_spell_id
        ),
        format!(
            "qFrightenedImmunityCondition={}",
            witness.frightened_immunity_condition
        ),
        format!(
            "qTurnStartTemporaryHitPointsSourceSpellId={}",
            witness.turn_start_temporary_hit_points_source_spell_id
        ),
        format!(
            "qTurnStartTemporaryHitPointsAmount={}",
            witness.turn_start_temporary_hit_points_amount
        ),
        format!(
            "qEnsnaringStrikeRestrainedBeforeEscape={}",
            witness.ensnaring_strike_restrained_before_escape
        ),
        format!("qTargetRestrained={}", witness.target_restrained),
        format!("qCasterConcentrating={}", witness.caster_concentrating),
        format!(
            "qEnsnaringStrikeSaveSourceSpellId={}",
            witness.ensnaring_strike_save_source_spell_id
        ),
        format!(
            "qEnsnaringStrikeSaveAbility={}",
            witness.ensnaring_strike_save_ability
        ),
        format!(
            "qTurnStartDamageSourceSpellId={}",
            witness.turn_start_damage_source_spell_id
        ),
        format!(
            "qTurnStartDamageDamageType={}",
            witness.turn_start_damage_damage_type
        ),
        format!("qTurnStartDamageDice={}", witness.turn_start_damage_dice),
        format!(
            "qTurnStartDamageDieSize={}",
            witness.turn_start_damage_die_size
        ),
        format!("qEscapeCheckAbility={}", witness.escape_check_ability),
        format!("qEscapeCheckSkill={}", witness.escape_check_skill),
        format!(
            "qHuntersMarkDamageHoleSourceSpellId={}",
            witness.hunters_mark_damage_hole_source_spell_id
        ),
        format!(
            "qHuntersMarkDamageHoleDamageType={}",
            witness.hunters_mark_damage_hole_damage_type
        ),
        format!(
            "qHuntersMarkDamageHoleDice={}",
            witness.hunters_mark_damage_hole_dice
        ),
        format!(
            "qHuntersMarkDamageHoleDieSize={}",
            witness.hunters_mark_damage_hole_die_size
        ),
        format!(
            "qHuntersMarkActiveMarkSourceSpellId={}",
            witness.hunters_mark_active_mark_source_spell_id
        ),
        format!(
            "qHuntersMarkActiveMarkTarget={}",
            witness.hunters_mark_active_mark_target
        ),
        format!(
            "qHuntersMarkConcentrationSourceSpellId={}",
            witness.hunters_mark_concentration_source_spell_id
        ),
        format!(
            "qHuntersMarkTransferKindOnDropTurn={}",
            witness.hunters_mark_transfer_kind_on_drop_turn
        ),
        format!(
            "qHuntersMarkActiveMarkTransferKind={}",
            witness.hunters_mark_active_mark_transfer_kind
        ),
        format!(
            "qHuntersMarkActiveMarkRetargetTiming={}",
            witness.hunters_mark_active_mark_retarget_timing
        ),
        format!(
            "qHuntersMarkTransferVisibleOnDropTurn={}",
            witness.hunters_mark_transfer_visible_on_drop_turn
        ),
        format!(
            "qHexDamageHoleSourceSpellId={}",
            witness.hex_damage_hole_source_spell_id
        ),
        format!(
            "qHexDamageHoleDamageType={}",
            witness.hex_damage_hole_damage_type
        ),
        format!("qHexDamageHoleDice={}", witness.hex_damage_hole_dice),
        format!("qHexDamageHoleDieSize={}", witness.hex_damage_hole_die_size),
        format!(
            "qHexActiveMarkSourceSpellId={}",
            witness.hex_active_mark_source_spell_id
        ),
        format!("qHexActiveMarkTarget={}", witness.hex_active_mark_target),
        format!(
            "qHexAbilityCheckAbility={}",
            witness.hex_ability_check_ability
        ),
        format!(
            "qHexTransferKindOnDropTurn={}",
            witness.hex_transfer_kind_on_drop_turn
        ),
        format!(
            "qHexActiveMarkTransferKind={}",
            witness.hex_active_mark_transfer_kind
        ),
        format!(
            "qHexActiveMarkRetargetTiming={}",
            witness.hex_active_mark_retarget_timing
        ),
        format!(
            "qHexTransferVisibleOnDropTurn={}",
            witness.hex_transfer_visible_on_drop_turn
        ),
        format!(
            "qSearingSmiteImmediateDamageSourceSpellId={}",
            witness.searing_smite_immediate_damage_source_spell_id
        ),
        format!(
            "qSearingSmiteImmediateDamageDamageType={}",
            witness.searing_smite_immediate_damage_damage_type
        ),
        format!(
            "qSearingSmiteImmediateDamageDice={}",
            witness.searing_smite_immediate_damage_dice
        ),
        format!(
            "qSearingSmiteImmediateDamageDieSize={}",
            witness.searing_smite_immediate_damage_die_size
        ),
        format!(
            "qSearingSmiteActiveBeforeSuccessfulSave={}",
            witness.searing_smite_active_before_successful_save
        ),
        format!(
            "qSearingSmiteTurnStartDamageSourceSpellId={}",
            witness.searing_smite_turn_start_damage_source_spell_id
        ),
        format!(
            "qSearingSmiteTurnStartDamageDamageType={}",
            witness.searing_smite_turn_start_damage_damage_type
        ),
        format!(
            "qSearingSmiteTurnStartDamageDice={}",
            witness.searing_smite_turn_start_damage_dice
        ),
        format!(
            "qSearingSmiteTurnStartDamageDieSize={}",
            witness.searing_smite_turn_start_damage_die_size
        ),
        format!(
            "qSearingSmiteTurnStartSaveSourceSpellId={}",
            witness.searing_smite_turn_start_save_source_spell_id
        ),
        format!(
            "qSearingSmiteTurnStartSaveAbility={}",
            witness.searing_smite_turn_start_save_ability
        ),
        format!(
            "qSearingSmiteTurnStartSaveSuccessEnds={}",
            witness.searing_smite_turn_start_save_success_ends
        ),
        format!(
            "qSearingSmiteActiveAfterSuccessfulSave={}",
            witness.searing_smite_active_after_successful_save
        ),
        format!(
            "qShillelaghOverrideSourceSpellId={}",
            witness.shillelagh_override_source_spell_id
        ),
        format!(
            "qShillelaghOverrideWeaponUnitId={}",
            witness.shillelagh_override_weapon_unit_id
        ),
        format!(
            "qShillelaghSpellcastingAbilityModifier={}",
            witness.shillelagh_spellcasting_ability_modifier
        ),
        format!(
            "qShillelaghOverrideAttackBonus={}",
            witness.shillelagh_override_attack_bonus
        ),
        format!(
            "qShillelaghOverrideDamageDice={}",
            witness.shillelagh_override_damage_dice
        ),
        format!(
            "qShillelaghOverrideDamageDieSize={}",
            witness.shillelagh_override_damage_die_size
        ),
        format!(
            "qShillelaghForceAttackName={}",
            witness.shillelagh_force_attack_name
        ),
        format!(
            "qShillelaghForceAttackBonus={}",
            witness.shillelagh_force_attack_bonus
        ),
        format!(
            "qShillelaghForceDamageType={}",
            witness.shillelagh_force_damage_type
        ),
        format!(
            "qShillelaghForceDamageDice={}",
            witness.shillelagh_force_damage_dice
        ),
        format!(
            "qShillelaghForceDamageDieSize={}",
            witness.shillelagh_force_damage_die_size
        ),
        format!(
            "qShillelaghForceDamageModifier={}",
            witness.shillelagh_force_damage_modifier
        ),
        format!(
            "qTrueStrikeSourceSpellId={}",
            witness.true_strike_source_spell_id
        ),
        format!(
            "qTrueStrikeComponentWeaponItemId={}",
            witness.true_strike_component_weapon_item_id
        ),
        format!(
            "qTrueStrikeWeaponUnitId={}",
            witness.true_strike_weapon_unit_id
        ),
        format!("qTrueStrikeAttackName={}", witness.true_strike_attack_name),
        format!(
            "qTrueStrikeAttackBonus={}",
            witness.true_strike_attack_bonus
        ),
        format!("qTrueStrikeDamageType={}", witness.true_strike_damage_type),
        format!("qTrueStrikeDamageDice={}", witness.true_strike_damage_dice),
        format!(
            "qTrueStrikeDamageDieSize={}",
            witness.true_strike_damage_die_size
        ),
        format!(
            "qTrueStrikeDamageModifier={}",
            witness.true_strike_damage_modifier
        ),
        format!("qScenarioOutcome={}", witness.scenario_outcome),
        format!("protocolResult={}", witness.protocol_result),
        format!("protocolHoles={}", joined_or_none(&witness.protocol_holes)),
    ]
    .join("\n")
}

fn witness_from_state(state: LevelOneSpellEffectState) -> LevelOneSpellEffectWitness {
    let projection = state.projection;
    let shillelagh = shillelagh_refs(projection.shillelagh);
    let true_strike = true_strike_refs(projection.true_strike);

    LevelOneSpellEffectWitness {
        divine_favor_active_rider_count: projection.divine_favor_active_rider_count,
        target_hp: projection.target_hit_points,
        longstrider_target_speed_feet: projection.longstrider.target_speed_feet,
        longstrider_speed_effect_source_spell_id: spell_ref(projection.longstrider.spell),
        longstrider_speed_effect_target: target_ref(projection.longstrider.target),
        longstrider_speed_delta_feet: projection.longstrider.delta_feet,
        caster_temp_hp: projection.caster_temporary_hit_points,
        caster_frightened: projection.caster_frightened,
        spell_slot_spent_this_turn: projection.spell_slot_spent_this_turn,
        level1_slots_remaining: projection.level_one_slots_remaining,
        damage_rider_source_spell_id: spell_ref(projection.damage_rider.spell),
        damage_rider_damage_type: damage_type_ref(projection.damage_rider.damage_type),
        damage_rider_dice: projection.damage_rider.dice.dice,
        damage_rider_die_size: projection.damage_rider.dice.die_size,
        temporary_hit_points_source_spell_id: spell_ref(projection.temporary_hit_points.spell),
        temporary_hit_points_dice: projection.temporary_hit_points.dice.dice,
        temporary_hit_points_die_size: projection.temporary_hit_points.dice.die_size,
        temporary_hit_points_flat: projection.temporary_hit_points.flat,
        frightened_immunity_source_spell_id: spell_ref(projection.frightened_immunity.spell),
        frightened_immunity_condition: condition_ref(projection.frightened_immunity.condition),
        turn_start_temporary_hit_points_source_spell_id: spell_ref(
            projection.turn_start_temporary_hit_points.spell,
        ),
        turn_start_temporary_hit_points_amount: projection.turn_start_temporary_hit_points.amount,
        ensnaring_strike_restrained_before_escape: projection
            .ensnaring_strike
            .restrained_before_escape,
        target_restrained: projection.ensnaring_strike.target_restrained,
        caster_concentrating: projection.caster_concentrating,
        ensnaring_strike_save_source_spell_id: spell_ref(projection.ensnaring_strike.save_spell),
        ensnaring_strike_save_ability: saving_throw_ability_ref(
            projection.ensnaring_strike.save_ability,
        ),
        turn_start_damage_source_spell_id: spell_ref(
            projection.ensnaring_strike.turn_start_damage.spell,
        ),
        turn_start_damage_damage_type: damage_type_ref(
            projection.ensnaring_strike.turn_start_damage.damage_type,
        ),
        turn_start_damage_dice: projection.ensnaring_strike.turn_start_damage.dice.dice,
        turn_start_damage_die_size: projection.ensnaring_strike.turn_start_damage.dice.die_size,
        escape_check_ability: ability_score_ref(projection.ensnaring_strike.escape_ability),
        escape_check_skill: skill_ref(projection.ensnaring_strike.escape_skill),
        hunters_mark_damage_hole_source_spell_id: spell_ref(projection.hunters_mark.damage.spell),
        hunters_mark_damage_hole_damage_type: damage_type_ref(
            projection.hunters_mark.damage.damage_type,
        ),
        hunters_mark_damage_hole_dice: projection.hunters_mark.damage.dice.dice,
        hunters_mark_damage_hole_die_size: projection.hunters_mark.damage.dice.die_size,
        hunters_mark_active_mark_source_spell_id: spell_ref(
            projection.hunters_mark.active_mark_spell,
        ),
        hunters_mark_active_mark_target: target_ref(projection.hunters_mark.active_mark_target),
        hunters_mark_concentration_source_spell_id: spell_ref(
            projection.hunters_mark.concentration_spell,
        ),
        hunters_mark_transfer_kind_on_drop_turn: transfer_availability_ref(
            projection.hunters_mark.transfer_on_drop_turn,
        ),
        hunters_mark_active_mark_transfer_kind: active_mark_transfer_ref(
            projection.hunters_mark.active_mark_transfer,
        ),
        hunters_mark_active_mark_retarget_timing: retarget_timing_ref(
            projection.hunters_mark.retarget_timing,
        ),
        hunters_mark_transfer_visible_on_drop_turn: projection
            .hunters_mark
            .transfer_visible_on_drop_turn,
        hex_damage_hole_source_spell_id: spell_ref(projection.hex.damage.spell),
        hex_damage_hole_damage_type: damage_type_ref(projection.hex.damage.damage_type),
        hex_damage_hole_dice: projection.hex.damage.dice.dice,
        hex_damage_hole_die_size: projection.hex.damage.dice.die_size,
        hex_active_mark_source_spell_id: spell_ref(projection.hex.active_mark_spell),
        hex_active_mark_target: target_ref(projection.hex.active_mark_target),
        hex_ability_check_ability: ability_score_ref(projection.hex.ability_check_ability),
        hex_transfer_kind_on_drop_turn: transfer_availability_ref(
            projection.hex.transfer_on_drop_turn,
        ),
        hex_active_mark_transfer_kind: active_mark_transfer_ref(
            projection.hex.active_mark_transfer,
        ),
        hex_active_mark_retarget_timing: retarget_timing_ref(projection.hex.retarget_timing),
        hex_transfer_visible_on_drop_turn: projection.hex.transfer_visible_on_drop_turn,
        searing_smite_immediate_damage_source_spell_id: spell_ref(
            projection.searing_smite.immediate_damage.spell,
        ),
        searing_smite_immediate_damage_damage_type: damage_type_ref(
            projection.searing_smite.immediate_damage.damage_type,
        ),
        searing_smite_immediate_damage_dice: projection.searing_smite.immediate_damage.dice.dice,
        searing_smite_immediate_damage_die_size: projection
            .searing_smite
            .immediate_damage
            .dice
            .die_size,
        searing_smite_active_before_successful_save: projection
            .searing_smite
            .active_before_successful_save,
        searing_smite_turn_start_damage_source_spell_id: spell_ref(
            projection.searing_smite.turn_start_damage.spell,
        ),
        searing_smite_turn_start_damage_damage_type: damage_type_ref(
            projection.searing_smite.turn_start_damage.damage_type,
        ),
        searing_smite_turn_start_damage_dice: projection.searing_smite.turn_start_damage.dice.dice,
        searing_smite_turn_start_damage_die_size: projection
            .searing_smite
            .turn_start_damage
            .dice
            .die_size,
        searing_smite_turn_start_save_source_spell_id: spell_ref(
            projection.searing_smite.turn_start_save_spell,
        ),
        searing_smite_turn_start_save_ability: saving_throw_ability_ref(
            projection.searing_smite.turn_start_save_ability,
        ),
        searing_smite_turn_start_save_success_ends: save_success_ending_ref(
            projection.searing_smite.successful_save_ending,
        ),
        searing_smite_active_after_successful_save: projection
            .searing_smite
            .active_after_successful_save,
        shillelagh_override_source_spell_id: shillelagh.spell,
        shillelagh_override_weapon_unit_id: shillelagh.weapon,
        shillelagh_spellcasting_ability_modifier: projection
            .shillelagh
            .spellcasting_ability_modifier,
        shillelagh_override_attack_bonus: projection.shillelagh.override_attack_bonus,
        shillelagh_override_damage_dice: projection.shillelagh.override_damage.dice,
        shillelagh_override_damage_die_size: projection.shillelagh.override_damage.die_size,
        shillelagh_force_attack_name: shillelagh.attack,
        shillelagh_force_attack_bonus: projection.shillelagh.attack_bonus,
        shillelagh_force_damage_type: damage_type_ref(projection.shillelagh.damage_type),
        shillelagh_force_damage_dice: projection.shillelagh.damage.dice,
        shillelagh_force_damage_die_size: projection.shillelagh.damage.die_size,
        shillelagh_force_damage_modifier: projection.shillelagh.damage_modifier,
        true_strike_source_spell_id: true_strike.spell,
        true_strike_component_weapon_item_id: true_strike.component_weapon,
        true_strike_weapon_unit_id: true_strike.weapon,
        true_strike_attack_name: true_strike.attack,
        true_strike_attack_bonus: projection.true_strike.attack_bonus,
        true_strike_damage_type: damage_type_ref(projection.true_strike.damage_type),
        true_strike_damage_dice: projection.true_strike.damage.dice,
        true_strike_damage_die_size: projection.true_strike.damage.die_size,
        true_strike_damage_modifier: projection.true_strike.damage_modifier,
        scenario_outcome: scenario_outcome_ref(state.scenario_outcome),
        protocol_result: protocol_result_ref(state.protocol),
        protocol_holes: protocol_holes(state.protocol),
    }
}

struct WeaponAttackRefs {
    spell: &'static str,
    weapon: &'static str,
    component_weapon: &'static str,
    attack: &'static str,
}

fn shillelagh_refs(effect: WeaponAttackOverrideEffect) -> WeaponAttackRefs {
    WeaponAttackRefs {
        spell: spell_ref(effect.spell),
        weapon: weapon_ref(effect.weapon),
        component_weapon: "none",
        attack: attack_ref(effect.attack_presentation),
    }
}

fn true_strike_refs(effect: SpellHostedWeaponAttackEffect) -> WeaponAttackRefs {
    WeaponAttackRefs {
        spell: spell_ref(effect.spell),
        weapon: weapon_ref(effect.weapon),
        component_weapon: component_weapon_ref(effect.component_weapon),
        attack: attack_ref(effect.attack_presentation),
    }
}

fn spell_ref(spell: Option<LevelOneSpell>) -> &'static str {
    match spell {
        Some(LevelOneSpell::DivineFavor) => "divine_favor",
        Some(LevelOneSpell::DivineSmite) => "divine_smite",
        Some(LevelOneSpell::EnsnaringStrike) => "ensnaring_strike",
        Some(LevelOneSpell::FalseLife) => "false_life",
        Some(LevelOneSpell::Heroism) => "heroism",
        Some(LevelOneSpell::HuntersMark) => "hunters_mark",
        Some(LevelOneSpell::Hex) => "hex",
        Some(LevelOneSpell::Longstrider) => "longstrider",
        Some(LevelOneSpell::SearingSmite) => "searing_smite",
        Some(LevelOneSpell::Shillelagh) => "shillelagh",
        Some(LevelOneSpell::TrueStrike) => "true_strike",
        None => "none",
    }
}

fn damage_type_ref(damage_type: Option<DamageType>) -> &'static str {
    match damage_type {
        Some(DamageType::Fire) => "fire",
        Some(DamageType::Force) => "force",
        Some(DamageType::Lightning) => "lightning",
        Some(DamageType::Necrotic) => "necrotic",
        Some(DamageType::Piercing) => "piercing",
        Some(DamageType::Radiant) => "radiant",
        None => "none",
    }
}

fn target_ref(target: LevelOneEffectTarget) -> &'static str {
    match target {
        LevelOneEffectTarget::None => "none",
        LevelOneEffectTarget::Target => "target",
        LevelOneEffectTarget::TransferTarget => "transferTarget",
    }
}

fn condition_ref(condition: Option<LevelOneCondition>) -> &'static str {
    match condition {
        Some(LevelOneCondition::Frightened) => "frightened",
        None => "none",
    }
}

fn saving_throw_ability_ref(ability: Option<SavingThrowAbility>) -> &'static str {
    match ability {
        Some(SavingThrowAbility::Strength) => "str",
        Some(SavingThrowAbility::Dexterity) => "dex",
        Some(SavingThrowAbility::Constitution) => "con",
        None => "none",
    }
}

fn ability_score_ref(ability: Option<AbilityScore>) -> &'static str {
    match ability {
        Some(AbilityScore::Strength) => "str",
        Some(AbilityScore::Wisdom) => "wis",
        None => "none",
    }
}

fn skill_ref(skill: Option<AbilityCheckSkill>) -> &'static str {
    match skill {
        Some(AbilityCheckSkill::Athletics) => "athletics",
        None => "none",
    }
}

fn transfer_availability_ref(availability: MarkTransferAvailability) -> &'static str {
    match availability {
        MarkTransferAvailability::None => "none",
        MarkTransferAvailability::Available => "available",
        MarkTransferAvailability::AvailableAfterTurn => "availableAfterTurn",
    }
}

fn active_mark_transfer_ref(transfer: ActiveMarkTransfer) -> &'static str {
    match transfer {
        ActiveMarkTransfer::None => "none",
        ActiveMarkTransfer::AwaitingTargetDrop => "awaitingTargetDrop",
    }
}

fn retarget_timing_ref(timing: MarkRetargetTiming) -> &'static str {
    match timing {
        MarkRetargetTiming::None => "none",
        MarkRetargetTiming::SameTurn => "sameTurn",
        MarkRetargetTiming::LaterTurn => "laterTurn",
    }
}

fn save_success_ending_ref(ending: SaveSuccessEnding) -> &'static str {
    match ending {
        SaveSuccessEnding::None => "none",
        SaveSuccessEnding::Spell => "spell",
    }
}

fn weapon_ref(weapon: SpellWeapon) -> &'static str {
    match weapon {
        SpellWeapon::None => "none",
        SpellWeapon::Quarterstaff => "weapon_quarterstaff",
        SpellWeapon::Dagger => "weapon_dagger",
    }
}

fn component_weapon_ref(component: SpellWeaponComponent) -> &'static str {
    match component {
        SpellWeaponComponent::None => "none",
        SpellWeaponComponent::MainDagger => "main:weapon_dagger",
    }
}

fn attack_ref(attack: WeaponAttackPresentation) -> &'static str {
    match attack {
        WeaponAttackPresentation::None => "none",
        WeaponAttackPresentation::QuarterstaffForce => "Quarterstaff (force)",
        WeaponAttackPresentation::Dagger => "Dagger",
    }
}

fn scenario_outcome_ref(outcome: LevelOneSpellEffectScenarioOutcome) -> &'static str {
    match outcome {
        LevelOneSpellEffectScenarioOutcome::Init => "Init",
        LevelOneSpellEffectScenarioOutcome::DivineFavor => "DivineFavor",
        LevelOneSpellEffectScenarioOutcome::DivineSmite => "DivineSmite",
        LevelOneSpellEffectScenarioOutcome::EnsnaringStrike => "EnsnaringStrike",
        LevelOneSpellEffectScenarioOutcome::FalseLife => "FalseLife",
        LevelOneSpellEffectScenarioOutcome::Heroism => "Heroism",
        LevelOneSpellEffectScenarioOutcome::HuntersMark => "HuntersMark",
        LevelOneSpellEffectScenarioOutcome::Hex => "Hex",
        LevelOneSpellEffectScenarioOutcome::Longstrider => "Longstrider",
        LevelOneSpellEffectScenarioOutcome::SearingSmite => "SearingSmite",
        LevelOneSpellEffectScenarioOutcome::Shillelagh => "Shillelagh",
        LevelOneSpellEffectScenarioOutcome::TrueStrike => "TrueStrike",
    }
}

fn protocol_result_ref(protocol: LevelOneSpellEffectProtocol) -> &'static str {
    match protocol {
        LevelOneSpellEffectProtocol::Init => "init",
        LevelOneSpellEffectProtocol::Resolved => "resolved",
    }
}

fn protocol_holes(protocol: LevelOneSpellEffectProtocol) -> Vec<&'static str> {
    match protocol {
        LevelOneSpellEffectProtocol::Init | LevelOneSpellEffectProtocol::Resolved => vec![],
    }
}

fn joined_or_none(values: &[&'static str]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}
