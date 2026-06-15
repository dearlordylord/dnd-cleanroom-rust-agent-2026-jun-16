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
use crate::rules::class_features::{
    apply_weapon_mastery_long_rest_reselection, cleric_divine_order_projection,
    druid_primal_order_projection, fighter_fighting_style_projection, level_two_feature_projection,
    weapon_mastery_projection, Cantrip, ClassLevel, ClericDivineOrder, DruidPrimalOrder,
    FeatureSet, FighterFightingStyleSelection, FightingStyleFeat, MetamagicOption, ProjectionError,
    Weapon, WeaponMasteryClass, WeaponMasteryReselectionFacts,
};
use crate::rules::feature_resources::{
    apply_lay_on_hands_resource, FeatureResourceHitPoints, ResourcePoolError, ResourcePoolFacts,
};
use crate::rules::hit_points::{
    complete_long_rest_benefits, fixed_higher_level_hit_point_gain, hit_point_maximum_projection,
    spend_hit_point_die, HitPointMaximumFacts, SheetHitPointState,
};
use crate::rules::origin_feats::{
    criminal_origin_feat_projection, initiative_handoff_projection, AlertInitiativeState,
    Background, InitiativeHandoffFacts, OriginFeat,
};
use crate::rules::spellbook_rituals::{
    can_cast_spellbook_ritual, spellbook_ritual_invocation, PreparationRequirement,
    RequiredSpellAccess, SpellSlotCost, SpellbookRitualAccess, SpellbookRitualAdept,
    SpellbookRitualFacts, SpellbookRitualResult, SpellbookSpellKind,
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
