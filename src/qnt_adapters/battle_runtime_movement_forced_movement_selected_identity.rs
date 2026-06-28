use super::battle_runtime_reducer_route::{
    route_discover_battle_acts_from_route_holes, route_resolve_battle_subject_from_route_holes,
    route_resolve_battle_subject_without_fill_from_route_holes, route_start_battle,
    ReducerRouteEvent, ReducerRouteFillKind, ReducerRouteHoleKind, ReducerRouteOwnerGroup,
    ReducerRouteResolutionOutcome, ReducerRouteSubjectFamily,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MovementForcedMovementSelectedIdentityWitness {
    pub caster_speed_feet: i16,
    pub caster_remaining_feet: i16,
    pub caster_dash_bonus_feet: i16,
    pub caster_bonus_action_available: bool,
    pub caster_concentrating: bool,
    pub spell_slot_spent_this_turn: bool,
    pub level_1_slots_remaining: i16,
    pub spell_dash_bonus_action_effect_count: i16,
    pub target_hp: i16,
    pub target_reaction_available: bool,
    pub dissonant_movement_fill_required: bool,
    pub target_movement_spent_feet: i16,
    pub command_movement_fill_required: bool,
    pub command_pending_effect_observed: bool,
    pub command_pending_effect_count: i16,
    pub climb_speed_feet: i16,
    pub swim_speed_feet: i16,
    pub scenario_outcome: &'static str,
    pub protocol_result: &'static str,
    pub protocol_holes: Vec<&'static str>,
}

pub const BRANCH_ACTIONS: [&str; 4] = [
    "doDissonantWhispersForcedReactionMovement",
    "doCommandFleeTargetTurn",
    "doExpeditiousRetreatImmediateDash",
    "doMonkUnarmoredMovementDash",
];

pub const OUT_OF_SCOPE_BRANCH_ACTIONS: [&str; 2] = [
    "doBarbarianFastMovementDash",
    "doRangerRovingClimbSwimMovement",
];

pub fn replay_observed_action(
    observed_action_taken: &str,
) -> MovementForcedMovementSelectedIdentityWitness {
    match observed_action_taken {
        "doDissonantWhispersForcedReactionMovement" => dissonant_whispers_witness(),
        "doCommandFleeTargetTurn" => command_flee_witness(),
        "doExpeditiousRetreatImmediateDash" => expeditious_retreat_witness(),
        "doMonkUnarmoredMovementDash" => monk_unarmored_movement_witness(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(
    observed_action_taken: &str,
) -> MovementForcedMovementSelectedIdentityWitness {
    match observed_action_taken {
        "doDissonantWhispersForcedReactionMovement" => {
            MovementForcedMovementSelectedIdentityWitness {
                spell_slot_spent_this_turn: true,
                level_1_slots_remaining: 1,
                target_hp: 18,
                target_reaction_available: false,
                dissonant_movement_fill_required: true,
                scenario_outcome: "DissonantWhispers",
                ..resolved_from_initial()
            }
        }
        "doCommandFleeTargetTurn" => MovementForcedMovementSelectedIdentityWitness {
            level_1_slots_remaining: 1,
            target_movement_spent_feet: 30,
            command_movement_fill_required: true,
            command_pending_effect_observed: true,
            scenario_outcome: "CommandFlee",
            ..resolved_from_initial()
        },
        "doExpeditiousRetreatImmediateDash" => MovementForcedMovementSelectedIdentityWitness {
            caster_remaining_feet: 60,
            caster_dash_bonus_feet: 30,
            caster_bonus_action_available: false,
            caster_concentrating: true,
            spell_slot_spent_this_turn: true,
            level_1_slots_remaining: 1,
            spell_dash_bonus_action_effect_count: 1,
            scenario_outcome: "ExpeditiousRetreat",
            ..resolved_from_initial()
        },
        "doMonkUnarmoredMovementDash" => MovementForcedMovementSelectedIdentityWitness {
            caster_speed_feet: 40,
            caster_remaining_feet: 80,
            caster_dash_bonus_feet: 40,
            level_1_slots_remaining: 0,
            scenario_outcome: "MonkUnarmoredMovement",
            ..resolved_from_initial()
        },
        action => panic!("unsupported expected mbt::actionTaken {action}"),
    }
}

pub fn projection_payload(witness: &MovementForcedMovementSelectedIdentityWitness) -> String {
    [
        format!("qCasterSpeedFeet={}", witness.caster_speed_feet),
        format!("qCasterRemainingFeet={}", witness.caster_remaining_feet),
        format!("qCasterDashBonusFeet={}", witness.caster_dash_bonus_feet),
        format!(
            "qCasterBonusActionAvailable={}",
            witness.caster_bonus_action_available
        ),
        format!("qCasterConcentrating={}", witness.caster_concentrating),
        format!(
            "qSpellSlotSpentThisTurn={}",
            witness.spell_slot_spent_this_turn
        ),
        format!("qLevel1SlotsRemaining={}", witness.level_1_slots_remaining),
        format!(
            "qSpellDashBonusActionEffectCount={}",
            witness.spell_dash_bonus_action_effect_count
        ),
        format!("qTargetHp={}", witness.target_hp),
        format!(
            "qTargetReactionAvailable={}",
            witness.target_reaction_available
        ),
        format!(
            "qDissonantMovementFillRequired={}",
            witness.dissonant_movement_fill_required
        ),
        format!(
            "qTargetMovementSpentFeet={}",
            witness.target_movement_spent_feet
        ),
        format!(
            "qCommandMovementFillRequired={}",
            witness.command_movement_fill_required
        ),
        format!(
            "qCommandPendingEffectObserved={}",
            witness.command_pending_effect_observed
        ),
        format!(
            "qCommandPendingEffectCount={}",
            witness.command_pending_effect_count
        ),
        format!("qClimbSpeedFeet={}", witness.climb_speed_feet),
        format!("qSwimSpeedFeet={}", witness.swim_speed_feet),
        format!("qScenarioOutcome={}", witness.scenario_outcome),
        format!("protocolResult={}", witness.protocol_result),
        format!("protocolHoles={}", joined_or_none(&witness.protocol_holes)),
    ]
    .join("\n")
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doDissonantWhispersForcedReactionMovement" => {
            route_after_dissonant_whispers_forced_reaction_movement()
        }
        "doCommandFleeTargetTurn" => route_after_command_flee_target_turn(),
        "doExpeditiousRetreatImmediateDash" => route_after_expeditious_retreat_dash(),
        "doMonkUnarmoredMovementDash" => route_after_passive_speed_dash(),
        action => panic!("unsupported route mbt::actionTaken {action}"),
    }
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doDissonantWhispersForcedReactionMovement" => {
            expected_route_after_dissonant_whispers_forced_reaction_movement()
        }
        "doCommandFleeTargetTurn" => expected_route_after_command_flee_target_turn(),
        "doExpeditiousRetreatImmediateDash" => expected_route_after_expeditious_retreat_dash(),
        "doMonkUnarmoredMovementDash" => expected_route_after_passive_speed_dash(),
        action => panic!("unsupported expected route mbt::actionTaken {action}"),
    }
}

