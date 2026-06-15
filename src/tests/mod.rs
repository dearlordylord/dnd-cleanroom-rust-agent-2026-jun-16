#[path = "../qnt_adapters/battle_runtime_adrenaline_rush.rs"]
mod battle_runtime_adrenaline_rush;
#[path = "../qnt_adapters/battle_runtime_attack_spell_shape_selected_identity.rs"]
mod battle_runtime_attack_spell_shape_selected_identity;
#[path = "../qnt_adapters/battle_runtime_chained_attack_sequence.rs"]
mod battle_runtime_chained_attack_sequence;
#[path = "../qnt_adapters/battle_runtime_command_option_next_turn.rs"]
mod battle_runtime_command_option_next_turn;
#[path = "../qnt_adapters/battle_runtime_command_ordering.rs"]
mod battle_runtime_command_ordering;
#[path = "../qnt_adapters/battle_runtime_concentration_break_teardown.rs"]
mod battle_runtime_concentration_break_teardown;
#[path = "../qnt_adapters/battle_runtime_creature_type_protection_and_charm_selected_identity.rs"]
mod battle_runtime_creature_type_protection_and_charm_selected_identity;
#[path = "../qnt_adapters/battle_runtime_danger_sense_selected_identity.rs"]
mod battle_runtime_danger_sense_selected_identity;
#[path = "../qnt_adapters/battle_runtime_death_saving_throw.rs"]
mod battle_runtime_death_saving_throw;
#[path = "../qnt_adapters/battle_runtime_dragonborn_breath_weapon.rs"]
mod battle_runtime_dragonborn_breath_weapon;
#[path = "../qnt_adapters/battle_runtime_druid_wild_shape_form_lifecycle.rs"]
mod battle_runtime_druid_wild_shape_form_lifecycle;
#[path = "../qnt_adapters/character_battle_origin_feat_selected_identity.rs"]
mod character_battle_origin_feat_selected_identity;
#[path = "../qnt_adapters/character_creation_class_feature_projections.rs"]
mod character_creation_class_feature_projections;
#[path = "../qnt_adapters/character_creation_cleric_druid_order_selected_identity.rs"]
mod character_creation_cleric_druid_order_selected_identity;
#[path = "../qnt_adapters/character_creation_fighter_fighting_style_selected_identity.rs"]
mod character_creation_fighter_fighting_style_selected_identity;
#[path = "../qnt_adapters/character_creation_runtime.rs"]
mod character_creation_runtime;
#[path = "../qnt_adapters/character_creation_weapon_mastery_containers_selected_identity.rs"]
mod character_creation_weapon_mastery_containers_selected_identity;
#[path = "../qnt_adapters/character_sheet_ability_check_proficiency_bonus.rs"]
mod character_sheet_ability_check_proficiency_bonus;
#[path = "../qnt_adapters/character_sheet_armor_class_base_selected_identity.rs"]
mod character_sheet_armor_class_base_selected_identity;
#[path = "../qnt_adapters/character_sheet_healing_resource_selected_identity.rs"]
mod character_sheet_healing_resource_selected_identity;
#[path = "../qnt_adapters/character_sheet_hit_point_maximum.rs"]
mod character_sheet_hit_point_maximum;
#[path = "../qnt_adapters/character_sheet_hp_rest_hit_dice.rs"]
mod character_sheet_hp_rest_hit_dice;
#[path = "../qnt_adapters/character_sheet_spellbook_ritual_selected_identity.rs"]
mod character_sheet_spellbook_ritual_selected_identity;
#[path = "../qnt_adapters/character_sheet_weapon_mastery_containers_selected_identity.rs"]
mod character_sheet_weapon_mastery_containers_selected_identity;

use crate::rules::ability_checks::{
    ability_check_proficiency_bonus, AbilityCheckProficiencyBonusKind, AbilityCheckSkillTraining,
    JackOfAllTradesState, OtherProficiencyBonusState,
};
use crate::rules::armor_class::{
    armor_class_projection, ArmorClassAbility, ArmorClassFacts, ArmorClassFormula,
    ArmorClassOption, ShieldArmorClassBonus,
};
use crate::rules::battle_features::{
    danger_sense_initial_state, danger_sense_saving_throw_roll_mode,
    dragonborn_breath_weapon_damage_die_count, dragonborn_breath_weapon_initial_state,
    dragonborn_breath_weapon_save_dc, project_danger_sense_dexterity_advantage,
    reject_dragonborn_breath_weapon_invalid_damage_roll,
    reject_dragonborn_breath_weapon_mismatched_area,
    reject_dragonborn_breath_weapon_missing_resource, resolve_adrenaline_rush_bonus_action_dash,
    resolve_dragonborn_breath_weapon, suppress_danger_sense_while_incapacitated,
    AdrenalineRushFacts, AdrenalineRushRejection, AdrenalineRushResult, BattleTurnFeatureState,
    BreathWeaponAreaShape, DangerSenseFacts, DangerSenseProtocol, DangerSenseScenarioOutcome,
    DragonbornBreathWeaponFacts, DragonbornBreathWeaponInvalidReason,
    DragonbornBreathWeaponProtocol, DragonbornBreathWeaponScenarioOutcome,
    PassiveSavingThrowRollMode, SavingThrowAbility,
};
use crate::rules::chained_spell_attacks::{
    choose_chromatic_orb_damage_type, choose_chromatic_orb_initial_target,
    choose_chromatic_orb_leap_target, chromatic_orb_can_leap, chromatic_orb_damage_die_count,
    chromatic_orb_damage_die_size, chromatic_orb_initial_state, resolve_chromatic_orb_attack_hit,
    resolve_chromatic_orb_damage, start_chromatic_orb_cast, ChainedAttackHole,
    ChainedAttackProtocol, ChainedAttackRollFacts, ChainedAttackScenarioOutcome, ChainedAttackStep,
    ChainedDamageRollFacts, ChainedDamageTypeChoice, ChainedTarget, ChromaticOrbSequenceState,
};
use crate::rules::class_features::{
    apply_weapon_mastery_long_rest_reselection, cleric_divine_order_projection,
    druid_primal_order_projection, fighter_fighting_style_projection, level_two_feature_projection,
    weapon_mastery_projection, Cantrip, ClassLevel, ClericDivineOrder, DruidPrimalOrder,
    FeatureSet, FighterFightingStyleSelection, FightingStyleFeat, MetamagicOption, ProjectionError,
    Weapon, WeaponMasteryClass, WeaponMasteryReselectionFacts,
};
use crate::rules::command_options::{
    cleanup_command_halt_turn, command_after_saving_throw_stage, command_fill_order_result,
    command_hole_frontier, command_hole_frontier_includes_table_facts,
    command_next_turn_initial_state, command_ordering_initial_state,
    decline_command_flee_opportunity_attack, follow_command_grovel,
    open_command_flee_opportunity_attack_window, record_command_failed_save_pending,
    reject_command_approach_movement, suppress_command_halt, CommandFillKind,
    CommandFillOrderResult, CommandFillOrderingError, CommandFrontierStage, CommandHoleKind,
    CommandNextTurnInvalidReason, CommandNextTurnOption, CommandNextTurnProtocol,
    CommandNextTurnScenario, CommandOrderingFillFacts, CommandOrderingProtocol, CommandTurnActor,
};
use crate::rules::concentration::{
    cast_concentration_spell, cast_replacement_concentration_spell, concentration_initial_state,
    concentration_save_dc_for_damage, fail_concentration_saving_throw,
    request_concentration_save_after_damage, voluntarily_end_concentration,
    ConcentrationBreakScenario, ConcentrationDamageFacts, ConcentrationProtocol,
    ConcentrationSavingThrowError, ConcentrationSavingThrowFacts,
};
use crate::rules::creature_type_protection::{
    creature_type_protection_initial_state, discover_animal_friendship_target_admission,
    project_protection_condition_and_possession_prevention, project_protection_scoped_attack_roll,
    protection_condition_is_prevented, protection_from_evil_and_good_creature_type_is_scoped,
    protection_prevents_condition_from_creature_type,
    protection_prevents_possession_from_creature_type, protection_relevant_effect_save_roll_mode,
    resolve_animal_friendship_damage_break, resolve_animal_friendship_failed_save,
    resolve_protection_from_evil_and_good_target, resolve_protection_relevant_charm_save,
    save_condition_target_creature_type_is_legal,
    spell_attack_roll_mode_from_creature_type_protection, ProtectedCondition,
    ProtectionAttackRollMode, ProtectionSavingThrowMode, RuntimeCreatureType,
    SaveGatedConditionSpell,
};
use crate::rules::feature_resources::{
    apply_lay_on_hands_resource, apply_temporary_hit_points, FeatureResourceHitPoints,
    ResourcePoolError, ResourcePoolFacts,
};
use crate::rules::hit_points::{
    complete_long_rest_benefits, death_saving_throw_initial_state, discover_death_saving_throw,
    fill_death_saving_throw, fixed_higher_level_hit_point_gain, hit_point_maximum_projection,
    reject_wrong_actor_after_resolved, spend_hit_point_die, DeathSavingThrowFacts,
    DeathSavingThrowInvalidReason, DeathSavingThrowProtocol, DeathSavingThrowTurnRole,
    HitPointMaximumFacts, SheetHitPointState,
};
use crate::rules::origin_feats::{
    criminal_origin_feat_projection, initiative_handoff_projection, AlertInitiativeState,
    Background, InitiativeHandoffFacts, OriginFeat,
};
use crate::rules::spell_shapes::{
    resolve_save_gated_spell_damage, resolve_spell_attack_hit, AttackSpellHitFacts,
    AttackSpellProfile, SaveGatedSpellDamageFacts, SaveGatedSpellProfile, SpellActiveEffect,
    SpellShapeOutcome, SpellShapeState,
};
use crate::rules::spellbook_rituals::{
    can_cast_spellbook_ritual, spellbook_ritual_invocation, PreparationRequirement,
    RequiredSpellAccess, SpellSlotCost, SpellbookRitualAccess, SpellbookRitualAdept,
    SpellbookRitualFacts, SpellbookRitualResult, SpellbookSpellKind,
};
use crate::rules::wild_shape::{
    assume_riding_horse_wild_shape, begin_wild_shape_next_turn, dismiss_wild_shape_form,
    reuse_wild_shape_as_cat, revert_wild_shape_due_to_death,
    revert_wild_shape_due_to_incapacitated, stutter_wild_shape_state, wild_shape_initial_state,
    WildShapeCreatureSize, WildShapeDruidStatus, WildShapeForm, WildShapeProtocol,
    WildShapeScenarioOutcome,
};

