#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WildShapeForm {
    TrueForm,
    RidingHorse,
    Cat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WildShapeCreatureSize {
    Tiny,
    Medium,
    Large,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WildShapeDruidStatus {
    Able,
    Incapacitated,
    Dead,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WildShapeScenarioOutcome {
    Init,
    AssumedRidingHorse,
    NextTurn,
    ReusedCat,
    Dismissed,
    FormIncapacitated,
    FormDead,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WildShapeProtocol {
    Init,
    Resolved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WildShapeState {
    pub active_form: WildShapeForm,
    pub bonus_action_available: bool,
    pub uses_remaining: i16,
    pub temporary_hit_points: i16,
    pub armor_class: i16,
    pub creature_size: WildShapeCreatureSize,
    pub speed_feet: i16,
    pub shove_dc: i16,
    pub spell_available: bool,
    pub active_form_effect_count: i16,
    pub merged_equipment_count: i16,
    pub druid_status: WildShapeDruidStatus,
    pub scenario_outcome: WildShapeScenarioOutcome,
    pub protocol: WildShapeProtocol,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WildShapeFormFacts {
    pub form: WildShapeForm,
    pub armor_class: i16,
    pub creature_size: WildShapeCreatureSize,
    pub speed_feet: i16,
    pub strength_modifier: i16,
}

pub const WILD_SHAPE_DRUID_LEVEL: i16 = 2;
pub const WILD_SHAPE_PROFICIENCY_BONUS: i16 = 2;
pub const WILD_SHAPE_MERGED_EQUIPMENT_COUNT: i16 = 2;

#[must_use]
pub fn wild_shape_initial_state() -> WildShapeState {
    WildShapeState {
        active_form: WildShapeForm::TrueForm,
        bonus_action_available: true,
        uses_remaining: 2,
        temporary_hit_points: 0,
        armor_class: 16,
        creature_size: WildShapeCreatureSize::Medium,
        speed_feet: 30,
        shove_dc: 13,
        spell_available: true,
        active_form_effect_count: 0,
        merged_equipment_count: 0,
        druid_status: WildShapeDruidStatus::Able,
        scenario_outcome: WildShapeScenarioOutcome::Init,
        protocol: WildShapeProtocol::Init,
    }
}

#[must_use]
pub fn riding_horse_wild_shape_form() -> WildShapeFormFacts {
    WildShapeFormFacts {
        form: WildShapeForm::RidingHorse,
        armor_class: 11,
        creature_size: WildShapeCreatureSize::Large,
        speed_feet: 60,
        strength_modifier: 3,
    }
}

#[must_use]
pub fn cat_wild_shape_form() -> WildShapeFormFacts {
    WildShapeFormFacts {
        form: WildShapeForm::Cat,
        armor_class: 12,
        creature_size: WildShapeCreatureSize::Tiny,
        speed_feet: 40,
        strength_modifier: -4,
    }
}

#[must_use]
pub fn assume_riding_horse_wild_shape(state: WildShapeState) -> WildShapeState {
    assume_wild_shape_form(
        state,
        riding_horse_wild_shape_form(),
        WildShapeScenarioOutcome::AssumedRidingHorse,
    )
}

#[must_use]
pub fn reuse_wild_shape_as_cat(state: WildShapeState) -> WildShapeState {
    assume_wild_shape_form(
        state,
        cat_wild_shape_form(),
        WildShapeScenarioOutcome::ReusedCat,
    )
}

#[must_use]
pub fn begin_wild_shape_next_turn(state: WildShapeState) -> WildShapeState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Rules-Glossary.md
    // "Bonus Action"; QNT:
    // cleanroom-input/qnt/battle-runtime/
    // battle-runtime-druid-wild-shape-form-lifecycle.mbt.qnt.
    if state.druid_status != WildShapeDruidStatus::Able || state.bonus_action_available {
        return state;
    }

    WildShapeState {
        bonus_action_available: true,
        scenario_outcome: WildShapeScenarioOutcome::NextTurn,
        protocol: WildShapeProtocol::Resolved,
        ..state
    }
}

#[must_use]
pub fn dismiss_wild_shape_form(state: WildShapeState) -> WildShapeState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Classes/Druid.md
    // "Level 2: Wild Shape" allows leaving the form early as a Bonus Action.
    if state.druid_status != WildShapeDruidStatus::Able
        || state.active_form == WildShapeForm::TrueForm
        || !state.bonus_action_available
    {
        return state;
    }

    true_form_projection(
        WildShapeState {
            bonus_action_available: false,
            scenario_outcome: WildShapeScenarioOutcome::Dismissed,
            protocol: WildShapeProtocol::Resolved,
            ..state
        },
        WildShapeDruidStatus::Able,
        true,
    )
}

#[must_use]
pub fn revert_wild_shape_due_to_incapacitated(state: WildShapeState) -> WildShapeState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Classes/Druid.md
    // "Level 2: Wild Shape" ends when the Druid has the Incapacitated
    // condition.
    true_form_projection(
        WildShapeState {
            scenario_outcome: WildShapeScenarioOutcome::FormIncapacitated,
            protocol: WildShapeProtocol::Resolved,
            ..state
        },
        WildShapeDruidStatus::Incapacitated,
        false,
    )
}

#[must_use]
pub fn revert_wild_shape_due_to_death(state: WildShapeState) -> WildShapeState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Rules-Glossary.md
    // "Shape-Shifting" and Classes/Druid.md "Level 2: Wild Shape".
    true_form_projection(
        WildShapeState {
            scenario_outcome: WildShapeScenarioOutcome::FormDead,
            protocol: WildShapeProtocol::Resolved,
            ..state
        },
        WildShapeDruidStatus::Dead,
        false,
    )
}

#[must_use]
pub fn stutter_wild_shape_state(state: WildShapeState) -> WildShapeState {
    state
}

fn assume_wild_shape_form(
    state: WildShapeState,
    form: WildShapeFormFacts,
    scenario_outcome: WildShapeScenarioOutcome,
) -> WildShapeState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Classes/Druid.md
    // "Level 2: Wild Shape"; QNT:
    // cleanroom-input/qnt/battle-runtime/battle-runtime-druid-wild-shape.qnt.
    if state.druid_status != WildShapeDruidStatus::Able
        || !state.bonus_action_available
        || state.uses_remaining <= 0
    {
        return state;
    }

    WildShapeState {
        active_form: form.form,
        bonus_action_available: false,
        uses_remaining: state.uses_remaining - 1,
        temporary_hit_points: state.temporary_hit_points.max(WILD_SHAPE_DRUID_LEVEL),
        armor_class: form.armor_class,
        creature_size: form.creature_size,
        speed_feet: form.speed_feet,
        shove_dc: 8 + form.strength_modifier + WILD_SHAPE_PROFICIENCY_BONUS,
        spell_available: false,
        active_form_effect_count: 1,
        merged_equipment_count: WILD_SHAPE_MERGED_EQUIPMENT_COUNT,
        druid_status: WildShapeDruidStatus::Able,
        scenario_outcome,
        protocol: WildShapeProtocol::Resolved,
    }
}

fn true_form_projection(
    state: WildShapeState,
    druid_status: WildShapeDruidStatus,
    spell_available: bool,
) -> WildShapeState {
    WildShapeState {
        active_form: WildShapeForm::TrueForm,
        armor_class: 16,
        creature_size: WildShapeCreatureSize::Medium,
        speed_feet: 30,
        shove_dc: 13,
        spell_available,
        active_form_effect_count: 0,
        merged_equipment_count: 0,
        druid_status,
        ..state
    }
}