fn initial_witness() -> MovementForcedMovementSelectedIdentityWitness {
    MovementForcedMovementSelectedIdentityWitness {
        caster_speed_feet: 30,
        caster_remaining_feet: 30,
        caster_dash_bonus_feet: 0,
        caster_bonus_action_available: true,
        caster_concentrating: false,
        spell_slot_spent_this_turn: false,
        level_1_slots_remaining: 2,
        spell_dash_bonus_action_effect_count: 0,
        target_hp: 12,
        target_reaction_available: true,
        dissonant_movement_fill_required: false,
        target_movement_spent_feet: 0,
        command_movement_fill_required: false,
        command_pending_effect_observed: false,
        command_pending_effect_count: 0,
        climb_speed_feet: 0,
        swim_speed_feet: 0,
        scenario_outcome: "Init",
        protocol_result: "init",
        protocol_holes: vec!["WitnessProtocolHole"],
    }
}

fn resolved_from_initial() -> MovementForcedMovementSelectedIdentityWitness {
    MovementForcedMovementSelectedIdentityWitness {
        protocol_result: "resolved",
        protocol_holes: vec![],
        ..initial_witness()
    }
}

fn dissonant_whispers_witness() -> MovementForcedMovementSelectedIdentityWitness {
    MovementForcedMovementSelectedIdentityWitness {
        spell_slot_spent_this_turn: true,
        level_1_slots_remaining: 1,
        target_hp: 18,
        target_reaction_available: false,
        dissonant_movement_fill_required: true,
        scenario_outcome: "DissonantWhispers",
        ..resolved_from_initial()
    }
}