use battle_runtime_adrenaline_rush::{
    expected_witness as expected_adrenaline_rush_witness,
    projection_payload as adrenaline_rush_projection_payload,
    replay_observed_action as replay_adrenaline_rush_action,
    BRANCH_ACTIONS as ADRENALINE_RUSH_BRANCH_ACTIONS,
};
use battle_runtime_attack_spell_shape_selected_identity::{
    expected_witness as expected_attack_spell_shape_witness,
    projection_payload as attack_spell_shape_projection_payload,
    replay_observed_action as replay_attack_spell_shape_action,
    BRANCH_ACTIONS as ATTACK_SPELL_SHAPE_BRANCH_ACTIONS,
};
use battle_runtime_chained_attack_sequence::{
    expected_witness as expected_chained_attack_sequence_witness,
    projection_payload as chained_attack_sequence_projection_payload,
    replay_observed_action as replay_chained_attack_sequence_action,
    BRANCH_ACTIONS as CHAINED_ATTACK_SEQUENCE_BRANCH_ACTIONS,
};
use battle_runtime_command_option_next_turn::{
    expected_witness as expected_command_option_next_turn_witness,
    projection_payload as command_option_next_turn_projection_payload,
    replay_observed_action as replay_command_option_next_turn_action,
    BRANCH_ACTIONS as COMMAND_OPTION_NEXT_TURN_BRANCH_ACTIONS,
};
use battle_runtime_command_ordering::{
    expected_witness as expected_command_ordering_witness,
    projection_payload as command_ordering_projection_payload,
    replay_observed_action as replay_command_ordering_action,
    BRANCH_ACTIONS as COMMAND_ORDERING_BRANCH_ACTIONS,
};
use battle_runtime_concentration_break_teardown::{
    expected_witness as expected_concentration_break_teardown_witness,
    projection_payload as concentration_break_teardown_projection_payload,
    replay_observed_action as replay_concentration_break_teardown_action, sampled_damage_total,
    BRANCH_ACTIONS as CONCENTRATION_BREAK_TEARDOWN_BRANCH_ACTIONS,
};
use battle_runtime_creature_type_protection_and_charm_selected_identity::{
    expected_witness as expected_creature_type_protection_witness,
    projection_payload as creature_type_protection_projection_payload,
    replay_observed_action as replay_creature_type_protection_action,
    BRANCH_ACTIONS as CREATURE_TYPE_PROTECTION_BRANCH_ACTIONS,
};
use battle_runtime_danger_sense_selected_identity::{
    expected_witness as expected_danger_sense_witness,
    projection_payload as danger_sense_projection_payload,
    replay_observed_action as replay_danger_sense_action,
    BRANCH_ACTIONS as DANGER_SENSE_BRANCH_ACTIONS,
};
use battle_runtime_death_saving_throw::{
    expected_witness as expected_death_saving_throw_witness,
    projection_payload as death_saving_throw_projection_payload, replay_fill_death_saving_throw,
    replay_observed_action as replay_death_saving_throw_action,
    BRANCH_ACTIONS as DEATH_SAVING_THROW_BRANCH_ACTIONS, FILL_SAMPLE_NATURAL_D20S,
};
use battle_runtime_dragonborn_breath_weapon::{
    expected_witness as expected_dragonborn_breath_weapon_witness,
    projection_payload as dragonborn_breath_weapon_projection_payload,
    replay_observed_action as replay_dragonborn_breath_weapon_action,
    BRANCH_ACTIONS as DRAGONBORN_BREATH_WEAPON_BRANCH_ACTIONS,
};
use battle_runtime_druid_wild_shape_form_lifecycle::{
    expected_witness as expected_druid_wild_shape_witness,
    projection_payload as druid_wild_shape_projection_payload,
    replay_observed_action as replay_druid_wild_shape_action,
    BRANCH_ACTIONS as DRUID_WILD_SHAPE_BRANCH_ACTIONS,
};
use character_battle_origin_feat_selected_identity::{
    expected_witness as expected_origin_feat_witness,
    projection_payload as origin_feat_projection_payload,
    replay_observed_action as replay_origin_feat_action,
    BRANCH_ACTIONS as ORIGIN_FEAT_BRANCH_ACTIONS,
};
use character_creation_class_feature_projections::{
    expected_monk_witness, expected_sorcerer_witness, projection_payload, replay_observed_action,
};
use character_creation_cleric_druid_order_selected_identity::{
    expected_cleric_protector_witness, expected_cleric_thaumaturge_witness,
    expected_druid_magician_witness, expected_druid_warden_witness,
    projection_payload as order_projection_payload, replay_observed_action as replay_order_action,
};
use character_creation_fighter_fighting_style_selected_identity::{
    expected_replace_defense_with_archery_witness, expected_select_defense_witness,
    projection_payload as fighting_style_projection_payload,
    replay_observed_action as replay_fighting_style_action,
};
use character_creation_runtime::{
    projection_payload as runtime_projection_payload,
    replay_observed_action as replay_runtime_action, BRANCH_ACTIONS,
};
use character_creation_weapon_mastery_containers_selected_identity::{
    expected_barbarian_witness, expected_fighter_witness, expected_paladin_witness,
    expected_ranger_witness, expected_rogue_witness,
    projection_payload as weapon_mastery_projection_payload,
    replay_observed_action as replay_weapon_mastery_action,
};
use character_sheet_ability_check_proficiency_bonus::{
    expected_expertise_witness, expected_jack_of_all_trades_level_two_witness,
    expected_jack_of_all_trades_rounded_down_witness, expected_missing_bard_level_two_witness,
    expected_other_proficiency_bonus_witness, expected_skill_proficiency_witness,
    projection_payload as ability_check_projection_payload,
    replay_observed_action as replay_ability_check_action,
};
use character_sheet_armor_class_base_selected_identity::{
    expected_barbarian_unarmored_defense_with_shield_witness,
    expected_barbarian_unarmored_defense_witness, expected_heavy_armor_with_shield_witness,
    expected_light_armor_witness, expected_medium_armor_dex_cap_witness,
    expected_monk_unarmored_defense_witness, projection_payload as armor_class_projection_payload,
    replay_observed_action as replay_armor_class_action,
};
use character_sheet_healing_resource_selected_identity::{
    expected_lay_on_hands_witness, projection_payload as healing_resource_projection_payload,
    replay_observed_action as replay_healing_resource_action,
};
use character_sheet_hit_point_maximum::{
    expected_witness as expected_hit_point_maximum_witness,
    projection_payload as hit_point_maximum_projection_payload,
    replay_observed_action as replay_hit_point_maximum_action,
    BRANCH_ACTIONS as HIT_POINT_MAXIMUM_BRANCH_ACTIONS,
};
use character_sheet_hp_rest_hit_dice::{
    expected_witness as expected_hp_rest_hit_dice_witness,
    projection_payload as hp_rest_hit_dice_projection_payload,
    replay_observed_action as replay_hp_rest_hit_dice_action,
    BRANCH_ACTIONS as HP_REST_HIT_DICE_BRANCH_ACTIONS,
};
use character_sheet_spellbook_ritual_selected_identity::{
    expected_witness as expected_spellbook_ritual_witness,
    projection_payload as spellbook_ritual_projection_payload,
    replay_observed_action as replay_spellbook_ritual_action,
    BRANCH_ACTIONS as SPELLBOOK_RITUAL_BRANCH_ACTIONS,
};
use character_sheet_weapon_mastery_containers_selected_identity::{
    expected_witness as expected_sheet_weapon_mastery_witness,
    projection_payload as sheet_weapon_mastery_projection_payload,
    replay_observed_action as replay_sheet_weapon_mastery_action,
    BRANCH_ACTIONS as SHEET_WEAPON_MASTERY_BRANCH_ACTIONS,
};

#[test]
fn class_feature_projection_adapter_replays_monk_branch() {
    // QNT: cleanroom-input/qnt/character-creation-runtime/
    // character-creation-class-feature-projections.mbt.qnt,
    // action doProjectMonkFocusAndUncannyMetabolism.
    let observed = replay_observed_action("doProjectMonkFocusAndUncannyMetabolism");

    assert_eq!(observed, expected_monk_witness());
    assert!(projection_payload(&observed).contains("resourceMaximum=2"));
    assert!(projection_payload(&observed).contains("martialArtsDieSize=6"));
}

#[test]
fn class_feature_projection_adapter_replays_sorcerer_branch() {
    // QNT: cleanroom-input/qnt/character-creation-runtime/
    // character-creation-class-feature-projections.mbt.qnt,
    // action doProjectSorcererFontAndMetamagic.
    let observed = replay_observed_action("doProjectSorcererFontAndMetamagic");

    assert_eq!(observed, expected_sorcerer_witness());
    assert!(projection_payload(&observed).contains("metamagicChoiceCount=2"));
    assert!(projection_payload(&observed).contains("secondMetamagicSorceryPointCost=2"));
}

#[test]
fn sorcerer_metamagic_projection_rejects_duplicate_options() {
    let result = level_two_feature_projection(FeatureSet::SorcererLevelTwo {
        metamagic_options: [
            MetamagicOption::EmpoweredSpell,
            MetamagicOption::EmpoweredSpell,
        ],
    });

    assert_eq!(result, Err(ProjectionError::DuplicateMetamagicOption));
}

#[test]
fn cleric_druid_order_adapter_replays_all_selected_identity_branches() {
    // QNT: cleanroom-input/qnt/character-creation-runtime/
    // character-creation-cleric-druid-order-selected-identity.mbt.qnt.
    let cleric_protector = replay_order_action("doSelectClericProtectorOrder");
    let cleric_thaumaturge = replay_order_action("doSelectClericThaumaturgeOrder");
    let druid_magician = replay_order_action("doSelectDruidMagicianOrder");
    let druid_warden = replay_order_action("doSelectDruidWardenOrder");

    assert_eq!(cleric_protector, expected_cleric_protector_witness());
    assert_eq!(cleric_thaumaturge, expected_cleric_thaumaturge_witness());
    assert_eq!(druid_magician, expected_druid_magician_witness());
    assert_eq!(druid_warden, expected_druid_warden_witness());
    assert!(order_projection_payload(&cleric_protector)
        .contains("martialWeaponProficiencyPresent=true"));
    assert!(order_projection_payload(&druid_magician).contains("abilityCheckBonusFeatureCount=1"));
}

#[test]
fn order_projection_keeps_base_and_selected_training_distinct() {
    // RAW: cleanroom-input/raw/srd-5.2.1/Classes/Cleric.md
    // "Core Cleric Traits" and "Level 1: Divine Order";
    // cleanroom-input/raw/srd-5.2.1/Classes/Druid.md
    // "Core Druid Traits" and "Level 1: Primal Order".
    let cleric_thaumaturge = cleric_divine_order_projection(ClericDivineOrder::Thaumaturge {
        extra_cantrip: Cantrip::Light,
    });
    let druid_magician = druid_primal_order_projection(DruidPrimalOrder::Magician {
        extra_cantrip: Cantrip::Guidance,
    });

    assert!(cleric_thaumaturge.training.medium_armor_training);
    assert!(!cleric_thaumaturge.training.heavy_armor_training);
    assert!(!druid_magician.training.medium_armor_training);
    assert_eq!(
        cleric_thaumaturge
            .ability_check_bonus
            .unwrap()
            .minimum_bonus,
        1
    );
    assert_eq!(druid_magician.ability_check_bonus.unwrap().minimum_bonus, 1);
}

