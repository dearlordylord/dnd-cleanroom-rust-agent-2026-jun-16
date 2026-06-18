use crate::rules::character_battle_handoff::{
    project_sheet_hit_points_armor_class_conditions_and_profiles_for_battle,
    project_sheet_spellcasting_and_metamagic_for_battle,
    reject_battle_initialization_sheet_hit_point_maximum_exceeds_build,
    reject_battle_initialization_stable_recovery_progress, CharacterBattleIdentity,
    CharacterBattleInitProjection, CharacterBattleInitProjectionAcceptance,
    CharacterBattleInitProjectionRejection,
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

pub const BRANCH_ACTIONS: [&str; 4] = [
    "doProjectSheetHitPointsArmorClassConditionsAndProfiles",
    "doProjectSheetSpellcastingAndMetamagic",
    "doRejectBuildMaximumAboveBuildMaximum",
    "doRejectStableRecoveryProgressDuringInit",
];

pub fn replay_observed_action(observed_action_taken: &str) -> CharacterBattleInitProjectionWitness {
    match observed_action_taken {
        "doProjectSheetHitPointsArmorClassConditionsAndProfiles" => {
            sheet_hit_points_armor_class_conditions_and_profiles()
        }
        "doProjectSheetSpellcastingAndMetamagic" => sheet_spellcasting_and_metamagic(),
        "doRejectBuildMaximumAboveBuildMaximum" => reject_build_maximum_above_build_maximum(),
        "doRejectStableRecoveryProgressDuringInit" => reject_stable_recovery_progress_during_init(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> CharacterBattleInitProjectionWitness {
    replay_observed_action(observed_action_taken)
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
        3,
    )
}

fn reject_stable_recovery_progress_during_init() -> CharacterBattleInitProjectionWitness {
    witness_from_projection(reject_battle_initialization_stable_recovery_progress(), 4)
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
        },
        CharacterBattleInitProjection::Rejected(rejected) => match rejected.reason() {
            CharacterBattleInitProjectionRejection::SheetHitPointMaximumExceedsBuildDerivedMaximum => {
                "build-maximum-above-build-maximum-rejected"
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
    }
}
