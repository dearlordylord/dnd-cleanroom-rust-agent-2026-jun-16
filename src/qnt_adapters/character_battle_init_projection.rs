use crate::rules::character_battle_handoff::{
    project_pure_pact_magic_slot_for_battle,
    project_sheet_hit_points_armor_class_conditions_and_profiles_for_battle,
    project_sheet_spellcasting_and_metamagic_for_battle,
    reject_battle_initialization_mixed_spell_and_pact_slot,
    reject_battle_initialization_sheet_hit_point_maximum_exceeds_build,
    reject_battle_initialization_stable_recovery_progress, route_enter_battle_runtime,
    route_project_character_sheet_to_battle, route_reject_character_battle_handoff,
    CharacterBattleIdentity, CharacterBattleInitProjection,
    CharacterBattleInitProjectionAcceptance, CharacterBattleInitProjectionRejection,
    CharacterBattleRouteEvent, CharacterBattleRouteFillFamily, CharacterBattleRouteHoleFamily,
    CharacterBattleRouteOwnerGroup, CharacterBattleRouteSubjectFamily,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterBattleInitProjectionWitness {
    pub last_result: &'static str,
    pub accepted: bool,
    pub message: &'static str,
    pub character_identity: &'static str,
    pub current_hp: i16,
    pub max_hp: i16,
    pub temporary_hit_points: i16,
    pub armor_class: i16,
    pub poisoned: bool,
    pub spell_level_1_count: i16,
    pub spell_level_1_expended: i16,
    pub spell_level_2_count: i16,
    pub spell_level_2_expended: i16,
    pub spell_level_3_count: i16,
    pub spell_level_3_expended: i16,
    pub passive_armor_class_profile_count: i16,
    pub metamagic_known_options: i16,
    pub replay_index: u8,
}

pub const BRANCH_ACTIONS: [&str; 5] = [
    "doProjectSheetHitPointsArmorClassConditionsAndProfiles",
    "doProjectPurePactMagicSlot",
    "doRejectMixedSpellAndPactSlotInit",
    "doRejectBuildMaximumAboveBuildMaximum",
    "doRejectStableRecoveryProgressDuringInit",
];

pub fn replay_observed_action(observed_action_taken: &str) -> CharacterBattleInitProjectionWitness {
    match observed_action_taken {
        "doProjectSheetHitPointsArmorClassConditionsAndProfiles" => {
            sheet_hit_points_armor_class_conditions_and_profiles()
        }
        "doProjectSheetSpellcastingAndMetamagic" => sheet_spellcasting_and_metamagic(),
        "doProjectPurePactMagicSlot" => pure_pact_magic_slot(),
        "doRejectMixedSpellAndPactSlotInit" => reject_mixed_spell_and_pact_slot_init(),
        "doRejectBuildMaximumAboveBuildMaximum" => reject_build_maximum_above_build_maximum(),
        "doRejectStableRecoveryProgressDuringInit" => reject_stable_recovery_progress_during_init(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> CharacterBattleInitProjectionWitness {
    match observed_action_taken {
        "doProjectSheetHitPointsArmorClassConditionsAndProfiles" => {
            expected_sheet_hit_points_armor_class_conditions_and_profiles()
        }
        "doProjectSheetSpellcastingAndMetamagic" => expected_sheet_spellcasting_and_metamagic(),
        "doProjectPurePactMagicSlot" => expected_pure_pact_magic_slot(),
        "doRejectMixedSpellAndPactSlotInit" => expected_reject_mixed_spell_and_pact_slot_init(),
        "doRejectBuildMaximumAboveBuildMaximum" => {
            expected_reject_build_maximum_above_build_maximum()
        }
        "doRejectStableRecoveryProgressDuringInit" => {
            expected_reject_stable_recovery_progress_during_init()
        }
        action => panic!("unsupported expected mbt::actionTaken {action}"),
    }
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<CharacterBattleRouteEvent> {
    match observed_action_taken {
        "doProjectSheetHitPointsArmorClassConditionsAndProfiles" => {
            route_after_sheet_hit_points_armor_class_conditions_and_profiles()
        }
        "doProjectSheetSpellcastingAndMetamagic" => route_after_sheet_spellcasting_and_metamagic(),
        "doProjectPurePactMagicSlot" => route_after_pure_pact_magic_slot(),
        "doRejectMixedSpellAndPactSlotInit" => route_after_reject_mixed_spell_and_pact_slot_init(),
        "doRejectBuildMaximumAboveBuildMaximum" => route_after_reject_build_maximum(),
        "doRejectStableRecoveryProgressDuringInit" => route_after_reject_stable_recovery_progress(),
        action => panic!("unsupported route mbt::actionTaken {action}"),
    }
}

pub fn expected_route(observed_action_taken: &str) -> Vec<CharacterBattleRouteEvent> {
    match observed_action_taken {
        "doProjectSheetHitPointsArmorClassConditionsAndProfiles" => {
            expected_route_after_sheet_hit_points_armor_class_conditions_and_profiles()
        }
        "doProjectSheetSpellcastingAndMetamagic" => {
            expected_route_after_sheet_spellcasting_and_metamagic()
        }
        "doProjectPurePactMagicSlot" => expected_route_after_pure_pact_magic_slot(),
        "doRejectMixedSpellAndPactSlotInit" => {
            expected_route_after_reject_mixed_spell_and_pact_slot_init()
        }
        "doRejectBuildMaximumAboveBuildMaximum" => expected_route_after_reject_build_maximum(),
        "doRejectStableRecoveryProgressDuringInit" => {
            expected_route_after_reject_stable_recovery_progress()
        }
        action => panic!("unsupported expected route mbt::actionTaken {action}"),
    }
}

pub fn projection_payload(witness: &CharacterBattleInitProjectionWitness) -> String {
    [
        format!("lastResult={}", witness.last_result),
        format!("accepted={}", witness.accepted),
        format!("message={}", witness.message),
        format!("characterIdentity={}", witness.character_identity),
        format!("currentHp={}", witness.current_hp),
        format!("maxHp={}", witness.max_hp),
        format!("temporaryHitPoints={}", witness.temporary_hit_points),
        format!("armorClass={}", witness.armor_class),
        format!("poisoned={}", witness.poisoned),
        format!("spellLevel1Count={}", witness.spell_level_1_count),
        format!("spellLevel1Expended={}", witness.spell_level_1_expended),
        format!("spellLevel2Count={}", witness.spell_level_2_count),
        format!("spellLevel2Expended={}", witness.spell_level_2_expended),
        format!("spellLevel3Count={}", witness.spell_level_3_count),
        format!("spellLevel3Expended={}", witness.spell_level_3_expended),
        format!(
            "passiveArmorClassProfileCount={}",
            witness.passive_armor_class_profile_count
        ),
        format!("metamagicKnownOptions={}", witness.metamagic_known_options),
        format!("replayIndex={}", witness.replay_index),
    ]
    .join("\n")
}

fn sheet_hit_points_armor_class_conditions_and_profiles() -> CharacterBattleInitProjectionWitness {
    witness_from_projection(
        project_sheet_hit_points_armor_class_conditions_and_profiles_for_battle(),
        1,
    )
}

fn sheet_spellcasting_and_metamagic() -> CharacterBattleInitProjectionWitness {
    witness_from_projection(project_sheet_spellcasting_and_metamagic_for_battle(), 2)
}

fn reject_build_maximum_above_build_maximum() -> CharacterBattleInitProjectionWitness {
    witness_from_projection(
        reject_battle_initialization_sheet_hit_point_maximum_exceeds_build(),
        5,
    )
}

fn reject_stable_recovery_progress_during_init() -> CharacterBattleInitProjectionWitness {
    witness_from_projection(reject_battle_initialization_stable_recovery_progress(), 6)
}

fn pure_pact_magic_slot() -> CharacterBattleInitProjectionWitness {
    witness_from_projection(project_pure_pact_magic_slot_for_battle(), 3)
}

fn reject_mixed_spell_and_pact_slot_init() -> CharacterBattleInitProjectionWitness {
    witness_from_projection(reject_battle_initialization_mixed_spell_and_pact_slot(), 4)
}

fn expected_sheet_hit_points_armor_class_conditions_and_profiles(
) -> CharacterBattleInitProjectionWitness {
    CharacterBattleInitProjectionWitness {
        last_result: "sheet-hit-points-armor-class-conditions-and-profiles",
        accepted: true,
        message: "none",
        character_identity: "character:battle-init-fighter",
        current_hp: 6,
        max_hp: 9,
        temporary_hit_points: 4,
        armor_class: 17,
        poisoned: true,
        spell_level_1_count: 0,
        spell_level_1_expended: 0,
        spell_level_2_count: 0,
        spell_level_2_expended: 0,
        spell_level_3_count: 0,
        spell_level_3_expended: 0,
        passive_armor_class_profile_count: 1,
        metamagic_known_options: 0,
        replay_index: 1,
    }
}

fn expected_sheet_spellcasting_and_metamagic() -> CharacterBattleInitProjectionWitness {
    CharacterBattleInitProjectionWitness {
        last_result: "sheet-spellcasting-and-metamagic",
        accepted: true,
        message: "none",
        character_identity: "character:battle-init-sorcerer",
        current_hp: 24,
        max_hp: 27,
        temporary_hit_points: 0,
        armor_class: 12,
        poisoned: false,
        spell_level_1_count: 4,
        spell_level_1_expended: 1,
        spell_level_2_count: 3,
        spell_level_2_expended: 0,
        spell_level_3_count: 3,
        spell_level_3_expended: 0,
        passive_armor_class_profile_count: 0,
        metamagic_known_options: 2,
        replay_index: 2,
    }
}

fn expected_reject_build_maximum_above_build_maximum() -> CharacterBattleInitProjectionWitness {
    expected_rejection_witness(
        "build-maximum-above-build-maximum-rejected",
        "Character battle initialization max HP exceeds build-derived max HP.",
        5,
    )
}

fn expected_reject_stable_recovery_progress_during_init() -> CharacterBattleInitProjectionWitness {
    expected_rejection_witness(
        "stable-recovery-progress-during-init-rejected",
        "Battle handoff cannot preserve in-progress Stable recovery timers.",
        6,
    )
}

fn expected_pure_pact_magic_slot() -> CharacterBattleInitProjectionWitness {
    CharacterBattleInitProjectionWitness {
        last_result: "pure-pact-magic-slot-projection",
        accepted: true,
        message: "none",
        character_identity: "character:battle-init-warlock",
        current_hp: 8,
        max_hp: 9,
        temporary_hit_points: 0,
        armor_class: 12,
        poisoned: false,
        spell_level_1_count: 1,
        spell_level_1_expended: 0,
        spell_level_2_count: 0,
        spell_level_2_expended: 0,
        spell_level_3_count: 0,
        spell_level_3_expended: 0,
        passive_armor_class_profile_count: 0,
        metamagic_known_options: 0,
        replay_index: 3,
    }
}

fn expected_reject_mixed_spell_and_pact_slot_init() -> CharacterBattleInitProjectionWitness {
    expected_rejection_witness(
        "mixed-spell-and-pact-slot-init-rejected",
        "Battle handoff cannot project mixed Spell Slot and Pact Slot state without origin-distinct battle slots.",
        4,
    )
}

fn expected_rejection_witness(
    last_result: &'static str,
    message: &'static str,
    replay_index: u8,
) -> CharacterBattleInitProjectionWitness {
    CharacterBattleInitProjectionWitness {
        last_result,
        accepted: false,
        message,
        character_identity: "none",
        current_hp: 0,
        max_hp: 0,
        temporary_hit_points: 0,
        armor_class: 0,
        poisoned: false,
        spell_level_1_count: 0,
        spell_level_1_expended: 0,
        spell_level_2_count: 0,
        spell_level_2_expended: 0,
        spell_level_3_count: 0,
        spell_level_3_expended: 0,
        passive_armor_class_profile_count: 0,
        metamagic_known_options: 0,
        replay_index,
    }
}

fn route_after_sheet_hit_points_armor_class_conditions_and_profiles(
) -> Vec<CharacterBattleRouteEvent> {
    let mut route = Vec::new();
    append_sheet_hit_points_armor_class_conditions_and_profiles_route(&mut route);
    route
}

fn route_after_sheet_spellcasting_and_metamagic() -> Vec<CharacterBattleRouteEvent> {
    let mut route = route_after_sheet_hit_points_armor_class_conditions_and_profiles();
    append_sheet_spellcasting_and_metamagic_route(&mut route);
    route
}

fn route_after_reject_build_maximum() -> Vec<CharacterBattleRouteEvent> {
    let mut route = route_after_reject_mixed_spell_and_pact_slot_init();
    append_reject_build_maximum_route(&mut route);
    route
}

fn route_after_reject_stable_recovery_progress() -> Vec<CharacterBattleRouteEvent> {
    let mut route = route_after_reject_build_maximum();
    append_reject_stable_recovery_route(&mut route);
    route
}

fn expected_route_after_sheet_hit_points_armor_class_conditions_and_profiles(
) -> Vec<CharacterBattleRouteEvent> {
    vec![
        expected_project_character_sheet_to_battle(
            CharacterBattleRouteSubjectFamily::SheetToBattleInitRouteSubject,
            CharacterBattleRouteOwnerGroup::CharacterBattleSheetOwner,
        ),
        expected_project_character_sheet_to_battle(
            CharacterBattleRouteSubjectFamily::SheetToBattleInitRouteSubject,
            CharacterBattleRouteOwnerGroup::CharacterBattleBuildProjectionOwner,
        ),
        expected_enter_battle_runtime(
            CharacterBattleRouteSubjectFamily::SheetToBattleInitRouteSubject,
            CharacterBattleRouteOwnerGroup::CharacterBattleInitProjectionOwner,
        ),
    ]
}

fn expected_route_after_sheet_spellcasting_and_metamagic() -> Vec<CharacterBattleRouteEvent> {
    vec![
        expected_project_character_sheet_to_battle(
            CharacterBattleRouteSubjectFamily::SheetToBattleInitRouteSubject,
            CharacterBattleRouteOwnerGroup::CharacterBattleSheetOwner,
        ),
        expected_project_character_sheet_to_battle(
            CharacterBattleRouteSubjectFamily::SheetToBattleInitRouteSubject,
            CharacterBattleRouteOwnerGroup::CharacterBattleBuildProjectionOwner,
        ),
        expected_enter_battle_runtime(
            CharacterBattleRouteSubjectFamily::SheetToBattleInitRouteSubject,
            CharacterBattleRouteOwnerGroup::CharacterBattleInitProjectionOwner,
        ),
        expected_project_character_sheet_to_battle(
            CharacterBattleRouteSubjectFamily::HandoffResourceProjectionRouteSubject,
            CharacterBattleRouteOwnerGroup::CharacterBattleSheetOwner,
        ),
        expected_project_character_sheet_to_battle(
            CharacterBattleRouteSubjectFamily::HandoffFeatureResourceProjectionRouteSubject,
            CharacterBattleRouteOwnerGroup::CharacterBattleResourceProjectionOwner,
        ),
        expected_enter_battle_runtime(
            CharacterBattleRouteSubjectFamily::HandoffResourceProjectionRouteSubject,
            CharacterBattleRouteOwnerGroup::CharacterBattleInitProjectionOwner,
        ),
    ]
}

fn expected_route_after_reject_build_maximum() -> Vec<CharacterBattleRouteEvent> {
    vec![
        expected_project_character_sheet_to_battle(
            CharacterBattleRouteSubjectFamily::SheetToBattleInitRouteSubject,
            CharacterBattleRouteOwnerGroup::CharacterBattleSheetOwner,
        ),
        expected_project_character_sheet_to_battle(
            CharacterBattleRouteSubjectFamily::SheetToBattleInitRouteSubject,
            CharacterBattleRouteOwnerGroup::CharacterBattleBuildProjectionOwner,
        ),
        expected_enter_battle_runtime(
            CharacterBattleRouteSubjectFamily::SheetToBattleInitRouteSubject,
            CharacterBattleRouteOwnerGroup::CharacterBattleInitProjectionOwner,
        ),
        expected_project_character_sheet_to_battle(
            CharacterBattleRouteSubjectFamily::HandoffResourceProjectionRouteSubject,
            CharacterBattleRouteOwnerGroup::CharacterBattleSheetOwner,
        ),
        expected_project_character_sheet_to_battle(
            CharacterBattleRouteSubjectFamily::HandoffFeatureResourceProjectionRouteSubject,
            CharacterBattleRouteOwnerGroup::CharacterBattleResourceProjectionOwner,
        ),
        expected_enter_battle_runtime(
            CharacterBattleRouteSubjectFamily::HandoffResourceProjectionRouteSubject,
            CharacterBattleRouteOwnerGroup::CharacterBattleInitProjectionOwner,
        ),
        expected_project_character_sheet_to_battle(
            CharacterBattleRouteSubjectFamily::HandoffResourceProjectionRouteSubject,
            CharacterBattleRouteOwnerGroup::CharacterBattleResourceProjectionOwner,
        ),
        expected_enter_battle_runtime(
            CharacterBattleRouteSubjectFamily::HandoffResourceProjectionRouteSubject,
            CharacterBattleRouteOwnerGroup::CharacterBattleInitProjectionOwner,
        ),
        expected_reject_character_battle_handoff(
            CharacterBattleRouteSubjectFamily::HandoffResourceProjectionRouteSubject,
            CharacterBattleRouteFillFamily::HandoffSheetProjectionFill,
            vec![
                CharacterBattleRouteHoleFamily::HandoffSpellResourceProjectionHoleFamily,
                CharacterBattleRouteHoleFamily::HandoffSettlementConflictHoleFamily,
            ],
            CharacterBattleRouteOwnerGroup::CharacterBattleResourceProjectionOwner,
        ),
        expected_reject_character_battle_handoff(
            CharacterBattleRouteSubjectFamily::SheetToBattleInitRouteSubject,
            CharacterBattleRouteFillFamily::HandoffSheetProjectionFill,
            vec![CharacterBattleRouteHoleFamily::HandoffHitPointProjectionHoleFamily],
            CharacterBattleRouteOwnerGroup::CharacterBattleBuildProjectionOwner,
        ),
    ]
}

fn route_after_pure_pact_magic_slot() -> Vec<CharacterBattleRouteEvent> {
    let mut route = route_after_sheet_spellcasting_and_metamagic();
    append_pure_pact_magic_slot_route(&mut route);
    route
}

fn route_after_reject_mixed_spell_and_pact_slot_init() -> Vec<CharacterBattleRouteEvent> {
    let mut route = route_after_pure_pact_magic_slot();
    append_reject_mixed_spell_and_pact_slot_init_route(&mut route);
    route
}

fn expected_route_after_pure_pact_magic_slot() -> Vec<CharacterBattleRouteEvent> {
    vec![
        expected_project_character_sheet_to_battle(
            CharacterBattleRouteSubjectFamily::SheetToBattleInitRouteSubject,
            CharacterBattleRouteOwnerGroup::CharacterBattleSheetOwner,
        ),
        expected_project_character_sheet_to_battle(
            CharacterBattleRouteSubjectFamily::SheetToBattleInitRouteSubject,
            CharacterBattleRouteOwnerGroup::CharacterBattleBuildProjectionOwner,
        ),
        expected_enter_battle_runtime(
            CharacterBattleRouteSubjectFamily::SheetToBattleInitRouteSubject,
            CharacterBattleRouteOwnerGroup::CharacterBattleInitProjectionOwner,
        ),
        expected_project_character_sheet_to_battle(
            CharacterBattleRouteSubjectFamily::HandoffResourceProjectionRouteSubject,
            CharacterBattleRouteOwnerGroup::CharacterBattleSheetOwner,
        ),
        expected_project_character_sheet_to_battle(
            CharacterBattleRouteSubjectFamily::HandoffFeatureResourceProjectionRouteSubject,
            CharacterBattleRouteOwnerGroup::CharacterBattleResourceProjectionOwner,
        ),
        expected_enter_battle_runtime(
            CharacterBattleRouteSubjectFamily::HandoffResourceProjectionRouteSubject,
            CharacterBattleRouteOwnerGroup::CharacterBattleInitProjectionOwner,
        ),
        expected_project_character_sheet_to_battle(
            CharacterBattleRouteSubjectFamily::HandoffResourceProjectionRouteSubject,
            CharacterBattleRouteOwnerGroup::CharacterBattleResourceProjectionOwner,
        ),
        expected_enter_battle_runtime(
            CharacterBattleRouteSubjectFamily::HandoffResourceProjectionRouteSubject,
            CharacterBattleRouteOwnerGroup::CharacterBattleInitProjectionOwner,
        ),
    ]
}

fn expected_route_after_reject_mixed_spell_and_pact_slot_init() -> Vec<CharacterBattleRouteEvent> {
    vec![
        expected_project_character_sheet_to_battle(
            CharacterBattleRouteSubjectFamily::SheetToBattleInitRouteSubject,
            CharacterBattleRouteOwnerGroup::CharacterBattleSheetOwner,
        ),
        expected_project_character_sheet_to_battle(
            CharacterBattleRouteSubjectFamily::SheetToBattleInitRouteSubject,
            CharacterBattleRouteOwnerGroup::CharacterBattleBuildProjectionOwner,
        ),
        expected_enter_battle_runtime(
            CharacterBattleRouteSubjectFamily::SheetToBattleInitRouteSubject,
            CharacterBattleRouteOwnerGroup::CharacterBattleInitProjectionOwner,
        ),
        expected_project_character_sheet_to_battle(
            CharacterBattleRouteSubjectFamily::HandoffResourceProjectionRouteSubject,
            CharacterBattleRouteOwnerGroup::CharacterBattleSheetOwner,
        ),
        expected_project_character_sheet_to_battle(
            CharacterBattleRouteSubjectFamily::HandoffFeatureResourceProjectionRouteSubject,
            CharacterBattleRouteOwnerGroup::CharacterBattleResourceProjectionOwner,
        ),
        expected_enter_battle_runtime(
            CharacterBattleRouteSubjectFamily::HandoffResourceProjectionRouteSubject,
            CharacterBattleRouteOwnerGroup::CharacterBattleInitProjectionOwner,
        ),
        expected_project_character_sheet_to_battle(
            CharacterBattleRouteSubjectFamily::HandoffResourceProjectionRouteSubject,
            CharacterBattleRouteOwnerGroup::CharacterBattleResourceProjectionOwner,
        ),
        expected_enter_battle_runtime(
            CharacterBattleRouteSubjectFamily::HandoffResourceProjectionRouteSubject,
            CharacterBattleRouteOwnerGroup::CharacterBattleInitProjectionOwner,
        ),
        expected_reject_character_battle_handoff(
            CharacterBattleRouteSubjectFamily::HandoffResourceProjectionRouteSubject,
            CharacterBattleRouteFillFamily::HandoffSheetProjectionFill,
            vec![
                CharacterBattleRouteHoleFamily::HandoffSpellResourceProjectionHoleFamily,
                CharacterBattleRouteHoleFamily::HandoffSettlementConflictHoleFamily,
            ],
            CharacterBattleRouteOwnerGroup::CharacterBattleResourceProjectionOwner,
        ),
    ]
}

fn expected_route_after_reject_stable_recovery_progress() -> Vec<CharacterBattleRouteEvent> {
    vec![
        expected_project_character_sheet_to_battle(
            CharacterBattleRouteSubjectFamily::SheetToBattleInitRouteSubject,
            CharacterBattleRouteOwnerGroup::CharacterBattleSheetOwner,
        ),
        expected_project_character_sheet_to_battle(
            CharacterBattleRouteSubjectFamily::SheetToBattleInitRouteSubject,
            CharacterBattleRouteOwnerGroup::CharacterBattleBuildProjectionOwner,
        ),
        expected_enter_battle_runtime(
            CharacterBattleRouteSubjectFamily::SheetToBattleInitRouteSubject,
            CharacterBattleRouteOwnerGroup::CharacterBattleInitProjectionOwner,
        ),
        expected_project_character_sheet_to_battle(
            CharacterBattleRouteSubjectFamily::HandoffResourceProjectionRouteSubject,
            CharacterBattleRouteOwnerGroup::CharacterBattleSheetOwner,
        ),
        expected_project_character_sheet_to_battle(
            CharacterBattleRouteSubjectFamily::HandoffFeatureResourceProjectionRouteSubject,
            CharacterBattleRouteOwnerGroup::CharacterBattleResourceProjectionOwner,
        ),
        expected_enter_battle_runtime(
            CharacterBattleRouteSubjectFamily::HandoffResourceProjectionRouteSubject,
            CharacterBattleRouteOwnerGroup::CharacterBattleInitProjectionOwner,
        ),
        expected_project_character_sheet_to_battle(
            CharacterBattleRouteSubjectFamily::HandoffResourceProjectionRouteSubject,
            CharacterBattleRouteOwnerGroup::CharacterBattleResourceProjectionOwner,
        ),
        expected_enter_battle_runtime(
            CharacterBattleRouteSubjectFamily::HandoffResourceProjectionRouteSubject,
            CharacterBattleRouteOwnerGroup::CharacterBattleInitProjectionOwner,
        ),
        expected_reject_character_battle_handoff(
            CharacterBattleRouteSubjectFamily::HandoffResourceProjectionRouteSubject,
            CharacterBattleRouteFillFamily::HandoffSheetProjectionFill,
            vec![
                CharacterBattleRouteHoleFamily::HandoffSpellResourceProjectionHoleFamily,
                CharacterBattleRouteHoleFamily::HandoffSettlementConflictHoleFamily,
            ],
            CharacterBattleRouteOwnerGroup::CharacterBattleResourceProjectionOwner,
        ),
        expected_reject_character_battle_handoff(
            CharacterBattleRouteSubjectFamily::SheetToBattleInitRouteSubject,
            CharacterBattleRouteFillFamily::HandoffSheetProjectionFill,
            vec![CharacterBattleRouteHoleFamily::HandoffHitPointProjectionHoleFamily],
            CharacterBattleRouteOwnerGroup::CharacterBattleBuildProjectionOwner,
        ),
        expected_reject_character_battle_handoff(
            CharacterBattleRouteSubjectFamily::SheetToBattleInitRouteSubject,
            CharacterBattleRouteFillFamily::HandoffSheetProjectionFill,
            vec![CharacterBattleRouteHoleFamily::HandoffSettlementConflictHoleFamily],
            CharacterBattleRouteOwnerGroup::CharacterBattleInitProjectionOwner,
        ),
    ]
}

fn append_sheet_hit_points_armor_class_conditions_and_profiles_route(
    route: &mut Vec<CharacterBattleRouteEvent>,
) {
    route.push(route_project_character_sheet_to_battle(
        CharacterBattleRouteSubjectFamily::SheetToBattleInitRouteSubject,
        CharacterBattleRouteOwnerGroup::CharacterBattleSheetOwner,
    ));
    route.push(route_project_character_sheet_to_battle(
        CharacterBattleRouteSubjectFamily::SheetToBattleInitRouteSubject,
        CharacterBattleRouteOwnerGroup::CharacterBattleBuildProjectionOwner,
    ));
    route.push(route_enter_battle_runtime(
        CharacterBattleRouteSubjectFamily::SheetToBattleInitRouteSubject,
        CharacterBattleRouteOwnerGroup::CharacterBattleInitProjectionOwner,
    ));
}

fn append_sheet_spellcasting_and_metamagic_route(route: &mut Vec<CharacterBattleRouteEvent>) {
    route.push(route_project_character_sheet_to_battle(
        CharacterBattleRouteSubjectFamily::HandoffResourceProjectionRouteSubject,
        CharacterBattleRouteOwnerGroup::CharacterBattleSheetOwner,
    ));
    route.push(route_project_character_sheet_to_battle(
        CharacterBattleRouteSubjectFamily::HandoffFeatureResourceProjectionRouteSubject,
        CharacterBattleRouteOwnerGroup::CharacterBattleResourceProjectionOwner,
    ));
    route.push(route_enter_battle_runtime(
        CharacterBattleRouteSubjectFamily::HandoffResourceProjectionRouteSubject,
        CharacterBattleRouteOwnerGroup::CharacterBattleInitProjectionOwner,
    ));
}

fn append_reject_build_maximum_route(route: &mut Vec<CharacterBattleRouteEvent>) {
    route.push(route_reject_character_battle_handoff(
        CharacterBattleRouteSubjectFamily::SheetToBattleInitRouteSubject,
        CharacterBattleRouteFillFamily::HandoffSheetProjectionFill,
        vec![CharacterBattleRouteHoleFamily::HandoffHitPointProjectionHoleFamily],
        CharacterBattleRouteOwnerGroup::CharacterBattleBuildProjectionOwner,
    ));
}

fn append_pure_pact_magic_slot_route(route: &mut Vec<CharacterBattleRouteEvent>) {
    route.push(route_project_character_sheet_to_battle(
        CharacterBattleRouteSubjectFamily::HandoffResourceProjectionRouteSubject,
        CharacterBattleRouteOwnerGroup::CharacterBattleResourceProjectionOwner,
    ));
    route.push(route_enter_battle_runtime(
        CharacterBattleRouteSubjectFamily::HandoffResourceProjectionRouteSubject,
        CharacterBattleRouteOwnerGroup::CharacterBattleInitProjectionOwner,
    ));
}

fn append_reject_mixed_spell_and_pact_slot_init_route(route: &mut Vec<CharacterBattleRouteEvent>) {
    route.push(route_reject_character_battle_handoff(
        CharacterBattleRouteSubjectFamily::HandoffResourceProjectionRouteSubject,
        CharacterBattleRouteFillFamily::HandoffSheetProjectionFill,
        vec![
            CharacterBattleRouteHoleFamily::HandoffSpellResourceProjectionHoleFamily,
            CharacterBattleRouteHoleFamily::HandoffSettlementConflictHoleFamily,
        ],
        CharacterBattleRouteOwnerGroup::CharacterBattleResourceProjectionOwner,
    ));
}

fn append_reject_stable_recovery_route(route: &mut Vec<CharacterBattleRouteEvent>) {
    route.push(route_reject_character_battle_handoff(
        CharacterBattleRouteSubjectFamily::SheetToBattleInitRouteSubject,
        CharacterBattleRouteFillFamily::HandoffSheetProjectionFill,
        vec![CharacterBattleRouteHoleFamily::HandoffSettlementConflictHoleFamily],
        CharacterBattleRouteOwnerGroup::CharacterBattleInitProjectionOwner,
    ));
}

fn expected_project_character_sheet_to_battle(
    subject: CharacterBattleRouteSubjectFamily,
    owner: CharacterBattleRouteOwnerGroup,
) -> CharacterBattleRouteEvent {
    CharacterBattleRouteEvent::RouteProjectCharacterSheetToBattle { subject, owner }
}

fn expected_enter_battle_runtime(
    subject: CharacterBattleRouteSubjectFamily,
    owner: CharacterBattleRouteOwnerGroup,
) -> CharacterBattleRouteEvent {
    CharacterBattleRouteEvent::RouteEnterBattleRuntime { subject, owner }
}

fn expected_reject_character_battle_handoff(
    subject: CharacterBattleRouteSubjectFamily,
    fill: CharacterBattleRouteFillFamily,
    mut holes: Vec<CharacterBattleRouteHoleFamily>,
    owner: CharacterBattleRouteOwnerGroup,
) -> CharacterBattleRouteEvent {
    holes.sort();
    CharacterBattleRouteEvent::RouteRejectCharacterBattleHandoff {
        subject,
        fill,
        holes,
        owner,
    }
}

fn witness_from_projection(
    projection: impl Into<CharacterBattleInitProjection>,
    replay_index: u8,
) -> CharacterBattleInitProjectionWitness {
    let projection = projection.into();
    let facts = projection.facts();

    CharacterBattleInitProjectionWitness {
        last_result: last_result_ref(&projection),
        accepted: projection.accepted(),
        message: projection.message().unwrap_or("none"),
        character_identity: character_identity_ref(facts.character_identity),
        current_hp: facts.current_hp,
        max_hp: facts.hit_point_maximum,
        temporary_hit_points: facts.temporary_hit_points,
        armor_class: facts.armor_class,
        poisoned: facts.poisoned,
        spell_level_1_count: facts.spell_level_1_count,
        spell_level_1_expended: facts.spell_level_1_expended,
        spell_level_2_count: facts.spell_level_2_count,
        spell_level_2_expended: facts.spell_level_2_expended,
        spell_level_3_count: facts.spell_level_3_count,
        spell_level_3_expended: facts.spell_level_3_expended,
        passive_armor_class_profile_count: facts.passive_armor_class_profile_count,
        metamagic_known_options: facts.metamagic_known_options,
        replay_index,
    }
}

fn last_result_ref(projection: &CharacterBattleInitProjection) -> &'static str {
    match projection {
        CharacterBattleInitProjection::Accepted(accepted) => match accepted.kind() {
            CharacterBattleInitProjectionAcceptance::SheetHitPointsArmorClassConditionsAndProfiles => {
                "sheet-hit-points-armor-class-conditions-and-profiles"
            }
            CharacterBattleInitProjectionAcceptance::SheetSpellcastingAndMetamagic => {
                "sheet-spellcasting-and-metamagic"
            }
            CharacterBattleInitProjectionAcceptance::PurePactMagicSlot => {
                "pure-pact-magic-slot-projection"
            }
        },
        CharacterBattleInitProjection::Rejected(rejected) => match rejected.reason() {
            CharacterBattleInitProjectionRejection::SheetHitPointMaximumExceedsBuildDerivedMaximum => {
                "build-maximum-above-build-maximum-rejected"
            }
            CharacterBattleInitProjectionRejection::MixedSpellAndPactSlot => {
                "mixed-spell-and-pact-slot-init-rejected"
            }
            CharacterBattleInitProjectionRejection::StableRecoveryProgressDuringInit => {
                "stable-recovery-progress-during-init-rejected"
            }
        },
    }
}

fn character_identity_ref(identity: CharacterBattleIdentity) -> &'static str {
    match identity {
        CharacterBattleIdentity::None => "none",
        CharacterBattleIdentity::BattleInitFighter => "character:battle-init-fighter",
        CharacterBattleIdentity::BattleInitSorcerer => "character:battle-init-sorcerer",
        CharacterBattleIdentity::BattleInitWarlock => "character:battle-init-warlock",
    }
}
