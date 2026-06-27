use super::battle_runtime_reducer_route::{
    observed_reducer_route, route_discover_battle_acts_from_route_holes,
    route_resolve_battle_subject_from_route_result,
    route_resolve_battle_subject_without_fill_from_route_result, route_start_battle,
    ReducerRouteEvent, ReducerRouteFillKind, ReducerRouteHoleKind, ReducerRouteOwnerGroup,
    ReducerRouteResolutionOutcome, ReducerRouteSubjectFamily,
};
use crate::rules::battle_reducer_spine::{
    discover_generic_route_subject_observed, resolve_battle_subject_observed,
    start_battle_observed, BattleEntrypointTrace, BattleGenericRouteFill, BattleResolutionRequest,
    BattleSetup, BattleSubjectKind,
};
use crate::rules::weapon_hosted_attack_and_riders::{
    cast_divine_favor, cast_shillelagh, clean_divine_favor_duration, clean_magic_weapon_duration,
    clean_shillelagh_after_let_go, discover_divine_favor_attack, discover_magic_weapon_target_item,
    discover_shillelagh_attack, discover_true_strike_hosted_attack, fill_divine_favor_damage_high,
    fill_divine_favor_damage_low, fill_divine_favor_hit, fill_divine_favor_target,
    fill_magic_weapon_target_item, fill_shillelagh_damage_high, fill_shillelagh_damage_low,
    fill_shillelagh_hit, fill_shillelagh_target, fill_true_strike_damage_high,
    fill_true_strike_damage_low, fill_true_strike_hit, fill_true_strike_radiant_target,
    finish_weapon_hosted_attack_and_riders, start_divine_favor_scenario,
    start_magic_weapon_scenario, start_shillelagh_scenario, WeaponHostedAttackAndRidersState,
    WeaponHostedHole, WeaponHostedPhase, WeaponHostedProtocol, WeaponHostedScenario,
};

pub const BRANCH_ACTIONS: [&str; 23] = [
    "doDiscoverTrueStrike",
    "doFillTrueStrikeRadiantTarget",
    "doFillTrueStrikeHit",
    "doFillTrueStrikeDamage",
    "doStartShillelagh",
    "doCastShillelagh",
    "doDiscoverShillelaghAttack",
    "doFillShillelaghTarget",
    "doFillShillelaghHit",
    "doFillShillelaghDamage",
    "doCleanShillelaghLetGo",
    "doStartDivineFavor",
    "doCastDivineFavor",
    "doDiscoverDivineFavorAttack",
    "doFillDivineFavorTarget",
    "doFillDivineFavorHit",
    "doFillDivineFavorDamage",
    "doCleanDivineFavorDuration",
    "doStartMagicWeapon",
    "doDiscoverMagicWeapon",
    "doFillMagicWeaponTarget",
    "doCleanMagicWeaponDuration",
    "doFinish",
];

pub const ACCEPTED_ROUTE_BRANCH_ACTIONS: [&str; 14] = [
    "doDiscoverTrueStrike",
    "doFillTrueStrikeRadiantTarget",
    "doFillTrueStrikeHit",
    "doFillTrueStrikeDamage",
    "doCastShillelagh",
    "doDiscoverShillelaghAttack",
    "doFillShillelaghTarget",
    "doFillShillelaghHit",
    "doFillShillelaghDamage",
    "doCleanShillelaghLetGo",
    "doCastDivineFavor",
    "doFillDivineFavorDamage",
    "doCleanDivineFavorDuration",
    "doCleanMagicWeaponDuration",
];

pub const BLOCKED_ROUTE_BRANCH_ACTIONS: [(&str, &str); 9] = [
    (
        "doStartShillelagh",
        "scenario transition has no reducer route surface",
    ),
    (
        "doStartDivineFavor",
        "scenario transition has no reducer route surface",
    ),
    (
        "doDiscoverDivineFavorAttack",
        "ordinary weapon target discovery is outside the rider route substrate",
    ),
    (
        "doFillDivineFavorTarget",
        "ordinary weapon target fill is outside the rider route substrate",
    ),
    (
        "doFillDivineFavorHit",
        "ordinary weapon attack-roll fill is outside the rider route substrate",
    ),
    (
        "doStartMagicWeapon",
        "scenario transition has no reducer route surface",
    ),
    (
        "doDiscoverMagicWeapon",
        "MagicWeaponTargetItem is not present in the copied reducer-route hole vocabulary",
    ),
    (
        "doFillMagicWeaponTarget",
        "MagicWeaponTargetItem is not present in the copied reducer-route fill vocabulary",
    ),
    (
        "doFinish",
        "scenario transition has no reducer route surface",
    ),
];

