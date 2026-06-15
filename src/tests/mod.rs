#[path = "../qnt_adapters/character_creation_class_feature_projections.rs"]
mod character_creation_class_feature_projections;
#[path = "../qnt_adapters/character_creation_cleric_druid_order_selected_identity.rs"]
mod character_creation_cleric_druid_order_selected_identity;

use crate::rules::class_features::{
    cleric_divine_order_projection, druid_primal_order_projection, level_two_feature_projection,
    Cantrip, ClericDivineOrder, DruidPrimalOrder, FeatureSet, MetamagicOption, ProjectionError,
};

use character_creation_class_feature_projections::{
    expected_monk_witness, expected_sorcerer_witness, projection_payload, replay_observed_action,
};
use character_creation_cleric_druid_order_selected_identity::{
    expected_cleric_protector_witness, expected_cleric_thaumaturge_witness,
    expected_druid_magician_witness, expected_druid_warden_witness,
    projection_payload as order_projection_payload, replay_observed_action as replay_order_action,
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
