use crate::rules::hit_points::{
    accepted_rest_hit_point_projection, complete_long_rest_benefits,
    effective_sheet_hit_point_maximum, interrupted_long_rest_projection,
    rejected_rest_hit_point_projection, spend_hit_point_die, RestDecision, RestHitPointProjection,
    RestRejection, SheetHitPointState, LONG_REST_BASE_TICKS, LONG_REST_WAIT_TICKS,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HpRestHitDiceWitness {
    pub last_result: &'static str,
    pub accepted: bool,
    pub message: &'static str,
    pub current_hp: i16,
    pub hit_point_maximum: i16,
    pub hit_point_maximum_reduction: i16,
    pub temporary_hit_points: i16,
    pub spent_hit_dice: i16,
    pub required_long_rest_ticks: i16,
    pub remaining_wait_ticks: i16,
    pub replay_index: u8,
}

pub const BRANCH_ACTIONS: [&str; 13] = [
    "doRejectLongRestStartAtZeroHp",
    "doRejectLongRestBeforeSixteenHourWait",
    "doSpendShortRestHitPointDie",
    "doInterruptShortRestNoBenefit",
    "doCompleteLongRestRestoresHpHitPointDiceAndMaximum",
    "doInterruptLongRestBeforeOneHourNoBenefit",
    "doInterruptLongRestWithShortRestBenefits",
    "doRejectShortRestStartAtZeroHp",
    "doRejectShortRestDurationTooShort",
    "doRejectLongRestDurationTooShort",
    "doRejectLongRestPhysicalExertionTooShort",
    "doSpendShortRestHitPointDiceSequentially",
    "doRejectLongRestInterruptionAtRequiredDuration",
];

pub fn replay_observed_action(observed_action_taken: &str) -> HpRestHitDiceWitness {
    match observed_action_taken {
        "doRejectLongRestStartAtZeroHp" => reject_long_rest_start_at_zero_hp(),
        "doRejectLongRestBeforeSixteenHourWait" => reject_long_rest_before_sixteen_hour_wait(),
        "doSpendShortRestHitPointDie" => spend_short_rest_hit_point_die(),
        "doInterruptShortRestNoBenefit" => interrupt_short_rest_no_benefit(),
        "doCompleteLongRestRestoresHpHitPointDiceAndMaximum" => complete_long_rest(),
        "doInterruptLongRestBeforeOneHourNoBenefit" => interrupt_long_rest_before_one_hour(),
        "doInterruptLongRestWithShortRestBenefits" => {
            interrupt_long_rest_with_short_rest_benefits()
        }
        "doRejectShortRestStartAtZeroHp" => reject_short_rest_start_at_zero_hp(),
        "doRejectShortRestDurationTooShort" => reject_short_rest_duration_too_short(),
        "doRejectLongRestDurationTooShort" => reject_long_rest_duration_too_short(),
        "doRejectLongRestPhysicalExertionTooShort" => {
            reject_long_rest_physical_exertion_too_short()
        }
        "doSpendShortRestHitPointDiceSequentially" => {
            spend_short_rest_hit_point_dice_sequentially()
        }
        "doRejectLongRestInterruptionAtRequiredDuration" => {
            reject_long_rest_interruption_at_required_duration()
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> HpRestHitDiceWitness {
    replay_observed_action(observed_action_taken)
}

pub fn projection_payload(witness: &HpRestHitDiceWitness) -> String {
    [
        format!("lastResult={}", witness.last_result),
        format!("accepted={}", witness.accepted),
        format!("message={}", witness.message),
        format!("currentHp={}", witness.current_hp),
        format!("hitPointMaximum={}", witness.hit_point_maximum),
        format!(
            "hitPointMaximumReduction={}",
            witness.hit_point_maximum_reduction
        ),
        format!("temporaryHitPoints={}", witness.temporary_hit_points),
        format!("spentHitDice={}", witness.spent_hit_dice),
        format!("requiredLongRestTicks={}", witness.required_long_rest_ticks),
        format!("remainingWaitTicks={}", witness.remaining_wait_ticks),
        format!("replayIndex={}", witness.replay_index),
    ]
    .join("\n")
}

fn reject_long_rest_start_at_zero_hp() -> HpRestHitDiceWitness {
    let projection = rejected_rest_hit_point_projection(
        RestRejection::LongRestStartAtZeroHitPoints,
        zero_hp_sheet(),
        0,
        0,
    );
    witness("long-rest-start-zero-hp-rejected", 1, projection)
}

fn reject_long_rest_before_sixteen_hour_wait() -> HpRestHitDiceWitness {
    let projection = rejected_rest_hit_point_projection(
        RestRejection::LongRestBeforeSixteenHourWait,
        full_hp_sheet(),
        0,
        LONG_REST_WAIT_TICKS - (LONG_REST_WAIT_TICKS - 1),
    );
    witness("long-rest-sixteen-hour-wait-rejected", 2, projection)
}

fn spend_short_rest_hit_point_die() -> HpRestHitDiceWitness {
    let projection =
        accepted_rest_hit_point_projection(spend_hit_point_die(wounded_wizard(), 4, 1));
    witness("short-rest-spend-hit-point-die", 3, projection)
}

fn interrupt_short_rest_no_benefit() -> HpRestHitDiceWitness {
    let projection = accepted_rest_hit_point_projection(wounded_wizard());
    witness("short-rest-interrupted-no-benefit", 4, projection)
}

fn complete_long_rest() -> HpRestHitDiceWitness {
    let projection =
        accepted_rest_hit_point_projection(complete_long_rest_benefits(SheetHitPointState {
            current_hp: 7,
            normal_hit_point_maximum: 18,
            hit_point_maximum_reduction: 6,
            temporary_hit_points: 2,
            spent_hit_dice: 1,
        }));
    witness(
        "long-rest-restores-hp-hit-point-dice-and-maximum",
        5,
        projection,
    )
}

fn interrupt_long_rest_before_one_hour() -> HpRestHitDiceWitness {
    let projection = interrupted_long_rest_projection(wounded_wizard(), false);
    witness(
        "long-rest-interrupted-before-one-hour-no-benefit",
        6,
        projection,
    )
}

fn interrupt_long_rest_with_short_rest_benefits() -> HpRestHitDiceWitness {
    let projection = interrupted_long_rest_projection(wounded_wizard(), true);
    witness(
        "long-rest-interrupted-with-short-rest-benefits",
        7,
        projection,
    )
}

fn reject_short_rest_start_at_zero_hp() -> HpRestHitDiceWitness {
    let projection = rejected_rest_hit_point_projection(
        RestRejection::ShortRestStartAtZeroHitPoints,
        zero_hp_sheet(),
        0,
        0,
    );
    witness("short-rest-start-zero-hp-rejected", 8, projection)
}

fn reject_short_rest_duration_too_short() -> HpRestHitDiceWitness {
    let projection = rejected_rest_hit_point_projection(
        RestRejection::ShortRestDurationTooShort,
        wounded_wizard(),
        0,
        0,
    );
    witness("short-rest-duration-too-short-rejected", 9, projection)
}

fn reject_long_rest_duration_too_short() -> HpRestHitDiceWitness {
    let projection = rejected_rest_hit_point_projection(
        RestRejection::LongRestDurationTooShort,
        wounded_wizard(),
        LONG_REST_BASE_TICKS,
        0,
    );
    witness("long-rest-duration-too-short-rejected", 10, projection)
}

fn reject_long_rest_physical_exertion_too_short() -> HpRestHitDiceWitness {
    let projection = rejected_rest_hit_point_projection(
        RestRejection::LongRestPhysicalExertionTooShort,
        wounded_wizard(),
        0,
        0,
    );
    witness(
        "long-rest-physical-exertion-too-short-rejected",
        11,
        projection,
    )
}

fn spend_short_rest_hit_point_dice_sequentially() -> HpRestHitDiceWitness {
    let first = spend_hit_point_die(wounded_wizard(), 4, 1);
    let second = spend_hit_point_die(first, 3, 1);
    let projection = accepted_rest_hit_point_projection(second);
    witness(
        "short-rest-spend-hit-point-dice-sequentially",
        12,
        projection,
    )
}

fn reject_long_rest_interruption_at_required_duration() -> HpRestHitDiceWitness {
    let projection = rejected_rest_hit_point_projection(
        RestRejection::LongRestInterruptionAtRequiredDuration,
        wounded_wizard(),
        LONG_REST_BASE_TICKS,
        0,
    );
    witness(
        "long-rest-interruption-at-required-duration-rejected",
        13,
        projection,
    )
}

fn wounded_wizard() -> SheetHitPointState {
    SheetHitPointState {
        current_hp: 7,
        normal_hit_point_maximum: 18,
        hit_point_maximum_reduction: 0,
        temporary_hit_points: 0,
        spent_hit_dice: 0,
    }
}

fn zero_hp_sheet() -> SheetHitPointState {
    SheetHitPointState {
        current_hp: 0,
        normal_hit_point_maximum: 12,
        hit_point_maximum_reduction: 0,
        temporary_hit_points: 0,
        spent_hit_dice: 0,
    }
}

fn full_hp_sheet() -> SheetHitPointState {
    SheetHitPointState {
        current_hp: 12,
        normal_hit_point_maximum: 12,
        hit_point_maximum_reduction: 0,
        temporary_hit_points: 0,
        spent_hit_dice: 0,
    }
}

fn witness(
    last_result: &'static str,
    replay_index: u8,
    projection: RestHitPointProjection,
) -> HpRestHitDiceWitness {
    HpRestHitDiceWitness {
        last_result,
        accepted: projection.decision == RestDecision::Accepted,
        message: message(projection.decision),
        current_hp: projection.sheet.current_hp,
        hit_point_maximum: effective_sheet_hit_point_maximum(projection.sheet),
        hit_point_maximum_reduction: projection.sheet.hit_point_maximum_reduction,
        temporary_hit_points: projection.sheet.temporary_hit_points,
        spent_hit_dice: projection.sheet.spent_hit_dice,
        required_long_rest_ticks: projection.required_long_rest_ticks,
        remaining_wait_ticks: projection.remaining_wait_ticks,
        replay_index,
    }
}

fn message(decision: RestDecision) -> &'static str {
    match decision {
        RestDecision::Accepted => "none",
        RestDecision::Rejected(RestRejection::LongRestStartAtZeroHitPoints) => {
            "Long Rest requires the Character Sheet to have at least 1 HP."
        }
        RestDecision::Rejected(RestRejection::LongRestBeforeSixteenHourWait) => {
            "Long Rest requires waiting 16 hours after finishing the previous Long Rest."
        }
        RestDecision::Rejected(RestRejection::ShortRestStartAtZeroHitPoints) => {
            "Short Rest requires the Character Sheet to have at least 1 HP."
        }
        RestDecision::Rejected(RestRejection::ShortRestDurationTooShort) => {
            "Short Rest requires 1 hour before benefits can be received."
        }
        RestDecision::Rejected(RestRejection::LongRestDurationTooShort) => {
            "Long Rest requires the full required duration before benefits can be received."
        }
        RestDecision::Rejected(RestRejection::LongRestPhysicalExertionTooShort) => {
            "Long Rest physical exertion interruption requires at least 1 hour."
        }
        RestDecision::Rejected(RestRejection::LongRestInterruptionAtRequiredDuration) => {
            "Long Rest interruption requires rested time before the required Long Rest duration."
        }
    }
}
