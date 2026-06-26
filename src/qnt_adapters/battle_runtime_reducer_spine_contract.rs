use crate::rules::battle_reducer_spine::{
    discover_battle_acts, discover_slot_spell_battle, end_turn_subject, resolve_battle_subject,
    resolve_battle_subject_test_fill, resolve_slot_spell_subject, slot_spell_holes_from_battle,
    start_fighter_skeleton_battle, Actor, AttackRollFacts, BattleFill, BattleHoleKind,
    BattleResolutionOutcome, BattleResolutionRequest, BattleSlotSpellFill, BattleSlotSpellHole,
    BattleState, BattleSubject, BattleSubjectKind, BattleTurnAdvanceResult, BattleTurnSpellSlotUse,
};

pub const BRANCH_ACTIONS: [&str; 9] = [
    "doStartBattle",
    "doDiscoverSlotSpell",
    "doResolveSlotSpellTargets",
    "doResolveSlotSpellDamage",
    "doEndTurnToTarget",
    "doDiscoverWeaponAttack",
    "doResolveWeaponTarget",
    "doResolveWeaponAttackHit",
    "doResolveWeaponDamage",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReducerSpineStage {
    NotStarted,
    BattleStarted,
    ActDiscovered,
    SubjectNeedsHoles,
    SubjectResolved,
    TurnAdvanced,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReducerSpineEntrypoint {
    None,
    StartBattle,
    DiscoverBattleActs,
    ResolveBattleSubject,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReducerSpineSubject {
    None,
    SlotSpell,
    WeaponAttack,
    EndTurn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReducerSpineActor {
    None,
    Caster,
    Target,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReducerSpineProtocolResult {
    Init,
    NeedsHoles,
    Resolved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReducerSpineHole {
    TargetChoice,
    SpellTargetAllocation,
    AttackRoll,
    RolledDice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReducerSpineSpellSlotUse {
    None,
    Pending,
    Committed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReducerSpineContractProjection {
    pub stage: ReducerSpineStage,
    pub entrypoint: ReducerSpineEntrypoint,
    pub subject: ReducerSpineSubject,
    pub current_actor: ReducerSpineActor,
    pub protocol_result: ReducerSpineProtocolResult,
    pub protocol_holes: Vec<ReducerSpineHole>,
    pub action_available: bool,
    pub bonus_action_available: bool,
    pub caster_reaction_available: bool,
    pub target_reaction_available: bool,
    pub spell_slot_use: ReducerSpineSpellSlotUse,
    pub interrupt_depth: u8,
    pub caster_hp: i16,
    pub target_hp: i16,
}

pub fn initial_witness_projection() -> ReducerSpineContractProjection {
    ReducerSpineContractProjection {
        stage: ReducerSpineStage::NotStarted,
        entrypoint: ReducerSpineEntrypoint::None,
        subject: ReducerSpineSubject::None,
        current_actor: ReducerSpineActor::None,
        protocol_result: ReducerSpineProtocolResult::Init,
        protocol_holes: vec![ReducerSpineHole::TargetChoice],
        action_available: false,
        bonus_action_available: false,
        caster_reaction_available: false,
        target_reaction_available: false,
        spell_slot_use: ReducerSpineSpellSlotUse::None,
        interrupt_depth: 0,
        caster_hp: 0,
        target_hp: 0,
    }
}

pub fn replay_observed_action(observed_action_taken: &str) -> ReducerSpineContractProjection {
    match observed_action_taken {
        "doStartBattle" => projection_from_battle(
            &start_fighter_skeleton_battle(),
            ReducerSpineStage::BattleStarted,
            ReducerSpineEntrypoint::StartBattle,
            ReducerSpineSubject::None,
            ReducerSpineProtocolResult::Resolved,
            Vec::new(),
        ),
        "doDiscoverSlotSpell" => {
            let state = slot_spell_discovered();
            projection_from_battle(
                &state,
                ReducerSpineStage::ActDiscovered,
                ReducerSpineEntrypoint::DiscoverBattleActs,
                ReducerSpineSubject::SlotSpell,
                ReducerSpineProtocolResult::NeedsHoles,
                slot_spell_holes_from_battle(&state)
                    .into_iter()
                    .map(slot_spell_hole)
                    .collect(),
            )
        }
        "doResolveSlotSpellTargets" => {
            let state = slot_spell_targets_resolved();
            projection_from_battle(
                &state,
                ReducerSpineStage::SubjectNeedsHoles,
                ReducerSpineEntrypoint::ResolveBattleSubject,
                ReducerSpineSubject::SlotSpell,
                ReducerSpineProtocolResult::NeedsHoles,
                slot_spell_holes_from_battle(&state)
                    .into_iter()
                    .map(slot_spell_hole)
                    .collect(),
            )
        }
        "doResolveSlotSpellDamage" => projection_from_battle(
            &slot_spell_damage_resolved(),
            ReducerSpineStage::SubjectResolved,
            ReducerSpineEntrypoint::ResolveBattleSubject,
            ReducerSpineSubject::SlotSpell,
            ReducerSpineProtocolResult::Resolved,
            Vec::new(),
        ),
        "doEndTurnToTarget" => {
            let result = end_turn_to_target();
            projection_from_turn_advance(
                &result,
                Actor::Fighter,
                Actor::Skeleton,
                1,
                ProjectionParts {
                    stage: ReducerSpineStage::TurnAdvanced,
                    entrypoint: ReducerSpineEntrypoint::ResolveBattleSubject,
                    subject: ReducerSpineSubject::EndTurn,
                    protocol_result: ReducerSpineProtocolResult::Resolved,
                    protocol_holes: Vec::new(),
                },
            )
        }
        "doDiscoverWeaponAttack" => {
            let state = end_turn_to_target_state();
            let act = discovered_skeleton_weapon_attack(&state);
            projection_from_battle(
                &state,
                ReducerSpineStage::ActDiscovered,
                ReducerSpineEntrypoint::DiscoverBattleActs,
                ReducerSpineSubject::WeaponAttack,
                ReducerSpineProtocolResult::NeedsHoles,
                weapon_holes(act.holes),
            )
        }
        "doResolveWeaponTarget" => {
            let (state, _subject, holes) = weapon_target_resolved();
            projection_from_battle(
                &state,
                ReducerSpineStage::SubjectNeedsHoles,
                ReducerSpineEntrypoint::ResolveBattleSubject,
                ReducerSpineSubject::WeaponAttack,
                ReducerSpineProtocolResult::NeedsHoles,
                weapon_holes(holes),
            )
        }
        "doResolveWeaponAttackHit" => {
            let (state, _subject, holes) = weapon_attack_hit_resolved();
            projection_from_battle(
                &state,
                ReducerSpineStage::SubjectNeedsHoles,
                ReducerSpineEntrypoint::ResolveBattleSubject,
                ReducerSpineSubject::WeaponAttack,
                ReducerSpineProtocolResult::NeedsHoles,
                weapon_holes(holes),
            )
        }
        "doResolveWeaponDamage" => projection_from_battle(
            &weapon_damage_resolved(),
            ReducerSpineStage::SubjectResolved,
            ReducerSpineEntrypoint::ResolveBattleSubject,
            ReducerSpineSubject::WeaponAttack,
            ReducerSpineProtocolResult::Resolved,
            Vec::new(),
        ),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> ReducerSpineContractProjection {
    match observed_action_taken {
        "doStartBattle" => expected(ExpectedProjection {
            stage: ReducerSpineStage::BattleStarted,
            entrypoint: ReducerSpineEntrypoint::StartBattle,
            subject: ReducerSpineSubject::None,
            current_actor: ReducerSpineActor::Caster,
            protocol_result: ReducerSpineProtocolResult::Resolved,
            protocol_holes: Vec::new(),
            action_available: true,
            bonus_action_available: true,
            spell_slot_use: ReducerSpineSpellSlotUse::None,
            caster_hp: 12,
            target_hp: 13,
        }),
        "doDiscoverSlotSpell" => expected(ExpectedProjection {
            stage: ReducerSpineStage::ActDiscovered,
            entrypoint: ReducerSpineEntrypoint::DiscoverBattleActs,
            subject: ReducerSpineSubject::SlotSpell,
            current_actor: ReducerSpineActor::Caster,
            protocol_result: ReducerSpineProtocolResult::NeedsHoles,
            protocol_holes: vec![ReducerSpineHole::SpellTargetAllocation],
            action_available: true,
            bonus_action_available: true,
            spell_slot_use: ReducerSpineSpellSlotUse::None,
            caster_hp: 12,
            target_hp: 13,
        }),
        "doResolveSlotSpellTargets" => expected(ExpectedProjection {
            stage: ReducerSpineStage::SubjectNeedsHoles,
            entrypoint: ReducerSpineEntrypoint::ResolveBattleSubject,
            subject: ReducerSpineSubject::SlotSpell,
            current_actor: ReducerSpineActor::Caster,
            protocol_result: ReducerSpineProtocolResult::NeedsHoles,
            protocol_holes: vec![ReducerSpineHole::RolledDice],
            action_available: true,
            bonus_action_available: true,
            spell_slot_use: ReducerSpineSpellSlotUse::None,
            caster_hp: 12,
            target_hp: 13,
        }),
        "doResolveSlotSpellDamage" => expected(ExpectedProjection {
            stage: ReducerSpineStage::SubjectResolved,
            entrypoint: ReducerSpineEntrypoint::ResolveBattleSubject,
            subject: ReducerSpineSubject::SlotSpell,
            current_actor: ReducerSpineActor::Caster,
            protocol_result: ReducerSpineProtocolResult::Resolved,
            protocol_holes: Vec::new(),
            action_available: false,
            bonus_action_available: true,
            spell_slot_use: ReducerSpineSpellSlotUse::Committed,
            caster_hp: 12,
            target_hp: 7,
        }),
        "doEndTurnToTarget" => expected(ExpectedProjection {
            stage: ReducerSpineStage::TurnAdvanced,
            entrypoint: ReducerSpineEntrypoint::ResolveBattleSubject,
            subject: ReducerSpineSubject::EndTurn,
            current_actor: ReducerSpineActor::Target,
            protocol_result: ReducerSpineProtocolResult::Resolved,
            protocol_holes: Vec::new(),
            action_available: true,
            bonus_action_available: true,
            spell_slot_use: ReducerSpineSpellSlotUse::None,
            caster_hp: 12,
            target_hp: 7,
        }),
        "doDiscoverWeaponAttack" => expected(ExpectedProjection {
            stage: ReducerSpineStage::ActDiscovered,
            entrypoint: ReducerSpineEntrypoint::DiscoverBattleActs,
            subject: ReducerSpineSubject::WeaponAttack,
            current_actor: ReducerSpineActor::Target,
            protocol_result: ReducerSpineProtocolResult::NeedsHoles,
            protocol_holes: vec![ReducerSpineHole::TargetChoice],
            action_available: true,
            bonus_action_available: true,
            spell_slot_use: ReducerSpineSpellSlotUse::None,
            caster_hp: 12,
            target_hp: 7,
        }),
        "doResolveWeaponTarget" => expected(ExpectedProjection {
            stage: ReducerSpineStage::SubjectNeedsHoles,
            entrypoint: ReducerSpineEntrypoint::ResolveBattleSubject,
            subject: ReducerSpineSubject::WeaponAttack,
            current_actor: ReducerSpineActor::Target,
            protocol_result: ReducerSpineProtocolResult::NeedsHoles,
            protocol_holes: vec![ReducerSpineHole::AttackRoll],
            action_available: true,
            bonus_action_available: true,
            spell_slot_use: ReducerSpineSpellSlotUse::None,
            caster_hp: 12,
            target_hp: 7,
        }),
        "doResolveWeaponAttackHit" => expected(ExpectedProjection {
            stage: ReducerSpineStage::SubjectNeedsHoles,
            entrypoint: ReducerSpineEntrypoint::ResolveBattleSubject,
            subject: ReducerSpineSubject::WeaponAttack,
            current_actor: ReducerSpineActor::Target,
            protocol_result: ReducerSpineProtocolResult::NeedsHoles,
            protocol_holes: vec![ReducerSpineHole::RolledDice],
            action_available: true,
            bonus_action_available: true,
            spell_slot_use: ReducerSpineSpellSlotUse::None,
            caster_hp: 12,
            target_hp: 7,
        }),
        "doResolveWeaponDamage" => expected(ExpectedProjection {
            stage: ReducerSpineStage::SubjectResolved,
            entrypoint: ReducerSpineEntrypoint::ResolveBattleSubject,
            subject: ReducerSpineSubject::WeaponAttack,
            current_actor: ReducerSpineActor::Target,
            protocol_result: ReducerSpineProtocolResult::Resolved,
            protocol_holes: Vec::new(),
            action_available: false,
            bonus_action_available: true,
            spell_slot_use: ReducerSpineSpellSlotUse::None,
            caster_hp: 6,
            target_hp: 7,
        }),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn projection_payload(state: &ReducerSpineContractProjection) -> String {
    [
        format!("qStage={}", stage_ref(state.stage)),
        format!("qEntrypoint={}", entrypoint_ref(state.entrypoint)),
        format!("qSubject={}", subject_ref(state.subject)),
        format!("qCurrentActor={}", actor_ref(state.current_actor)),
        format!(
            "protocolResult={}",
            protocol_result_ref(state.protocol_result)
        ),
        format!("protocolHoles={}", joined_holes(&state.protocol_holes)),
        format!("qActionAvailable={}", state.action_available),
        format!("qBonusActionAvailable={}", state.bonus_action_available),
        format!(
            "qCasterReactionAvailable={}",
            state.caster_reaction_available
        ),
        format!(
            "qTargetReactionAvailable={}",
            state.target_reaction_available
        ),
        format!("qSpellSlotUse={}", spell_slot_use_ref(state.spell_slot_use)),
        format!("qInterruptDepth={}", state.interrupt_depth),
        format!("qCasterHp={}", state.caster_hp),
        format!("qTargetHp={}", state.target_hp),
    ]
    .join("\n")
}

struct ExpectedProjection {
    stage: ReducerSpineStage,
    entrypoint: ReducerSpineEntrypoint,
    subject: ReducerSpineSubject,
    current_actor: ReducerSpineActor,
    protocol_result: ReducerSpineProtocolResult,
    protocol_holes: Vec<ReducerSpineHole>,
    action_available: bool,
    bonus_action_available: bool,
    spell_slot_use: ReducerSpineSpellSlotUse,
    caster_hp: i16,
    target_hp: i16,
}

struct ProjectionParts {
    stage: ReducerSpineStage,
    entrypoint: ReducerSpineEntrypoint,
    subject: ReducerSpineSubject,
    protocol_result: ReducerSpineProtocolResult,
    protocol_holes: Vec<ReducerSpineHole>,
}

fn expected(parts: ExpectedProjection) -> ReducerSpineContractProjection {
    ReducerSpineContractProjection {
        stage: parts.stage,
        entrypoint: parts.entrypoint,
        subject: parts.subject,
        current_actor: parts.current_actor,
        protocol_result: parts.protocol_result,
        protocol_holes: parts.protocol_holes,
        action_available: parts.action_available,
        bonus_action_available: parts.bonus_action_available,
        caster_reaction_available: true,
        target_reaction_available: true,
        spell_slot_use: parts.spell_slot_use,
        interrupt_depth: 0,
        caster_hp: parts.caster_hp,
        target_hp: parts.target_hp,
    }
}

fn projection_from_battle(
    state: &BattleState,
    stage: ReducerSpineStage,
    entrypoint: ReducerSpineEntrypoint,
    subject: ReducerSpineSubject,
    protocol_result: ReducerSpineProtocolResult,
    protocol_holes: Vec<ReducerSpineHole>,
) -> ReducerSpineContractProjection {
    ReducerSpineContractProjection {
        stage,
        entrypoint,
        subject,
        current_actor: actor_from_battle(state.initiative.still_to_act.actor),
        protocol_result,
        protocol_holes,
        action_available: state.action_available,
        bonus_action_available: state.bonus_action_available,
        caster_reaction_available: state.fighter.reaction_available,
        target_reaction_available: state.skeleton.reaction_available,
        spell_slot_use: spell_slot_use_from_battle(state),
        interrupt_depth: 0,
        caster_hp: state.fighter.hp,
        target_hp: state.skeleton.hp,
    }
}

fn projection_from_turn_advance(
    result: &BattleTurnAdvanceResult,
    expected_previous_actor: Actor,
    expected_next_actor: Actor,
    expected_round: i16,
    parts: ProjectionParts,
) -> ReducerSpineContractProjection {
    assert_eq!(result.previous_actor, expected_previous_actor);
    assert_eq!(result.next_actor, expected_next_actor);
    assert_eq!(result.round, expected_round);

    ReducerSpineContractProjection {
        stage: parts.stage,
        entrypoint: parts.entrypoint,
        subject: parts.subject,
        current_actor: actor_from_battle(result.next_actor),
        protocol_result: parts.protocol_result,
        protocol_holes: parts.protocol_holes,
        action_available: result.state.action_available,
        bonus_action_available: result.state.bonus_action_available,
        caster_reaction_available: result.state.fighter.reaction_available,
        target_reaction_available: result.state.skeleton.reaction_available,
        spell_slot_use: spell_slot_use_from_battle(&result.state),
        interrupt_depth: 0,
        caster_hp: result.state.fighter.hp,
        target_hp: result.state.skeleton.hp,
    }
}

fn slot_spell_discovered() -> BattleState {
    discover_slot_spell_battle(start_fighter_skeleton_battle())
}

fn slot_spell_targets_resolved() -> BattleState {
    resolve_slot_spell_subject(
        slot_spell_discovered(),
        BattleSlotSpellFill::TargetAllocation(Actor::Skeleton),
    )
}

fn slot_spell_damage_resolved() -> BattleState {
    resolve_slot_spell_subject(
        slot_spell_targets_resolved(),
        BattleSlotSpellFill::DamageRoll(3),
    )
}

fn end_turn_to_target() -> BattleTurnAdvanceResult {
    let state = slot_spell_damage_resolved();
    let request = BattleResolutionRequest::end_turn(end_turn_subject(&state))
        .expect("end turn subject should accept the no-fill end-turn request");
    let result = resolve_battle_subject(state, request);
    let outcome = result.outcome();
    result
        .into_turn_advance()
        .unwrap_or_else(|| panic!("end turn should advance the turn, got {outcome:?}"))
}

fn end_turn_to_target_state() -> BattleState {
    end_turn_to_target().state
}

fn discovered_skeleton_weapon_attack(
    state: &BattleState,
) -> crate::rules::battle_reducer_spine::AvailableBattleAct {
    discover_battle_acts(state)
        .into_available_acts()
        .into_iter()
        .find(|act| {
            act.subject.kind == BattleSubjectKind::WeaponAttack
                && act.subject.actor == Actor::Skeleton
        })
        .expect("QNT reducer-spine contract should discover one Skeleton weapon attack")
}

fn weapon_target_resolved() -> (BattleState, BattleSubject, Vec<BattleHoleKind>) {
    let state = end_turn_to_target_state();
    let act = discovered_skeleton_weapon_attack(&state);
    let result = resolve_battle_subject_test_fill(
        state,
        act.subject,
        BattleFill::TargetChoice(Actor::Fighter),
    );
    let outcome = result.outcome();
    let needs_holes = result.into_needs_holes().unwrap_or_else(|| {
        panic!("weapon target choice should need attack-roll holes, got {outcome:?}")
    });
    (needs_holes.state, needs_holes.subject, needs_holes.holes)
}

fn weapon_attack_hit_resolved() -> (BattleState, BattleSubject, Vec<BattleHoleKind>) {
    let (state, subject, _holes) = weapon_target_resolved();
    let result = resolve_battle_subject_test_fill(
        state,
        subject,
        BattleFill::AttackRoll(AttackRollFacts {
            total: 18,
            natural_d20: 14,
        }),
    );
    let outcome = result.outcome();
    let needs_holes = result
        .into_needs_holes()
        .unwrap_or_else(|| panic!("weapon attack hit should need damage holes, got {outcome:?}"));
    (needs_holes.state, needs_holes.subject, needs_holes.holes)
}

fn weapon_damage_resolved() -> BattleState {
    let (state, subject, _holes) = weapon_attack_hit_resolved();
    let result = resolve_battle_subject_test_fill(state, subject, BattleFill::DamageRoll(3));
    let outcome = result.outcome();
    if outcome != BattleResolutionOutcome::Resolved {
        panic!("weapon damage should resolve, got {outcome:?}");
    }
    result.into_state()
}

fn actor_from_battle(actor: Actor) -> ReducerSpineActor {
    match actor {
        Actor::Fighter => ReducerSpineActor::Caster,
        Actor::Skeleton => ReducerSpineActor::Target,
        Actor::Goblin | Actor::Rogue => ReducerSpineActor::None,
    }
}

fn spell_slot_use_from_battle(state: &BattleState) -> ReducerSpineSpellSlotUse {
    if state
        .spell_slot_uses_this_turn
        .contains(&BattleTurnSpellSlotUse::Committed(Actor::Fighter))
    {
        ReducerSpineSpellSlotUse::Committed
    } else if state
        .spell_slot_uses_this_turn
        .contains(&BattleTurnSpellSlotUse::Pending(Actor::Fighter))
    {
        ReducerSpineSpellSlotUse::Pending
    } else {
        ReducerSpineSpellSlotUse::None
    }
}

fn slot_spell_hole(hole: BattleSlotSpellHole) -> ReducerSpineHole {
    match hole {
        BattleSlotSpellHole::SpellTargetAllocation => ReducerSpineHole::SpellTargetAllocation,
        BattleSlotSpellHole::RolledDice => ReducerSpineHole::RolledDice,
    }
}

fn weapon_holes(holes: Vec<BattleHoleKind>) -> Vec<ReducerSpineHole> {
    holes
        .into_iter()
        .map(|hole| match hole {
            BattleHoleKind::TargetChoice => ReducerSpineHole::TargetChoice,
            BattleHoleKind::AttackRoll => ReducerSpineHole::AttackRoll,
            BattleHoleKind::RolledDice => ReducerSpineHole::RolledDice,
            BattleHoleKind::SpellTargetAllocation
            | BattleHoleKind::SpellTargetList
            | BattleHoleKind::DamageTypeChoice
            | BattleHoleKind::ConditionChoice
            | BattleHoleKind::SavingThrowOutcome
            | BattleHoleKind::HitPointHealingDistribution
            | BattleHoleKind::DeathSavingThrow
            | BattleHoleKind::ConcentrationSavingThrow
            | BattleHoleKind::StatBlockRechargeRoll => {
                panic!("weapon projection received non-weapon reducer hole {hole:?}")
            }
        })
        .collect()
}

fn stage_ref(stage: ReducerSpineStage) -> &'static str {
    match stage {
        ReducerSpineStage::NotStarted => "ReducerNotStarted",
        ReducerSpineStage::BattleStarted => "ReducerBattleStarted",
        ReducerSpineStage::ActDiscovered => "ReducerActDiscovered",
        ReducerSpineStage::SubjectNeedsHoles => "ReducerSubjectNeedsHoles",
        ReducerSpineStage::SubjectResolved => "ReducerSubjectResolved",
        ReducerSpineStage::TurnAdvanced => "ReducerTurnAdvanced",
    }
}

fn entrypoint_ref(entrypoint: ReducerSpineEntrypoint) -> &'static str {
    match entrypoint {
        ReducerSpineEntrypoint::None => "NoReducerEntrypoint",
        ReducerSpineEntrypoint::StartBattle => "StartBattleEntrypoint",
        ReducerSpineEntrypoint::DiscoverBattleActs => "DiscoverBattleActsEntrypoint",
        ReducerSpineEntrypoint::ResolveBattleSubject => "ResolveBattleSubjectEntrypoint",
    }
}

fn subject_ref(subject: ReducerSpineSubject) -> &'static str {
    match subject {
        ReducerSpineSubject::None => "NoReducerSubject",
        ReducerSpineSubject::SlotSpell => "SlotSpellSubject",
        ReducerSpineSubject::WeaponAttack => "WeaponAttackSubject",
        ReducerSpineSubject::EndTurn => "EndTurnSubject",
    }
}

fn actor_ref(actor: ReducerSpineActor) -> &'static str {
    match actor {
        ReducerSpineActor::None => "NoSpineActor",
        ReducerSpineActor::Caster => "CasterActor",
        ReducerSpineActor::Target => "TargetActor",
    }
}

fn protocol_result_ref(result: ReducerSpineProtocolResult) -> &'static str {
    match result {
        ReducerSpineProtocolResult::Init => "init",
        ReducerSpineProtocolResult::NeedsHoles => "needsHoles",
        ReducerSpineProtocolResult::Resolved => "resolved",
    }
}

fn spell_slot_use_ref(use_state: ReducerSpineSpellSlotUse) -> &'static str {
    match use_state {
        ReducerSpineSpellSlotUse::None => "NoSpellSlotUse",
        ReducerSpineSpellSlotUse::Pending => "PendingSpellSlotUse",
        ReducerSpineSpellSlotUse::Committed => "CommittedSpellSlotUse",
    }
}

fn joined_holes(holes: &[ReducerSpineHole]) -> String {
    if holes.is_empty() {
        return "none".to_string();
    }
    holes.iter().map(hole_ref).collect::<Vec<_>>().join(",")
}

fn hole_ref(hole: &ReducerSpineHole) -> &'static str {
    match hole {
        ReducerSpineHole::TargetChoice => "TargetChoiceHoleKind",
        ReducerSpineHole::SpellTargetAllocation => "SpellTargetAllocationHoleKind",
        ReducerSpineHole::AttackRoll => "AttackRollHoleKind",
        ReducerSpineHole::RolledDice => "RolledDiceHoleKind",
    }
}