pub fn replay_observed_action(observed_action_taken: &str) -> WeaponHostedAttackAndRidersState {
    match observed_action_taken {
        "doDiscoverTrueStrike" => discover_true_strike_hosted_attack(),
        "doFillTrueStrikeRadiantTarget" => fill_true_strike_radiant_target(),
        "doFillTrueStrikeHit" => fill_true_strike_hit(),
        "doFillTrueStrikeDamage" => fill_true_strike_damage_high(),
        "doStartShillelagh" => start_shillelagh_scenario(),
        "doCastShillelagh" => cast_shillelagh(),
        "doDiscoverShillelaghAttack" => discover_shillelagh_attack(),
        "doFillShillelaghTarget" => fill_shillelagh_target(),
        "doFillShillelaghHit" => fill_shillelagh_hit(),
        "doFillShillelaghDamage" => fill_shillelagh_damage_high(),
        "doCleanShillelaghLetGo" => clean_shillelagh_after_let_go(),
        "doStartDivineFavor" => start_divine_favor_scenario(),
        "doCastDivineFavor" => cast_divine_favor(),
        "doDiscoverDivineFavorAttack" => discover_divine_favor_attack(),
        "doFillDivineFavorTarget" => fill_divine_favor_target(),
        "doFillDivineFavorHit" => fill_divine_favor_hit(),
        "doFillDivineFavorDamage" => fill_divine_favor_damage_high(),
        "doCleanDivineFavorDuration" => clean_divine_favor_duration(),
        "doStartMagicWeapon" => start_magic_weapon_scenario(),
        "doDiscoverMagicWeapon" => discover_magic_weapon_target_item(),
        "doFillMagicWeaponTarget" => fill_magic_weapon_target_item(),
        "doCleanMagicWeaponDuration" => clean_magic_weapon_duration(),
        "doFinish" => finish_weapon_hosted_attack_and_riders(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> WeaponHostedAttackAndRidersState {
    replay_observed_action(observed_action_taken)
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doDiscoverTrueStrike" => replay_generic_route_sequence(&[(
            BattleSubjectKind::SpellHostedWeaponAttackDamageTypeChoice,
            BattleGenericRouteFill::DamageTypeChoice,
        )]),
        "doFillTrueStrikeRadiantTarget" => replay_generic_route_sequence(&[
            (
                BattleSubjectKind::SpellHostedWeaponAttackDamageTypeChoice,
                BattleGenericRouteFill::DamageTypeChoice,
            ),
            (
                BattleSubjectKind::SpellHostedWeaponAttackTargetChoice,
                BattleGenericRouteFill::TargetChoice,
            ),
        ]),
        "doFillTrueStrikeHit" => replay_generic_route_sequence(&[(
            BattleSubjectKind::SpellHostedWeaponAttackAttackRoll,
            BattleGenericRouteFill::AttackRoll,
        )]),
        "doFillTrueStrikeDamage" => replay_generic_route_sequence(&[(
            BattleSubjectKind::SpellHostedWeaponAttackDamage,
            BattleGenericRouteFill::RolledDice,
        )]),
        "doCastShillelagh" => replay_generic_route_sequence(&[(
            BattleSubjectKind::HeldWeaponActiveEffectApply,
            BattleGenericRouteFill::WithoutFill,
        )]),
        "doDiscoverShillelaghAttack" | "doFillShillelaghTarget" => {
            replay_generic_route_sequence(&[(
                BattleSubjectKind::HeldWeaponActiveEffectTargetChoice,
                BattleGenericRouteFill::TargetChoice,
            )])
        }
        "doFillShillelaghHit" => replay_generic_route_sequence(&[(
            BattleSubjectKind::HeldWeaponActiveEffectAttackRoll,
            BattleGenericRouteFill::AttackRoll,
        )]),
        "doFillShillelaghDamage" => replay_generic_route_sequence(&[(
            BattleSubjectKind::HeldWeaponActiveEffectDamage,
            BattleGenericRouteFill::RolledDice,
        )]),
        "doCleanShillelaghLetGo" | "doCleanMagicWeaponDuration" => {
            replay_generic_route_sequence(&[(
                BattleSubjectKind::HeldWeaponActiveEffectCleanup,
                BattleGenericRouteFill::WithoutFill,
            )])
        }
        "doCastDivineFavor" => replay_generic_route_sequence(&[(
            BattleSubjectKind::WeaponDamageRiderActiveEffect,
            BattleGenericRouteFill::WithoutFill,
        )]),
        "doFillDivineFavorDamage" => replay_generic_route_sequence(&[(
            BattleSubjectKind::WeaponDamageRiderDamage,
            BattleGenericRouteFill::RolledDice,
        )]),
        "doCleanDivineFavorDuration" => replay_generic_route_sequence(&[(
            BattleSubjectKind::WeaponDamageRiderCleanup,
            BattleGenericRouteFill::WithoutFill,
        )]),
        action if blocker_reason(action).is_some() => {
            panic!("{}: {}", action, blocker_reason(action).unwrap())
        }
        action => panic!("unsupported route mbt::actionTaken {action}"),
    }
}

