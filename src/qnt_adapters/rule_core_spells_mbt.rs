use crate::rules::level_one_armor_spells::MAGE_ARMOR_BASE_ARMOR_CLASS;
use crate::rules::rule_core_components::{
    rule_core_component_route, rule_core_component_route_event_ref, RuleCoreComponentOwner,
    RuleCoreComponentRouteEvent,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpellProcedureHole {
    AttackRoll,
    DamageRoll,
    SavingThrowOutcome,
    SpellTargetAllocation,
    SpellTargetList,
    TargetChoice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpellProcedureProtocolResult {
    Init,
    NeedsHoles,
    Resolved,
    InvalidStaleSubject,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SpellInvocationAction {
    Action,
    BonusAction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ActiveEffectKind {
    None,
    SpeedDelta,
    SpellBaseArmorClass,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpellProcedureState {
    pub action_available: bool,
    pub bonus_action_available: bool,
    pub caster_reaction_available: bool,
    pub caster_hp: i16,
    pub target_hp: i16,
    pub second_target_hp: i16,
    pub target_unconscious: bool,
    pub target_death_successes: i16,
    pub target_death_failures: i16,
    pub spell_slot_spent_this_turn: bool,
    pub level_one_slots_remaining: i16,
    pub active_effect_kind: &'static str,
    pub readied_held: bool,
    pub readied_released: bool,
    pub concentration_active: bool,
    pub protocol_result: SpellProcedureProtocolResult,
    pub protocol_holes: Vec<SpellProcedureHole>,
    pub component_route: Vec<RuleCoreComponentRouteEvent>,
}

pub const ACCEPTED_BRANCH_ACTIONS: [&str; 24] = [
    "doMagicMissileLow",
    "doMagicMissileNeedsAllocation",
    "doRayOfFrostMiss",
    "doRayOfFrostHit",
    "doRayOfFrostCritical",
    "doRayOfFrostNeedsTarget",
    "doRayOfFrostNeedsAttackRoll",
    "doRayOfFrostNeedsDamageRoll",
    "doAcidSplashAllSuccess",
    "doAcidSplashOneFail",
    "doAcidSplashNeedsSavingThrow",
    "doAcidSplashNeedsDamageRoll",
    "doHealingWordWounded",
    "doHealingWordZeroHp",
    "doHealingWordNeedsTarget",
    "doHealingWordNeedsHealingRoll",
    "doCureWoundsWounded",
    "doCureWoundsNeedsTarget",
    "doCureWoundsNeedsHealingRoll",
    "doMageArmor",
    "doMageArmorNeedsTarget",
    "doRejectSecondSlotSpell",
    "doReadySpellHold",
    "doReleaseReadiedSpell",
];

pub const OUT_OF_SCOPE_MASS_SPELL_BRANCH_ACTIONS: [&str; 6] = [
    "doMassHealingWordWounded",
    "doMassHealingWordNeedsTargetList",
    "doMassHealingWordNeedsHealingRoll",
    "doMassCureWoundsWounded",
    "doMassCureWoundsNeedsTargetList",
    "doMassCureWoundsNeedsHealingRoll",
];

pub const BRANCH_ACTIONS: [&str; 30] = [
    "doMagicMissileLow",
    "doMagicMissileNeedsAllocation",
    "doRayOfFrostMiss",
    "doRayOfFrostHit",
    "doRayOfFrostCritical",
    "doRayOfFrostNeedsTarget",
    "doRayOfFrostNeedsAttackRoll",
    "doRayOfFrostNeedsDamageRoll",
    "doAcidSplashAllSuccess",
    "doAcidSplashOneFail",
    "doAcidSplashNeedsSavingThrow",
    "doAcidSplashNeedsDamageRoll",
    "doHealingWordWounded",
    "doHealingWordZeroHp",
    "doHealingWordNeedsTarget",
    "doHealingWordNeedsHealingRoll",
    "doCureWoundsWounded",
    "doCureWoundsNeedsTarget",
    "doCureWoundsNeedsHealingRoll",
    "doMassHealingWordWounded",
    "doMassHealingWordNeedsTargetList",
    "doMassHealingWordNeedsHealingRoll",
    "doMassCureWoundsWounded",
    "doMassCureWoundsNeedsTargetList",
    "doMassCureWoundsNeedsHealingRoll",
    "doMageArmor",
    "doMageArmorNeedsTarget",
    "doRejectSecondSlotSpell",
    "doReadySpellHold",
    "doReleaseReadiedSpell",
];

const INITIAL_CASTER_HP: i16 = 12;
const INITIAL_TARGET_HP: i16 = 13;
const INITIAL_LEVEL_ONE_SLOTS_REMAINING: i16 = 2;

pub fn replay_observed_action(observed_action_taken: &str) -> SpellProcedureState {
    match observed_action_taken {
        "doMagicMissileNeedsAllocation" => {
            needs_holes(vec![SpellProcedureHole::SpellTargetAllocation])
        }
        "doMagicMissileLow" => with_invocation(
            SpellInvocationAction::Action,
            1,
            INITIAL_LEVEL_ONE_SLOTS_REMAINING,
            |state| {
                state.target_hp -= magic_missile_damage(3, 3);
            },
        ),
        "doRayOfFrostNeedsTarget" => needs_holes(vec![SpellProcedureHole::TargetChoice]),
        "doRayOfFrostNeedsAttackRoll" => needs_holes(vec![SpellProcedureHole::AttackRoll]),
        "doRayOfFrostNeedsDamageRoll" => needs_holes(vec![SpellProcedureHole::DamageRoll]),
        "doRayOfFrostMiss" => with_invocation(
            SpellInvocationAction::Action,
            0,
            INITIAL_LEVEL_ONE_SLOTS_REMAINING,
            |_| {},
        ),
        "doRayOfFrostHit" => with_invocation(
            SpellInvocationAction::Action,
            0,
            INITIAL_LEVEL_ONE_SLOTS_REMAINING,
            |state| apply_ray_of_frost_hit(state, 10, 14, 13, 1, 1, 4),
        ),
        "doRayOfFrostCritical" => with_invocation(
            SpellInvocationAction::Action,
            0,
            INITIAL_LEVEL_ONE_SLOTS_REMAINING,
            |state| apply_ray_of_frost_hit(state, 20, 20, 13, 1, 2, 8),
        ),
        "doAcidSplashNeedsSavingThrow" => needs_holes(vec![SpellProcedureHole::SavingThrowOutcome]),
        "doAcidSplashNeedsDamageRoll" => needs_holes(vec![SpellProcedureHole::DamageRoll]),
        "doAcidSplashAllSuccess" => with_invocation(
            SpellInvocationAction::Action,
            0,
            INITIAL_LEVEL_ONE_SLOTS_REMAINING,
            |_| {},
        ),
        "doAcidSplashOneFail" => with_invocation(
            SpellInvocationAction::Action,
            0,
            INITIAL_LEVEL_ONE_SLOTS_REMAINING,
            |state| {
                state.target_hp -= acid_splash_damage(true, 4);
            },
        ),
        "doHealingWordNeedsTarget" => wounded_needs_holes(
            SpellInvocationAction::BonusAction,
            vec![SpellProcedureHole::TargetChoice],
        ),
        "doHealingWordNeedsHealingRoll" => wounded_needs_holes(
            SpellInvocationAction::BonusAction,
            vec![SpellProcedureHole::DamageRoll],
        ),
        "doHealingWordWounded" => healing_word_projection(4, 12),
        "doHealingWordZeroHp" => healing_word_projection(0, 5),
        "doCureWoundsNeedsTarget" => wounded_needs_holes(
            SpellInvocationAction::Action,
            vec![SpellProcedureHole::TargetChoice],
        ),
        "doCureWoundsNeedsHealingRoll" => wounded_needs_holes(
            SpellInvocationAction::Action,
            vec![SpellProcedureHole::DamageRoll],
        ),
        "doCureWoundsWounded" => cure_wounds_projection(12),
        "doMassHealingWordNeedsTargetList" => {
            mass_healing_needs_holes(vec![SpellProcedureHole::SpellTargetList])
        }
        "doMassHealingWordNeedsHealingRoll" => {
            mass_healing_needs_holes(vec![SpellProcedureHole::DamageRoll])
        }
        "doMassHealingWordWounded" => mass_healing_word_projection(),
        "doMassCureWoundsNeedsTargetList" => {
            mass_healing_needs_holes(vec![SpellProcedureHole::SpellTargetList])
        }
        "doMassCureWoundsNeedsHealingRoll" => {
            mass_healing_needs_holes(vec![SpellProcedureHole::DamageRoll])
        }
        "doMassCureWoundsWounded" => mass_cure_wounds_projection(),
        "doMageArmorNeedsTarget" => needs_holes(vec![SpellProcedureHole::TargetChoice]),
        "doMageArmor" => with_invocation(
            SpellInvocationAction::Action,
            1,
            INITIAL_LEVEL_ONE_SLOTS_REMAINING,
            |state| {
                let active = mage_armor_effect_kind(MAGE_ARMOR_BASE_ARMOR_CLASS);
                state.active_effect_kind = active_effect_ref(active);
            },
        ),
        "doRejectSecondSlotSpell" => {
            let mut state = wounded_state(4);
            state.action_available = false;
            state.spell_slot_spent_this_turn = true;
            state.level_one_slots_remaining = 1;
            state.protocol_result = SpellProcedureProtocolResult::InvalidStaleSubject;
            state
        }
        "doReadySpellHold" => ready_spell_hold_projection(),
        "doReleaseReadiedSpell" => release_readied_spell_projection(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> SpellProcedureState {
    match observed_action_taken {
        "doMagicMissileNeedsAllocation" => literal_spell_state(
            true,
            true,
            true,
            12,
            13,
            13,
            false,
            0,
            0,
            false,
            2,
            "none",
            false,
            false,
            false,
            SpellProcedureProtocolResult::NeedsHoles,
            vec![SpellProcedureHole::SpellTargetAllocation],
        ),
        "doMagicMissileLow" => resolved_literal(false, true, true, 12, 4, 13, true, 1, "none"),
        "doRayOfFrostNeedsTarget" => target_choice_literal(),
        "doRayOfFrostNeedsAttackRoll" => needs_holes_literal(vec![SpellProcedureHole::AttackRoll]),
        "doRayOfFrostNeedsDamageRoll" => needs_holes_literal(vec![SpellProcedureHole::DamageRoll]),
        "doRayOfFrostMiss" => resolved_literal(false, true, true, 12, 13, 13, false, 2, "none"),
        "doRayOfFrostHit" => resolved_literal(false, true, true, 12, 9, 13, false, 2, "speedDelta"),
        "doRayOfFrostCritical" => {
            resolved_literal(false, true, true, 12, 5, 13, false, 2, "speedDelta")
        }
        "doAcidSplashNeedsSavingThrow" => {
            needs_holes_literal(vec![SpellProcedureHole::SavingThrowOutcome])
        }
        "doAcidSplashNeedsDamageRoll" => needs_holes_literal(vec![SpellProcedureHole::DamageRoll]),
        "doAcidSplashAllSuccess" => {
            resolved_literal(false, true, true, 12, 13, 13, false, 2, "none")
        }
        "doAcidSplashOneFail" => resolved_literal(false, true, true, 12, 9, 13, false, 2, "none"),
        "doHealingWordNeedsTarget" => {
            wounded_needs_holes_literal(vec![SpellProcedureHole::TargetChoice])
        }
        "doHealingWordNeedsHealingRoll" => {
            wounded_needs_holes_literal(vec![SpellProcedureHole::DamageRoll])
        }
        "doHealingWordWounded" => resolved_literal(true, false, true, 12, 12, 13, true, 1, "none"),
        "doHealingWordZeroHp" => resolved_literal(true, false, true, 12, 5, 13, true, 1, "none"),
        "doCureWoundsNeedsTarget" => {
            wounded_needs_holes_literal(vec![SpellProcedureHole::TargetChoice])
        }
        "doCureWoundsNeedsHealingRoll" => {
            wounded_needs_holes_literal(vec![SpellProcedureHole::DamageRoll])
        }
        "doCureWoundsWounded" => resolved_literal(false, true, true, 12, 12, 13, true, 1, "none"),
        "doMassHealingWordNeedsTargetList" => {
            mass_healing_needs_holes_literal(vec![SpellProcedureHole::SpellTargetList])
        }
        "doMassHealingWordNeedsHealingRoll" => {
            mass_healing_needs_holes_literal(vec![SpellProcedureHole::DamageRoll])
        }
        "doMassHealingWordWounded" => {
            resolved_literal(true, false, true, 12, 12, 12, true, 0, "none")
        }
        "doMassCureWoundsNeedsTargetList" => {
            mass_healing_needs_holes_literal(vec![SpellProcedureHole::SpellTargetList])
        }
        "doMassCureWoundsNeedsHealingRoll" => {
            mass_healing_needs_holes_literal(vec![SpellProcedureHole::DamageRoll])
        }
        "doMassCureWoundsWounded" => {
            resolved_literal(false, true, true, 12, 12, 12, true, 0, "none")
        }
        "doMageArmorNeedsTarget" => target_choice_literal(),
        "doMageArmor" => resolved_literal(
            false,
            true,
            true,
            12,
            13,
            13,
            true,
            1,
            "spellBaseArmorClass",
        ),
        "doRejectSecondSlotSpell" => literal_spell_state(
            false,
            true,
            true,
            12,
            4,
            13,
            false,
            0,
            0,
            true,
            1,
            "none",
            false,
            false,
            false,
            SpellProcedureProtocolResult::InvalidStaleSubject,
            Vec::new(),
        ),
        "doReadySpellHold" => literal_spell_state(
            false,
            true,
            true,
            12,
            13,
            13,
            false,
            0,
            0,
            true,
            1,
            "none",
            true,
            false,
            true,
            SpellProcedureProtocolResult::Resolved,
            Vec::new(),
        ),
        "doReleaseReadiedSpell" => literal_spell_state(
            false,
            true,
            false,
            11,
            4,
            13,
            false,
            0,
            0,
            false,
            1,
            "none",
            false,
            true,
            false,
            SpellProcedureProtocolResult::Resolved,
            Vec::new(),
        ),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_component_route() -> Vec<RuleCoreComponentRouteEvent> {
    component_route()
}

pub fn projection_payload(state: &SpellProcedureState) -> String {
    [
        format!("qActionAvailable={}", state.action_available),
        format!("qBonusActionAvailable={}", state.bonus_action_available),
        format!(
            "qCasterReactionAvailable={}",
            state.caster_reaction_available
        ),
        format!("qCasterHp={}", state.caster_hp),
        format!("qTargetHp={}", state.target_hp),
        format!("qSecondTargetHp={}", state.second_target_hp),
        format!("qTargetUnconscious={}", state.target_unconscious),
        format!("qTargetDeathSuccesses={}", state.target_death_successes),
        format!("qTargetDeathFailures={}", state.target_death_failures),
        format!(
            "qSpellSlotSpentThisTurn={}",
            state.spell_slot_spent_this_turn
        ),
        format!("qLevel1SlotsRemaining={}", state.level_one_slots_remaining),
        format!("qActiveEffectKind={}", state.active_effect_kind),
        format!("qReadiedHeld={}", state.readied_held),
        format!("qReadiedReleased={}", state.readied_released),
        format!("qConcentrationActive={}", state.concentration_active),
        format!(
            "protocolResult={}",
            protocol_result_ref(state.protocol_result)
        ),
        format!(
            "protocolInvalidReason={}",
            protocol_invalid_reason_ref(state.protocol_result)
        ),
        format!("protocolHoles={}", joined_holes(&state.protocol_holes)),
        format!(
            "qComponentRoute={}",
            component_route_payload(&state.component_route)
        ),
    ]
    .join("\n")
}

pub fn component_route_payload(route: &[RuleCoreComponentRouteEvent]) -> String {
    route
        .iter()
        .map(rule_core_component_route_event_ref)
        .collect::<Vec<_>>()
        .join(">")
}

fn target_choice_literal() -> SpellProcedureState {
    needs_holes_literal(vec![SpellProcedureHole::TargetChoice])
}

fn needs_holes_literal(holes: Vec<SpellProcedureHole>) -> SpellProcedureState {
    literal_spell_state(
        true,
        true,
        true,
        12,
        13,
        13,
        false,
        0,
        0,
        false,
        2,
        "none",
        false,
        false,
        false,
        SpellProcedureProtocolResult::NeedsHoles,
        holes,
    )
}

fn wounded_needs_holes_literal(holes: Vec<SpellProcedureHole>) -> SpellProcedureState {
    literal_spell_state(
        true,
        true,
        true,
        12,
        4,
        13,
        false,
        0,
        0,
        false,
        2,
        "none",
        false,
        false,
        false,
        SpellProcedureProtocolResult::NeedsHoles,
        holes,
    )
}

fn mass_healing_needs_holes_literal(holes: Vec<SpellProcedureHole>) -> SpellProcedureState {
    literal_spell_state(
        true,
        true,
        true,
        12,
        4,
        4,
        false,
        0,
        0,
        false,
        0,
        "none",
        false,
        false,
        false,
        SpellProcedureProtocolResult::NeedsHoles,
        holes,
    )
}

#[allow(clippy::too_many_arguments)]
fn resolved_literal(
    action_available: bool,
    bonus_action_available: bool,
    caster_reaction_available: bool,
    caster_hp: i16,
    target_hp: i16,
    second_target_hp: i16,
    spell_slot_spent_this_turn: bool,
    level_one_slots_remaining: i16,
    active_effect_kind: &'static str,
) -> SpellProcedureState {
    literal_spell_state(
        action_available,
        bonus_action_available,
        caster_reaction_available,
        caster_hp,
        target_hp,
        second_target_hp,
        false,
        0,
        0,
        spell_slot_spent_this_turn,
        level_one_slots_remaining,
        active_effect_kind,
        false,
        false,
        false,
        SpellProcedureProtocolResult::Resolved,
        Vec::new(),
    )
}

#[allow(clippy::too_many_arguments)]
fn literal_spell_state(
    action_available: bool,
    bonus_action_available: bool,
    caster_reaction_available: bool,
    caster_hp: i16,
    target_hp: i16,
    second_target_hp: i16,
    target_unconscious: bool,
    target_death_successes: i16,
    target_death_failures: i16,
    spell_slot_spent_this_turn: bool,
    level_one_slots_remaining: i16,
    active_effect_kind: &'static str,
    readied_held: bool,
    readied_released: bool,
    concentration_active: bool,
    protocol_result: SpellProcedureProtocolResult,
    protocol_holes: Vec<SpellProcedureHole>,
) -> SpellProcedureState {
    SpellProcedureState {
        action_available,
        bonus_action_available,
        caster_reaction_available,
        caster_hp,
        target_hp,
        second_target_hp,
        target_unconscious,
        target_death_successes,
        target_death_failures,
        spell_slot_spent_this_turn,
        level_one_slots_remaining,
        active_effect_kind,
        readied_held,
        readied_released,
        concentration_active,
        protocol_result,
        protocol_holes,
        component_route: component_route(),
    }
}

fn with_invocation(
    action: SpellInvocationAction,
    slot_level: i16,
    slots_remaining: i16,
    apply: impl FnOnce(&mut SpellProcedureState),
) -> SpellProcedureState {
    let mut state = initial_state();
    spend_spell_invocation(&mut state, action, slot_level, slots_remaining);
    state.protocol_result = SpellProcedureProtocolResult::Resolved;
    apply(&mut state);
    state
}

fn spend_spell_invocation(
    state: &mut SpellProcedureState,
    action: SpellInvocationAction,
    slot_level: i16,
    slots_remaining: i16,
) {
    match action {
        SpellInvocationAction::Action => state.action_available = false,
        SpellInvocationAction::BonusAction => state.bonus_action_available = false,
    }
    if slot_level > 0 {
        state.spell_slot_spent_this_turn = true;
        state.level_one_slots_remaining = (slots_remaining - 1).max(0);
    } else {
        state.level_one_slots_remaining = slots_remaining;
    }
}

fn needs_holes(holes: Vec<SpellProcedureHole>) -> SpellProcedureState {
    let mut state = initial_state();
    state.protocol_result = SpellProcedureProtocolResult::NeedsHoles;
    state.protocol_holes = holes;
    state
}

fn wounded_needs_holes(
    _action: SpellInvocationAction,
    holes: Vec<SpellProcedureHole>,
) -> SpellProcedureState {
    let mut state = wounded_state(4);
    state.protocol_result = SpellProcedureProtocolResult::NeedsHoles;
    state.protocol_holes = holes;
    state
}

fn mass_healing_needs_holes(holes: Vec<SpellProcedureHole>) -> SpellProcedureState {
    let mut state = initial_state();
    state.target_hp = 4;
    state.second_target_hp = 4;
    state.level_one_slots_remaining = 0;
    state.protocol_result = SpellProcedureProtocolResult::NeedsHoles;
    state.protocol_holes = holes;
    state
}

fn healing_word_projection(starting_target_hp: i16, healed_target_hp: i16) -> SpellProcedureState {
    let mut state = wounded_state(starting_target_hp);
    spend_spell_invocation(
        &mut state,
        SpellInvocationAction::BonusAction,
        1,
        INITIAL_LEVEL_ONE_SLOTS_REMAINING,
    );
    state.target_hp = healed_target_hp;
    state.protocol_result = SpellProcedureProtocolResult::Resolved;
    state
}

fn cure_wounds_projection(healed_target_hp: i16) -> SpellProcedureState {
    let mut state = wounded_state(4);
    spend_spell_invocation(
        &mut state,
        SpellInvocationAction::Action,
        1,
        INITIAL_LEVEL_ONE_SLOTS_REMAINING,
    );
    state.target_hp = healed_target_hp;
    state.protocol_result = SpellProcedureProtocolResult::Resolved;
    state
}

fn mass_healing_word_projection() -> SpellProcedureState {
    let mut state = mass_healing_wounded_state();
    spend_spell_invocation(&mut state, SpellInvocationAction::BonusAction, 3, 1);
    state.target_hp = 12;
    state.second_target_hp = 12;
    state.protocol_result = SpellProcedureProtocolResult::Resolved;
    state
}

fn mass_cure_wounds_projection() -> SpellProcedureState {
    let mut state = mass_healing_wounded_state();
    spend_spell_invocation(&mut state, SpellInvocationAction::Action, 5, 1);
    state.target_hp = 12;
    state.second_target_hp = 12;
    state.protocol_result = SpellProcedureProtocolResult::Resolved;
    state
}

fn ready_spell_hold_projection() -> SpellProcedureState {
    let mut state = with_invocation(
        SpellInvocationAction::Action,
        1,
        INITIAL_LEVEL_ONE_SLOTS_REMAINING,
        |_| {},
    );
    state.readied_held = true;
    state.concentration_active = true;
    state
}

fn release_readied_spell_projection() -> SpellProcedureState {
    let mut state = initial_state();
    state.action_available = false;
    state.caster_reaction_available = false;
    state.caster_hp = INITIAL_CASTER_HP - 1;
    state.target_hp = INITIAL_TARGET_HP - magic_missile_damage(3, 3);
    state.level_one_slots_remaining = 1;
    state.readied_released = true;
    state.protocol_result = SpellProcedureProtocolResult::Resolved;
    state
}

fn initial_state() -> SpellProcedureState {
    SpellProcedureState {
        action_available: true,
        bonus_action_available: true,
        caster_reaction_available: true,
        caster_hp: INITIAL_CASTER_HP,
        target_hp: INITIAL_TARGET_HP,
        second_target_hp: INITIAL_TARGET_HP,
        target_unconscious: false,
        target_death_successes: 0,
        target_death_failures: 0,
        spell_slot_spent_this_turn: false,
        level_one_slots_remaining: INITIAL_LEVEL_ONE_SLOTS_REMAINING,
        active_effect_kind: active_effect_ref(ActiveEffectKind::None),
        readied_held: false,
        readied_released: false,
        concentration_active: false,
        protocol_result: SpellProcedureProtocolResult::Init,
        protocol_holes: Vec::new(),
        component_route: component_route(),
    }
}

fn wounded_state(hit_points: i16) -> SpellProcedureState {
    SpellProcedureState {
        target_hp: hit_points,
        ..initial_state()
    }
}

fn mass_healing_wounded_state() -> SpellProcedureState {
    SpellProcedureState {
        target_hp: 4,
        second_target_hp: 4,
        level_one_slots_remaining: 1,
        ..initial_state()
    }
}

fn magic_missile_damage(allocated_darts: i16, damage_per_dart: i16) -> i16 {
    allocated_darts * damage_per_dart
}

fn apply_ray_of_frost_hit(
    state: &mut SpellProcedureState,
    natural_d20: i16,
    total: i16,
    armor_class: i16,
    base_damage_dice: i16,
    rolled_damage_dice_count: i16,
    damage_roll: i16,
) {
    let hits = natural_d20 == 20 || (natural_d20 != 1 && total >= armor_class);
    let critical = natural_d20 == 20;
    let expected_dice_count = if critical {
        base_damage_dice * 2
    } else {
        base_damage_dice
    };
    if hits && rolled_damage_dice_count == expected_dice_count {
        state.target_hp -= damage_roll;
        state.active_effect_kind = active_effect_ref(ActiveEffectKind::SpeedDelta);
    }
}

fn acid_splash_damage(saving_throw_failed: bool, damage_roll: i16) -> i16 {
    if saving_throw_failed {
        damage_roll
    } else {
        0
    }
}

fn mage_armor_effect_kind(base_armor_class: i8) -> ActiveEffectKind {
    if base_armor_class > 10 {
        ActiveEffectKind::SpellBaseArmorClass
    } else {
        ActiveEffectKind::None
    }
}

fn component_route() -> Vec<RuleCoreComponentRouteEvent> {
    rule_core_component_route(RuleCoreComponentOwner::SpellProcedureProfile)
}

fn active_effect_ref(effect: ActiveEffectKind) -> &'static str {
    match effect {
        ActiveEffectKind::None => "none",
        ActiveEffectKind::SpeedDelta => "speedDelta",
        ActiveEffectKind::SpellBaseArmorClass => "spellBaseArmorClass",
    }
}

fn protocol_result_ref(result: SpellProcedureProtocolResult) -> &'static str {
    match result {
        SpellProcedureProtocolResult::Init => "init",
        SpellProcedureProtocolResult::NeedsHoles => "needsHoles",
        SpellProcedureProtocolResult::Resolved => "resolved",
        SpellProcedureProtocolResult::InvalidStaleSubject => "invalid",
    }
}

fn protocol_invalid_reason_ref(result: SpellProcedureProtocolResult) -> &'static str {
    match result {
        SpellProcedureProtocolResult::InvalidStaleSubject => "WStaleSubject",
        SpellProcedureProtocolResult::Init
        | SpellProcedureProtocolResult::NeedsHoles
        | SpellProcedureProtocolResult::Resolved => "",
    }
}

fn joined_holes(holes: &[SpellProcedureHole]) -> String {
    if holes.is_empty() {
        return "none".to_string();
    }
    holes.iter().map(hole_ref).collect::<Vec<_>>().join(",")
}

fn hole_ref(hole: &SpellProcedureHole) -> &'static str {
    match hole {
        SpellProcedureHole::AttackRoll => "AttackRoll",
        SpellProcedureHole::DamageRoll => "DamageRoll",
        SpellProcedureHole::SavingThrowOutcome => "SavingThrowOutcome",
        SpellProcedureHole::SpellTargetAllocation => "SpellTargetAllocation",
        SpellProcedureHole::SpellTargetList => "SpellTargetList",
        SpellProcedureHole::TargetChoice => "TargetChoice",
    }
}
