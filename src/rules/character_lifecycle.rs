#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterLifecycleLayer {
    Draft,
    Build,
    Sheet,
    BattleInitProjection,
    BattleRuntime,
    Settlement,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CharacterLifecycleFacts {
    draft_has_open_holes: bool,
    build_finalized: bool,
    sheet_owns_hit_points: bool,
    sheet_current_hp: i16,
    sheet_max_hp: i16,
    battle_init_character_combatant: bool,
    battle_runtime_character_hp: i16,
    battle_runtime_hp_changed: bool,
    settlement_current_hp: i16,
    settlement_persisted_battle_hp: bool,
    build_identity_unchanged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CharacterLifecycleProjection {
    layer: CharacterLifecycleLayer,
    facts: CharacterLifecycleFacts,
    replay_index: i16,
}

const SHEET_WITNESS_HIT_POINTS: i16 = 12;
const SHEET_WITNESS_HIT_POINT_MAXIMUM: i16 = 12;
const POST_SKELETON_SHORTSWORD_HIT_POINTS: i16 = 8;

impl CharacterLifecycleFacts {
    #[must_use]
    pub fn draft_has_open_holes(self) -> bool {
        self.draft_has_open_holes
    }

    #[must_use]
    pub fn build_finalized(self) -> bool {
        self.build_finalized
    }

    #[must_use]
    pub fn sheet_owns_hit_points(self) -> bool {
        self.sheet_owns_hit_points
    }

    #[must_use]
    pub fn sheet_current_hp(self) -> i16 {
        self.sheet_current_hp
    }

    #[must_use]
    pub fn sheet_max_hp(self) -> i16 {
        self.sheet_max_hp
    }

    #[must_use]
    pub fn battle_init_character_combatant(self) -> bool {
        self.battle_init_character_combatant
    }

    #[must_use]
    pub fn battle_runtime_character_hp(self) -> i16 {
        self.battle_runtime_character_hp
    }

    #[must_use]
    pub fn battle_runtime_hp_changed(self) -> bool {
        self.battle_runtime_hp_changed
    }

    #[must_use]
    pub fn settlement_current_hp(self) -> i16 {
        self.settlement_current_hp
    }

    #[must_use]
    pub fn settlement_persisted_battle_hp(self) -> bool {
        self.settlement_persisted_battle_hp
    }

    #[must_use]
    pub fn build_identity_unchanged(self) -> bool {
        self.build_identity_unchanged
    }
}

impl CharacterLifecycleProjection {
    #[must_use]
    pub fn layer(self) -> CharacterLifecycleLayer {
        self.layer
    }

    #[must_use]
    pub fn facts(self) -> CharacterLifecycleFacts {
        self.facts
    }

    #[must_use]
    pub fn replay_index(self) -> i16 {
        self.replay_index
    }
}

#[must_use]
pub fn empty_lifecycle_facts() -> CharacterLifecycleFacts {
    CharacterLifecycleFacts {
        draft_has_open_holes: false,
        build_finalized: false,
        sheet_owns_hit_points: false,
        sheet_current_hp: 0,
        sheet_max_hp: 0,
        battle_init_character_combatant: false,
        battle_runtime_character_hp: 0,
        battle_runtime_hp_changed: false,
        settlement_current_hp: 0,
        settlement_persisted_battle_hp: false,
        build_identity_unchanged: false,
    }
}

#[must_use]
pub fn draft_character_lifecycle_projection() -> CharacterLifecycleProjection {
    // QNT: cleanroom-input/qnt/character-battle-runtime/
    // character-layer-projection-lifecycle.mbt.qnt `init`.
    record_projection(
        CharacterLifecycleLayer::Draft,
        CharacterLifecycleFacts {
            draft_has_open_holes: true,
            ..empty_lifecycle_facts()
        },
        0,
    )
}

#[must_use]
pub fn finalize_draft_to_build() -> CharacterLifecycleProjection {
    // QNT: character-layer-projection-lifecycle.mbt.qnt
    // `doFinalizeDraftToBuild`.
    record_projection(CharacterLifecycleLayer::Build, finalized_build_facts(), 1)
}

#[must_use]
pub fn create_sheet_from_build() -> CharacterLifecycleProjection {
    // RAW: cleanroom-input/raw/srd-5.2.1/Character-Creation.md
    // "Hit Points"; domain: Character Sheet owns persistent PC HP.
    // QNT: character-layer-projection-lifecycle.mbt.qnt
    // `doCreateSheetFromBuild`.
    record_projection(CharacterLifecycleLayer::Sheet, character_sheet_facts(), 2)
}

#[must_use]
pub fn project_sheet_to_battle_init() -> CharacterLifecycleProjection {
    // Domain: a Character Sheet projects creature-level combat statistics.
    // QNT: character-layer-projection-lifecycle.mbt.qnt
    // `doProjectSheetToBattleInit`.
    record_projection(
        CharacterLifecycleLayer::BattleInitProjection,
        battle_init_projection_facts(),
        3,
    )
}

#[must_use]
pub fn resolve_skeleton_shortsword_attack() -> CharacterLifecycleProjection {
    // RAW: cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md
    // "Hit Points"; QNT: character-layer-projection-lifecycle.mbt.qnt
    // `doResolveSkeletonShortswordAttack`.
    record_projection(
        CharacterLifecycleLayer::BattleRuntime,
        battle_runtime_facts(),
        4,
    )
}

#[must_use]
pub fn settle_battle_to_sheet() -> CharacterLifecycleProjection {
    // Settlement writes accepted battle-owned HP deltas back to the
    // Character Sheet while preserving build identity. QNT:
    // character-layer-projection-lifecycle.mbt.qnt `doSettleBattleToSheet`.
    record_projection(CharacterLifecycleLayer::Settlement, settlement_facts(), 5)
}

fn record_projection(
    layer: CharacterLifecycleLayer,
    facts: CharacterLifecycleFacts,
    replay_index: i16,
) -> CharacterLifecycleProjection {
    CharacterLifecycleProjection {
        layer,
        facts,
        replay_index,
    }
}

fn finalized_build_facts() -> CharacterLifecycleFacts {
    CharacterLifecycleFacts {
        build_finalized: true,
        ..empty_lifecycle_facts()
    }
}

fn character_sheet_facts() -> CharacterLifecycleFacts {
    CharacterLifecycleFacts {
        sheet_owns_hit_points: true,
        sheet_current_hp: SHEET_WITNESS_HIT_POINTS,
        sheet_max_hp: SHEET_WITNESS_HIT_POINT_MAXIMUM,
        ..finalized_build_facts()
    }
}

fn battle_init_projection_facts() -> CharacterLifecycleFacts {
    CharacterLifecycleFacts {
        battle_init_character_combatant: true,
        ..character_sheet_facts()
    }
}

fn battle_runtime_facts() -> CharacterLifecycleFacts {
    CharacterLifecycleFacts {
        battle_runtime_character_hp: POST_SKELETON_SHORTSWORD_HIT_POINTS,
        battle_runtime_hp_changed: true,
        ..battle_init_projection_facts()
    }
}

fn settlement_facts() -> CharacterLifecycleFacts {
    CharacterLifecycleFacts {
        settlement_current_hp: POST_SKELETON_SHORTSWORD_HIT_POINTS,
        settlement_persisted_battle_hp: true,
        build_identity_unchanged: true,
        ..battle_runtime_facts()
    }
}
