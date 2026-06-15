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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SavingThrowAbility {
    Strength,
    Dexterity,
    Constitution,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PassiveSavingThrowRollMode {
    Normal,
    Advantage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DangerSenseScenarioOutcome {
    Init,
    DexterityAdvantage,
    IncapacitatedSuppressed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DangerSenseProtocol {
    Init,
    Resolved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DangerSenseState {
    pub scenario_outcome: DangerSenseScenarioOutcome,
    pub feature_present: bool,
    pub dexterity_roll_mode_count: i16,
    pub constitution_roll_mode_count: i16,
    pub suppressed: bool,
    pub accepted: bool,
    pub protocol: DangerSenseProtocol,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DangerSenseFacts {
    pub saving_throw_ability: SavingThrowAbility,
    pub actor_incapacitated: bool,
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

#[must_use]
pub fn danger_sense_initial_state() -> DangerSenseState {
    DangerSenseState {
        scenario_outcome: DangerSenseScenarioOutcome::Init,
        feature_present: false,
        dexterity_roll_mode_count: 0,
        constitution_roll_mode_count: 0,
        suppressed: false,
        accepted: false,
        protocol: DangerSenseProtocol::Init,
    }
}

#[must_use]
pub fn danger_sense_saving_throw_roll_mode(facts: DangerSenseFacts) -> PassiveSavingThrowRollMode {
    // RAW: cleanroom-input/raw/srd-5.2.1/Classes/Barbarian.md
    // "Level 2: Danger Sense"; QNT:
    // unit-feature-save-damage-core-examples.qnt passiveSavingThrowRollMode.
    if facts.saving_throw_ability == SavingThrowAbility::Dexterity && !facts.actor_incapacitated {
        PassiveSavingThrowRollMode::Advantage
    } else {
        PassiveSavingThrowRollMode::Normal
    }
}

#[must_use]
pub fn project_danger_sense_dexterity_advantage(_state: DangerSenseState) -> DangerSenseState {
    let mode = danger_sense_saving_throw_roll_mode(DangerSenseFacts {
        saving_throw_ability: SavingThrowAbility::Dexterity,
        actor_incapacitated: false,
    });
    DangerSenseState {
        scenario_outcome: DangerSenseScenarioOutcome::DexterityAdvantage,
        feature_present: true,
        dexterity_roll_mode_count: i16::from(mode == PassiveSavingThrowRollMode::Advantage),
        constitution_roll_mode_count: 0,
        suppressed: false,
        accepted: true,
        protocol: DangerSenseProtocol::Resolved,
    }
}

#[must_use]
pub fn suppress_danger_sense_while_incapacitated(_state: DangerSenseState) -> DangerSenseState {
    let mode = danger_sense_saving_throw_roll_mode(DangerSenseFacts {
        saving_throw_ability: SavingThrowAbility::Dexterity,
        actor_incapacitated: true,
    });
    DangerSenseState {
        scenario_outcome: DangerSenseScenarioOutcome::IncapacitatedSuppressed,
        feature_present: true,
        dexterity_roll_mode_count: i16::from(mode == PassiveSavingThrowRollMode::Advantage),
        constitution_roll_mode_count: 0,
        suppressed: true,
        accepted: true,
        protocol: DangerSenseProtocol::Resolved,
    }
}