#[test]
fn fighter_fighting_style_adapter_replays_selected_identity_branches() {
    // QNT: cleanroom-input/qnt/character-creation-runtime/
    // character-creation-fighter-fighting-style-selected-identity.mbt.qnt.
    let selected = replay_fighting_style_action("doSelectDefenseFightingStyle");
    let replaced = replay_fighting_style_action("doReplaceDefenseWithArcheryOnFighterLevelGain");

    assert_eq!(selected, expected_select_defense_witness());
    assert_eq!(replaced, expected_replace_defense_with_archery_witness());
    assert!(fighting_style_projection_payload(&selected).contains("totalLevel=1"));
    assert!(fighting_style_projection_payload(&replaced).contains("totalLevel=2"));
}

#[test]
fn fighter_fighting_style_replacement_records_previous_and_new_feat() {
    // RAW: cleanroom-input/raw/srd-5.2.1/Classes/Fighter.md
    // "Level 1: Fighting Style"; cleanroom-input/raw/srd-5.2.1/Feats.md
    // "Fighting Style Feats", Archery and Defense.
    let projection =
        fighter_fighting_style_projection(FighterFightingStyleSelection::ReplacementOnLevelGain {
            previous: FightingStyleFeat::Defense,
            replacement: FightingStyleFeat::Archery,
            new_total_level: ClassLevel::Two,
        });

    assert_eq!(projection.replaced_feat, Some(FightingStyleFeat::Defense));
    assert_eq!(projection.selected_feat, FightingStyleFeat::Archery);
    assert_eq!(projection.total_level, ClassLevel::Two);
}

#[test]
fn character_creation_runtime_adapter_replays_all_branch_actions() {
    // QNT: cleanroom-input/qnt/character-creation-runtime/character-creation-runtime.mbt.qnt,
    // action step; slice: character-creation-runtime-slice.qnt.
    // RAW: cleanroom-input/raw/srd-5.2.1/Character-Creation.md "Choose a Class",
    // "Choose Languages", "Determine Ability Scores", "Choose Alignment", and
    // "Choose Starting Equipment"; cleanroom-input/raw/srd-5.2.1/Classes/Fighter.md
    // "Core Fighter Traits".
    for action in BRANCH_ACTIONS {
        let witness = replay_runtime_action(action);
        let payload = runtime_projection_payload(&witness);
        assert!(payload.contains("qDraft.revision="));
        assert!(matches!(witness.q_last_result, "accepted" | "rejected"));
    }
}

#[test]
fn character_creation_runtime_accepts_manifest_progression() {
    let initial_manifest = replay_runtime_action("doFillInitialManifest");
    let manifest_choices = replay_runtime_action("doFillManifestChoices");
    let manifest_purchase = replay_runtime_action("doFillManifestPurchase");
    let manifest_loadout = replay_runtime_action("doFillManifestLoadout");

    assert_eq!(initial_manifest.q_last_result, "accepted");
    assert_eq!(initial_manifest.q_draft_revision, 1);
    assert_eq!(
        initial_manifest.q_draft_progression,
        "SelectedProgression(FighterLevel1)"
    );
    assert_eq!(
        initial_manifest.q_holes,
        vec![
            "HClassSkills",
            "HFighterFightingStyle",
            "HFighterWeaponMastery",
            "HBackgroundAbilityScoreIncrease",
            "HBackgroundTool",
            "HClassEquipment",
            "HBackgroundEquipment"
        ]
    );

    assert_eq!(manifest_choices.q_draft_revision, 2);
    assert_eq!(manifest_choices.q_holes, vec!["HEquipmentPurchase"]);
    assert_eq!(manifest_purchase.q_draft_revision, 3);
    assert_eq!(
        manifest_purchase.q_holes,
        vec!["HLoadoutArmor", "HLoadoutShield", "HLoadoutWeapon"]
    );
    assert_eq!(manifest_loadout.q_draft_revision, 4);
    assert!(manifest_loadout.q_holes.is_empty());
    assert_eq!(manifest_loadout.q_finalization, "Ready");
}

#[test]
fn character_creation_runtime_rejects_manifest_protocol_issues() {
    let stale = replay_runtime_action("doRejectStaleInitialManifest");
    let unsupported_language = replay_runtime_action("doRejectUnsupportedLanguage");
    let duplicate_language = replay_runtime_action("doRejectDuplicateLanguage");
    let duplicate_fill = replay_runtime_action("doRejectDuplicateFill");
    let too_few_languages = replay_runtime_action("doRejectTooFewLanguages");
    let too_many_languages = replay_runtime_action("doRejectTooManyLanguages");
    let wrong_kind = replay_runtime_action("doRejectWrongKindPrimaryClass");
    let closed_progression = replay_runtime_action("doRejectClosedInitialProgressionHole");
    let unknown_loadout = replay_runtime_action("doRejectUnknownLoadoutArmor");
    let unsupported_class_equipment = replay_runtime_action("doRejectUnsupportedClassEquipment");

    assert_eq!(stale.q_last_result, "rejected");
    assert_eq!(stale.q_last_batch_issue_codes, vec!["staleRevision"]);
    assert!(stale.q_last_fill_issues.is_empty());

    assert_eq!(
        runtime_projection_payload(&unsupported_language),
        runtime_projection_payload(&unsupported_language)
    );
    assert!(runtime_projection_payload(&unsupported_language)
        .contains("qLastFillIssues=0:HLanguages:unsupportedChoice"));
    assert!(runtime_projection_payload(&duplicate_language)
        .contains("qLastFillIssues=0:HLanguages:invalidChoice"));
    assert!(runtime_projection_payload(&duplicate_fill)
        .contains("qLastFillIssues=1:HLanguages:duplicateFill"));
    assert!(runtime_projection_payload(&too_few_languages)
        .contains("qLastFillIssues=0:HLanguages:tooFewChoices"));
    assert!(runtime_projection_payload(&too_many_languages).contains("0:HLanguages:tooManyChoices"));
    assert!(
        runtime_projection_payload(&too_many_languages).contains("0:HLanguages:unsupportedChoice")
    );
    assert!(runtime_projection_payload(&wrong_kind)
        .contains("qLastFillIssues=0:HProgression:wrongFillKind"));
    assert!(runtime_projection_payload(&closed_progression)
        .contains("qLastFillIssues=0:HProgression:unknownHole"));
    assert!(runtime_projection_payload(&unknown_loadout)
        .contains("qLastFillIssues=0:HLoadoutArmor:unknownHole"));
    assert!(runtime_projection_payload(&unsupported_class_equipment)
        .contains("qLastFillIssues=0:HClassEquipment:unsupportedChoice"));
}

#[test]
fn weapon_mastery_adapter_replays_container_identity_branches() {
    // QNT: cleanroom-input/qnt/character-creation-runtime/
    // character-creation-weapon-mastery-containers-selected-identity.mbt.qnt.
    let barbarian = replay_weapon_mastery_action("doFinalizeBarbarianWeaponMastery");
    let fighter = replay_weapon_mastery_action("doFinalizeFighterWeaponMastery");
    let paladin = replay_weapon_mastery_action("doFinalizePaladinWeaponMastery");
    let ranger = replay_weapon_mastery_action("doFinalizeRangerWeaponMastery");
    let rogue = replay_weapon_mastery_action("doFinalizeRogueWeaponMastery");

    assert_eq!(barbarian, expected_barbarian_witness());
    assert_eq!(fighter, expected_fighter_witness());
    assert_eq!(paladin, expected_paladin_witness());
    assert_eq!(ranger, expected_ranger_witness());
    assert_eq!(rogue, expected_rogue_witness());
    assert!(weapon_mastery_projection_payload(&fighter).contains("selectedMasteryChoiceCount=3"));
    assert!(
        weapon_mastery_projection_payload(&rogue).contains("secondWeaponUnitId=weapon_shortsword")
    );
}

#[test]
fn weapon_mastery_projection_enforces_class_choice_count() {
    // RAW: cleanroom-input/raw/srd-5.2.1/Classes/Fighter.md
    // "Level 1: Weapon Mastery"; cleanroom-input/raw/srd-5.2.1/Classes/Barbarian.md,
    // Paladin.md, Ranger.md, and Rogue.md "Level 1: Weapon Mastery".
    let fighter = weapon_mastery_projection(
        WeaponMasteryClass::Fighter,
        &[Weapon::Longsword, Weapon::Spear, Weapon::Flail],
    )
    .expect("fighter has three level-1 weapon mastery choices");
    let short_fighter = weapon_mastery_projection(
        WeaponMasteryClass::Fighter,
        &[Weapon::Longsword, Weapon::Spear],
    );

    assert_eq!(fighter.selected_mastery_choice_count, 3);
    assert_eq!(
        short_fighter,
        Err(ProjectionError::WrongWeaponMasteryChoiceCount)
    );
}

#[test]
fn ability_check_proficiency_adapter_replays_all_branches() {
    // QNT: cleanroom-input/qnt/character-sheet-runtime/
    // character-sheet-ability-check-proficiency-bonus.mbt.qnt;
    // shared algebra: cleanroom-input/qnt/shared-algebras/proofs/rule-core/
    // ability-check-proficiency-bonus.qnt.
    let jack_level_two = replay_ability_check_action("doProjectJackOfAllTradesLevelTwo");
    let jack_rounded = replay_ability_check_action("doProjectJackOfAllTradesRoundedDown");
    let skill = replay_ability_check_action("doProjectSkillProficiency");
    let expertise = replay_ability_check_action("doProjectExpertise");
    let other = replay_ability_check_action("doRejectOtherProficiencyBonus");
    let missing_bard = replay_ability_check_action("doRejectMissingBardLevelTwo");

    assert_eq!(
        jack_level_two,
        expected_jack_of_all_trades_level_two_witness()
    );
    assert_eq!(
        jack_rounded,
        expected_jack_of_all_trades_rounded_down_witness()
    );
    assert_eq!(skill, expected_skill_proficiency_witness());
    assert_eq!(expertise, expected_expertise_witness());
    assert_eq!(other, expected_other_proficiency_bonus_witness());
    assert_eq!(missing_bard, expected_missing_bard_level_two_witness());
    assert!(ability_check_projection_payload(&expertise).contains("bonus=6"));
}

#[test]
fn ability_check_proficiency_prefers_training_over_jack_of_all_trades() {
    // RAW: cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md "Proficiency Bonus";
    // cleanroom-input/raw/srd-5.2.1/Classes/Bard.md "Level 2: Jack of All Trades";
    // cleanroom-input/raw/srd-5.2.1/Rules-Glossary.md "Expertise".
    let proficient = ability_check_proficiency_bonus(
        3,
        AbilityCheckSkillTraining::Proficient,
        JackOfAllTradesState::Present,
        OtherProficiencyBonusState::NoOtherProficiencyBonus,
    );
    let blocked_jack = ability_check_proficiency_bonus(
        3,
        AbilityCheckSkillTraining::Unproficient,
        JackOfAllTradesState::Present,
        OtherProficiencyBonusState::OtherProficiencyBonusApplies,
    );

    assert_eq!(
        proficient.kind,
        AbilityCheckProficiencyBonusKind::SkillProficiency
    );
    assert_eq!(proficient.bonus, 3);
    assert_eq!(blocked_jack.kind, AbilityCheckProficiencyBonusKind::None);
    assert_eq!(blocked_jack.bonus, 0);
}