fn replay_generic_route_sequence(
    steps: &[(BattleSubjectKind, BattleGenericRouteFill)],
) -> Vec<ReducerRouteEvent> {
    let mut trace = BattleEntrypointTrace::default();
    let mut state = start_battle_observed(BattleSetup::standard(), &mut trace).state;
    for (subject_kind, fill) in steps {
        let discovered = discover_generic_route_subject_observed(state, *subject_kind, &mut trace);
        state = resolve_battle_subject_observed(
            discovered.0,
            BattleResolutionRequest::generic_route(discovered.1, *fill)
                .expect("generic route subject should accept generic route fills"),
            &mut trace,
        )
        .into_state();
    }
    observed_reducer_route(
        &trace,
        &[
            ReducerRouteSubjectFamily::SpellHostedWeaponAttack,
            ReducerRouteSubjectFamily::HeldWeaponActiveEffect,
            ReducerRouteSubjectFamily::WeaponDamageRider,
        ],
    )
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doDiscoverTrueStrike" => vec![
            route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
            route_discover_battle_acts_from_route_holes(
                ReducerRouteSubjectFamily::SpellHostedWeaponAttack,
                vec![
                    ReducerRouteHoleKind::DamageTypeChoice,
                    ReducerRouteHoleKind::TargetChoice,
                ],
                ReducerRouteOwnerGroup::ActionEconomy,
            ),
            route_resolve_battle_subject_from_route_result(
                ReducerRouteSubjectFamily::SpellHostedWeaponAttack,
                ReducerRouteFillKind::DamageTypeChoice,
                ReducerRouteResolutionOutcome::NeedsHoles,
                vec![ReducerRouteHoleKind::TargetChoice],
                ReducerRouteOwnerGroup::HoleFrontier,
            ),
        ],
        "doFillTrueStrikeRadiantTarget" => {
            let mut route = expected_route("doDiscoverTrueStrike");
            route.extend(
                expected_single_fill_route(
                    ReducerRouteSubjectFamily::SpellHostedWeaponAttack,
                    vec![ReducerRouteHoleKind::TargetChoice],
                    ReducerRouteOwnerGroup::TargetSelection,
                    ReducerRouteFillKind::TargetChoice,
                    ReducerRouteResolutionOutcome::NeedsHoles,
                    vec![ReducerRouteHoleKind::AttackRoll],
                    ReducerRouteOwnerGroup::TargetSelection,
                )
                .into_iter()
                .skip(1),
            );
            route
        }
        "doFillTrueStrikeHit" => expected_single_fill_route(
            ReducerRouteSubjectFamily::SpellHostedWeaponAttack,
            vec![ReducerRouteHoleKind::AttackRoll],
            ReducerRouteOwnerGroup::AttackRoll,
            ReducerRouteFillKind::AttackRoll,
            ReducerRouteResolutionOutcome::NeedsHoles,
            vec![ReducerRouteHoleKind::RolledDice],
            ReducerRouteOwnerGroup::AttackRoll,
        ),
        "doFillTrueStrikeDamage" => expected_damage_route(
            ReducerRouteSubjectFamily::SpellHostedWeaponAttack,
            ReducerRouteOwnerGroup::HitPoint,
            ReducerRouteOwnerGroup::HitPoint,
        ),
        "doCastShillelagh" => expected_without_fill_route(
            ReducerRouteSubjectFamily::HeldWeaponActiveEffect,
            ReducerRouteOwnerGroup::ActionEconomy,
            ReducerRouteOwnerGroup::ActiveEffect,
        ),
        "doDiscoverShillelaghAttack" | "doFillShillelaghTarget" => expected_single_fill_route(
            ReducerRouteSubjectFamily::HeldWeaponActiveEffect,
            vec![ReducerRouteHoleKind::TargetChoice],
            ReducerRouteOwnerGroup::ActiveEffect,
            ReducerRouteFillKind::TargetChoice,
            ReducerRouteResolutionOutcome::NeedsHoles,
            vec![ReducerRouteHoleKind::AttackRoll],
            ReducerRouteOwnerGroup::TargetSelection,
        ),
        "doFillShillelaghHit" => expected_single_fill_route(
            ReducerRouteSubjectFamily::HeldWeaponActiveEffect,
            vec![ReducerRouteHoleKind::AttackRoll],
            ReducerRouteOwnerGroup::ActiveEffect,
            ReducerRouteFillKind::AttackRoll,
            ReducerRouteResolutionOutcome::NeedsHoles,
            vec![ReducerRouteHoleKind::RolledDice],
            ReducerRouteOwnerGroup::AttackRoll,
        ),
        "doFillShillelaghDamage" => expected_damage_route(
            ReducerRouteSubjectFamily::HeldWeaponActiveEffect,
            ReducerRouteOwnerGroup::ActiveEffect,
            ReducerRouteOwnerGroup::HitPoint,
        ),
        "doCleanShillelaghLetGo" | "doCleanMagicWeaponDuration" => expected_without_fill_route(
            ReducerRouteSubjectFamily::HeldWeaponActiveEffect,
            ReducerRouteOwnerGroup::ActiveEffect,
            ReducerRouteOwnerGroup::ActiveEffect,
        ),
        "doCastDivineFavor" => expected_without_fill_route(
            ReducerRouteSubjectFamily::WeaponDamageRider,
            ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
            ReducerRouteOwnerGroup::ActiveEffect,
        ),
        "doFillDivineFavorDamage" => expected_damage_route(
            ReducerRouteSubjectFamily::WeaponDamageRider,
            ReducerRouteOwnerGroup::ActiveEffect,
            ReducerRouteOwnerGroup::HitPoint,
        ),
        "doCleanDivineFavorDuration" => expected_without_fill_route(
            ReducerRouteSubjectFamily::WeaponDamageRider,
            ReducerRouteOwnerGroup::ActiveEffect,
            ReducerRouteOwnerGroup::ActiveEffect,
        ),
        action if blocker_reason(action).is_some() => {
            panic!("{}: {}", action, blocker_reason(action).unwrap())
        }
        action => panic!("unsupported expected route mbt::actionTaken {action}"),
    }
}

