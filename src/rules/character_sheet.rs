#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FullCasterSpellSlotTableLevel {
    One,
    Two,
    Three,
    Four,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PactMagicSpellSlotTableLevel {
    One,
    Two,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrdinarySpellSlotTable {
    pub level1_capacity: i16,
    pub level2_capacity: i16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PactMagicSlotTable {
    pub slot_level: i16,
    pub capacity: i16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrdinarySpellSlotFacts {
    pub level1_capacity: i16,
    pub level1_expended: i16,
    pub level2_capacity: i16,
    pub level2_expended: i16,
    pub created_level1_capacity: i16,
    pub created_level1_expended: i16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PactSlotFacts {
    pub slot_level: i16,
    pub capacity: i16,
    pub expended: i16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SheetSlotFacts {
    pub ordinary: OrdinarySpellSlotFacts,
    pub pact: PactSlotFacts,
    pub arcane_recovery_used_since_long_rest: bool,
    pub magical_cunning_used_since_long_rest: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SheetClassFeatureFacts {
    pub expected_spell_present: bool,
    pub spell_count: i16,
    pub ability_check_bonus: i16,
    pub spellcasting_source_present: bool,
    pub land_choice_present: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SheetClassFeatureProjection {
    pub expected_spell_present: bool,
    pub spell_count: i16,
    pub ability_check_bonus: i16,
    pub spellcasting_source_present: bool,
    pub land_choice_present: bool,
    pub accepted: bool,
}

#[must_use]
pub fn full_caster_spell_slot_table(
    level: FullCasterSpellSlotTableLevel,
) -> OrdinarySpellSlotTable {
    // QNT: cleanroom-input/qnt/shared-algebras/proofs/rule-core/
    // spell-slot-table.qnt; RAW: class Spell Slots tables.
    match level {
        FullCasterSpellSlotTableLevel::One => OrdinarySpellSlotTable {
            level1_capacity: 2,
            level2_capacity: 0,
        },
        FullCasterSpellSlotTableLevel::Two => OrdinarySpellSlotTable {
            level1_capacity: 3,
            level2_capacity: 0,
        },
        FullCasterSpellSlotTableLevel::Three => OrdinarySpellSlotTable {
            level1_capacity: 4,
            level2_capacity: 2,
        },
        FullCasterSpellSlotTableLevel::Four => OrdinarySpellSlotTable {
            level1_capacity: 4,
            level2_capacity: 3,
        },
    }
}

#[must_use]
pub fn pact_magic_spell_slot_table(level: PactMagicSpellSlotTableLevel) -> PactMagicSlotTable {
    // QNT: cleanroom-input/qnt/shared-algebras/proofs/rule-core/
    // spell-slot-table.qnt; RAW: Warlock "Pact Magic".
    match level {
        PactMagicSpellSlotTableLevel::One => PactMagicSlotTable {
            slot_level: 1,
            capacity: 1,
        },
        PactMagicSpellSlotTableLevel::Two => PactMagicSlotTable {
            slot_level: 1,
            capacity: 2,
        },
    }
}

#[must_use]
pub fn no_ordinary_spell_slots() -> OrdinarySpellSlotFacts {
    OrdinarySpellSlotFacts {
        level1_capacity: 0,
        level1_expended: 0,
        level2_capacity: 0,
        level2_expended: 0,
        created_level1_capacity: 0,
        created_level1_expended: 0,
    }
}

#[must_use]
pub fn no_pact_slots() -> PactSlotFacts {
    PactSlotFacts {
        slot_level: 0,
        capacity: 0,
        expended: 0,
    }
}

#[must_use]
pub fn empty_slot_facts() -> SheetSlotFacts {
    SheetSlotFacts {
        ordinary: no_ordinary_spell_slots(),
        pact: no_pact_slots(),
        arcane_recovery_used_since_long_rest: false,
        magical_cunning_used_since_long_rest: false,
    }
}

#[must_use]
pub fn ordinary_slots_from_table(
    table: OrdinarySpellSlotTable,
    level1_expended: i16,
    level2_expended: i16,
    created_level1_capacity: i16,
    created_level1_expended: i16,
) -> OrdinarySpellSlotFacts {
    OrdinarySpellSlotFacts {
        level1_capacity: table.level1_capacity,
        level1_expended,
        level2_capacity: table.level2_capacity,
        level2_expended,
        created_level1_capacity,
        created_level1_expended,
    }
}

#[must_use]
pub fn pact_slots_from_table(table: PactMagicSlotTable, expended: i16) -> PactSlotFacts {
    PactSlotFacts {
        slot_level: table.slot_level,
        capacity: table.capacity,
        expended,
    }
}

#[must_use]
pub fn ordinary_level1_capacity_matches(sheet: SheetSlotFacts, expected_capacity: i16) -> bool {
    sheet.ordinary.level1_capacity == expected_capacity
}

#[must_use]
pub fn pact_slot_expenditure_within_capacity(sheet: SheetSlotFacts) -> bool {
    sheet.pact.expended <= sheet.pact.capacity
}

#[must_use]
pub fn recover_pact_slots(sheet: SheetSlotFacts) -> SheetSlotFacts {
    SheetSlotFacts {
        pact: PactSlotFacts {
            expended: 0,
            ..sheet.pact
        },
        ..sheet
    }
}

#[must_use]
pub fn complete_long_rest_slot_benefits(sheet: SheetSlotFacts) -> SheetSlotFacts {
    // RAW: Spell Slots return on Long Rest; Pact Magic slots return on Short
    // or Long Rest; Arcane Recovery and Magical Cunning reset on Long Rest.
    SheetSlotFacts {
        ordinary: OrdinarySpellSlotFacts {
            level1_expended: 0,
            level2_expended: 0,
            created_level1_capacity: 0,
            created_level1_expended: 0,
            ..sheet.ordinary
        },
        pact: PactSlotFacts {
            expended: 0,
            ..sheet.pact
        },
        arcane_recovery_used_since_long_rest: false,
        magical_cunning_used_since_long_rest: false,
    }
}

#[must_use]
pub fn can_recover_ordinary_spell_slots(
    sheet: SheetSlotFacts,
    level1_count: i16,
    level2_count: i16,
) -> bool {
    level1_count <= sheet.ordinary.level1_expended && level2_count <= sheet.ordinary.level2_expended
}

#[must_use]
pub fn apply_arcane_recovery(
    sheet: SheetSlotFacts,
    level1_count: i16,
    level2_count: i16,
) -> SheetSlotFacts {
    // RAW: Wizard "Arcane Recovery"; QNT: spell-slot-transitions.qnt.
    SheetSlotFacts {
        ordinary: OrdinarySpellSlotFacts {
            level1_expended: sheet.ordinary.level1_expended - level1_count,
            level2_expended: sheet.ordinary.level2_expended - level2_count,
            ..sheet.ordinary
        },
        arcane_recovery_used_since_long_rest: true,
        ..sheet
    }
}

#[must_use]
pub fn magical_cunning_recovery_count(sheet: SheetSlotFacts) -> i16 {
    (sheet.pact.capacity + 1) / 2
}

#[must_use]
pub fn can_apply_magical_cunning(sheet: SheetSlotFacts) -> bool {
    sheet.pact.expended > 0
}

#[must_use]
pub fn apply_magical_cunning(sheet: SheetSlotFacts) -> SheetSlotFacts {
    // RAW: Warlock "Magical Cunning"; QNT: spell-slot-transitions.qnt.
    let recovered = magical_cunning_recovery_count(sheet);
    SheetSlotFacts {
        pact: PactSlotFacts {
            expended: (sheet.pact.expended - recovered).max(0),
            ..sheet.pact
        },
        magical_cunning_used_since_long_rest: true,
        ..sheet
    }
}

#[must_use]
pub fn project_sheet_class_feature(facts: SheetClassFeatureFacts) -> SheetClassFeatureProjection {
    // QNT: character-sheet-class-feature-selected-identity.mbt.qnt. This
    // projects shape facts selected at the catalog boundary; production logic
    // does not branch on feature names or ids.
    SheetClassFeatureProjection {
        expected_spell_present: facts.expected_spell_present,
        spell_count: facts.spell_count,
        ability_check_bonus: facts.ability_check_bonus,
        spellcasting_source_present: facts.spellcasting_source_present,
        land_choice_present: facts.land_choice_present,
        accepted: true,
    }
}