#[test]
fn armor_class_adapter_replays_selected_identity_branches() {
    // QNT: cleanroom-input/qnt/character-sheet-runtime/
    // character-sheet-armor-class-base-selected-identity.mbt.qnt;
    // shared algebra: cleanroom-input/qnt/shared-algebras/proofs/rule-core/armor-class-base.qnt.
    let barbarian = replay_armor_class_action("doSelectBarbarianUnarmoredDefense");
    let barbarian_shield = replay_armor_class_action("doSelectBarbarianUnarmoredDefenseWithShield");
    let monk = replay_armor_class_action("doSelectMonkUnarmoredDefense");
    let light = replay_armor_class_action("doProjectLightArmor");
    let medium = replay_armor_class_action("doProjectMediumArmorDexCap");
    let heavy = replay_armor_class_action("doProjectHeavyArmorWithShield");

    assert_eq!(barbarian, expected_barbarian_unarmored_defense_witness());
    assert_eq!(
        barbarian_shield,
        expected_barbarian_unarmored_defense_with_shield_witness()
    );
    assert_eq!(monk, expected_monk_unarmored_defense_witness());
    assert_eq!(light, expected_light_armor_witness());
    assert_eq!(medium, expected_medium_armor_dex_cap_witness());
    assert_eq!(heavy, expected_heavy_armor_with_shield_witness());
    assert!(armor_class_projection_payload(&heavy).contains("armorClass=18"));
}

#[test]
fn armor_class_projection_caps_medium_dex_and_requires_trained_shield() {
    // RAW: cleanroom-input/raw/srd-5.2.1/Equipment.md "Armor" and "Shield";
    // cleanroom-input/raw/srd-5.2.1/Classes/Barbarian.md and Classes/Monk.md
    // "Level 1: Unarmored Defense".
    let medium = armor_class_projection(
        ArmorClassOption::ChainShirt,
        ArmorClassFacts {
            dexterity_modifier: 5,
            constitution_modifier: 0,
            wisdom_modifier: 0,
            charisma_modifier: 0,
            formula: ArmorClassFormula::MediumDexMax2 { base: 13 },
            shield_bonus: None,
        },
    );
    let untrained_shield = armor_class_projection(
        ArmorClassOption::DefaultUnarmored,
        ArmorClassFacts {
            dexterity_modifier: 2,
            constitution_modifier: 0,
            wisdom_modifier: 0,
            charisma_modifier: 0,
            formula: ArmorClassFormula::AbilitySum {
                base: 10,
                abilities: vec![ArmorClassAbility::Dexterity],
            },
            shield_bonus: Some(ShieldArmorClassBonus {
                bonus: 2,
                wielding_shield: true,
                shield_training: false,
            }),
        },
    );

    assert_eq!(medium.armor_class, 15);
    assert_eq!(untrained_shield.armor_class, 12);
    assert_eq!(untrained_shield.shield_bonus, 0);
}

#[test]
fn healing_resource_adapter_replays_lay_on_hands_branch() {
    // QNT: cleanroom-input/qnt/character-sheet-runtime/
    // character-sheet-healing-resource-selected-identity.mbt.qnt;
    // shared algebra: cleanroom-input/qnt/shared-algebras/proofs/rule-core/
    // lay-on-hands-resource.qnt.
    let observed = replay_healing_resource_action("doLayOnHandsRestoreHpAndRemovePoisoned");

    assert_eq!(observed, expected_lay_on_hands_witness());
    assert!(healing_resource_projection_payload(&observed).contains("poolRemaining=3"));
}

#[test]
fn lay_on_hands_spends_condition_cost_separately_from_healing() {
    // RAW: cleanroom-input/raw/srd-5.2.1/Classes/Paladin.md
    // "Level 1: Lay On Hands"; Playing-the-Game.md "Healing".
    let projection = apply_lay_on_hands_resource(
        ResourcePoolFacts {
            capacity: 10,
            expended: 0,
        },
        FeatureResourceHitPoints {
            current_hit_points: 11,
            hit_point_maximum: 12,
            temporary_hit_points: 0,
        },
        4,
        1,
    )
    .expect("pool has enough remaining points");

    assert_eq!(projection.pool_spend, 9);
    assert_eq!(projection.healing_pool.expended, 9);
    assert_eq!(projection.target_hit_points.current_hit_points, 12);
    assert!(projection.condition_removed);

    let insufficient = apply_lay_on_hands_resource(
        ResourcePoolFacts {
            capacity: 10,
            expended: 4,
        },
        FeatureResourceHitPoints {
            current_hit_points: 3,
            hit_point_maximum: 12,
            temporary_hit_points: 0,
        },
        2,
        1,
    );
    assert!(matches!(
        insufficient,
        Err(ResourcePoolError::InsufficientRemaining)
    ));
}

#[test]
fn hit_point_maximum_adapter_replays_all_branches() {
    // QNT: cleanroom-input/qnt/character-sheet-runtime/
    // character-sheet-hit-point-maximum.mbt.qnt;
    // shared algebra: cleanroom-input/qnt/shared-algebras/proofs/rule-core/
    // hit-point-maximum.qnt.
    for action in HIT_POINT_MAXIMUM_BRANCH_ACTIONS {
        let observed = replay_hit_point_maximum_action(action);
        assert_eq!(observed, expected_hit_point_maximum_witness(action));
        assert!(hit_point_maximum_projection_payload(&observed).contains("hitDiceTotal="));
    }
}

#[test]
fn hit_point_maximum_applies_fixed_gain_minimum_and_reduction() {
    // RAW: cleanroom-input/raw/srd-5.2.1/Character-Creation.md
    // "Gaining a Level", "Fixed Hit Points by Class", and "Hit Points and Hit
    // Point Dice".
    assert_eq!(fixed_higher_level_hit_point_gain(6, -4), 1);

    let projection = hit_point_maximum_projection(HitPointMaximumFacts {
        starting_hit_point_die: 10,
        constitution_modifier: 1,
        fixed_higher_level_hit_point_dice: vec![10],
        hit_point_maximum_bonus: 0,
        hit_point_maximum_reduction: 3,
    })
    .expect("reduction remains below normal maximum");

    assert_eq!(projection.normal_hit_point_maximum, 18);
    assert_eq!(projection.effective_hit_point_maximum, 15);
    assert_eq!(projection.hit_dice_total, 2);
}

#[test]
fn hp_rest_hit_dice_adapter_replays_all_branches() {
    // QNT: cleanroom-input/qnt/character-sheet-runtime/
    // character-sheet-hp-rest-hit-dice.mbt.qnt; RAW:
    // cleanroom-input/raw/srd-5.2.1/Rules-Glossary.md "Short Rest" and
    // "Long Rest".
    for action in HP_REST_HIT_DICE_BRANCH_ACTIONS {
        let observed = replay_hp_rest_hit_dice_action(action);
        assert_eq!(observed, expected_hp_rest_hit_dice_witness(action));
        assert!(hp_rest_hit_dice_projection_payload(&observed).contains("replayIndex="));
    }
}

#[test]
fn hp_rest_hit_dice_spending_caps_healing_and_long_rest_resets() {
    // RAW: cleanroom-input/raw/srd-5.2.1/Rules-Glossary.md "Short Rest" and
    // "Long Rest"; QNT: character-sheet-hp-rest-hit-dice.mbt.qnt.
    let nearly_full = SheetHitPointState {
        current_hp: 17,
        normal_hit_point_maximum: 18,
        hit_point_maximum_reduction: 0,
        temporary_hit_points: 0,
        spent_hit_dice: 0,
    };
    let healed = spend_hit_point_die(nearly_full, 8, 3);

    assert_eq!(healed.current_hp, 18);
    assert_eq!(healed.spent_hit_dice, 1);

    let rested = complete_long_rest_benefits(SheetHitPointState {
        current_hp: 7,
        normal_hit_point_maximum: 18,
        hit_point_maximum_reduction: 6,
        temporary_hit_points: 2,
        spent_hit_dice: 1,
    });
    assert_eq!(rested.current_hp, 18);
    assert_eq!(rested.hit_point_maximum_reduction, 0);
    assert_eq!(rested.temporary_hit_points, 0);
    assert_eq!(rested.spent_hit_dice, 0);
}

#[test]
fn spellbook_ritual_adapter_replays_all_branches() {
    // QNT: cleanroom-input/qnt/character-sheet-runtime/
    // character-sheet-spellbook-ritual-selected-identity.mbt.qnt;
    // shared algebra: cleanroom-input/qnt/shared-algebras/proofs/rule-core/
    // spellbook-ritual-access.qnt.
    for action in SPELLBOOK_RITUAL_BRANCH_ACTIONS {
        let observed = replay_spellbook_ritual_action(action);
        assert_eq!(observed, expected_spellbook_ritual_witness(action));
        assert!(spellbook_ritual_projection_payload(&observed).contains("lastResult="));
    }
}

#[test]
fn spellbook_ritual_requires_spellbook_access_and_ritual_adept() {
    // RAW: cleanroom-input/raw/srd-5.2.1/Classes/Wizard.md
    // "Level 1: Ritual Adept" and "Spellbook";
    // cleanroom-input/raw/srd-5.2.1/Spells/Gaining-and-Casting.md
    // "Casting without Slots"; cleanroom-input/raw/srd-5.2.1/Rules-Glossary.md
    // "Ritual".
    let accepted_facts = SpellbookRitualFacts {
        spell_kind: SpellbookSpellKind::LevelOnePlusRitual,
        access: SpellbookRitualAccess::InSpellbook,
        ritual_adept: SpellbookRitualAdept::Present,
    };
    let prepared_only = SpellbookRitualFacts {
        access: SpellbookRitualAccess::PreparedOnly,
        ..accepted_facts
    };
    let non_leveled = SpellbookRitualFacts {
        spell_kind: SpellbookSpellKind::NonLeveled,
        ..accepted_facts
    };

    assert!(can_cast_spellbook_ritual(accepted_facts));
    let SpellbookRitualResult::Accepted(invocation) = spellbook_ritual_invocation(accepted_facts)
    else {
        panic!("spellbook ritual facts should be accepted");
    };
    assert_eq!(invocation.spell_slot_cost, SpellSlotCost::None);
    assert_eq!(
        invocation.preparation_requirement,
        PreparationRequirement::NotRequired
    );
    assert_eq!(invocation.required_access, RequiredSpellAccess::Spellbook);
    assert_eq!(invocation.additional_casting_time_minutes, 10);
    assert!(invocation.requires_reading_spellbook);
    assert_eq!(invocation.first_level_spell_slots_expended, 0);

    assert_eq!(
        spellbook_ritual_invocation(prepared_only),
        SpellbookRitualResult::Rejected
    );
    assert_eq!(
        spellbook_ritual_invocation(non_leveled),
        SpellbookRitualResult::Rejected
    );
}