fn expected_single_fill_route(
    subject: ReducerRouteSubjectFamily,
    holes: Vec<ReducerRouteHoleKind>,
    discovery_owner: ReducerRouteOwnerGroup,
    fill: ReducerRouteFillKind,
    outcome: ReducerRouteResolutionOutcome,
    next_holes: Vec<ReducerRouteHoleKind>,
    resolution_owner: ReducerRouteOwnerGroup,
) -> Vec<ReducerRouteEvent> {
    vec![
        route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
        route_discover_battle_acts_from_route_holes(subject, holes, discovery_owner),
        route_resolve_battle_subject_from_route_result(
            subject,
            fill,
            outcome,
            next_holes,
            resolution_owner,
        ),
    ]
}

fn expected_damage_route(
    subject: ReducerRouteSubjectFamily,
    discovery_owner: ReducerRouteOwnerGroup,
    resolution_owner: ReducerRouteOwnerGroup,
) -> Vec<ReducerRouteEvent> {
    expected_single_fill_route(
        subject,
        vec![ReducerRouteHoleKind::RolledDice],
        discovery_owner,
        ReducerRouteFillKind::RolledDice,
        ReducerRouteResolutionOutcome::Resolved,
        Vec::new(),
        resolution_owner,
    )
}

fn expected_without_fill_route(
    subject: ReducerRouteSubjectFamily,
    discovery_owner: ReducerRouteOwnerGroup,
    resolution_owner: ReducerRouteOwnerGroup,
) -> Vec<ReducerRouteEvent> {
    vec![
        route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
        route_discover_battle_acts_from_route_holes(subject, Vec::new(), discovery_owner),
        route_resolve_battle_subject_without_fill_from_route_result(
            subject,
            ReducerRouteResolutionOutcome::Resolved,
            Vec::new(),
            resolution_owner,
        ),
    ]
}

#[must_use]
pub fn blocker_reason(observed_action_taken: &str) -> Option<&'static str> {
    BLOCKED_ROUTE_BRANCH_ACTIONS
        .iter()
        .find_map(|(action, reason)| (*action == observed_action_taken).then_some(*reason))
}

