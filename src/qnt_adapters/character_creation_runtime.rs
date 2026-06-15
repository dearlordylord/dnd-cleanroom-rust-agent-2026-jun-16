use crate::rules::character_creation::{
    ability_scores_only_fills, accepted_draft, choice, empty_draft, fill_creation_holes,
    initial_choices_only_fills, initial_manifest_fills, manifest_choice_fills,
    manifest_loadout_fills, manifest_purchase_fills, BatchIssueCode, ChoiceOption, Draft, Fill,
    FillBatchResult, FillIssue, FillIssueCode, FinalizationStatus, Hole, Progression,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeWitness {
    pub q_last_result: &'static str,
    pub q_draft_revision: u8,
    pub q_draft_progression: &'static str,
    pub q_draft_background: bool,
    pub q_draft_species: bool,
    pub q_draft_ability_scores: bool,
    pub q_draft_languages: bool,
    pub q_draft_alignment: bool,
    pub q_draft_class_skills: bool,
    pub q_draft_fighter_fighting_style: bool,
    pub q_draft_fighter_weapon_mastery: bool,
    pub q_draft_background_ability_score_increase: bool,
    pub q_draft_background_tool: bool,
    pub q_draft_class_equipment: bool,
    pub q_draft_background_equipment: bool,
    pub q_draft_equipment_purchase: bool,
    pub q_draft_loadout_armor: bool,
    pub q_draft_loadout_shield: bool,
    pub q_draft_loadout_weapon: bool,
    pub q_holes: Vec<&'static str>,
    pub q_finalization: &'static str,
    pub q_last_batch_issue_codes: Vec<&'static str>,
    pub q_last_fill_issues: Vec<RuntimeFillIssueWitness>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeFillIssueWitness {
    pub fill_index: u8,
    pub hole: &'static str,
    pub code: &'static str,
}

pub const BRANCH_ACTIONS: &[&str] = &[
    "doFillAbilityScoresOnly",
    "doFillInitialChoicesOnly",
    "doFillInitialManifest",
    "doFillManifestChoices",
    "doFillManifestLoadout",
    "doFillManifestPurchase",
    "doRejectClosedInitialProgressionHole",
    "doRejectDuplicateFill",
    "doRejectDuplicateLanguage",
    "doRejectStaleInitialManifest",
    "doRejectTooFewLanguages",
    "doRejectTooManyLanguages",
    "doRejectUnknownLoadoutArmor",
    "doRejectUnsupportedClassEquipment",
    "doRejectUnsupportedLanguage",
    "doRejectWrongKindPrimaryClass",
];

pub fn replay_observed_action(observed_action_taken: &str) -> RuntimeWitness {
    match observed_action_taken {
        "doFillAbilityScoresOnly" => replay_from_empty(ability_scores_only_fills(), 0),
        "doFillInitialChoicesOnly" => replay_from_empty(initial_choices_only_fills(), 0),
        "doFillInitialManifest" => replay_from_empty(initial_manifest_fills(), 0),
        "doFillManifestChoices" => {
            let draft = after_initial_manifest();
            replay_from_draft(draft, manifest_choice_fills(), None)
        }
        "doFillManifestLoadout" => {
            let draft = after_manifest_purchase();
            replay_from_draft(draft, manifest_loadout_fills(), None)
        }
        "doFillManifestPurchase" => {
            let draft = after_manifest_choices();
            replay_from_draft(draft, manifest_purchase_fills(), None)
        }
        "doRejectClosedInitialProgressionHole" => {
            let draft = accepted_draft(fill_creation_holes(
                &empty_draft(),
                0,
                &initial_choices_only_fills(),
            ));
            replay_from_draft(
                draft,
                vec![choice(
                    Hole::Progression,
                    &[ChoiceOption::ClassFighterLevel1],
                )],
                None,
            )
        }
        "doRejectDuplicateFill" => replay_from_empty(
            vec![
                choice(
                    Hole::Languages,
                    &[ChoiceOption::LanguageDwarvish, ChoiceOption::LanguageGoblin],
                ),
                choice(
                    Hole::Languages,
                    &[ChoiceOption::LanguageDwarvish, ChoiceOption::LanguageGoblin],
                ),
            ],
            0,
        ),
        "doRejectDuplicateLanguage" => replay_from_empty(
            vec![choice(
                Hole::Languages,
                &[
                    ChoiceOption::LanguageDwarvish,
                    ChoiceOption::LanguageDwarvish,
                ],
            )],
            0,
        ),
        "doRejectStaleInitialManifest" => replay_from_empty(initial_manifest_fills(), 999),
        "doRejectTooFewLanguages" => replay_from_empty(
            vec![choice(Hole::Languages, &[ChoiceOption::LanguageDwarvish])],
            0,
        ),
        "doRejectTooManyLanguages" => replay_from_empty(
            vec![choice(
                Hole::Languages,
                &[
                    ChoiceOption::LanguageDwarvish,
                    ChoiceOption::LanguageGoblin,
                    ChoiceOption::LanguageElvish,
                ],
            )],
            0,
        ),
        "doRejectUnknownLoadoutArmor" => replay_from_empty(
            vec![choice(Hole::LoadoutArmor, &[ChoiceOption::LoadoutWorn])],
            0,
        ),
        "doRejectUnsupportedClassEquipment" => {
            let draft = after_initial_manifest();
            replay_from_draft(
                draft,
                vec![choice(
                    Hole::ClassEquipment,
                    &[ChoiceOption::ClassEquipmentPackageA],
                )],
                None,
            )
        }
        "doRejectUnsupportedLanguage" => replay_from_empty(
            vec![choice(
                Hole::Languages,
                &[ChoiceOption::LanguageDwarvish, ChoiceOption::LanguageElvish],
            )],
            0,
        ),
        "doRejectWrongKindPrimaryClass" => replay_from_empty(
            vec![Fill::AbilityScores {
                hole: Hole::Progression,
                method: crate::rules::character_creation::AbilityScoreMethod::StandardArray,
                scores: crate::rules::character_creation::fighter_standard_array(),
            }],
            0,
        ),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn projection_payload(witness: &RuntimeWitness) -> String {
    [
        format!("qLastResult={}", witness.q_last_result),
        format!("qDraft.revision={}", witness.q_draft_revision),
        format!("qDraft.progression={}", witness.q_draft_progression),
        format!("qDraft.background={}", witness.q_draft_background),
        format!("qDraft.species={}", witness.q_draft_species),
        format!("qDraft.abilityScores={}", witness.q_draft_ability_scores),
        format!("qDraft.languages={}", witness.q_draft_languages),
        format!("qDraft.alignment={}", witness.q_draft_alignment),
        format!("qDraft.classSkills={}", witness.q_draft_class_skills),
        format!(
            "qDraft.fighterFightingStyle={}",
            witness.q_draft_fighter_fighting_style
        ),
        format!(
            "qDraft.fighterWeaponMastery={}",
            witness.q_draft_fighter_weapon_mastery
        ),
        format!(
            "qDraft.backgroundAbilityScoreIncrease={}",
            witness.q_draft_background_ability_score_increase
        ),
        format!("qDraft.backgroundTool={}", witness.q_draft_background_tool),
        format!("qDraft.classEquipment={}", witness.q_draft_class_equipment),
        format!(
            "qDraft.backgroundEquipment={}",
            witness.q_draft_background_equipment
        ),
        format!(
            "qDraft.equipmentPurchase={}",
            witness.q_draft_equipment_purchase
        ),
        format!("qDraft.loadoutArmor={}", witness.q_draft_loadout_armor),
        format!("qDraft.loadoutShield={}", witness.q_draft_loadout_shield),
        format!("qDraft.loadoutWeapon={}", witness.q_draft_loadout_weapon),
        format!("qHoles={}", witness.q_holes.join(",")),
        format!("qFinalization={}", witness.q_finalization),
        format!(
            "qLastBatchIssueCodes={}",
            witness.q_last_batch_issue_codes.join(",")
        ),
        format!("qLastFillIssues={}", fill_issue_payload(witness)),
    ]
    .join("\n")
}

fn replay_from_empty(fills: Vec<Fill>, expected_revision: u16) -> RuntimeWitness {
    let draft = empty_draft();
    replay_from_draft(draft, fills, Some(expected_revision))
}

fn replay_from_draft(
    draft: Draft,
    fills: Vec<Fill>,
    expected_revision: Option<u16>,
) -> RuntimeWitness {
    let revision = expected_revision.unwrap_or_else(|| u16::from(draft.revision));
    witness_from_result(fill_creation_holes(&draft, revision, &fills))
}

fn after_initial_manifest() -> Draft {
    accepted_draft(fill_creation_holes(
        &empty_draft(),
        0,
        &initial_manifest_fills(),
    ))
}

fn after_manifest_choices() -> Draft {
    let draft = after_initial_manifest();
    accepted_draft(fill_creation_holes(
        &draft,
        u16::from(draft.revision),
        &manifest_choice_fills(),
    ))
}

fn after_manifest_purchase() -> Draft {
    let draft = after_manifest_choices();
    accepted_draft(fill_creation_holes(
        &draft,
        u16::from(draft.revision),
        &manifest_purchase_fills(),
    ))
}

fn witness_from_result(result: FillBatchResult) -> RuntimeWitness {
    match result {
        FillBatchResult::Accepted {
            draft,
            holes,
            finalization,
        } => RuntimeWitness {
            q_last_result: "accepted",
            q_holes: holes.into_iter().map(hole_name).collect(),
            q_finalization: finalization_name(finalization),
            q_last_batch_issue_codes: Vec::new(),
            q_last_fill_issues: Vec::new(),
            ..draft_witness_fields(&draft)
        },
        FillBatchResult::Rejected {
            draft,
            holes,
            issues,
            finalization,
        } => RuntimeWitness {
            q_last_result: "rejected",
            q_holes: holes.into_iter().map(hole_name).collect(),
            q_finalization: finalization_name(finalization),
            q_last_batch_issue_codes: issues
                .batch
                .into_iter()
                .map(batch_issue_code_name)
                .collect(),
            q_last_fill_issues: issues.fills.into_iter().map(fill_issue_witness).collect(),
            ..draft_witness_fields(&draft)
        },
    }
}

fn draft_witness_fields(draft: &Draft) -> RuntimeWitness {
    RuntimeWitness {
        q_last_result: "init",
        q_draft_revision: draft.revision,
        q_draft_progression: progression_name(draft.progression),
        q_draft_background: draft.background,
        q_draft_species: draft.species,
        q_draft_ability_scores: draft.ability_scores,
        q_draft_languages: draft.languages,
        q_draft_alignment: draft.alignment,
        q_draft_class_skills: draft.class_skills,
        q_draft_fighter_fighting_style: draft.fighter_fighting_style,
        q_draft_fighter_weapon_mastery: draft.fighter_weapon_mastery,
        q_draft_background_ability_score_increase: draft.background_ability_score_increase,
        q_draft_background_tool: draft.background_tool,
        q_draft_class_equipment: draft.class_equipment,
        q_draft_background_equipment: draft.background_equipment,
        q_draft_equipment_purchase: draft.equipment_purchase,
        q_draft_loadout_armor: draft.loadout_armor,
        q_draft_loadout_shield: draft.loadout_shield,
        q_draft_loadout_weapon: draft.loadout_weapon,
        q_holes: Vec::new(),
        q_finalization: "Incomplete",
        q_last_batch_issue_codes: Vec::new(),
        q_last_fill_issues: Vec::new(),
    }
}

fn fill_issue_witness(issue: FillIssue) -> RuntimeFillIssueWitness {
    RuntimeFillIssueWitness {
        fill_index: issue.fill_index,
        hole: hole_name(issue.hole),
        code: fill_issue_code_name(issue.code),
    }
}

fn fill_issue_payload(witness: &RuntimeWitness) -> String {
    witness
        .q_last_fill_issues
        .iter()
        .map(|issue| format!("{}:{}:{}", issue.fill_index, issue.hole, issue.code))
        .collect::<Vec<_>>()
        .join("|")
}

fn progression_name(progression: Option<Progression>) -> &'static str {
    match progression {
        None => "NoProgression",
        Some(Progression::FighterLevelOne) => "SelectedProgression(FighterLevel1)",
        Some(Progression::FighterLevelTwo) => "SelectedProgression(FighterLevel2)",
        Some(Progression::WizardLevelOne) => "SelectedProgression(WizardLevel1)",
    }
}

fn finalization_name(finalization: FinalizationStatus) -> &'static str {
    match finalization {
        FinalizationStatus::Ready => "Ready",
        FinalizationStatus::Incomplete => "Incomplete",
        FinalizationStatus::Invalid => "Invalid",
    }
}

fn batch_issue_code_name(code: BatchIssueCode) -> &'static str {
    match code {
        BatchIssueCode::StaleRevision => "staleRevision",
    }
}

fn fill_issue_code_name(code: FillIssueCode) -> &'static str {
    match code {
        FillIssueCode::UnknownHole => "unknownHole",
        FillIssueCode::DuplicateFill => "duplicateFill",
        FillIssueCode::WrongFillKind => "wrongFillKind",
        FillIssueCode::InvalidChoice => "invalidChoice",
        FillIssueCode::InvalidAbilityScores => "invalidAbilityScores",
        FillIssueCode::TooFewChoices => "tooFewChoices",
        FillIssueCode::TooManyChoices => "tooManyChoices",
        FillIssueCode::UnsupportedChoice => "unsupportedChoice",
    }
}

fn hole_name(hole: Hole) -> &'static str {
    match hole {
        Hole::Progression => "HProgression",
        Hole::Background => "HBackground",
        Hole::Species => "HSpecies",
        Hole::AbilityScores => "HAbilityScores",
        Hole::Languages => "HLanguages",
        Hole::Alignment => "HAlignment",
        Hole::ClassSkills => "HClassSkills",
        Hole::FighterFightingStyle => "HFighterFightingStyle",
        Hole::FighterWeaponMastery => "HFighterWeaponMastery",
        Hole::BackgroundAbilityScoreIncrease => "HBackgroundAbilityScoreIncrease",
        Hole::BackgroundTool => "HBackgroundTool",
        Hole::ClassEquipment => "HClassEquipment",
        Hole::BackgroundEquipment => "HBackgroundEquipment",
        Hole::EquipmentPurchase => "HEquipmentPurchase",
        Hole::LoadoutArmor => "HLoadoutArmor",
        Hole::LoadoutShield => "HLoadoutShield",
        Hole::LoadoutWeapon => "HLoadoutWeapon",
    }
}