#[test]
fn sheet_weapon_mastery_adapter_replays_all_branches() {
    // QNT: cleanroom-input/qnt/character-sheet-runtime/
    // character-sheet-weapon-mastery-containers-selected-identity.mbt.qnt;
    // shared algebra: cleanroom-input/qnt/shared-algebras/proofs/rule-core/
    // weapon-mastery-reselection.qnt.
    for action in SHEET_WEAPON_MASTERY_BRANCH_ACTIONS {
        let observed = replay_sheet_weapon_mastery_action(action);
        assert_eq!(observed, expected_sheet_weapon_mastery_witness(action));
        assert!(sheet_weapon_mastery_projection_payload(&observed).contains("choiceCount=2"));
    }
}

#[test]
fn weapon_mastery_reselection_applies_long_rest_change_limit() {
    // RAW: cleanroom-input/raw/srd-5.2.1/Classes/Paladin.md,
    // Classes/Ranger.md, and Classes/Rogue.md "Level 1: Weapon Mastery";
    // QNT: cleanroom-input/qnt/shared-algebras/proofs/rule-core/
    // weapon-mastery-reselection.qnt.
    let one_change = WeaponMasteryReselectionFacts {
        choice_count: 2,
        long_rest_change_count: 1,
        current_weapons: vec![Weapon::Longsword, Weapon::Dagger],
        requested_weapons: vec![Weapon::Longsword, Weapon::Spear],
        eligible_weapons: vec![Weapon::Longsword, Weapon::Spear, Weapon::Flail],
    };
    let projection = apply_weapon_mastery_long_rest_reselection(&one_change)
        .expect("one changed Weapon Mastery choice is within the Long Rest limit");

    assert_eq!(
        projection.selected_weapons,
        vec![Weapon::Longsword, Weapon::Spear]
    );
    assert_eq!(projection.changed_choice_count, 1);
    assert_eq!(projection.long_rest_change_count, 1);

    let too_many = WeaponMasteryReselectionFacts {
        requested_weapons: vec![Weapon::Spear, Weapon::Flail],
        eligible_weapons: vec![Weapon::Spear, Weapon::Flail],
        ..one_change
    };
    assert_eq!(
        apply_weapon_mastery_long_rest_reselection(&too_many),
        Err(ProjectionError::TooManyWeaponMasteryChanges)
    );
}

#[test]
fn origin_feat_adapter_replays_all_branches() {
    // QNT: cleanroom-input/qnt/character-battle-runtime/
    // character-battle-origin-feat-selected-identity.mbt.qnt.
    for action in ORIGIN_FEAT_BRANCH_ACTIONS {
        let observed = replay_origin_feat_action(action);
        assert_eq!(observed, expected_origin_feat_witness(action));
        assert!(origin_feat_projection_payload(&observed).contains("originFeatUnitId=alert"));
    }
}

#[test]
fn alert_origin_feat_adds_proficiency_to_initiative_score() {
    // RAW: cleanroom-input/raw/srd-5.2.1/Character-Origins.md "Criminal";
    // cleanroom-input/raw/srd-5.2.1/Feats.md "Alert";
    // cleanroom-input/raw/srd-5.2.1/Rules-Glossary.md "Initiative".
    let origin = criminal_origin_feat_projection();
    let alert = initiative_handoff_projection(InitiativeHandoffFacts {
        dexterity_modifier: 4,
        proficiency_bonus: 2,
        alert_initiative: AlertInitiativeState::Present,
    });
    let no_alert = initiative_handoff_projection(InitiativeHandoffFacts {
        dexterity_modifier: 4,
        proficiency_bonus: 2,
        alert_initiative: AlertInitiativeState::Absent,
    });

    assert_eq!(origin.background, Background::Criminal);
    assert_eq!(origin.origin_feat, OriginFeat::Alert);
    assert_eq!(alert.initiative_score, 16);
    assert_eq!(no_alert.initiative_score, 14);
}

#[test]
fn adrenaline_rush_adapter_replays_all_branches() {
    // QNT: cleanroom-input/qnt/battle-runtime/
    // battle-runtime-adrenaline-rush.mbt.qnt; shared movement model:
    // cleanroom-input/qnt/battle-runtime/battle-runtime-movement.qnt.
    for action in ADRENALINE_RUSH_BRANCH_ACTIONS {
        let observed = replay_adrenaline_rush_action(action);
        assert_eq!(observed, expected_adrenaline_rush_witness(action));
        assert!(adrenaline_rush_projection_payload(&observed).contains("protocolHoleCount=0"));
    }
}

#[test]
fn adrenaline_rush_bonus_action_dash_spends_use_and_keeps_higher_temp_hp() {
    // RAW: cleanroom-input/raw/srd-5.2.1/Character-Origins.md
    // "Orc" and "Adrenaline Rush"; Playing-the-Game.md "Dash",
    // "Bonus Actions", and "Temporary Hit Points".
    let facts = AdrenalineRushFacts {
        turn: BattleTurnFeatureState {
            actor_temporary_hit_points: 1,
            bonus_action_available: true,
            dash_bonus_feet: 0,
            feature_uses_remaining: 3,
        },
        speed_feet: 30,
        proficiency_bonus: 3,
        actor_owns_turn: true,
        actor_unconscious: false,
        actor_incapacitated: false,
    };
    let resolved = resolve_adrenaline_rush_bonus_action_dash(facts);
    let stale = resolve_adrenaline_rush_bonus_action_dash(AdrenalineRushFacts {
        turn: resolved.turn,
        ..facts
    });

    assert_eq!(resolved.result, AdrenalineRushResult::Resolved);
    assert_eq!(resolved.turn.actor_temporary_hit_points, 3);
    assert!(!resolved.turn.bonus_action_available);
    assert_eq!(resolved.turn.dash_bonus_feet, 30);
    assert_eq!(resolved.turn.feature_uses_remaining, 2);
    assert_eq!(stale.turn, resolved.turn);
    assert_eq!(
        stale.result,
        AdrenalineRushResult::Invalid(AdrenalineRushRejection::StaleSubject)
    );
    assert_eq!(apply_temporary_hit_points(5, 3), 5);
}

#[test]
fn attack_spell_shape_adapter_replays_all_branches() {
    // QNT: cleanroom-input/qnt/battle-runtime/
    // battle-runtime-attack-spell-shape-selected-identity.mbt.qnt;
    // shared algebra: cleanroom-input/qnt/shared-algebras/proofs/rule-core/
    // spell-attack-damage-projection-core.qnt and spell-save-damage-projection-core.qnt.
    for action in ATTACK_SPELL_SHAPE_BRANCH_ACTIONS {
        let observed = replay_attack_spell_shape_action(action);
        assert_eq!(observed, expected_attack_spell_shape_witness(action));
        assert!(
            attack_spell_shape_projection_payload(&observed).contains("protocolResult=resolved")
        );
    }
}

#[test]
fn attack_spell_shapes_project_slots_effects_and_save_damage() {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-A-D.md
    // "Chill Touch"; Descriptions-E-L.md "Fire Bolt", "Guiding Bolt", and
    // "Inflict Wounds"; Descriptions-S-Z.md "Shocking Grasp".
    let initial = SpellShapeState {
        target_hit_points: 12,
        spell_slot_spent_this_turn: false,
        level_one_slots_remaining: 2,
        active_effects: vec![],
    };
    let chill_touch = resolve_spell_attack_hit(
        initial.clone(),
        AttackSpellHitFacts {
            spell: AttackSpellProfile::ChillTouch,
            damage_roll: 4,
        },
    );
    let guiding_bolt = resolve_spell_attack_hit(
        initial.clone(),
        AttackSpellHitFacts {
            spell: AttackSpellProfile::GuidingBolt,
            damage_roll: 4,
        },
    );
    let inflict_success = resolve_save_gated_spell_damage(
        initial,
        SaveGatedSpellDamageFacts {
            spell: SaveGatedSpellProfile::InflictWounds,
            damage_roll: 6,
            saving_throw_succeeded: true,
        },
    );

    assert_eq!(chill_touch.state.target_hit_points, 8);
    assert_eq!(
        chill_touch.state.active_effects,
        vec![SpellActiveEffect::HitPointRegainPrevented]
    );
    assert_eq!(chill_touch.outcome, SpellShapeOutcome::ChillTouchHit);
    assert!(!chill_touch.state.spell_slot_spent_this_turn);

    assert_eq!(guiding_bolt.state.level_one_slots_remaining, 1);
    assert!(guiding_bolt.state.spell_slot_spent_this_turn);
    assert_eq!(
        guiding_bolt.state.active_effects,
        vec![SpellActiveEffect::NextAttackRollAgainstTargetAdvantage]
    );

    assert_eq!(inflict_success.state.target_hit_points, 9);
    assert_eq!(
        inflict_success.outcome,
        SpellShapeOutcome::InflictWoundsSuccessfulSave
    );
    assert!(inflict_success.state.spell_slot_spent_this_turn);
}

#[test]
fn chained_attack_sequence_adapter_replays_all_branches() {
    // QNT: cleanroom-input/qnt/battle-runtime/
    // battle-runtime-chained-attack-sequence.mbt.qnt; shared algebra:
    // cleanroom-input/qnt/shared-algebras/proofs/rule-core/
    // spell-chained-attack-damage-projection-core.qnt.
    for action in CHAINED_ATTACK_SEQUENCE_BRANCH_ACTIONS {
        let observed = replay_chained_attack_sequence_action(action);
        assert_eq!(observed, expected_chained_attack_sequence_witness(action));
        assert!(chained_attack_sequence_projection_payload(&observed).contains("protocolResult="));
    }
}

