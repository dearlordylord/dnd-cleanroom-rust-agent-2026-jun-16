use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Hole {
    Progression,
    Background,
    Species,
    AbilityScores,
    Languages,
    Alignment,
    ClassSkills,
    FighterFightingStyle,
    FighterWeaponMastery,
    BackgroundAbilityScoreIncrease,
    BackgroundTool,
    ClassEquipment,
    BackgroundEquipment,
    EquipmentPurchase,
    LoadoutArmor,
    LoadoutShield,
    LoadoutWeapon,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HoleKind {
    Choice,
    AbilityScore,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ChoiceOption {
    ClassFighterLevel1,
    ClassFighterLevel2,
    ClassWizardLevel1,
    BackgroundSoldier,
    SpeciesOrc,
    LanguageCommonSignLanguage,
    LanguageDraconic,
    LanguageDwarvish,
    LanguageElvish,
    LanguageGiant,
    LanguageGnomish,
    LanguageGoblin,
    LanguageHalfling,
    LanguageOrc,
    AlignmentLawfulGood,
    AlignmentNeutralGood,
    AlignmentChaoticGood,
    AlignmentLawfulNeutral,
    AlignmentNeutralNeutral,
    AlignmentChaoticNeutral,
    AlignmentLawfulEvil,
    AlignmentNeutralEvil,
    AlignmentChaoticEvil,
    SkillAcrobatics,
    SkillAnimalHandling,
    SkillAthletics,
    SkillHistory,
    SkillInsight,
    SkillIntimidation,
    SkillPersuasion,
    SkillPerception,
    SkillSurvival,
    FightingStyleDefense,
    WeaponLongsword,
    WeaponDagger,
    WeaponSpear,
    WeaponFlail,
    WeaponShortbow,
    BackgroundAsiStrDex,
    BackgroundAsiStrCon,
    BackgroundAsiDexStr,
    BackgroundAsiDexCon,
    BackgroundAsiConStr,
    BackgroundAsiConDex,
    BackgroundAsiOneEach,
    ToolDiceSet,
    ClassEquipmentPackageA,
    ClassEquipmentPackageB,
    ClassEquipmentCoins,
    BackgroundEquipmentPack,
    BackgroundEquipmentCoins,
    ArmorChainMail,
    EquipmentShield,
    LoadoutWorn,
    LoadoutWielded,
    LoadoutWieldedOneHanded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbilityScoreMethod {
    StandardArray,
    PointBuy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AbilityScores {
    pub strength: u8,
    pub dexterity: u8,
    pub constitution: u8,
    pub intelligence: u8,
    pub wisdom: u8,
    pub charisma: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Progression {
    FighterLevelOne,
    FighterLevelTwo,
    WizardLevelOne,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Draft {
    pub revision: u8,
    pub progression: Option<Progression>,
    pub background: bool,
    pub species: bool,
    pub ability_scores: bool,
    pub languages: bool,
    pub alignment: bool,
    pub class_skills: bool,
    pub fighter_fighting_style: bool,
    pub fighter_weapon_mastery: bool,
    pub background_ability_score_increase: bool,
    pub background_tool: bool,
    pub class_equipment: bool,
    pub background_equipment: bool,
    pub equipment_purchase: bool,
    pub loadout_armor: bool,
    pub loadout_shield: bool,
    pub loadout_weapon: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FinalizationStatus {
    Ready,
    Incomplete,
    Invalid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FillIssueCode {
    UnknownHole,
    DuplicateFill,
    WrongFillKind,
    InvalidChoice,
    InvalidAbilityScores,
    TooFewChoices,
    TooManyChoices,
    UnsupportedChoice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BatchIssueCode {
    StaleRevision,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FillIssue {
    pub fill_index: u8,
    pub hole: Hole,
    pub code: FillIssueCode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreationIssues {
    pub batch: BTreeSet<BatchIssueCode>,
    pub fills: BTreeSet<FillIssue>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Fill {
    Choice {
        hole: Hole,
        options: Vec<ChoiceOption>,
    },
    AbilityScores {
        hole: Hole,
        method: AbilityScoreMethod,
        scores: AbilityScores,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FillBatchResult {
    Accepted {
        draft: Draft,
        holes: BTreeSet<Hole>,
        finalization: FinalizationStatus,
    },
    Rejected {
        draft: Draft,
        holes: BTreeSet<Hole>,
        issues: CreationIssues,
        finalization: FinalizationStatus,
    },
}

#[must_use]
pub fn empty_draft() -> Draft {
    Draft {
        revision: 0,
        progression: None,
        background: false,
        species: false,
        ability_scores: false,
        languages: false,
        alignment: false,
        class_skills: false,
        fighter_fighting_style: false,
        fighter_weapon_mastery: false,
        background_ability_score_increase: false,
        background_tool: false,
        class_equipment: false,
        background_equipment: false,
        equipment_purchase: false,
        loadout_armor: false,
        loadout_shield: false,
        loadout_weapon: false,
    }
}

#[must_use]
pub fn fighter_standard_array() -> AbilityScores {
    // cleanroom-input/qnt/character-creation-runtime/character-creation-runtime-slice.qnt
    // `fighterStandardArray`, traced to cleanroom-input/raw/srd-5.2.1/Character-Creation.md
    // "Determine Ability Scores".
    AbilityScores {
        strength: 15,
        dexterity: 14,
        constitution: 13,
        intelligence: 8,
        wisdom: 10,
        charisma: 12,
    }
}

#[must_use]
pub fn open_creation_holes(draft: &Draft) -> BTreeSet<Hole> {
    let mut holes = BTreeSet::new();
    if draft.progression.is_none() {
        holes.insert(Hole::Progression);
    }
    if !draft.background {
        holes.insert(Hole::Background);
    }
    if !draft.species {
        holes.insert(Hole::Species);
    }
    if !draft.ability_scores {
        holes.insert(Hole::AbilityScores);
    }
    if !draft.languages {
        holes.insert(Hole::Languages);
    }
    if !draft.alignment {
        holes.insert(Hole::Alignment);
    }

    if has_fighter_progression(draft) {
        if !draft.class_skills {
            holes.insert(Hole::ClassSkills);
        }
        if !draft.fighter_fighting_style {
            holes.insert(Hole::FighterFightingStyle);
        }
        if !draft.fighter_weapon_mastery {
            holes.insert(Hole::FighterWeaponMastery);
        }
        if !draft.class_equipment {
            holes.insert(Hole::ClassEquipment);
        }
    } else if has_wizard_progression(draft) {
        if !draft.class_skills {
            holes.insert(Hole::ClassSkills);
        }
        if !draft.class_equipment {
            holes.insert(Hole::ClassEquipment);
        }
    }

    if draft.background {
        if !draft.background_ability_score_increase {
            holes.insert(Hole::BackgroundAbilityScoreIncrease);
        }
        if !draft.background_tool {
            holes.insert(Hole::BackgroundTool);
        }
        if !draft.background_equipment {
            holes.insert(Hole::BackgroundEquipment);
        }
    }

    if draft.class_equipment && draft.background_equipment {
        if !draft.equipment_purchase {
            holes.insert(Hole::EquipmentPurchase);
        }
        if draft.equipment_purchase && !draft.loadout_armor {
            holes.insert(Hole::LoadoutArmor);
        }
        if draft.equipment_purchase && !draft.loadout_shield {
            holes.insert(Hole::LoadoutShield);
        }
        if draft.equipment_purchase && !draft.loadout_weapon {
            holes.insert(Hole::LoadoutWeapon);
        }
    }

    holes
}

#[must_use]
pub fn finalize_draft(draft: &Draft) -> FinalizationStatus {
    if !open_creation_holes(draft).is_empty() {
        return FinalizationStatus::Incomplete;
    }

    let class_ready = (has_fighter_progression(draft)
        && draft.class_skills
        && draft.fighter_fighting_style
        && draft.fighter_weapon_mastery)
        || (has_wizard_progression(draft) && draft.class_skills);

    if draft.progression.is_some()
        && draft.background
        && draft.species
        && draft.ability_scores
        && draft.languages
        && draft.alignment
        && class_ready
        && draft.background_ability_score_increase
        && draft.background_tool
        && draft.class_equipment
        && draft.background_equipment
        && draft.equipment_purchase
        && draft.loadout_armor
        && draft.loadout_shield
        && draft.loadout_weapon
    {
        FinalizationStatus::Ready
    } else {
        FinalizationStatus::Invalid
    }
}

#[must_use]
pub fn fill_creation_holes(
    draft: &Draft,
    expected_revision: u16,
    fills: &[Fill],
) -> FillBatchResult {
    let open = open_creation_holes(draft);
    let mut batch_issues = BTreeSet::new();
    if expected_revision != u16::from(draft.revision) {
        batch_issues.insert(BatchIssueCode::StaleRevision);
    }
    let fill_issues = fill_issues_for_batch(fills, &open);
    let issues = CreationIssues {
        batch: batch_issues,
        fills: fill_issues,
    };

    if !issues.batch.is_empty() || !issues.fills.is_empty() {
        return FillBatchResult::Rejected {
            draft: draft.clone(),
            holes: open,
            issues,
            finalization: finalize_draft(draft),
        };
    }

    let mut next_draft = draft.clone();
    for fill in fills {
        apply_fill(&mut next_draft, fill);
    }
    next_draft.revision = draft.revision + 1;

    FillBatchResult::Accepted {
        holes: open_creation_holes(&next_draft),
        finalization: finalize_draft(&next_draft),
        draft: next_draft,
    }
}

#[must_use]
pub fn initial_manifest_fills() -> Vec<Fill> {
    vec![
        choice(Hole::Progression, &[ChoiceOption::ClassFighterLevel1]),
        choice(Hole::Background, &[ChoiceOption::BackgroundSoldier]),
        choice(Hole::Species, &[ChoiceOption::SpeciesOrc]),
        Fill::AbilityScores {
            hole: Hole::AbilityScores,
            method: AbilityScoreMethod::StandardArray,
            scores: fighter_standard_array(),
        },
        choice(
            Hole::Languages,
            &[ChoiceOption::LanguageDwarvish, ChoiceOption::LanguageGoblin],
        ),
        choice(Hole::Alignment, &[ChoiceOption::AlignmentLawfulGood]),
    ]
}

#[must_use]
pub fn initial_choices_only_fills() -> Vec<Fill> {
    vec![
        choice(Hole::Progression, &[ChoiceOption::ClassFighterLevel1]),
        choice(Hole::Background, &[ChoiceOption::BackgroundSoldier]),
        choice(Hole::Species, &[ChoiceOption::SpeciesOrc]),
        choice(
            Hole::Languages,
            &[ChoiceOption::LanguageDwarvish, ChoiceOption::LanguageGoblin],
        ),
        choice(Hole::Alignment, &[ChoiceOption::AlignmentLawfulGood]),
    ]
}

#[must_use]
pub fn ability_scores_only_fills() -> Vec<Fill> {
    vec![Fill::AbilityScores {
        hole: Hole::AbilityScores,
        method: AbilityScoreMethod::StandardArray,
        scores: fighter_standard_array(),
    }]
}

#[must_use]
pub fn manifest_choice_fills() -> Vec<Fill> {
    vec![
        choice(
            Hole::ClassSkills,
            &[ChoiceOption::SkillPerception, ChoiceOption::SkillSurvival],
        ),
        choice(
            Hole::FighterFightingStyle,
            &[ChoiceOption::FightingStyleDefense],
        ),
        choice(
            Hole::FighterWeaponMastery,
            &[
                ChoiceOption::WeaponLongsword,
                ChoiceOption::WeaponSpear,
                ChoiceOption::WeaponFlail,
            ],
        ),
        choice(
            Hole::BackgroundAbilityScoreIncrease,
            &[ChoiceOption::BackgroundAsiStrCon],
        ),
        choice(Hole::BackgroundTool, &[ChoiceOption::ToolDiceSet]),
        choice(Hole::ClassEquipment, &[ChoiceOption::ClassEquipmentCoins]),
        choice(
            Hole::BackgroundEquipment,
            &[ChoiceOption::BackgroundEquipmentCoins],
        ),
    ]
}

#[must_use]
pub fn manifest_purchase_fills() -> Vec<Fill> {
    vec![choice(
        Hole::EquipmentPurchase,
        &[
            ChoiceOption::ArmorChainMail,
            ChoiceOption::WeaponLongsword,
            ChoiceOption::EquipmentShield,
        ],
    )]
}

#[must_use]
pub fn manifest_loadout_fills() -> Vec<Fill> {
    vec![
        choice(Hole::LoadoutArmor, &[ChoiceOption::LoadoutWorn]),
        choice(Hole::LoadoutShield, &[ChoiceOption::LoadoutWielded]),
        choice(
            Hole::LoadoutWeapon,
            &[ChoiceOption::LoadoutWieldedOneHanded],
        ),
    ]
}

#[must_use]
pub fn accepted_draft(result: FillBatchResult) -> Draft {
    match result {
        FillBatchResult::Accepted { draft, .. } => draft,
        FillBatchResult::Rejected { .. } => empty_draft(),
    }
}

#[must_use]
pub fn choice(hole: Hole, options: &[ChoiceOption]) -> Fill {
    Fill::Choice {
        hole,
        options: options.to_vec(),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CreationRouteSubjectFamily {
    DraftState,
    OptionDiscovery,
    FillBatch,
    RetainedReference,
    BuildProjection,
    Finalization,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CreationRouteHoleFamily {
    DraftStructure,
    UnitChoice,
    AbilityScore,
    EquipmentSelection,
    Loadout,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CreationRouteFillFamily {
    ChoiceSet,
    AbilityScoreAssignment,
    EquipmentSelection,
    LoadoutSelection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CreationRouteOwnerGroup {
    CharacterDraft,
    CharacterBuild,
    CreationHoleFrontier,
    CreationSupportProfileAdmission,
    CreationRetainedReference,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CreationRouteFactFamily {
    PartialFillDraft,
    StaleFillRejection,
    SelectedReferenceRetention,
    BuildProjectionInput,
    HitPointMaximumBuildInput,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CreationRouteEvent {
    CreateCharacterDraft {
        owner: CreationRouteOwnerGroup,
    },
    DiscoverCreationHoles {
        subject: CreationRouteSubjectFamily,
        holes: BTreeSet<CreationRouteHoleFamily>,
        owner: CreationRouteOwnerGroup,
    },
    ApplyCreationFillBatch {
        subject: CreationRouteSubjectFamily,
        fills: BTreeSet<CreationRouteFillFamily>,
        holes: BTreeSet<CreationRouteHoleFamily>,
        owner: CreationRouteOwnerGroup,
    },
    RetainCreationRetainedReferences {
        subject: CreationRouteSubjectFamily,
        owner: CreationRouteOwnerGroup,
    },
    ProjectCharacterBuildFacts {
        subject: CreationRouteSubjectFamily,
        owner: CreationRouteOwnerGroup,
    },
    RecordCreationFacts {
        subject: CreationRouteSubjectFamily,
        facts: BTreeSet<CreationRouteFactFamily>,
        owner: CreationRouteOwnerGroup,
    },
    FinalizeCharacterDraft {
        subject: CreationRouteSubjectFamily,
        owner: CreationRouteOwnerGroup,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterCreationState {
    pub draft: Draft,
    pub route: Vec<CreationRouteEvent>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterCreationStep {
    pub state: CharacterCreationState,
    pub result: FillBatchResult,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreationRetainedReferenceOperation {
    RetainOnly,
    RetainAndProject,
    ProjectBuildFacts,
    ReplaceAndProject,
    RejectSelection,
}

#[must_use]
pub fn new_character_creation_state() -> CharacterCreationState {
    CharacterCreationState {
        draft: empty_draft(),
        route: initial_creation_route(),
    }
}

#[must_use]
pub fn initial_creation_route() -> Vec<CreationRouteEvent> {
    vec![
        CreationRouteEvent::CreateCharacterDraft {
            owner: CreationRouteOwnerGroup::CharacterDraft,
        },
        CreationRouteEvent::DiscoverCreationHoles {
            subject: CreationRouteSubjectFamily::DraftState,
            holes: route_hole_families(&[
                CreationRouteHoleFamily::DraftStructure,
                CreationRouteHoleFamily::AbilityScore,
            ]),
            owner: CreationRouteOwnerGroup::CreationHoleFrontier,
        },
    ]
}

#[must_use]
pub fn apply_creation_fill_batch(
    state: &CharacterCreationState,
    expected_revision: u16,
    fills: &[Fill],
    owner: CreationRouteOwnerGroup,
) -> CharacterCreationStep {
    let result = fill_creation_holes(&state.draft, expected_revision, fills);
    let route_holes = match &result {
        FillBatchResult::Accepted { holes, .. } | FillBatchResult::Rejected { holes, .. } => {
            route_hole_families_from_open_holes(holes)
        }
    };
    let mut route = apply_fill_batch_route(
        &state.route,
        fill_families_from_fills(fills),
        route_holes,
        owner,
    );
    append_creation_fill_facts(&mut route, expected_revision, fills, &result, owner);
    let draft = match &result {
        FillBatchResult::Accepted { draft, .. } => draft.clone(),
        FillBatchResult::Rejected { .. } => state.draft.clone(),
    };
    if matches!(
        &result,
        FillBatchResult::Accepted {
            finalization: FinalizationStatus::Ready,
            ..
        }
    ) {
        route.push(CreationRouteEvent::ProjectCharacterBuildFacts {
            subject: CreationRouteSubjectFamily::BuildProjection,
            owner: CreationRouteOwnerGroup::CharacterBuild,
        });
        route.push(build_projection_input_facts());
        route.push(CreationRouteEvent::FinalizeCharacterDraft {
            subject: CreationRouteSubjectFamily::Finalization,
            owner: CreationRouteOwnerGroup::CharacterBuild,
        });
    }

    CharacterCreationStep {
        state: CharacterCreationState { draft, route },
        result,
    }
}

#[must_use]
pub fn completed_fighter_creation_state() -> CharacterCreationState {
    let initial = new_character_creation_state();
    let after_initial = apply_creation_fill_batch(
        &initial,
        0,
        &initial_manifest_fills(),
        CreationRouteOwnerGroup::CharacterDraft,
    )
    .state;
    let after_choices = apply_creation_fill_batch(
        &after_initial,
        u16::from(after_initial.draft.revision),
        &manifest_choice_fills(),
        CreationRouteOwnerGroup::CreationSupportProfileAdmission,
    )
    .state;
    let after_purchase = apply_creation_fill_batch(
        &after_choices,
        u16::from(after_choices.draft.revision),
        &manifest_purchase_fills(),
        CreationRouteOwnerGroup::CreationSupportProfileAdmission,
    )
    .state;
    apply_creation_fill_batch(
        &after_purchase,
        u16::from(after_purchase.draft.revision),
        &manifest_loadout_fills(),
        CreationRouteOwnerGroup::CharacterDraft,
    )
    .state
}

#[must_use]
pub fn apply_creation_retained_reference_operation(
    state: &CharacterCreationState,
    operation: CreationRetainedReferenceOperation,
) -> CharacterCreationState {
    let mut route = state.route.clone();
    match operation {
        CreationRetainedReferenceOperation::RetainOnly => {
            route.push(CreationRouteEvent::RetainCreationRetainedReferences {
                subject: CreationRouteSubjectFamily::RetainedReference,
                owner: CreationRouteOwnerGroup::CreationRetainedReference,
            });
            route.push(CreationRouteEvent::RecordCreationFacts {
                subject: CreationRouteSubjectFamily::RetainedReference,
                facts: route_fact_families(&[CreationRouteFactFamily::SelectedReferenceRetention]),
                owner: CreationRouteOwnerGroup::CreationRetainedReference,
            });
        }
        CreationRetainedReferenceOperation::RetainAndProject => {
            route.push(CreationRouteEvent::RetainCreationRetainedReferences {
                subject: CreationRouteSubjectFamily::RetainedReference,
                owner: CreationRouteOwnerGroup::CreationRetainedReference,
            });
            route.push(CreationRouteEvent::RecordCreationFacts {
                subject: CreationRouteSubjectFamily::RetainedReference,
                facts: route_fact_families(&[CreationRouteFactFamily::SelectedReferenceRetention]),
                owner: CreationRouteOwnerGroup::CreationRetainedReference,
            });
            route.push(CreationRouteEvent::ProjectCharacterBuildFacts {
                subject: CreationRouteSubjectFamily::BuildProjection,
                owner: CreationRouteOwnerGroup::CharacterBuild,
            });
            route.push(build_projection_input_facts());
        }
        CreationRetainedReferenceOperation::ProjectBuildFacts => {
            route.push(CreationRouteEvent::ProjectCharacterBuildFacts {
                subject: CreationRouteSubjectFamily::BuildProjection,
                owner: CreationRouteOwnerGroup::CharacterBuild,
            });
            route.push(build_projection_input_facts());
        }
        CreationRetainedReferenceOperation::ReplaceAndProject => {
            route.push(CreationRouteEvent::ApplyCreationFillBatch {
                subject: CreationRouteSubjectFamily::RetainedReference,
                fills: route_fill_families(&[CreationRouteFillFamily::ChoiceSet]),
                holes: BTreeSet::new(),
                owner: CreationRouteOwnerGroup::CharacterBuild,
            });
            route.push(CreationRouteEvent::RetainCreationRetainedReferences {
                subject: CreationRouteSubjectFamily::RetainedReference,
                owner: CreationRouteOwnerGroup::CreationRetainedReference,
            });
            route.push(CreationRouteEvent::RecordCreationFacts {
                subject: CreationRouteSubjectFamily::RetainedReference,
                facts: route_fact_families(&[CreationRouteFactFamily::SelectedReferenceRetention]),
                owner: CreationRouteOwnerGroup::CreationRetainedReference,
            });
            route.push(CreationRouteEvent::ProjectCharacterBuildFacts {
                subject: CreationRouteSubjectFamily::BuildProjection,
                owner: CreationRouteOwnerGroup::CharacterBuild,
            });
            route.push(build_projection_input_facts());
        }
        CreationRetainedReferenceOperation::RejectSelection => {
            route.push(CreationRouteEvent::ApplyCreationFillBatch {
                subject: CreationRouteSubjectFamily::RetainedReference,
                fills: route_fill_families(&[CreationRouteFillFamily::ChoiceSet]),
                holes: route_hole_families(&[CreationRouteHoleFamily::UnitChoice]),
                owner: CreationRouteOwnerGroup::CreationSupportProfileAdmission,
            });
        }
    }

    CharacterCreationState {
        draft: state.draft.clone(),
        route,
    }
}

#[must_use]
pub fn apply_fill_batch_route(
    route: &[CreationRouteEvent],
    fills: BTreeSet<CreationRouteFillFamily>,
    holes: BTreeSet<CreationRouteHoleFamily>,
    owner: CreationRouteOwnerGroup,
) -> Vec<CreationRouteEvent> {
    let mut next = route.to_vec();
    next.push(CreationRouteEvent::ApplyCreationFillBatch {
        subject: CreationRouteSubjectFamily::FillBatch,
        fills,
        holes: holes.clone(),
        owner,
    });
    next.push(CreationRouteEvent::DiscoverCreationHoles {
        subject: CreationRouteSubjectFamily::OptionDiscovery,
        holes,
        owner: CreationRouteOwnerGroup::CreationHoleFrontier,
    });
    next
}

fn append_creation_fill_facts(
    route: &mut Vec<CreationRouteEvent>,
    expected_revision: u16,
    fills: &[Fill],
    result: &FillBatchResult,
    owner: CreationRouteOwnerGroup,
) {
    let fill_families = fill_families_from_fills(fills);
    if matches!(
        result,
        FillBatchResult::Accepted {
            finalization: FinalizationStatus::Incomplete,
            ..
        }
    ) && expected_revision == 0
        && fill_families.len() == 1
        && (fill_families.contains(&CreationRouteFillFamily::ChoiceSet)
            || fill_families.contains(&CreationRouteFillFamily::AbilityScoreAssignment))
    {
        route.push(CreationRouteEvent::RecordCreationFacts {
            subject: CreationRouteSubjectFamily::DraftState,
            facts: route_fact_families(&[CreationRouteFactFamily::PartialFillDraft]),
            owner,
        });
    }

    if matches!(
        result,
        FillBatchResult::Rejected { issues, .. }
            if issues.batch.contains(&BatchIssueCode::StaleRevision)
    ) {
        route.push(CreationRouteEvent::RecordCreationFacts {
            subject: CreationRouteSubjectFamily::FillBatch,
            facts: route_fact_families(&[CreationRouteFactFamily::StaleFillRejection]),
            owner,
        });
    }
}

#[must_use]
pub fn route_hole_families_from_open_holes(
    holes: &BTreeSet<Hole>,
) -> BTreeSet<CreationRouteHoleFamily> {
    holes.iter().map(route_hole_family).collect()
}

#[must_use]
pub fn fill_families_from_fills(fills: &[Fill]) -> BTreeSet<CreationRouteFillFamily> {
    fills.iter().map(route_fill_family).collect()
}

#[must_use]
pub fn route_payload(route: &[CreationRouteEvent]) -> String {
    route
        .iter()
        .map(route_event_payload)
        .collect::<Vec<_>>()
        .join("\n")
}

fn route_hole_families(families: &[CreationRouteHoleFamily]) -> BTreeSet<CreationRouteHoleFamily> {
    families.iter().copied().collect()
}

fn route_fill_families(families: &[CreationRouteFillFamily]) -> BTreeSet<CreationRouteFillFamily> {
    families.iter().copied().collect()
}

fn route_fact_families(families: &[CreationRouteFactFamily]) -> BTreeSet<CreationRouteFactFamily> {
    families.iter().copied().collect()
}

fn build_projection_input_facts() -> CreationRouteEvent {
    CreationRouteEvent::RecordCreationFacts {
        subject: CreationRouteSubjectFamily::BuildProjection,
        facts: route_fact_families(&[
            CreationRouteFactFamily::BuildProjectionInput,
            CreationRouteFactFamily::HitPointMaximumBuildInput,
        ]),
        owner: CreationRouteOwnerGroup::CharacterBuild,
    }
}

fn route_hole_family(hole: &Hole) -> CreationRouteHoleFamily {
    match hole {
        Hole::Progression
        | Hole::Background
        | Hole::Species
        | Hole::Languages
        | Hole::Alignment => CreationRouteHoleFamily::DraftStructure,
        Hole::AbilityScores => CreationRouteHoleFamily::AbilityScore,
        Hole::ClassSkills
        | Hole::FighterFightingStyle
        | Hole::FighterWeaponMastery
        | Hole::BackgroundAbilityScoreIncrease
        | Hole::BackgroundTool => CreationRouteHoleFamily::UnitChoice,
        Hole::ClassEquipment | Hole::BackgroundEquipment | Hole::EquipmentPurchase => {
            CreationRouteHoleFamily::EquipmentSelection
        }
        Hole::LoadoutArmor | Hole::LoadoutShield | Hole::LoadoutWeapon => {
            CreationRouteHoleFamily::Loadout
        }
    }
}

fn route_fill_family(fill: &Fill) -> CreationRouteFillFamily {
    match fill {
        Fill::AbilityScores { .. } => CreationRouteFillFamily::AbilityScoreAssignment,
        Fill::Choice { hole, .. } => match hole {
            Hole::ClassEquipment | Hole::BackgroundEquipment | Hole::EquipmentPurchase => {
                CreationRouteFillFamily::EquipmentSelection
            }
            Hole::LoadoutArmor | Hole::LoadoutShield | Hole::LoadoutWeapon => {
                CreationRouteFillFamily::LoadoutSelection
            }
            _ => CreationRouteFillFamily::ChoiceSet,
        },
    }
}

fn route_event_payload(event: &CreationRouteEvent) -> String {
    match event {
        CreationRouteEvent::CreateCharacterDraft { owner } => {
            format!("RouteCreateCharacterDraft(owner={})", owner_ref(*owner))
        }
        CreationRouteEvent::DiscoverCreationHoles {
            subject,
            holes,
            owner,
        } => format!(
            "RouteDiscoverCreationHoles(subject={},holes={},owner={})",
            subject_ref(*subject),
            hole_family_payload(holes),
            owner_ref(*owner)
        ),
        CreationRouteEvent::ApplyCreationFillBatch {
            subject,
            fills,
            holes,
            owner,
        } => format!(
            "RouteApplyCreationFillBatch(subject={},fills={},holes={},owner={})",
            subject_ref(*subject),
            fill_family_payload(fills),
            hole_family_payload(holes),
            owner_ref(*owner)
        ),
        CreationRouteEvent::RetainCreationRetainedReferences { subject, owner } => format!(
            "RouteRetainCreationRetainedReferences(subject={},owner={})",
            subject_ref(*subject),
            owner_ref(*owner)
        ),
        CreationRouteEvent::ProjectCharacterBuildFacts { subject, owner } => format!(
            "RouteProjectCharacterBuildFacts(subject={},owner={})",
            subject_ref(*subject),
            owner_ref(*owner)
        ),
        CreationRouteEvent::RecordCreationFacts {
            subject,
            facts,
            owner,
        } => format!(
            "RouteRecordCreationFacts(subject={},facts={},owner={})",
            subject_ref(*subject),
            fact_family_payload(facts),
            owner_ref(*owner)
        ),
        CreationRouteEvent::FinalizeCharacterDraft { subject, owner } => format!(
            "RouteFinalizeCharacterDraft(subject={},owner={})",
            subject_ref(*subject),
            owner_ref(*owner)
        ),
    }
}

fn hole_family_payload(holes: &BTreeSet<CreationRouteHoleFamily>) -> String {
    holes
        .iter()
        .map(|hole| hole_family_ref(*hole))
        .collect::<Vec<_>>()
        .join(",")
}

fn fill_family_payload(fills: &BTreeSet<CreationRouteFillFamily>) -> String {
    fills
        .iter()
        .map(|fill| fill_family_ref(*fill))
        .collect::<Vec<_>>()
        .join(",")
}

fn fact_family_payload(facts: &BTreeSet<CreationRouteFactFamily>) -> String {
    facts
        .iter()
        .map(|fact| match fact {
            CreationRouteFactFamily::PartialFillDraft => "CreationPartialFillDraftFact",
            CreationRouteFactFamily::StaleFillRejection => "CreationStaleFillRejectionFact",
            CreationRouteFactFamily::SelectedReferenceRetention => {
                "CreationSelectedReferenceRetentionFact"
            }
            CreationRouteFactFamily::BuildProjectionInput => "CreationBuildProjectionInputFact",
            CreationRouteFactFamily::HitPointMaximumBuildInput => {
                "CreationHitPointMaximumBuildInputFact"
            }
        })
        .collect::<Vec<_>>()
        .join(",")
}

fn subject_ref(subject: CreationRouteSubjectFamily) -> &'static str {
    match subject {
        CreationRouteSubjectFamily::DraftState => "CreationDraftStateRouteSubject",
        CreationRouteSubjectFamily::OptionDiscovery => "CreationOptionDiscoveryRouteSubject",
        CreationRouteSubjectFamily::FillBatch => "CreationFillBatchRouteSubject",
        CreationRouteSubjectFamily::RetainedReference => "CreationRetainedReferenceRouteSubject",
        CreationRouteSubjectFamily::BuildProjection => "CreationBuildProjectionRouteSubject",
        CreationRouteSubjectFamily::Finalization => "CreationFinalizationRouteSubject",
    }
}

fn hole_family_ref(hole: CreationRouteHoleFamily) -> &'static str {
    match hole {
        CreationRouteHoleFamily::DraftStructure => "CreationDraftStructureHoleFamily",
        CreationRouteHoleFamily::UnitChoice => "CreationUnitChoiceHoleFamily",
        CreationRouteHoleFamily::AbilityScore => "CreationAbilityScoreHoleFamily",
        CreationRouteHoleFamily::EquipmentSelection => "CreationEquipmentSelectionHoleFamily",
        CreationRouteHoleFamily::Loadout => "CreationLoadoutHoleFamily",
    }
}

fn fill_family_ref(fill: CreationRouteFillFamily) -> &'static str {
    match fill {
        CreationRouteFillFamily::ChoiceSet => "CreationChoiceSetFill",
        CreationRouteFillFamily::AbilityScoreAssignment => "CreationAbilityScoreAssignmentFill",
        CreationRouteFillFamily::EquipmentSelection => "CreationEquipmentSelectionFill",
        CreationRouteFillFamily::LoadoutSelection => "CreationLoadoutSelectionFill",
    }
}

fn owner_ref(owner: CreationRouteOwnerGroup) -> &'static str {
    match owner {
        CreationRouteOwnerGroup::CharacterDraft => "CharacterDraftOwner",
        CreationRouteOwnerGroup::CharacterBuild => "CharacterBuildOwner",
        CreationRouteOwnerGroup::CreationHoleFrontier => "CreationHoleFrontierOwner",
        CreationRouteOwnerGroup::CreationSupportProfileAdmission => {
            "CreationSupportProfileAdmissionOwner"
        }
        CreationRouteOwnerGroup::CreationRetainedReference => "CreationRetainedReferenceOwner",
    }
}

fn has_fighter_progression(draft: &Draft) -> bool {
    matches!(
        draft.progression,
        Some(Progression::FighterLevelOne | Progression::FighterLevelTwo)
    )
}

fn has_wizard_progression(draft: &Draft) -> bool {
    matches!(draft.progression, Some(Progression::WizardLevelOne))
}

fn hole_kind(hole: Hole) -> HoleKind {
    match hole {
        Hole::AbilityScores => HoleKind::AbilityScore,
        _ => HoleKind::Choice,
    }
}

fn fill_hole(fill: &Fill) -> Hole {
    match fill {
        Fill::Choice { hole, .. } | Fill::AbilityScores { hole, .. } => *hole,
    }
}

fn fill_kind_matches_hole(fill: &Fill, hole: Hole) -> bool {
    match fill {
        Fill::Choice { .. } => hole_kind(hole) == HoleKind::Choice,
        Fill::AbilityScores { .. } => hole_kind(hole) == HoleKind::AbilityScore,
    }
}

fn valid_options(hole: Hole) -> BTreeSet<ChoiceOption> {
    match hole {
        Hole::Progression => set(&[
            ChoiceOption::ClassFighterLevel1,
            ChoiceOption::ClassFighterLevel2,
            ChoiceOption::ClassWizardLevel1,
        ]),
        Hole::Background => set(&[ChoiceOption::BackgroundSoldier]),
        Hole::Species => set(&[ChoiceOption::SpeciesOrc]),
        Hole::Languages => set(&[
            ChoiceOption::LanguageCommonSignLanguage,
            ChoiceOption::LanguageDraconic,
            ChoiceOption::LanguageDwarvish,
            ChoiceOption::LanguageElvish,
            ChoiceOption::LanguageGiant,
            ChoiceOption::LanguageGnomish,
            ChoiceOption::LanguageGoblin,
            ChoiceOption::LanguageHalfling,
            ChoiceOption::LanguageOrc,
        ]),
        Hole::Alignment => set(&[
            ChoiceOption::AlignmentLawfulGood,
            ChoiceOption::AlignmentNeutralGood,
            ChoiceOption::AlignmentChaoticGood,
            ChoiceOption::AlignmentLawfulNeutral,
            ChoiceOption::AlignmentNeutralNeutral,
            ChoiceOption::AlignmentChaoticNeutral,
            ChoiceOption::AlignmentLawfulEvil,
            ChoiceOption::AlignmentNeutralEvil,
            ChoiceOption::AlignmentChaoticEvil,
        ]),
        Hole::ClassSkills => set(&[
            ChoiceOption::SkillAcrobatics,
            ChoiceOption::SkillAnimalHandling,
            ChoiceOption::SkillAthletics,
            ChoiceOption::SkillHistory,
            ChoiceOption::SkillInsight,
            ChoiceOption::SkillIntimidation,
            ChoiceOption::SkillPersuasion,
            ChoiceOption::SkillPerception,
            ChoiceOption::SkillSurvival,
        ]),
        Hole::FighterFightingStyle => set(&[ChoiceOption::FightingStyleDefense]),
        Hole::FighterWeaponMastery => set(&[
            ChoiceOption::WeaponLongsword,
            ChoiceOption::WeaponSpear,
            ChoiceOption::WeaponFlail,
            ChoiceOption::WeaponShortbow,
        ]),
        Hole::BackgroundAbilityScoreIncrease => set(&[
            ChoiceOption::BackgroundAsiStrDex,
            ChoiceOption::BackgroundAsiStrCon,
            ChoiceOption::BackgroundAsiDexStr,
            ChoiceOption::BackgroundAsiDexCon,
            ChoiceOption::BackgroundAsiConStr,
            ChoiceOption::BackgroundAsiConDex,
            ChoiceOption::BackgroundAsiOneEach,
        ]),
        Hole::BackgroundTool => set(&[ChoiceOption::ToolDiceSet]),
        Hole::ClassEquipment => set(&[
            ChoiceOption::ClassEquipmentPackageA,
            ChoiceOption::ClassEquipmentPackageB,
            ChoiceOption::ClassEquipmentCoins,
        ]),
        Hole::BackgroundEquipment => set(&[
            ChoiceOption::BackgroundEquipmentPack,
            ChoiceOption::BackgroundEquipmentCoins,
        ]),
        Hole::EquipmentPurchase => set(&[
            ChoiceOption::ArmorChainMail,
            ChoiceOption::WeaponLongsword,
            ChoiceOption::WeaponDagger,
            ChoiceOption::WeaponFlail,
            ChoiceOption::EquipmentShield,
        ]),
        Hole::LoadoutArmor => set(&[ChoiceOption::LoadoutWorn]),
        Hole::LoadoutShield => set(&[ChoiceOption::LoadoutWielded]),
        Hole::LoadoutWeapon => set(&[ChoiceOption::LoadoutWieldedOneHanded]),
        Hole::AbilityScores => BTreeSet::new(),
    }
}

fn supported_options(hole: Hole) -> BTreeSet<ChoiceOption> {
    match hole {
        Hole::Languages => set(&[ChoiceOption::LanguageDwarvish, ChoiceOption::LanguageGoblin]),
        Hole::Alignment => set(&[ChoiceOption::AlignmentLawfulGood]),
        Hole::ClassSkills => set(&[ChoiceOption::SkillPerception, ChoiceOption::SkillSurvival]),
        Hole::FighterWeaponMastery => set(&[
            ChoiceOption::WeaponLongsword,
            ChoiceOption::WeaponSpear,
            ChoiceOption::WeaponFlail,
        ]),
        Hole::BackgroundAbilityScoreIncrease => set(&[ChoiceOption::BackgroundAsiStrCon]),
        Hole::ClassEquipment => set(&[ChoiceOption::ClassEquipmentCoins]),
        Hole::BackgroundEquipment => set(&[ChoiceOption::BackgroundEquipmentCoins]),
        _ => valid_options(hole),
    }
}

fn required_choice_count(hole: Hole) -> usize {
    match hole {
        Hole::Languages => 2,
        Hole::ClassSkills => 2,
        Hole::FighterWeaponMastery => 3,
        Hole::EquipmentPurchase => 3,
        _ => 1,
    }
}

fn fill_issues_for_batch(fills: &[Fill], open: &BTreeSet<Hole>) -> BTreeSet<FillIssue> {
    let mut prior = Vec::new();
    let mut issues = BTreeSet::new();
    for (index, fill) in fills.iter().enumerate() {
        issues.extend(fill_issue(fill, index as u8, open, &prior));
        prior.push(fill.clone());
    }
    issues
}

fn fill_issue(
    fill: &Fill,
    fill_index: u8,
    open: &BTreeSet<Hole>,
    previous_fills: &[Fill],
) -> BTreeSet<FillIssue> {
    let hole = fill_hole(fill);
    if previous_fills
        .iter()
        .any(|previous| fill_hole(previous) == hole)
    {
        return issue(fill_index, hole, FillIssueCode::DuplicateFill);
    }
    if !open.contains(&hole) {
        return issue(fill_index, hole, FillIssueCode::UnknownHole);
    }
    if !fill_kind_matches_hole(fill, hole) {
        return issue(fill_index, hole, FillIssueCode::WrongFillKind);
    }

    match fill {
        Fill::Choice { options, .. } => choice_fill_issues(fill_index, hole, options),
        Fill::AbilityScores { method, scores, .. } => {
            if is_valid_ability_score_assignment(*method, *scores) {
                BTreeSet::new()
            } else {
                issue(fill_index, hole, FillIssueCode::InvalidAbilityScores)
            }
        }
    }
}

fn choice_fill_issues(fill_index: u8, hole: Hole, options: &[ChoiceOption]) -> BTreeSet<FillIssue> {
    let mut issues = BTreeSet::new();
    let required = required_choice_count(hole);
    if options.len() < required {
        issues.extend(issue(fill_index, hole, FillIssueCode::TooFewChoices));
    }
    if options.len() > required {
        issues.extend(issue(fill_index, hole, FillIssueCode::TooManyChoices));
    }

    if has_duplicate_options(options) || !all_options_in(options, &valid_options(hole)) {
        issues.extend(issue(fill_index, hole, FillIssueCode::InvalidChoice));
    } else if !all_options_in(options, &supported_options(hole)) {
        issues.extend(issue(fill_index, hole, FillIssueCode::UnsupportedChoice));
    }
    issues
}

fn is_valid_ability_score_assignment(method: AbilityScoreMethod, scores: AbilityScores) -> bool {
    match method {
        AbilityScoreMethod::StandardArray => {
            ability_score_set(scores) == set_u8(&[15, 14, 13, 12, 10, 8])
        }
        AbilityScoreMethod::PointBuy => {
            [
                scores.strength,
                scores.dexterity,
                scores.constitution,
                scores.intelligence,
                scores.wisdom,
                scores.charisma,
            ]
            .iter()
            .all(|score| (8..=15).contains(score))
                && point_buy_total_cost(scores) <= 27
        }
    }
}

fn point_buy_total_cost(scores: AbilityScores) -> u8 {
    [
        scores.strength,
        scores.dexterity,
        scores.constitution,
        scores.intelligence,
        scores.wisdom,
        scores.charisma,
    ]
    .iter()
    .map(|score| point_buy_cost(*score))
    .sum()
}

fn point_buy_cost(score: u8) -> u8 {
    match score {
        8 => 0,
        9 => 1,
        10 => 2,
        11 => 3,
        12 => 4,
        13 => 5,
        14 => 7,
        15 => 9,
        _ => 0,
    }
}

fn ability_score_set(scores: AbilityScores) -> BTreeSet<u8> {
    set_u8(&[
        scores.strength,
        scores.dexterity,
        scores.constitution,
        scores.intelligence,
        scores.wisdom,
        scores.charisma,
    ])
}

fn apply_fill(draft: &mut Draft, fill: &Fill) {
    match fill {
        Fill::Choice { hole, options } => match hole {
            Hole::Progression => draft.progression = progression_selection(options),
            Hole::Background => draft.background = true,
            Hole::Species => draft.species = true,
            Hole::Languages => draft.languages = true,
            Hole::Alignment => draft.alignment = true,
            Hole::ClassSkills => draft.class_skills = true,
            Hole::FighterFightingStyle => draft.fighter_fighting_style = true,
            Hole::FighterWeaponMastery => draft.fighter_weapon_mastery = true,
            Hole::BackgroundAbilityScoreIncrease => draft.background_ability_score_increase = true,
            Hole::BackgroundTool => draft.background_tool = true,
            Hole::ClassEquipment => draft.class_equipment = true,
            Hole::BackgroundEquipment => draft.background_equipment = true,
            Hole::EquipmentPurchase => draft.equipment_purchase = true,
            Hole::LoadoutArmor => draft.loadout_armor = true,
            Hole::LoadoutShield => draft.loadout_shield = true,
            Hole::LoadoutWeapon => draft.loadout_weapon = true,
            Hole::AbilityScores => {}
        },
        Fill::AbilityScores { .. } => draft.ability_scores = true,
    }
}

fn progression_selection(options: &[ChoiceOption]) -> Option<Progression> {
    if options.contains(&ChoiceOption::ClassFighterLevel1) {
        Some(Progression::FighterLevelOne)
    } else if options.contains(&ChoiceOption::ClassFighterLevel2) {
        Some(Progression::FighterLevelTwo)
    } else if options.contains(&ChoiceOption::ClassWizardLevel1) {
        Some(Progression::WizardLevelOne)
    } else {
        None
    }
}

fn has_duplicate_options(options: &[ChoiceOption]) -> bool {
    let mut seen = BTreeSet::new();
    options.iter().any(|option| !seen.insert(*option))
}

fn all_options_in(options: &[ChoiceOption], allowed: &BTreeSet<ChoiceOption>) -> bool {
    options.iter().all(|option| allowed.contains(option))
}

fn issue(fill_index: u8, hole: Hole, code: FillIssueCode) -> BTreeSet<FillIssue> {
    BTreeSet::from([FillIssue {
        fill_index,
        hole,
        code,
    }])
}

fn set(options: &[ChoiceOption]) -> BTreeSet<ChoiceOption> {
    options.iter().copied().collect()
}

fn set_u8(values: &[u8]) -> BTreeSet<u8> {
    values.iter().copied().collect()
}
