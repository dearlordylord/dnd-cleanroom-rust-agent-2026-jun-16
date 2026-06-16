use crate::rules::quickened_spell_governor::{
    reject_quickened_one_per_spell, reject_quickened_prior_level_one_plus_spell,
    reject_quickened_unaffordable, reject_quickened_unknown_option,
    reject_quickened_unsupported_second_option, resolve_quickened_after_magic_action_spent,
    resolve_quickened_direct_condition, resolve_quickened_restoration,
    resolve_quickened_roll_modifier, resolve_quickened_save_gated_condition,
    resolve_quickened_save_gated_condition_immunity, QuickenedSpellGovernorState,
    QuickenedSpellInvalidKind, QuickenedSpellProtocol, QuickenedSpellScenarioOutcome,
};

pub const BRANCH_ACTIONS: [&str; 11] = [
    "doResolveQuickenedRestoration",
    "doResolveQuickenedSaveGatedCondition",
    "doResolveQuickenedSaveGatedConditionImmunity",
    "doResolveQuickenedDirectCondition",
    "doResolveQuickenedRollModifier",
    "doResolveQuickenedAfterMagicActionSpent",
    "doRejectUnaffordable",
    "doRejectUnknownOption",
    "doRejectUnsupportedSecondOption",
    "doRejectOnePerSpell",
    "doRejectPriorLevelOnePlusSpell",
];

pub fn replay_observed_action(observed_action_taken: &str) -> QuickenedSpellGovernorState {
    match observed_action_taken {
        "doResolveQuickenedRestoration" => resolve_quickened_restoration(),
        "doResolveQuickenedSaveGatedCondition" => resolve_quickened_save_gated_condition(),
        "doResolveQuickenedSaveGatedConditionImmunity" => {
            resolve_quickened_save_gated_condition_immunity()
        }
        "doResolveQuickenedDirectCondition" => resolve_quickened_direct_condition(),
        "doResolveQuickenedRollModifier" => resolve_quickened_roll_modifier(),
        "doResolveQuickenedAfterMagicActionSpent" => resolve_quickened_after_magic_action_spent(),
        "doRejectUnaffordable" => reject_quickened_unaffordable(),
        "doRejectUnknownOption" => reject_quickened_unknown_option(),
        "doRejectUnsupportedSecondOption" => reject_quickened_unsupported_second_option(),
        "doRejectOnePerSpell" => reject_quickened_one_per_spell(),
        "doRejectPriorLevelOnePlusSpell" => reject_quickened_prior_level_one_plus_spell(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> QuickenedSpellGovernorState {
    replay_observed_action(observed_action_taken)
}

pub fn projection_payload(state: &QuickenedSpellGovernorState) -> String {
    [
        format!(
            "qQuickenedCureWoundsOffered={}",
            state.quickened_cure_wounds_offered
        ),
        format!("qColorSprayBlinded={}", state.color_spray_blinded),
        format!("qCalmEmotionsImmunity={}", state.calm_emotions_immunity),
        format!("qInvisibilityActive={}", state.invisibility_active),
        format!("qBlessActive={}", state.bless_active),
        format!("qMagicActionAvailable={}", state.magic_action_available),
        format!("qBonusActionAvailable={}", state.bonus_action_available),
        format!("qSorceryPointsRemaining={}", state.sorcery_points_remaining),
        format!("qTargetHp={}", state.target_hit_points),
        format!("qSpellSlotCommitted={}", state.spell_slot_committed),
        format!(
            "qLevelOnePlusCastThisTurn={}",
            state.level_one_plus_cast_this_turn
        ),
        format!(
            "qQuickenedLevelOnePlusCastThisTurn={}",
            state.quickened_level_one_plus_cast_this_turn
        ),
        format!(
            "qSpellSlotActsAvailable={}",
            state.spell_slot_acts_available
        ),
        format!("qInvalidKind={}", invalid_kind_ref(state.invalid_kind)),
        format!(
            "qScenarioOutcome={}",
            scenario_outcome_ref(state.scenario_outcome)
        ),
        format!("protocolResult={}", protocol_ref(state.protocol)),
        "protocolHoles=none".to_string(),
    ]
    .join("\n")
}

fn invalid_kind_ref(invalid_kind: QuickenedSpellInvalidKind) -> &'static str {
    match invalid_kind {
        QuickenedSpellInvalidKind::None => "none",
        QuickenedSpellInvalidKind::Unaffordable => "unaffordable",
        QuickenedSpellInvalidKind::UnknownOption => "unknownOption",
        QuickenedSpellInvalidKind::UnsupportedSecondOption => "unsupportedSecondOption",
        QuickenedSpellInvalidKind::OnePerSpell => "onePerSpell",
        QuickenedSpellInvalidKind::SameTurnLevelOnePlus => "sameTurnLevelOnePlus",
    }
}

fn scenario_outcome_ref(outcome: QuickenedSpellScenarioOutcome) -> &'static str {
    match outcome {
        QuickenedSpellScenarioOutcome::Init => "Init",
        QuickenedSpellScenarioOutcome::ResolvedQuickenedRestoration => {
            "ResolvedQuickenedRestoration"
        }
        QuickenedSpellScenarioOutcome::ResolvedQuickenedSaveGatedCondition => {
            "ResolvedQuickenedSaveGatedCondition"
        }
        QuickenedSpellScenarioOutcome::ResolvedQuickenedSaveGatedConditionImmunity => {
            "ResolvedQuickenedSaveGatedConditionImmunity"
        }
        QuickenedSpellScenarioOutcome::ResolvedQuickenedDirectCondition => {
            "ResolvedQuickenedDirectCondition"
        }
        QuickenedSpellScenarioOutcome::ResolvedQuickenedRollModifier => {
            "ResolvedQuickenedRollModifier"
        }
        QuickenedSpellScenarioOutcome::ResolvedAfterMagicActionSpent => {
            "ResolvedAfterMagicActionSpent"
        }
        QuickenedSpellScenarioOutcome::RejectedUnaffordable => "RejectedUnaffordable",
        QuickenedSpellScenarioOutcome::RejectedUnknownOption => "RejectedUnknownOption",
        QuickenedSpellScenarioOutcome::RejectedUnsupportedSecondOption => {
            "RejectedUnsupportedSecondOption"
        }
        QuickenedSpellScenarioOutcome::RejectedOnePerSpell => "RejectedOnePerSpell",
        QuickenedSpellScenarioOutcome::RejectedPriorLevelOnePlusSpell => {
            "RejectedPriorLevelOnePlusSpell"
        }
    }
}

fn protocol_ref(protocol: QuickenedSpellProtocol) -> &'static str {
    match protocol {
        QuickenedSpellProtocol::Init => "init",
        QuickenedSpellProtocol::Resolved => "resolved",
    }
}