#[test]
fn chromatic_orb_sequence_tracks_duplicate_leap_limit() {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-A-D.md
    // "Chromatic Orb"; QNT: battle-runtime-chained-spell-attack.qnt.
    fn sequence_through_first_leap(
        slot_level: i16,
        step0_damage_total: i16,
    ) -> ChromaticOrbSequenceState {
        let state = start_chromatic_orb_cast(chromatic_orb_initial_state(), slot_level)
            .expect("slot level is legal");
        let state = choose_chromatic_orb_damage_type(state, ChainedDamageTypeChoice::Fire)
            .expect("damage type choice is legal");
        let state = choose_chromatic_orb_initial_target(state, ChainedTarget::First)
            .expect("initial target is legal");
        let state = resolve_chromatic_orb_attack_hit(
            state,
            ChainedAttackStep::Step0,
            ChainedAttackRollFacts {
                attack_total: 18,
                natural_d20: 12,
            },
        )
        .expect("step 0 attack resolves");
        let state = resolve_chromatic_orb_damage(
            state,
            ChainedAttackStep::Step0,
            ChainedDamageRollFacts {
                damage_total: step0_damage_total,
                has_duplicate: true,
            },
        )
        .expect("duplicate step 0 damage resolves");
        choose_chromatic_orb_leap_target(state, ChainedTarget::Second, true)
            .expect("first leap target is legal")
    }

    let slot_one_step1 = resolve_chromatic_orb_attack_hit(
        sequence_through_first_leap(1, 9),
        ChainedAttackStep::Step1,
        ChainedAttackRollFacts {
            attack_total: 18,
            natural_d20: 12,
        },
    )
    .expect("slot 1 step 1 attack resolves");
    let slot_one_done = resolve_chromatic_orb_damage(
        slot_one_step1,
        ChainedAttackStep::Step1,
        ChainedDamageRollFacts {
            damage_total: 3,
            has_duplicate: true,
        },
    )
    .expect("slot 1 duplicate step 1 damage resolves");

    let slot_two_step1 = resolve_chromatic_orb_attack_hit(
        sequence_through_first_leap(2, 10),
        ChainedAttackStep::Step1,
        ChainedAttackRollFacts {
            attack_total: 18,
            natural_d20: 12,
        },
    )
    .expect("slot 2 step 1 attack resolves");
    let slot_two_awaits_second_leap = resolve_chromatic_orb_damage(
        slot_two_step1,
        ChainedAttackStep::Step1,
        ChainedDamageRollFacts {
            damage_total: 4,
            has_duplicate: true,
        },
    )
    .expect("slot 2 duplicate step 1 damage resolves");

    assert_eq!(chromatic_orb_damage_die_size(), 8);
    assert_eq!(chromatic_orb_damage_die_count(1), 3);
    assert_eq!(chromatic_orb_damage_die_count(2), 4);
    assert!(!chromatic_orb_can_leap(1, 1, true));
    assert!(chromatic_orb_can_leap(2, 1, true));

    assert_eq!(
        slot_one_done.outcome,
        ChainedAttackScenarioOutcome::Slot1LeapLimitComplete
    );
    assert_eq!(slot_one_done.protocol, ChainedAttackProtocol::Resolved);
    assert_eq!(slot_one_done.first_target_hit_points, 3);
    assert_eq!(slot_one_done.second_target_hit_points, 9);

    assert_eq!(
        slot_two_awaits_second_leap.outcome,
        ChainedAttackScenarioOutcome::AwaitingSecondLeapTarget
    );
    assert_eq!(
        slot_two_awaits_second_leap.protocol,
        ChainedAttackProtocol::NeedsHole(ChainedAttackHole::TargetChoice2)
    );
    assert_eq!(
        slot_two_awaits_second_leap.targeted,
        vec![ChainedTarget::First, ChainedTarget::Second]
    );
    assert_eq!(slot_two_awaits_second_leap.leaps_used, 1);
    assert_eq!(slot_two_awaits_second_leap.first_target_hit_points, 2);
    assert_eq!(slot_two_awaits_second_leap.second_target_hit_points, 8);
}

#[test]
fn command_option_next_turn_adapter_replays_all_branches() {
    // QNT: cleanroom-input/qnt/battle-runtime/
    // battle-runtime-command-option-next-turn.mbt.qnt; RAW:
    // cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-A-D.md "Command".
    for action in COMMAND_OPTION_NEXT_TURN_BRANCH_ACTIONS {
        let observed = replay_command_option_next_turn_action(action);
        assert_eq!(observed, expected_command_option_next_turn_witness(action));
        assert!(command_option_next_turn_projection_payload(&observed).contains("protocolResult="));
    }
}

#[test]
fn command_options_project_next_turn_effects_and_cleanup() {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-A-D.md
    // "Command"; Playing-the-Game.md and Rules-Glossary.md "Opportunity
    // Attacks"; Rules-Glossary.md "Prone".
    let pending_grovel = record_command_failed_save_pending(
        command_next_turn_initial_state(),
        CommandNextTurnOption::Grovel,
    );
    let grovel = follow_command_grovel(pending_grovel.clone());
    let halt = suppress_command_halt(
        record_command_failed_save_pending(
            command_next_turn_initial_state(),
            CommandNextTurnOption::Halt,
        ),
        30,
    );
    let halt_cleanup = cleanup_command_halt_turn(halt.clone());
    let approach_rejected = reject_command_approach_movement(record_command_failed_save_pending(
        command_next_turn_initial_state(),
        CommandNextTurnOption::Approach,
    ));
    let flee_window =
        open_command_flee_opportunity_attack_window(record_command_failed_save_pending(
            command_next_turn_initial_state(),
            CommandNextTurnOption::Flee,
        ));
    let flee_continues = decline_command_flee_opportunity_attack(flee_window.clone(), 30);

    assert_eq!(pending_grovel.target_effect_count, 1);
    assert_eq!(
        pending_grovel.pending_option,
        Some(CommandNextTurnOption::Grovel)
    );
    assert!(!pending_grovel.action_available);

    assert!(grovel.target_prone);
    assert_eq!(grovel.target_effect_count, 0);
    assert_eq!(grovel.pending_option, None);

    assert_eq!(halt.scenario, CommandNextTurnScenario::HaltSuppresses);
    assert!(!halt.action_available);
    assert!(!halt.bonus_action_available);
    assert!(halt.halt_suppressed);
    assert_eq!(halt_cleanup.current_actor, CommandTurnActor::Caster);
    assert!(!halt_cleanup.halt_suppressed);

    assert_eq!(
        approach_rejected.protocol,
        CommandNextTurnProtocol::Invalid(CommandNextTurnInvalidReason::InvalidFill)
    );
    assert_eq!(
        approach_rejected.pending_option,
        Some(CommandNextTurnOption::Approach)
    );

    assert_eq!(
        flee_window.protocol,
        CommandNextTurnProtocol::NeedsInterruptDecision
    );
    assert!(flee_window.reaction_window_open);
    assert_eq!(flee_continues.movement_spent_feet, 30);
    assert!(!flee_continues.reaction_window_open);
    assert_eq!(flee_continues.current_actor, CommandTurnActor::Caster);
}

#[test]
fn command_ordering_adapter_replays_all_branches() {
    // QNT: cleanroom-input/qnt/battle-runtime/
    // battle-runtime-command-ordering.mbt.qnt and
    // battle-runtime-command-ordering.qnt.
    for action in COMMAND_ORDERING_BRANCH_ACTIONS {
        let observed = replay_command_ordering_action(action);
        assert_eq!(observed, expected_command_ordering_witness(action));
        assert!(command_ordering_projection_payload(&observed).contains("protocolResult="));
    }
}

#[test]
fn command_ordering_frontier_requires_target_option_save_then_movement() {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-A-D.md
    // "Command"; QNT: battle-runtime-command-ordering.qnt.
    let initial = command_ordering_initial_state();
    let submit_option_too_early = command_fill_order_result(
        CommandFrontierStage::TargetListAndOptionChoice,
        CommandOrderingFillFacts {
            fill_kind: CommandFillKind::CommandOptionChoice,
            option: CommandNextTurnOption::Grovel,
            failed_saving_throw: true,
            movement_available: false,
            held_object_facts_required: false,
            moved_within_five_feet_of_caster: false,
            opened_opportunity_attack: false,
        },
    );
    let submit_save_too_early = command_fill_order_result(
        CommandFrontierStage::OptionChoice,
        CommandOrderingFillFacts {
            fill_kind: CommandFillKind::SavingThrowOutcome,
            option: CommandNextTurnOption::Grovel,
            failed_saving_throw: true,
            movement_available: false,
            held_object_facts_required: false,
            moved_within_five_feet_of_caster: false,
            opened_opportunity_attack: false,
        },
    );
    let flee_without_movement = command_fill_order_result(
        CommandFrontierStage::FleeMovement,
        CommandOrderingFillFacts {
            fill_kind: CommandFillKind::Movement,
            option: CommandNextTurnOption::Flee,
            failed_saving_throw: true,
            movement_available: false,
            held_object_facts_required: false,
            moved_within_five_feet_of_caster: false,
            opened_opportunity_attack: false,
        },
    );

    assert_eq!(initial.stage, CommandFrontierStage::ActSelection);
    assert_eq!(initial.protocol, CommandOrderingProtocol::Init);
    assert_eq!(
        command_hole_frontier(CommandFrontierStage::TargetListAndOptionChoice),
        vec![
            CommandHoleKind::SpellTargetList,
            CommandHoleKind::CommandOptionChoice
        ]
    );
    assert!(command_hole_frontier_includes_table_facts(
        CommandFrontierStage::DropHeldObjectFacts
    ));
    assert_eq!(
        command_after_saving_throw_stage(CommandNextTurnOption::Drop, true, false, true),
        CommandFrontierStage::DropHeldObjectFacts
    );
    assert_eq!(
        submit_option_too_early,
        CommandFillOrderResult::RequestedEarlier {
            error: CommandFillOrderingError::TargetListRequired,
            stage: CommandFrontierStage::TargetList
        }
    );
    assert_eq!(
        submit_save_too_early,
        CommandFillOrderResult::RequestedEarlier {
            error: CommandFillOrderingError::OptionChoiceRequired,
            stage: CommandFrontierStage::OptionChoice
        }
    );
    assert_eq!(
        flee_without_movement,
        CommandFillOrderResult::Rejected(CommandFillOrderingError::MovementRequired)
    );
}

#[test]
fn concentration_break_teardown_adapter_replays_all_branches() {
    // QNT: cleanroom-input/qnt/battle-runtime/
    // battle-runtime-concentration-break-teardown.mbt.qnt; RAW:
    // cleanroom-input/raw/srd-5.2.1/Rules-Glossary.md "Concentration".
    for action in CONCENTRATION_BREAK_TEARDOWN_BRANCH_ACTIONS {
        let observed = replay_concentration_break_teardown_action(action);
        assert_eq!(
            observed,
            expected_concentration_break_teardown_witness(action)
        );
        assert!(
            concentration_break_teardown_projection_payload(&observed).contains("protocolResult=")
        );
    }
}

#[test]
fn concentration_breaks_after_failed_save_voluntary_end_or_replacement() {
    // RAW: cleanroom-input/raw/srd-5.2.1/Rules-Glossary.md
    // "Concentration"; QNT: battle-runtime-concentration.qnt.
    assert_eq!(sampled_damage_total(), 6);
    assert_eq!(concentration_save_dc_for_damage(3), 10);
    assert_eq!(concentration_save_dc_for_damage(22), 11);
    assert_eq!(concentration_save_dc_for_damage(80), 30);

    let concentrating = cast_concentration_spell(concentration_initial_state());
    assert!(concentrating.caster_concentrating);
    assert_eq!(concentrating.active_concentration_effect_count, 1);

    let save_needed = request_concentration_save_after_damage(
        concentrating,
        ConcentrationDamageFacts {
            damage_roll_result: 4,
            damage_bonus: 2,
        },
    );
    assert_eq!(
        save_needed.scenario,
        ConcentrationBreakScenario::DamageSaveNeeded
    );
    assert_eq!(
        save_needed.protocol,
        ConcentrationProtocol::NeedsSavingThrow
    );
    assert_eq!(save_needed.save_dc, 10);

    let passed = fail_concentration_saving_throw(
        save_needed.clone(),
        ConcentrationSavingThrowFacts {
            saving_throw_total: 10,
        },
    );
    assert_eq!(passed, Err(ConcentrationSavingThrowError::SaveDidNotFail));

    let broken = fail_concentration_saving_throw(
        save_needed,
        ConcentrationSavingThrowFacts {
            saving_throw_total: 9,
        },
    )
    .expect("saving throw below DC breaks concentration");
    assert_eq!(
        broken.scenario,
        ConcentrationBreakScenario::DamageFailedTeardownBeforeNextCommand
    );
    assert!(!broken.caster_concentrating);
    assert_eq!(broken.active_concentration_effect_count, 0);
    assert!(broken.teardown_before_next_command);

    let voluntarily_ended = voluntarily_end_concentration(concentration_initial_state());
    assert_eq!(
        voluntarily_ended.scenario,
        ConcentrationBreakScenario::VoluntaryEndTeardown
    );
    assert!(!voluntarily_ended.caster_concentrating);
    assert!(voluntarily_ended.teardown_before_next_command);

    let replacement = cast_replacement_concentration_spell(concentration_initial_state());
    assert_eq!(
        replacement.scenario,
        ConcentrationBreakScenario::ReplacementTeardownBeforeNewEffect
    );
    assert!(replacement.caster_concentrating);
    assert!(replacement.replacement_started_after_teardown);
}

