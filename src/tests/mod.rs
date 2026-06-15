#[path = "../qnt_adapters/character_creation_class_feature_projections.rs"]
mod character_creation_class_feature_projections;
#[path = "../qnt_adapters/character_creation_cleric_druid_order_selected_identity.rs"]
mod character_creation_cleric_druid_order_selected_identity;
#[path = "../qnt_adapters/character_creation_fighter_fighting_style_selected_identity.rs"]
mod character_creation_fighter_fighting_style_selected_identity;
#[path = "../qnt_adapters/character_creation_runtime.rs"]
mod character_creation_runtime;

use crate::rules::class_features::{
    cleric_divine_order_projection, druid_primal_order_projection,
    fighter_fighting_style_projection, level_two_feature_projection, Cantrip, ClassLevel,
    ClericDivineOrder, DruidPrimalOrder, FeatureSet, FighterFightingStyleSelection,
    FightingStyleFeat, MetamagicOption, ProjectionError,
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