pub fn replay_true_strike_damage_sample(
    weapon_die_pip: i16,
    rider_die_pip: i16,
) -> WeaponHostedAttackAndRidersState {
    if weapon_die_pip == 1 && rider_die_pip == 1 {
        fill_true_strike_damage_low()
    } else {
        fill_true_strike_damage_high()
    }
}

pub fn replay_shillelagh_damage_sample(damage_die_pip: i16) -> WeaponHostedAttackAndRidersState {
    if damage_die_pip == 1 {
        fill_shillelagh_damage_low()
    } else {
        fill_shillelagh_damage_high()
    }
}

pub fn replay_divine_favor_damage_sample(
    weapon_die_pip: i16,
    rider_die_pip: i16,
) -> WeaponHostedAttackAndRidersState {
    if weapon_die_pip == 1 && rider_die_pip == 1 {
        fill_divine_favor_damage_low()
    } else {
        fill_divine_favor_damage_high()
    }
}

pub fn projection_payload(state: &WeaponHostedAttackAndRidersState) -> String {
    let protocol_holes = protocol_holes(&state.protocol);
    [
        format!("qScenario={}", scenario_ref(state.scenario)),
        format!("qPhase={}", phase_ref(state.phase)),
        format!("qTargetHp={}", state.target_hit_points),
        format!("qBonusActionAvailable={}", state.bonus_action_available),
        format!("qSlotExpended={}", state.slot_expended),
        format!("qActiveEffectPresent={}", state.active_effect_present),
        format!("qAttackBonus={}", state.attack_bonus),
        format!(
            "qDamageTypeChoiceApplied={}",
            state.damage_type_choice_applied
        ),
        format!("qDamageRiderPresent={}", state.damage_rider_present),
        format!("qWeaponEnhancementBonus={}", state.weapon_enhancement_bonus),
        format!("qHoles={}", joined_or_none(&protocol_holes)),
        format!("protocolResult={}", protocol_result_ref(&state.protocol)),
        format!("protocolHoles={}", joined_or_none(&protocol_holes)),
    ]
    .join("\n")
}

fn scenario_ref(scenario: WeaponHostedScenario) -> &'static str {
    match scenario {
        WeaponHostedScenario::TrueStrikeRadiantHit => "TrueStrikeRadiantHit",
        WeaponHostedScenario::ShillelaghHeldWeaponOverride => "ShillelaghHeldWeaponOverride",
        WeaponHostedScenario::DivineFavorWeaponDamageRider => "DivineFavorWeaponDamageRider",
        WeaponHostedScenario::MagicWeaponEnhancement => "MagicWeaponEnhancement",
        WeaponHostedScenario::Done => "Done",
    }
}

fn phase_ref(phase: WeaponHostedPhase) -> &'static str {
    match phase {
        WeaponHostedPhase::Fresh => "Fresh",
        WeaponHostedPhase::SpellChoiceNeeded => "SpellChoiceNeeded",
        WeaponHostedPhase::AttackRollNeeded => "AttackRollNeeded",
        WeaponHostedPhase::AttackDamageNeeded => "AttackDamageNeeded",
        WeaponHostedPhase::ActiveEffectApplied => "ActiveEffectApplied",
        WeaponHostedPhase::WeaponTargetNeeded => "WeaponTargetNeeded",
        WeaponHostedPhase::AfterWeaponDamage => "AfterWeaponDamage",
        WeaponHostedPhase::Cleaned => "Cleaned",
    }
}

fn protocol_result_ref(protocol: &WeaponHostedProtocol) -> &'static str {
    match protocol {
        WeaponHostedProtocol::Init(_) => "init",
        WeaponHostedProtocol::NeedsHoles(_) => "needsHoles",
        WeaponHostedProtocol::Resolved => "resolved",
    }
}

fn protocol_holes(protocol: &WeaponHostedProtocol) -> Vec<&'static str> {
    match protocol {
        WeaponHostedProtocol::Init(holes) | WeaponHostedProtocol::NeedsHoles(holes) => {
            holes.iter().map(hole_ref).collect()
        }
        WeaponHostedProtocol::Resolved => Vec::new(),
    }
}

fn hole_ref(hole: &WeaponHostedHole) -> &'static str {
    match hole {
        WeaponHostedHole::DamageTypeChoice => "DamageTypeChoice",
        WeaponHostedHole::TargetChoice => "TargetChoice",
        WeaponHostedHole::AttackRoll => "AttackRoll",
        WeaponHostedHole::AttackDamageRoll => "AttackDamageRoll",
        WeaponHostedHole::MagicWeaponTargetItem => "MagicWeaponTargetItem",
    }
}

fn joined_or_none(values: &[&str]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}