fn command_flee_witness() -> MovementForcedMovementSelectedIdentityWitness {
    MovementForcedMovementSelectedIdentityWitness {
        level_1_slots_remaining: 1,
        target_movement_spent_feet: 30,
        command_movement_fill_required: true,
        command_pending_effect_observed: true,
        scenario_outcome: "CommandFlee",
        ..resolved_from_initial()
    }
}

fn expeditious_retreat_witness() -> MovementForcedMovementSelectedIdentityWitness {
    MovementForcedMovementSelectedIdentityWitness {
        caster_remaining_feet: 60,
        caster_dash_bonus_feet: 30,
        caster_bonus_action_available: false,
        caster_concentrating: true,
        spell_slot_spent_this_turn: true,
        level_1_slots_remaining: 1,
        spell_dash_bonus_action_effect_count: 1,
        scenario_outcome: "ExpeditiousRetreat",
        ..resolved_from_initial()
    }
}

fn monk_unarmored_movement_witness() -> MovementForcedMovementSelectedIdentityWitness {
    MovementForcedMovementSelectedIdentityWitness {
        caster_speed_feet: 40,
        caster_remaining_feet: 80,
        caster_dash_bonus_feet: 40,
        level_1_slots_remaining: 0,
        scenario_outcome: "MonkUnarmoredMovement",
        ..resolved_from_initial()
    }
}

fn route_after_dissonant_whispers_forced_reaction_movement() -> Vec<ReducerRouteEvent> {
    append_forced_movement_route(
        initial_route(),
        vec![
            ReducerRouteHoleKind::TargetChoice,
            ReducerRouteHoleKind::SavingThrowOutcome,
            ReducerRouteHoleKind::RolledDice,
            ReducerRouteHoleKind::Movement,
        ],
    )
}

fn route_after_command_flee_target_turn() -> Vec<ReducerRouteEvent> {
    let mut route = append_forced_movement_route(
        initial_route(),
        vec![
            ReducerRouteHoleKind::SpellTargetList,
            ReducerRouteHoleKind::CommandOptionChoice,
            ReducerRouteHoleKind::SavingThrowOutcome,
            ReducerRouteHoleKind::Movement,
        ],
    );
    route.push(resolve_without_fill(
        ReducerRouteSubjectFamily::ForcedMovement,
        ReducerRouteOwnerGroup::ActiveEffect,
    ));
    route.push(resolve_without_fill(
        ReducerRouteSubjectFamily::ForcedMovement,
        ReducerRouteOwnerGroup::TurnBoundary,
    ));
    route
}

fn route_after_expeditious_retreat_dash() -> Vec<ReducerRouteEvent> {
    let mut route = initial_route();
    route.push(discover(
        ReducerRouteSubjectFamily::MovementResource,
        Vec::new(),
        ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
    ));
    route.push(resolve_without_fill(
        ReducerRouteSubjectFamily::MovementResource,
        ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
    ));
    route.push(resolve_without_fill(
        ReducerRouteSubjectFamily::MovementResource,
        ReducerRouteOwnerGroup::ActiveEffect,
    ));
    route.push(resolve_without_fill(
        ReducerRouteSubjectFamily::MovementResource,
        ReducerRouteOwnerGroup::MovementResource,
    ));
    route
}