#[test]
fn creature_type_protection_adapter_replays_all_branches() {
    // QNT: cleanroom-input/qnt/battle-runtime/
    // battle-runtime-creature-type-protection-and-charm-selected-identity.mbt.qnt;
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-A-D.md
    // "Animal Friendship" and "Charm Person";
    // cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-M-P.md
    // "Protection from Evil and Good".
    for action in CREATURE_TYPE_PROTECTION_BRANCH_ACTIONS {
        let observed = replay_creature_type_protection_action(action);
        assert_eq!(observed, expected_creature_type_protection_witness(action));
        assert!(creature_type_protection_projection_payload(&observed).contains("protocolResult="));
    }
}

#[test]
fn creature_type_protection_projects_charm_and_scoped_protection_rules() {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-A-D.md
    // "Animal Friendship" and "Charm Person"; RAW:
    // cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-M-P.md
    // "Protection from Evil and Good"; QNT:
    // battle-runtime-creature-type-protection.qnt and
    // spell-save-condition-projection-core.qnt.
    assert!(save_condition_target_creature_type_is_legal(
        SaveGatedConditionSpell::AnimalFriendship,
        RuntimeCreatureType::Beast
    ));
    assert!(!save_condition_target_creature_type_is_legal(
        SaveGatedConditionSpell::AnimalFriendship,
        RuntimeCreatureType::Humanoid
    ));
    assert!(save_condition_target_creature_type_is_legal(
        SaveGatedConditionSpell::CharmPerson,
        RuntimeCreatureType::Humanoid
    ));
    assert!(!save_condition_target_creature_type_is_legal(
        SaveGatedConditionSpell::CharmPerson,
        RuntimeCreatureType::Beast
    ));

    let discovered =
        discover_animal_friendship_target_admission(creature_type_protection_initial_state());
    let charmed = resolve_animal_friendship_failed_save(creature_type_protection_initial_state());
    let damage_break =
        resolve_animal_friendship_damage_break(creature_type_protection_initial_state());

    assert!(discovered.beast_target_admitted);
    assert!(discovered.action_available);
    assert!(charmed.target_charmed);
    assert!(charmed.animal_friendship_effect_present);
    assert_eq!(charmed.first_level_slots_expended, 1);
    assert!(!damage_break.target_charmed);
    assert!(!damage_break.animal_friendship_effect_present);

    assert!(protection_from_evil_and_good_creature_type_is_scoped(
        RuntimeCreatureType::Fey
    ));
    assert!(!protection_from_evil_and_good_creature_type_is_scoped(
        RuntimeCreatureType::Humanoid
    ));
    assert_eq!(
        spell_attack_roll_mode_from_creature_type_protection(RuntimeCreatureType::Fiend, true),
        ProtectionAttackRollMode::Disadvantage
    );
    assert_eq!(
        spell_attack_roll_mode_from_creature_type_protection(RuntimeCreatureType::Humanoid, true),
        ProtectionAttackRollMode::Normal
    );
    assert!(protection_condition_is_prevented(
        ProtectedCondition::Charmed
    ));
    assert!(protection_condition_is_prevented(
        ProtectedCondition::Frightened
    ));
    assert!(protection_prevents_condition_from_creature_type(
        RuntimeCreatureType::Undead,
        true,
        ProtectedCondition::Charmed
    ));
    assert!(!protection_prevents_condition_from_creature_type(
        RuntimeCreatureType::Humanoid,
        true,
        ProtectedCondition::Charmed
    ));
    assert!(protection_prevents_possession_from_creature_type(
        RuntimeCreatureType::Fiend,
        true
    ));
    assert_eq!(
        protection_relevant_effect_save_roll_mode(RuntimeCreatureType::Fiend, true, true),
        ProtectionSavingThrowMode::Advantage
    );
    assert_eq!(
        protection_relevant_effect_save_roll_mode(RuntimeCreatureType::Fiend, true, false),
        ProtectionSavingThrowMode::Normal
    );

    let protection =
        resolve_protection_from_evil_and_good_target(creature_type_protection_initial_state());
    let attack_projection =
        project_protection_scoped_attack_roll(creature_type_protection_initial_state());
    let prevention_projection = project_protection_condition_and_possession_prevention(
        creature_type_protection_initial_state(),
    );
    let save_projection =
        resolve_protection_relevant_charm_save(creature_type_protection_initial_state());

    assert!(protection.known_willing_protection_target_admitted);
    assert!(protection.plain_protection_target_rejected);
    assert!(protection.protection_effect_present);
    assert!(attack_projection.scoped_attack_roll_disadvantage);
    assert!(attack_projection.unscoped_attack_roll_normal);
    assert!(prevention_projection.scoped_charm_prevented);
    assert!(prevention_projection.unscoped_charm_applied);
    assert!(prevention_projection.scoped_possession_prevented);
    assert!(prevention_projection.unscoped_possession_unprevented);
    assert!(save_projection.relevant_charm_save_has_advantage);
    assert!(save_projection.relevant_charm_save_cleared);
}

#[test]
fn danger_sense_adapter_replays_all_branches() {
    // QNT: cleanroom-input/qnt/battle-runtime/
    // battle-runtime-danger-sense-selected-identity.mbt.qnt; RAW:
    // cleanroom-input/raw/srd-5.2.1/Classes/Barbarian.md
    // "Level 2: Danger Sense".
    for action in DANGER_SENSE_BRANCH_ACTIONS {
        let observed = replay_danger_sense_action(action);
        assert_eq!(observed, expected_danger_sense_witness(action));
        assert!(danger_sense_projection_payload(&observed).contains("protocolResult="));
    }
}

#[test]
fn danger_sense_only_advantages_dexterity_saves_while_not_incapacitated() {
    // RAW: cleanroom-input/raw/srd-5.2.1/Classes/Barbarian.md
    // "Level 2: Danger Sense"; QNT:
    // unit-feature-save-damage-core-examples.qnt passiveSavingThrowRollMode.
    assert_eq!(
        danger_sense_saving_throw_roll_mode(DangerSenseFacts {
            saving_throw_ability: SavingThrowAbility::Dexterity,
            actor_incapacitated: false,
        }),
        PassiveSavingThrowRollMode::Advantage
    );
    assert_eq!(
        danger_sense_saving_throw_roll_mode(DangerSenseFacts {
            saving_throw_ability: SavingThrowAbility::Constitution,
            actor_incapacitated: false,
        }),
        PassiveSavingThrowRollMode::Normal
    );
    assert_eq!(
        danger_sense_saving_throw_roll_mode(DangerSenseFacts {
            saving_throw_ability: SavingThrowAbility::Dexterity,
            actor_incapacitated: true,
        }),
        PassiveSavingThrowRollMode::Normal
    );

    let projected = project_danger_sense_dexterity_advantage(danger_sense_initial_state());
    let suppressed = suppress_danger_sense_while_incapacitated(danger_sense_initial_state());

    assert_eq!(
        projected.scenario_outcome,
        DangerSenseScenarioOutcome::DexterityAdvantage
    );
    assert_eq!(projected.protocol, DangerSenseProtocol::Resolved);
    assert_eq!(projected.dexterity_roll_mode_count, 1);
    assert_eq!(projected.constitution_roll_mode_count, 0);
    assert!(projected.feature_present);
    assert!(projected.accepted);
    assert!(!projected.suppressed);

    assert_eq!(
        suppressed.scenario_outcome,
        DangerSenseScenarioOutcome::IncapacitatedSuppressed
    );
    assert_eq!(suppressed.protocol, DangerSenseProtocol::Resolved);
    assert_eq!(suppressed.dexterity_roll_mode_count, 0);
    assert!(suppressed.feature_present);
    assert!(suppressed.accepted);
    assert!(suppressed.suppressed);
}

#[test]
fn death_saving_throw_adapter_replays_all_branches() {
    // QNT: cleanroom-input/qnt/battle-runtime/
    // battle-runtime-death-saving-throw.mbt.qnt; RAW:
    // cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md
    // "Death Saving Throws".
    for action in DEATH_SAVING_THROW_BRANCH_ACTIONS {
        let observed = replay_death_saving_throw_action(action);
        assert_eq!(observed, expected_death_saving_throw_witness(action));
        assert!(death_saving_throw_projection_payload(&observed).contains("protocolResult="));
    }

    for natural_d20 in FILL_SAMPLE_NATURAL_D20S {
        let observed = replay_fill_death_saving_throw(natural_d20);
        assert!(death_saving_throw_projection_payload(&observed).contains("currentTurnRole=target"));
    }
}

