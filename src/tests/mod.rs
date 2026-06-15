#[path = "../qnt_adapters/character_creation_class_feature_projections.rs"]
mod character_creation_class_feature_projections;

use crate::rules::class_features::{
    level_two_feature_projection, FeatureSet, MetamagicOption, ProjectionError,
};

use character_creation_class_feature_projections::{
    expected_monk_witness, expected_sorcerer_witness, projection_payload, replay_observed_action,
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