fn route_after_passive_speed_dash() -> Vec<ReducerRouteEvent> {
    let mut route = initial_route();
    route.push(discover(
        ReducerRouteSubjectFamily::MovementResource,
        Vec::new(),
        ReducerRouteOwnerGroup::ActionEconomy,
    ));
    route.push(resolve_without_fill(
        ReducerRouteSubjectFamily::MovementResource,
        ReducerRouteOwnerGroup::ActionEconomy,
    ));
    route.push(resolve_without_fill(
        ReducerRouteSubjectFamily::MovementResource,
        ReducerRouteOwnerGroup::CreatureState,
    ));
    route.push(resolve_without_fill(
        ReducerRouteSubjectFamily::MovementResource,
        ReducerRouteOwnerGroup::MovementResource,
    ));
    route
}

fn append_forced_movement_route(
    mut route: Vec<ReducerRouteEvent>,
    holes: Vec<ReducerRouteHoleKind>,
) -> Vec<ReducerRouteEvent> {
    route.push(discover(
        ReducerRouteSubjectFamily::ForcedMovement,
        holes,
        ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
    ));
    route.push(resolve_with_movement_fill(
        ReducerRouteSubjectFamily::ForcedMovement,
        ReducerRouteOwnerGroup::MovementResource,
    ));
    route.push(resolve_without_fill(
        ReducerRouteSubjectFamily::ForcedMovement,
        ReducerRouteOwnerGroup::ActionEconomy,
    ));
    route.push(resolve_without_fill(
        ReducerRouteSubjectFamily::ForcedMovement,
        ReducerRouteOwnerGroup::InterruptStack,
    ));
    route
}

fn initial_route() -> Vec<ReducerRouteEvent> {
    vec![route_start_battle(ReducerRouteOwnerGroup::ActionEconomy)]
}

fn discover(
    subject: ReducerRouteSubjectFamily,
    holes: Vec<ReducerRouteHoleKind>,
    owner: ReducerRouteOwnerGroup,
) -> ReducerRouteEvent {
    route_discover_battle_acts_from_route_holes(subject, holes, owner)
}

fn resolve_with_movement_fill(
    subject: ReducerRouteSubjectFamily,
    owner: ReducerRouteOwnerGroup,
) -> ReducerRouteEvent {
    route_resolve_battle_subject_from_route_holes(
        subject,
        ReducerRouteFillKind::Movement,
        Vec::new(),
        owner,
    )
}

fn resolve_without_fill(
    subject: ReducerRouteSubjectFamily,
    owner: ReducerRouteOwnerGroup,
) -> ReducerRouteEvent {
    route_resolve_battle_subject_without_fill_from_route_holes(subject, Vec::new(), owner)
}

fn expected_route_after_dissonant_whispers_forced_reaction_movement() -> Vec<ReducerRouteEvent> {
    append_expected_forced_movement_route(
        expected_initial_route(),
        vec![
            ReducerRouteHoleKind::Movement,
            ReducerRouteHoleKind::RolledDice,
            ReducerRouteHoleKind::SavingThrowOutcome,
            ReducerRouteHoleKind::TargetChoice,
        ],
    )
}

fn expected_route_after_command_flee_target_turn() -> Vec<ReducerRouteEvent> {
    let mut route = append_expected_forced_movement_route(
        expected_initial_route(),
        vec![
            ReducerRouteHoleKind::CommandOptionChoice,
            ReducerRouteHoleKind::Movement,
            ReducerRouteHoleKind::SavingThrowOutcome,
            ReducerRouteHoleKind::SpellTargetList,
        ],
    );
    route.push(expected_resolve_without_fill(
        ReducerRouteSubjectFamily::ForcedMovement,
        ReducerRouteOwnerGroup::ActiveEffect,
    ));
    route.push(expected_resolve_without_fill(
        ReducerRouteSubjectFamily::ForcedMovement,
        ReducerRouteOwnerGroup::TurnBoundary,
    ));
    route
}

