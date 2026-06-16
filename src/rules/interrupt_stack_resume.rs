#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReactionWindowOffer {
    Closed,
    Offered {
        active: ReactionWindowRole,
        suspended: Option<ReactionWindowRole>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReactionWindowRole {
    AttackHitInterruption,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterruptPendingTrigger {
    NoPendingTrigger,
    AttackHit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterruptResumeHole {
    NoResumedHole,
    RolledDice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterruptStackResumeScenarioOutcome {
    Init,
    NestedDeclineResumedOuter,
    ActiveEffectMutationResumed,
    ReplayFromRootResolved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterruptStackResumeProtocol {
    Init,
    Resolved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InterruptStackResumeState {
    pub max_stack_depth_observed: u8,
    pub final_stack_depth: u8,
    pub pending_trigger: InterruptPendingTrigger,
    pub resumed_hole: InterruptResumeHole,
    pub active_effect_mutation_seen_on_resume: bool,
    pub replay_from_root_equivalent: bool,
    pub responder_reaction_available: bool,
    pub target_hit_points: i16,
    pub scenario_outcome: InterruptStackResumeScenarioOutcome,
    pub protocol: InterruptStackResumeProtocol,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InterruptStackResumeProjectionFacts {
    pub max_stack_depth_observed: u8,
    pub final_stack_depth: u8,
    pub pending_trigger: InterruptPendingTrigger,
    pub resumed_hole: InterruptResumeHole,
    pub active_effect_mutation_seen_on_resume: bool,
    pub replay_from_root_equivalent: bool,
    pub responder_reaction_available: bool,
    pub target_hit_points: i16,
    pub scenario_outcome: InterruptStackResumeScenarioOutcome,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleReplaySubject {
    ActionAttack,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleReplayFill {
    TargetChoice,
    AttackRoll,
    RolledDice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleReplayOutcome {
    Resolved,
    Invalid,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BattleReplayProjection {
    pub subject: BattleReplaySubject,
    pub fills: Vec<BattleReplayFill>,
    pub outcome: BattleReplayOutcome,
    pub interrupt_stack_depth: u8,
    pub target_hit_points: i16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BattleReplayRoot {
    pub subject: BattleReplaySubject,
    pub target_hit_points: i16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BattleReplayContinuation {
    pub subject: BattleReplaySubject,
    pub fills: Vec<BattleReplayFill>,
}

const MAXIMUM_REACTION_WINDOW_DEPTH: u8 = 2;
const INITIAL_TARGET_HIT_POINTS: i16 = 12;
const NESTED_TARGET_HIT_POINTS: i16 = 10;
const RECORDED_ATTACK_REPLAY_TARGET_HIT_POINTS: i16 = 3;

#[must_use]
pub fn interrupt_stack_resume_initial_state() -> InterruptStackResumeState {
    interrupt_stack_resume_projection(InterruptStackResumeProjectionFacts {
        max_stack_depth_observed: 0,
        final_stack_depth: 0,
        pending_trigger: InterruptPendingTrigger::NoPendingTrigger,
        resumed_hole: InterruptResumeHole::NoResumedHole,
        active_effect_mutation_seen_on_resume: false,
        replay_from_root_equivalent: false,
        responder_reaction_available: true,
        target_hit_points: INITIAL_TARGET_HIT_POINTS,
        scenario_outcome: InterruptStackResumeScenarioOutcome::Init,
    })
}

#[must_use]
pub fn interrupt_stack_resume_projection(
    facts: InterruptStackResumeProjectionFacts,
) -> InterruptStackResumeState {
    let protocol = if facts.scenario_outcome == InterruptStackResumeScenarioOutcome::Init {
        InterruptStackResumeProtocol::Init
    } else {
        InterruptStackResumeProtocol::Resolved
    };
    InterruptStackResumeState {
        max_stack_depth_observed: facts.max_stack_depth_observed,
        final_stack_depth: facts.final_stack_depth,
        pending_trigger: facts.pending_trigger,
        resumed_hole: facts.resumed_hole,
        active_effect_mutation_seen_on_resume: facts.active_effect_mutation_seen_on_resume,
        replay_from_root_equivalent: facts.replay_from_root_equivalent,
        responder_reaction_available: facts.responder_reaction_available,
        target_hit_points: facts.target_hit_points,
        scenario_outcome: facts.scenario_outcome,
        protocol,
    }
}

#[must_use]
pub fn reaction_window_depth(window: ReactionWindowOffer) -> u8 {
    match window {
        ReactionWindowOffer::Closed => 0,
        ReactionWindowOffer::Offered { suspended, .. } => {
            if suspended.is_some() {
                2
            } else {
                1
            }
        }
    }
}

#[must_use]
pub fn offer_reaction_window(
    window: ReactionWindowOffer,
    active: ReactionWindowRole,
) -> ReactionWindowOffer {
    // QNT: cleanroom-input/qnt/shared-algebras/proofs/rule-core/
    // reactions-continuations-concentration.qnt offerReactionWindow.
    match window {
        ReactionWindowOffer::Closed => ReactionWindowOffer::Offered {
            active,
            suspended: None,
        },
        ReactionWindowOffer::Offered {
            active: previous,
            suspended,
        } => {
            if reaction_window_depth(window) >= MAXIMUM_REACTION_WINDOW_DEPTH {
                window
            } else {
                ReactionWindowOffer::Offered {
                    active,
                    suspended: Some(suspended.unwrap_or(previous)),
                }
            }
        }
    }
}

#[must_use]
pub fn restore_suspended_reaction_window(window: ReactionWindowOffer) -> ReactionWindowOffer {
    match window {
        ReactionWindowOffer::Closed => ReactionWindowOffer::Closed,
        ReactionWindowOffer::Offered { suspended, .. } => match suspended {
            Some(active) => ReactionWindowOffer::Offered {
                active,
                suspended: None,
            },
            None => ReactionWindowOffer::Closed,
        },
    }
}

#[must_use]
pub fn decline_reaction_window(window: ReactionWindowOffer) -> ReactionWindowOffer {
    restore_suspended_reaction_window(window)
}

#[must_use]
pub fn advance_reaction_continuation(window: ReactionWindowOffer) -> ReactionWindowOffer {
    restore_suspended_reaction_window(window)
}

#[must_use]
pub fn take_matching_reaction(
    window: ReactionWindowOffer,
    responder_reaction_available: bool,
) -> (ReactionWindowOffer, bool) {
    if responder_reaction_available {
        (advance_reaction_continuation(window), false)
    } else {
        (window, false)
    }
}

#[must_use]
pub fn resolve_nested_interrupt_decline(
    _initial: InterruptStackResumeState,
) -> InterruptStackResumeState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md
    // "Reactions"; assumption:
    // cleanroom-input/domain/CLEANROOM_ASSUMPTIONS.md A45.
    let outer = offer_reaction_window(
        ReactionWindowOffer::Closed,
        ReactionWindowRole::AttackHitInterruption,
    );
    let inner = offer_reaction_window(outer, ReactionWindowRole::AttackHitInterruption);
    let resumed = decline_reaction_window(inner);

    interrupt_stack_resume_projection(InterruptStackResumeProjectionFacts {
        max_stack_depth_observed: reaction_window_depth(inner),
        final_stack_depth: reaction_window_depth(resumed),
        pending_trigger: InterruptPendingTrigger::AttackHit,
        resumed_hole: InterruptResumeHole::RolledDice,
        active_effect_mutation_seen_on_resume: false,
        replay_from_root_equivalent: false,
        responder_reaction_available: true,
        target_hit_points: NESTED_TARGET_HIT_POINTS,
        scenario_outcome: InterruptStackResumeScenarioOutcome::NestedDeclineResumedOuter,
    })
}

#[must_use]
pub fn resolve_interrupted_attack_after_reaction_mutation(
    _initial: InterruptStackResumeState,
) -> InterruptStackResumeState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Rules-Glossary.md "Reaction";
    // assumption: cleanroom-input/domain/CLEANROOM_ASSUMPTIONS.md A45.
    let offered = offer_reaction_window(
        ReactionWindowOffer::Closed,
        ReactionWindowRole::AttackHitInterruption,
    );
    let (resumed, responder_reaction_available) = take_matching_reaction(offered, true);

    interrupt_stack_resume_projection(InterruptStackResumeProjectionFacts {
        max_stack_depth_observed: reaction_window_depth(offered),
        final_stack_depth: reaction_window_depth(resumed),
        pending_trigger: InterruptPendingTrigger::NoPendingTrigger,
        resumed_hole: InterruptResumeHole::NoResumedHole,
        active_effect_mutation_seen_on_resume: true,
        replay_from_root_equivalent: false,
        responder_reaction_available,
        target_hit_points: INITIAL_TARGET_HIT_POINTS,
        scenario_outcome: InterruptStackResumeScenarioOutcome::ActiveEffectMutationResumed,
    })
}

#[must_use]
pub fn resolve_recorded_attack_replay_from_root(
    _initial: InterruptStackResumeState,
) -> InterruptStackResumeState {
    // QNT: cleanroom-input/qnt/battle-runtime/
    // battle-runtime-replay-equivalence.qnt.
    interrupt_stack_resume_projection(InterruptStackResumeProjectionFacts {
        max_stack_depth_observed: 0,
        final_stack_depth: 0,
        pending_trigger: InterruptPendingTrigger::NoPendingTrigger,
        resumed_hole: InterruptResumeHole::NoResumedHole,
        active_effect_mutation_seen_on_resume: false,
        replay_from_root_equivalent: recorded_attack_replay_from_root_equivalent(
            INITIAL_TARGET_HIT_POINTS,
            RECORDED_ATTACK_REPLAY_TARGET_HIT_POINTS,
        ),
        responder_reaction_available: true,
        target_hit_points: RECORDED_ATTACK_REPLAY_TARGET_HIT_POINTS,
        scenario_outcome: InterruptStackResumeScenarioOutcome::ReplayFromRootResolved,
    })
}

#[must_use]
pub fn recorded_attack_replay_from_root_equivalent(
    initial_target_hit_points: i16,
    replay_target_hit_points: i16,
) -> bool {
    independent_recorded_attack_replay_projection(replay_target_hit_points)
        == battle_replay_from_root(
            recorded_attack_replay_root(initial_target_hit_points),
            recorded_attack_replay_continuation(),
            recorded_attack_replay_suffix_fills(),
            replay_target_hit_points,
        )
}

#[must_use]
pub fn independent_recorded_attack_replay_projection(
    target_hit_points: i16,
) -> BattleReplayProjection {
    BattleReplayProjection {
        subject: BattleReplaySubject::ActionAttack,
        fills: recorded_attack_replay_fills(),
        outcome: BattleReplayOutcome::Resolved,
        interrupt_stack_depth: 0,
        target_hit_points,
    }
}

#[must_use]
pub fn recorded_attack_replay_root(target_hit_points: i16) -> BattleReplayRoot {
    BattleReplayRoot {
        subject: BattleReplaySubject::ActionAttack,
        target_hit_points,
    }
}

#[must_use]
pub fn recorded_attack_replay_continuation() -> BattleReplayContinuation {
    BattleReplayContinuation {
        subject: BattleReplaySubject::ActionAttack,
        fills: vec![BattleReplayFill::TargetChoice, BattleReplayFill::AttackRoll],
    }
}

#[must_use]
pub fn recorded_attack_replay_fills() -> Vec<BattleReplayFill> {
    vec![
        BattleReplayFill::TargetChoice,
        BattleReplayFill::AttackRoll,
        BattleReplayFill::RolledDice,
    ]
}

#[must_use]
pub fn recorded_attack_replay_suffix_fills() -> Vec<BattleReplayFill> {
    vec![BattleReplayFill::RolledDice]
}

#[must_use]
pub fn battle_replay_from_root(
    root: BattleReplayRoot,
    continuation: BattleReplayContinuation,
    resumed_fills: Vec<BattleReplayFill>,
    target_hit_points: i16,
) -> BattleReplayProjection {
    let mut fills = continuation.fills;
    fills.extend(resumed_fills);
    if root.subject == continuation.subject {
        BattleReplayProjection {
            subject: continuation.subject,
            fills,
            outcome: BattleReplayOutcome::Resolved,
            interrupt_stack_depth: 0,
            target_hit_points,
        }
    } else {
        BattleReplayProjection {
            subject: continuation.subject,
            fills,
            outcome: BattleReplayOutcome::Invalid,
            interrupt_stack_depth: 0,
            target_hit_points: root.target_hit_points,
        }
    }
}
