use crate::rules::feature_resources::apply_temporary_hit_points;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BattleTurnFeatureState {
    pub actor_temporary_hit_points: i16,
    pub bonus_action_available: bool,
    pub dash_bonus_feet: i16,
    pub feature_uses_remaining: i16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdrenalineRushFacts {
    pub turn: BattleTurnFeatureState,
    pub speed_feet: i16,
    pub proficiency_bonus: i16,
    pub actor_owns_turn: bool,
    pub actor_unconscious: bool,
    pub actor_incapacitated: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdrenalineRushRejection {
    WrongActor,
    StaleSubject,
    NoUsesRemaining,
    InvalidFill,
    UnableToAct,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdrenalineRushResult {
    Resolved,
    Invalid(AdrenalineRushRejection),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdrenalineRushProjection {
    pub turn: BattleTurnFeatureState,
    pub result: AdrenalineRushResult,
}

#[must_use]
pub fn resolve_adrenaline_rush_bonus_action_dash(
    facts: AdrenalineRushFacts,
) -> AdrenalineRushProjection {
    // RAW: cleanroom-input/raw/srd-5.2.1/Character-Origins.md "Orc",
    // cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md "Dash",
    // "Bonus Actions", and "Temporary Hit Points"; QNT:
    // cleanroom-input/qnt/battle-runtime/battle-runtime-adrenaline-rush.mbt.qnt
    // and battle-runtime-movement.qnt bonusActionDashTemporaryHitPointsFighter.
    if !facts.actor_owns_turn {
        return rejected(facts.turn, AdrenalineRushRejection::WrongActor);
    }
    if !facts.turn.bonus_action_available {
        return rejected(facts.turn, AdrenalineRushRejection::StaleSubject);
    }
    if facts.turn.feature_uses_remaining <= 0 {
        return rejected(facts.turn, AdrenalineRushRejection::NoUsesRemaining);
    }
    if !(2..=6).contains(&facts.proficiency_bonus) || facts.speed_feet < 0 {
        return rejected(facts.turn, AdrenalineRushRejection::InvalidFill);
    }
    if facts.actor_unconscious || facts.actor_incapacitated {
        return rejected(facts.turn, AdrenalineRushRejection::UnableToAct);
    }

    AdrenalineRushProjection {
        turn: BattleTurnFeatureState {
            actor_temporary_hit_points: apply_temporary_hit_points(
                facts.turn.actor_temporary_hit_points,
                facts.proficiency_bonus,
            ),
            bonus_action_available: false,
            dash_bonus_feet: facts.turn.dash_bonus_feet + facts.speed_feet,
            feature_uses_remaining: facts.turn.feature_uses_remaining - 1,
        },
        result: AdrenalineRushResult::Resolved,
    }
}

fn rejected(
    turn: BattleTurnFeatureState,
    rejection: AdrenalineRushRejection,
) -> AdrenalineRushProjection {
    AdrenalineRushProjection {
        turn,
        result: AdrenalineRushResult::Invalid(rejection),
    }
}