fn expected_route_after_expeditious_retreat_dash() -> Vec<ReducerRouteEvent> {
    let mut route = expected_initial_route();
    route.push(expected_discover(
        ReducerRouteSubjectFamily::MovementResource,
        Vec::new(),
        ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
    ));
    route.push(expected_resolve_without_fill(
        ReducerRouteSubjectFamily::MovementResource,
        ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
    ));
    route.push(expected_resolve_without_fill(
        ReducerRouteSubjectFamily::MovementResource,
        ReducerRouteOwnerGroup::ActiveEffect,
    ));
    route.push(expected_resolve_without_fill(
        ReducerRouteSubjectFamily::MovementResource,
        ReducerRouteOwnerGroup::MovementResource,
    ));
    route
}

fn expected_route_after_passive_speed_dash() -> Vec<ReducerRouteEvent> {
    let mut route = expected_initial_route();
    route.push(expected_discover(
        ReducerRouteSubjectFamily::MovementResource,
        Vec::new(),
        ReducerRouteOwnerGroup::ActionEconomy,
    ));
    route.push(expected_resolve_without_fill(
        ReducerRouteSubjectFamily::MovementResource,
        ReducerRouteOwnerGroup::ActionEconomy,
    ));
    route.push(expected_resolve_without_fill(
        ReducerRouteSubjectFamily::MovementResource,
        ReducerRouteOwnerGroup::CreatureState,
    ));
    route.push(expected_resolve_without_fill(
        ReducerRouteSubjectFamily::MovementResource,
        ReducerRouteOwnerGroup::MovementResource,
    ));
    route
}

fn append_expected_forced_movement_route(
    mut route: Vec<ReducerRouteEvent>,
    holes: Vec<ReducerRouteHoleKind>,
) -> Vec<ReducerRouteEvent> {
    route.push(expected_discover(
        ReducerRouteSubjectFamily::ForcedMovement,
        holes,
        ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
    ));
    route.push(expected_resolve_with_movement_fill(
        ReducerRouteSubjectFamily::ForcedMovement,
        ReducerRouteOwnerGroup::MovementResource,
    ));
    route.push(expected_resolve_without_fill(
        ReducerRouteSubjectFamily::ForcedMovement,
        ReducerRouteOwnerGroup::ActionEconomy,
    ));
    route.push(expected_resolve_without_fill(
        ReducerRouteSubjectFamily::ForcedMovement,
        ReducerRouteOwnerGroup::InterruptStack,
    ));
    route
}

fn expected_initial_route() -> Vec<ReducerRouteEvent> {
    vec![ReducerRouteEvent::StartBattle {
        owner: ReducerRouteOwnerGroup::ActionEconomy,
    }]
}

fn expected_discover(
    subject: ReducerRouteSubjectFamily,
    holes: Vec<ReducerRouteHoleKind>,
    owner: ReducerRouteOwnerGroup,
) -> ReducerRouteEvent {
    ReducerRouteEvent::DiscoverBattleActs {
        subject,
        holes,
        owner,
    }
}

fn expected_resolve_with_movement_fill(
    subject: ReducerRouteSubjectFamily,
    owner: ReducerRouteOwnerGroup,
) -> ReducerRouteEvent {
    ReducerRouteEvent::ResolveBattleSubject {
        subject,
        fill: super::battle_runtime_reducer_route::ReducerRouteFillEvidence::FillKind(
            ReducerRouteFillKind::Movement,
        ),
        outcome: ReducerRouteResolutionOutcome::Resolved,
        holes: Vec::new(),
        owner,
    }
}

fn expected_resolve_without_fill(
    subject: ReducerRouteSubjectFamily,
    owner: ReducerRouteOwnerGroup,
) -> ReducerRouteEvent {
    ReducerRouteEvent::ResolveBattleSubjectWithoutFill {
        subject,
        outcome: ReducerRouteResolutionOutcome::Resolved,
        holes: Vec::new(),
        owner,
    }
}

fn joined_or_none(values: &[&'static str]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}