#[test]
fn death_saving_throw_resolves_all_sampled_edges() {
    // RAW: cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md
    // "Death Saving Throws"; RAW:
    // cleanroom-input/raw/srd-5.2.1/Rules-Glossary.md
    // "Death Saving Throw"; QNT:
    // battle-runtime-death-saving-throw.mbt.qnt.
    let discovered = discover_death_saving_throw(death_saving_throw_initial_state());
    assert_eq!(
        discovered.protocol,
        DeathSavingThrowProtocol::NeedsSavingThrow
    );

    let early_state = crate::rules::hit_points::DeathSavingThrowState {
        target_death_successes: 0,
        target_death_failures: 0,
        ..discovered
    };
    let early_success =
        fill_death_saving_throw(early_state, DeathSavingThrowFacts { natural_d20: 10 });
    let early_failure =
        fill_death_saving_throw(early_state, DeathSavingThrowFacts { natural_d20: 5 });
    assert_eq!(early_success.target_death_successes, 1);
    assert!(!early_success.target_stable);
    assert_eq!(early_failure.target_death_failures, 1);
    assert!(!early_failure.target_dead);

    let natural_one = fill_death_saving_throw(discovered, DeathSavingThrowFacts { natural_d20: 1 });
    assert_eq!(
        natural_one.current_turn_role,
        DeathSavingThrowTurnRole::Target
    );
    assert!(natural_one.target_dead);
    assert_eq!(natural_one.target_death_failures, 3);
    assert_eq!(natural_one.protocol, DeathSavingThrowProtocol::Resolved);

    let ordinary_failure =
        fill_death_saving_throw(discovered, DeathSavingThrowFacts { natural_d20: 5 });
    assert_eq!(ordinary_failure.target_death_failures, 2);
    assert!(!ordinary_failure.target_dead);
    assert_eq!(
        ordinary_failure.protocol,
        DeathSavingThrowProtocol::Resolved
    );

    let stable = fill_death_saving_throw(discovered, DeathSavingThrowFacts { natural_d20: 10 });
    assert!(stable.target_stable);
    assert_eq!(stable.target_death_successes, 0);
    assert_eq!(stable.target_death_failures, 0);
    assert_eq!(stable.protocol, DeathSavingThrowProtocol::Resolved);

    let natural_twenty =
        fill_death_saving_throw(discovered, DeathSavingThrowFacts { natural_d20: 20 });
    assert_eq!(natural_twenty.target_hp, 1);
    assert!(!natural_twenty.target_unconscious);
    assert_eq!(natural_twenty.target_death_successes, 0);
    assert_eq!(natural_twenty.target_death_failures, 0);
    assert_eq!(natural_twenty.protocol, DeathSavingThrowProtocol::Resolved);

    let invalid_fill =
        fill_death_saving_throw(discovered, DeathSavingThrowFacts { natural_d20: 0 });
    assert_eq!(
        invalid_fill.protocol,
        DeathSavingThrowProtocol::Invalid(DeathSavingThrowInvalidReason::InvalidFill)
    );

    let wrong_actor = reject_wrong_actor_after_resolved(natural_twenty);
    assert_eq!(
        wrong_actor.protocol,
        DeathSavingThrowProtocol::Invalid(DeathSavingThrowInvalidReason::WrongActor)
    );
}

#[test]
fn dragonborn_breath_weapon_adapter_replays_all_branches() {
    // QNT: cleanroom-input/qnt/battle-runtime/
    // battle-runtime-dragonborn-breath-weapon.mbt.qnt; RAW:
    // cleanroom-input/raw/srd-5.2.1/Character-Origins.md
    // "Dragonborn", "Breath Weapon".
    for action in DRAGONBORN_BREATH_WEAPON_BRANCH_ACTIONS {
        let observed = replay_dragonborn_breath_weapon_action(action);
        assert_eq!(observed, expected_dragonborn_breath_weapon_witness(action));
        assert!(dragonborn_breath_weapon_projection_payload(&observed).contains("protocolResult="));
    }
}

#[test]
fn dragonborn_breath_weapon_spends_use_and_replaces_attack_with_save_damage() {
    // RAW: cleanroom-input/raw/srd-5.2.1/Character-Origins.md
    // "Dragonborn", "Breath Weapon"; domain:
    // cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md "Action Resource Modeling",
    // "Saving Throw", "Proficiency Bonus", "Damage Type", and
    // "Area of Effect".
    assert_eq!(dragonborn_breath_weapon_save_dc(3, 2), 13);
    assert_eq!(dragonborn_breath_weapon_damage_die_count(1), Some(1));
    assert_eq!(dragonborn_breath_weapon_damage_die_count(5), Some(2));
    assert_eq!(dragonborn_breath_weapon_damage_die_count(11), Some(3));
    assert_eq!(dragonborn_breath_weapon_damage_die_count(17), Some(4));
    assert_eq!(dragonborn_breath_weapon_damage_die_count(0), None);

    let resolved = resolve_dragonborn_breath_weapon(
        dragonborn_breath_weapon_initial_state(),
        DragonbornBreathWeaponFacts {
            character_level: 1,
            damage_roll: 10,
            target_saving_throw_succeeded: false,
            second_target_in_area: true,
            second_target_saving_throw_succeeded: true,
            area_shape: BreathWeaponAreaShape::Cone15Feet,
            expected_area_shape: BreathWeaponAreaShape::Cone15Feet,
            opens_extra_attack_slot: false,
        },
    );
    assert_eq!(
        resolved.scenario_outcome,
        DragonbornBreathWeaponScenarioOutcome::Resolved
    );
    assert_eq!(resolved.target_hit_points, 10);
    assert_eq!(resolved.second_target_hit_points, 15);
    assert_eq!(resolved.breath_weapon_uses_remaining, 2);
    assert_eq!(resolved.attack_action_attacks_remaining, 0);
    assert_eq!(resolved.protocol, DragonbornBreathWeaponProtocol::Resolved);

    let opened_extra_attack = resolve_dragonborn_breath_weapon(
        dragonborn_breath_weapon_initial_state(),
        DragonbornBreathWeaponFacts {
            character_level: 1,
            damage_roll: 10,
            target_saving_throw_succeeded: false,
            second_target_in_area: false,
            second_target_saving_throw_succeeded: true,
            area_shape: BreathWeaponAreaShape::Cone15Feet,
            expected_area_shape: BreathWeaponAreaShape::Cone15Feet,
            opens_extra_attack_slot: true,
        },
    );
    assert_eq!(
        opened_extra_attack.scenario_outcome,
        DragonbornBreathWeaponScenarioOutcome::OpenedExtraAttack
    );
    assert_eq!(opened_extra_attack.target_hit_points, 10);
    assert_eq!(opened_extra_attack.second_target_hit_points, 20);
    assert_eq!(opened_extra_attack.breath_weapon_uses_remaining, 2);
    assert_eq!(opened_extra_attack.attack_action_attacks_remaining, 1);

    let missing_resource =
        reject_dragonborn_breath_weapon_missing_resource(dragonborn_breath_weapon_initial_state());
    let mismatched_area =
        reject_dragonborn_breath_weapon_mismatched_area(dragonborn_breath_weapon_initial_state());
    let invalid_damage = reject_dragonborn_breath_weapon_invalid_damage_roll(
        dragonborn_breath_weapon_initial_state(),
    );

    assert_eq!(
        missing_resource.scenario_outcome,
        DragonbornBreathWeaponScenarioOutcome::RejectMissingResource
    );
    assert_eq!(missing_resource.breath_weapon_uses_remaining, 0);
    assert_eq!(
        mismatched_area.scenario_outcome,
        DragonbornBreathWeaponScenarioOutcome::RejectMismatchedArea
    );
    assert_eq!(
        invalid_damage.scenario_outcome,
        DragonbornBreathWeaponScenarioOutcome::RejectInvalidDamageRoll
    );
    assert_eq!(
        invalid_damage.protocol,
        DragonbornBreathWeaponProtocol::Invalid(DragonbornBreathWeaponInvalidReason::InvalidFill)
    );
}

#[test]
fn druid_wild_shape_form_lifecycle_adapter_replays_all_branches() {
    // QNT: cleanroom-input/qnt/battle-runtime/
    // battle-runtime-druid-wild-shape-form-lifecycle.mbt.qnt; RAW:
    // cleanroom-input/raw/srd-5.2.1/Classes/Druid.md
    // "Level 2: Wild Shape".
    for action in DRUID_WILD_SHAPE_BRANCH_ACTIONS {
        let observed = replay_druid_wild_shape_action(action);
        assert_eq!(observed, expected_druid_wild_shape_witness(action));
        assert!(druid_wild_shape_projection_payload(&observed).contains("protocolResult="));
    }
}

#[test]
fn druid_wild_shape_assumes_reuses_dismisses_and_reverts_forms() {
    // RAW: cleanroom-input/raw/srd-5.2.1/Classes/Druid.md
    // "Level 2: Wild Shape"; RAW:
    // cleanroom-input/raw/srd-5.2.1/Rules-Glossary.md
    // "Shape-Shifting" and "Incapacitated"; QNT:
    // battle-runtime-druid-wild-shape.qnt.
    let initial = wild_shape_initial_state();
    assert_eq!(initial.active_form, WildShapeForm::TrueForm);
    assert!(initial.bonus_action_available);
    assert_eq!(initial.uses_remaining, 2);
    assert!(initial.spell_available);

    let assumed = assume_riding_horse_wild_shape(initial);
    assert_eq!(assumed.active_form, WildShapeForm::RidingHorse);
    assert!(!assumed.bonus_action_available);
    assert_eq!(assumed.uses_remaining, 1);
    assert_eq!(assumed.temporary_hit_points, 2);
    assert_eq!(assumed.armor_class, 11);
    assert_eq!(assumed.creature_size, WildShapeCreatureSize::Large);
    assert_eq!(assumed.speed_feet, 60);
    assert_eq!(assumed.shove_dc, 13);
    assert!(!assumed.spell_available);
    assert_eq!(assumed.active_form_effect_count, 1);
    assert_eq!(assumed.merged_equipment_count, 2);
    assert_eq!(
        assumed.scenario_outcome,
        WildShapeScenarioOutcome::AssumedRidingHorse
    );
    assert_eq!(assumed.protocol, WildShapeProtocol::Resolved);

    let next_turn = begin_wild_shape_next_turn(assumed);
    assert!(next_turn.bonus_action_available);
    assert_eq!(
        next_turn.scenario_outcome,
        WildShapeScenarioOutcome::NextTurn
    );

    let reused = reuse_wild_shape_as_cat(next_turn);
    assert_eq!(reused.active_form, WildShapeForm::Cat);
    assert_eq!(reused.uses_remaining, 0);
    assert_eq!(reused.creature_size, WildShapeCreatureSize::Tiny);
    assert_eq!(reused.speed_feet, 40);
    assert_eq!(reused.shove_dc, 6);
    assert_eq!(reused.scenario_outcome, WildShapeScenarioOutcome::ReusedCat);

    let dismissed = dismiss_wild_shape_form(next_turn);
    assert_eq!(dismissed.active_form, WildShapeForm::TrueForm);
    assert!(!dismissed.bonus_action_available);
    assert_eq!(dismissed.uses_remaining, 1);
    assert_eq!(dismissed.temporary_hit_points, 2);
    assert_eq!(dismissed.armor_class, 16);
    assert_eq!(dismissed.creature_size, WildShapeCreatureSize::Medium);
    assert!(dismissed.spell_available);
    assert_eq!(dismissed.active_form_effect_count, 0);
    assert_eq!(dismissed.merged_equipment_count, 0);

    let incapacitated = revert_wild_shape_due_to_incapacitated(assumed);
    assert_eq!(incapacitated.active_form, WildShapeForm::TrueForm);
    assert_eq!(
        incapacitated.druid_status,
        WildShapeDruidStatus::Incapacitated
    );
    assert!(!incapacitated.spell_available);
    assert_eq!(
        incapacitated.scenario_outcome,
        WildShapeScenarioOutcome::FormIncapacitated
    );

    let dead = revert_wild_shape_due_to_death(assumed);
    assert_eq!(dead.active_form, WildShapeForm::TrueForm);
    assert_eq!(dead.druid_status, WildShapeDruidStatus::Dead);
    assert!(!dead.spell_available);
    assert_eq!(stutter_wild_shape_state(dead), dead);
}
