#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpellbookSpellKind {
    LevelOnePlusRitual,
    LevelOnePlusNonRitual,
    NonLeveled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpellbookRitualAccess {
    InSpellbook,
    PreparedOnly,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpellbookRitualAdept {
    Present,
    Missing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpellbookRitualFacts {
    pub spell_kind: SpellbookSpellKind,
    pub access: SpellbookRitualAccess,
    pub ritual_adept: SpellbookRitualAdept,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpellSlotCost {
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreparationRequirement {
    NotRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequiredSpellAccess {
    Spellbook,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpellbookRitualInvocation {
    pub spell_slot_cost: SpellSlotCost,
    pub preparation_requirement: PreparationRequirement,
    pub required_access: RequiredSpellAccess,
    pub additional_casting_time_minutes: i16,
    pub requires_reading_spellbook: bool,
    pub first_level_spell_slots_expended: i16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpellbookRitualResult {
    Accepted(SpellbookRitualInvocation),
    Rejected,
}

#[must_use]
pub fn can_cast_spellbook_ritual(facts: SpellbookRitualFacts) -> bool {
    // QNT: cleanroom-input/qnt/shared-algebras/proofs/rule-core/spellbook-ritual-access.qnt;
    // RAW: cleanroom-input/raw/srd-5.2.1/Classes/Wizard.md "Level 1: Ritual Adept".
    facts.spell_kind == SpellbookSpellKind::LevelOnePlusRitual
        && facts.access == SpellbookRitualAccess::InSpellbook
        && facts.ritual_adept == SpellbookRitualAdept::Present
}

#[must_use]
pub fn spellbook_ritual_invocation(facts: SpellbookRitualFacts) -> SpellbookRitualResult {
    if can_cast_spellbook_ritual(facts) {
        SpellbookRitualResult::Accepted(SpellbookRitualInvocation {
            spell_slot_cost: SpellSlotCost::None,
            preparation_requirement: PreparationRequirement::NotRequired,
            required_access: RequiredSpellAccess::Spellbook,
            additional_casting_time_minutes: 10,
            requires_reading_spellbook: true,
            first_level_spell_slots_expended: 0,
        })
    } else {
        SpellbookRitualResult::Rejected
    }
}
