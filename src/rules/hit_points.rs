use crate::rules::feature_resources::{
    apply_feature_resource_hit_point_healing, FeatureResourceHitPoints,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HitPointMaximumFacts {
    pub starting_hit_point_die: i16,
    pub constitution_modifier: i16,
    pub fixed_higher_level_hit_point_dice: Vec<i16>,
    pub hit_point_maximum_bonus: i16,
    pub hit_point_maximum_reduction: i16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HitPointMaximumProjection {
    pub normal_hit_point_maximum: i16,
    pub effective_hit_point_maximum: i16,
    pub hit_dice_total: i16,
    pub hit_point_maximum_reduction: i16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HitPointMaximumError {
    IllegalFacts,
    HitDiceOverflow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SheetHitPointState {
    pub current_hp: i16,
    pub normal_hit_point_maximum: i16,
    pub hit_point_maximum_reduction: i16,
    pub temporary_hit_points: i16,
    pub spent_hit_dice: i16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RestRejection {
    ShortRestStartAtZeroHitPoints,
    ShortRestDurationTooShort,
    LongRestStartAtZeroHitPoints,
    LongRestBeforeSixteenHourWait,
    LongRestDurationTooShort,
    LongRestPhysicalExertionTooShort,
    LongRestInterruptionAtRequiredDuration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RestDecision {
    Accepted,
    Rejected(RestRejection),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RestHitPointProjection {
    pub decision: RestDecision,
    pub sheet: SheetHitPointState,
    pub required_long_rest_ticks: i16,
    pub remaining_wait_ticks: i16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeathSavingThrowTurnRole {
    Actor,
    Target,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeathSavingThrowInvalidReason {
    WrongActor,
    InvalidFill,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeathSavingThrowProtocol {
    Init,
    NeedsSavingThrow,
    Resolved,
    Invalid(DeathSavingThrowInvalidReason),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeathSavingThrowState {
    pub current_turn_role: DeathSavingThrowTurnRole,
    pub target_hp: i16,
    pub target_unconscious: bool,
    pub target_stable: bool,
    pub target_dead: bool,
    pub target_death_successes: i16,
    pub target_death_failures: i16,
    pub protocol: DeathSavingThrowProtocol,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeathSavingThrowFacts {
    pub natural_d20: i16,
}

pub const SHORT_REST_TICKS: i16 = 600;
pub const LONG_REST_BASE_TICKS: i16 = 8 * SHORT_REST_TICKS;
pub const LONG_REST_WAIT_TICKS: i16 = 16 * SHORT_REST_TICKS;

#[must_use]
pub fn fixed_higher_level_hit_point_gain(hit_point_die: i16, constitution_modifier: i16) -> i16 {
    // QNT: cleanroom-input/qnt/shared-algebras/proofs/rule-core/hit-point-maximum.qnt;
    // RAW: cleanroom-input/raw/srd-5.2.1/Character-Creation.md
    // "Gaining a Level" and "Fixed Hit Points by Class".
    let gain = (hit_point_die / 2) + 1 + constitution_modifier;
    gain.max(1)
}

#[must_use]
pub fn normal_hit_point_maximum(facts: &HitPointMaximumFacts) -> i16 {
    facts.starting_hit_point_die
        + facts.constitution_modifier
        + facts
            .fixed_higher_level_hit_point_dice
            .iter()
            .map(|die| fixed_higher_level_hit_point_gain(*die, facts.constitution_modifier))
            .sum::<i16>()
        + facts.hit_point_maximum_bonus
}

#[must_use]
pub fn effective_hit_point_maximum(facts: &HitPointMaximumFacts) -> i16 {
    normal_hit_point_maximum(facts) - facts.hit_point_maximum_reduction
}

#[must_use]
pub fn legal_hit_point_maximum_facts(facts: &HitPointMaximumFacts) -> bool {
    facts.starting_hit_point_die > 0
        && facts
            .fixed_higher_level_hit_point_dice
            .iter()
            .all(|die| *die > 0)
        && normal_hit_point_maximum(facts) > 0
        && facts.hit_point_maximum_reduction >= 0
        && facts.hit_point_maximum_reduction < normal_hit_point_maximum(facts)
}

pub fn hit_point_maximum_projection(
    facts: HitPointMaximumFacts,
) -> Result<HitPointMaximumProjection, HitPointMaximumError> {
    // QNT: cleanroom-input/qnt/shared-algebras/proofs/rule-core/hit-point-maximum.qnt;
    // RAW: Character-Creation.md "Level 1 Hit Points by Class",
    // "Fixed Hit Points by Class", and "Hit Points and Hit Point Dice".
    if !legal_hit_point_maximum_facts(&facts) {
        return Err(HitPointMaximumError::IllegalFacts);
    }

    let hit_dice_total = i16::try_from(facts.fixed_higher_level_hit_point_dice.len())
        .map_err(|_| HitPointMaximumError::HitDiceOverflow)?
        .checked_add(1)
        .ok_or(HitPointMaximumError::HitDiceOverflow)?;

    Ok(HitPointMaximumProjection {
        normal_hit_point_maximum: normal_hit_point_maximum(&facts),
        effective_hit_point_maximum: effective_hit_point_maximum(&facts),
        hit_dice_total,
        hit_point_maximum_reduction: facts.hit_point_maximum_reduction,
    })
}

#[must_use]
pub fn effective_sheet_hit_point_maximum(sheet: SheetHitPointState) -> i16 {
    sheet.normal_hit_point_maximum - sheet.hit_point_maximum_reduction
}

#[must_use]
pub fn heal_sheet_hit_points(sheet: SheetHitPointState, raw_healing: i16) -> SheetHitPointState {
    // QNT: cleanroom-input/qnt/character-sheet-runtime/
    // character-sheet-hp-rest-hit-dice.mbt.qnt healSheetHitPoints.
    let healed = apply_feature_resource_hit_point_healing(
        FeatureResourceHitPoints {
            current_hit_points: sheet.current_hp,
            hit_point_maximum: effective_sheet_hit_point_maximum(sheet),
            temporary_hit_points: sheet.temporary_hit_points,
        },
        raw_healing,
    );

    SheetHitPointState {
        current_hp: healed.current_hit_points,
        normal_hit_point_maximum: sheet.normal_hit_point_maximum,
        hit_point_maximum_reduction: sheet.hit_point_maximum_reduction,
        temporary_hit_points: healed.temporary_hit_points,
        spent_hit_dice: sheet.spent_hit_dice,
    }
}

#[must_use]
pub fn spend_hit_point_die(
    sheet: SheetHitPointState,
    die_roll: i16,
    constitution_modifier: i16,
) -> SheetHitPointState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Rules-Glossary.md "Short Rest";
    // QNT: character-sheet-hp-rest-hit-dice.mbt.qnt spendHitPointDie.
    let healing = (die_roll + constitution_modifier).max(1);
    let healed = heal_sheet_hit_points(sheet, healing);

    SheetHitPointState {
        spent_hit_dice: healed.spent_hit_dice + 1,
        ..healed
    }
}

#[must_use]
pub fn complete_long_rest_benefits(sheet: SheetHitPointState) -> SheetHitPointState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Rules-Glossary.md "Long Rest";
    // QNT: character-sheet-hp-rest-hit-dice.mbt.qnt completeLongRestBenefits.
    SheetHitPointState {
        current_hp: sheet.normal_hit_point_maximum,
        normal_hit_point_maximum: sheet.normal_hit_point_maximum,
        hit_point_maximum_reduction: 0,
        temporary_hit_points: 0,
        spent_hit_dice: 0,
    }
}

#[must_use]
pub fn accepted_rest_hit_point_projection(sheet: SheetHitPointState) -> RestHitPointProjection {
    RestHitPointProjection {
        decision: RestDecision::Accepted,
        sheet,
        required_long_rest_ticks: 0,
        remaining_wait_ticks: 0,
    }
}

#[must_use]
pub fn interrupted_long_rest_projection(
    sheet: SheetHitPointState,
    rested_at_least_one_hour: bool,
) -> RestHitPointProjection {
    let sheet = if rested_at_least_one_hour {
        spend_hit_point_die(sheet, 4, 1)
    } else {
        sheet
    };

    RestHitPointProjection {
        decision: RestDecision::Accepted,
        sheet,
        required_long_rest_ticks: LONG_REST_BASE_TICKS + SHORT_REST_TICKS,
        remaining_wait_ticks: 0,
    }
}

#[must_use]
pub fn rejected_rest_hit_point_projection(
    rejection: RestRejection,
    sheet: SheetHitPointState,
    required_long_rest_ticks: i16,
    remaining_wait_ticks: i16,
) -> RestHitPointProjection {
    RestHitPointProjection {
        decision: RestDecision::Rejected(rejection),
        sheet,
        required_long_rest_ticks,
        remaining_wait_ticks,
    }
}

#[must_use]
pub fn death_saving_throw_initial_state() -> DeathSavingThrowState {
    DeathSavingThrowState {
        current_turn_role: DeathSavingThrowTurnRole::Actor,
        target_hp: 0,
        target_unconscious: true,
        target_stable: false,
        target_dead: false,
        target_death_successes: 2,
        target_death_failures: 1,
        protocol: DeathSavingThrowProtocol::Init,
    }
}

#[must_use]
pub fn discover_death_saving_throw(state: DeathSavingThrowState) -> DeathSavingThrowState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md
    // "Death Saving Throws"; QNT:
    // battle-runtime-death-saving-throw.mbt.qnt.
    if state.current_turn_role != DeathSavingThrowTurnRole::Actor {
        return DeathSavingThrowState {
            protocol: DeathSavingThrowProtocol::Invalid(DeathSavingThrowInvalidReason::WrongActor),
            ..state
        };
    }

    DeathSavingThrowState {
        protocol: DeathSavingThrowProtocol::NeedsSavingThrow,
        ..state
    }
}

#[must_use]
pub fn fill_death_saving_throw(
    state: DeathSavingThrowState,
    facts: DeathSavingThrowFacts,
) -> DeathSavingThrowState {
    if state.protocol != DeathSavingThrowProtocol::NeedsSavingThrow {
        return DeathSavingThrowState {
            protocol: DeathSavingThrowProtocol::Invalid(DeathSavingThrowInvalidReason::InvalidFill),
            ..state
        };
    }

    let mut resolved = DeathSavingThrowState {
        current_turn_role: DeathSavingThrowTurnRole::Target,
        target_hp: 0,
        target_unconscious: true,
        target_stable: false,
        target_dead: false,
        target_death_successes: state.target_death_successes,
        target_death_failures: state.target_death_failures,
        protocol: DeathSavingThrowProtocol::Resolved,
    };

    match facts.natural_d20 {
        1 => {
            resolved.target_death_failures = (state.target_death_failures + 2).min(3);
            resolved.target_dead = resolved.target_death_failures >= 3;
        }
        2..=9 => {
            resolved.target_death_failures = (state.target_death_failures + 1).min(3);
            resolved.target_dead = resolved.target_death_failures >= 3;
        }
        10..=19 => {
            resolved.target_death_successes = state.target_death_successes + 1;
            if resolved.target_death_successes >= 3 {
                resolved.target_stable = true;
                resolved.target_death_successes = 0;
                resolved.target_death_failures = 0;
            }
        }
        20 => {
            resolved.target_hp = 1;
            resolved.target_unconscious = false;
            resolved.target_death_successes = 0;
            resolved.target_death_failures = 0;
        }
        _ => {
            return DeathSavingThrowState {
                protocol: DeathSavingThrowProtocol::Invalid(
                    DeathSavingThrowInvalidReason::InvalidFill,
                ),
                ..state
            };
        }
    }

    resolved
}

#[must_use]
pub fn reject_wrong_actor_after_resolved(state: DeathSavingThrowState) -> DeathSavingThrowState {
    DeathSavingThrowState {
        protocol: DeathSavingThrowProtocol::Invalid(DeathSavingThrowInvalidReason::WrongActor),
        ..state
    }
}
