use crate::rules::battle_reducer_spine::{
    discover_battle_acts, resolve_battle_subject, save_gated_spell_ordering_from_battle,
    start_fighter_skeleton_battle, BattleFill, BattleHoleKind, BattleResolutionResult,
    BattleSaveGatedSpellFill, BattleState, BattleSubject, BattleSubjectKind,
};
use crate::rules::save_gated_spell_ordering::{
    discover_area_save_damage, discover_target_list_condition_choice, fill_area_damage_dice,
    fill_area_save_failed, fill_condition_choice_after_target_list,
    fill_condition_choice_before_target_list, fill_condition_saving_throw,
    fill_target_list_after_condition_choice, fill_target_list_before_condition_choice,
    submit_damage_before_saving_throw, SaveGatedSpellFillOrderingError,
    SaveGatedSpellFrontierStage, SaveGatedSpellHoleKind, SaveGatedSpellOrderingProtocol,
    SaveGatedSpellOrderingState,
};

use super::battle_runtime_reducer_route::{
    route_discover_battle_acts, route_resolve_battle_subject, route_start_battle,
    ReducerRouteEvent, ReducerRouteFillKind, ReducerRouteOwnerGroup, ReducerRouteSubjectFamily,
};

pub const BRANCH_ACTIONS: [&str; 10] = [
    "doDiscoverAreaSaveDamage",
    "doSubmitDamageBeforeSavingThrow",
    "doFillAreaSaveFailed",
    "doFillAreaDamageDice",
    "doDiscoverTargetListConditionChoice",
    "doFillTargetListBeforeConditionChoice",
    "doFillConditionChoiceAfterTargetList",
    "doFillConditionChoiceBeforeTargetList",
    "doFillTargetListAfterConditionChoice",
    "doFillConditionSavingThrow",
];

pub fn replay_observed_action(observed_action_taken: &str) -> SaveGatedSpellOrderingState {
    match observed_action_taken {
        "doDiscoverAreaSaveDamage" => discovered_area_save_damage(),
        "doSubmitDamageBeforeSavingThrow" => {
            state_after_area_fill(BattleSaveGatedSpellFill::DamageRoll)
        }
        "doFillAreaSaveFailed" => {
            state_after_area_fill(BattleSaveGatedSpellFill::SavingThrowOutcome)
        }
        "doFillAreaDamageDice" => state_after_area_damage_dice(),
        "doDiscoverTargetListConditionChoice" => discovered_target_list_condition_choice(),
        "doFillTargetListBeforeConditionChoice" => {
            state_after_condition_path_fill(BattleSaveGatedSpellFill::SpellTargetList)
        }
        "doFillConditionChoiceAfterTargetList" => state_after_target_list_then_condition_choice(),
        "doFillConditionChoiceBeforeTargetList" => {
            state_after_condition_path_fill(BattleSaveGatedSpellFill::ConditionChoice)
        }
        "doFillTargetListAfterConditionChoice" => state_after_condition_choice_then_target_list(),
        "doFillConditionSavingThrow" => state_after_condition_saving_throw(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> SaveGatedSpellOrderingState {
    match observed_action_taken {
        "doDiscoverAreaSaveDamage" => discover_area_save_damage(),
        "doSubmitDamageBeforeSavingThrow" => submit_damage_before_saving_throw(),
        "doFillAreaSaveFailed" => fill_area_save_failed(),
        "doFillAreaDamageDice" => fill_area_damage_dice(),
        "doDiscoverTargetListConditionChoice" => discover_target_list_condition_choice(),
        "doFillTargetListBeforeConditionChoice" => fill_target_list_before_condition_choice(),
        "doFillConditionChoiceAfterTargetList" => fill_condition_choice_after_target_list(),
        "doFillConditionChoiceBeforeTargetList" => fill_condition_choice_before_target_list(),
        "doFillTargetListAfterConditionChoice" => fill_target_list_after_condition_choice(),
        "doFillConditionSavingThrow" => fill_condition_saving_throw(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doDiscoverAreaSaveDamage" => area_discovery_route().2,
        "doSubmitDamageBeforeSavingThrow" => {
            area_route_after_fill(BattleSaveGatedSpellFill::DamageRoll).1
        }
        "doFillAreaSaveFailed" => {
            area_route_after_fill(BattleSaveGatedSpellFill::SavingThrowOutcome).1
        }
        "doFillAreaDamageDice" => area_damage_dice_route(),
        "doDiscoverTargetListConditionChoice" => condition_discovery_route().2,
        "doFillTargetListBeforeConditionChoice" => {
            condition_route_after_fill(BattleSaveGatedSpellFill::SpellTargetList).1
        }
        "doFillConditionChoiceAfterTargetList" => target_list_then_condition_route(),
        "doFillConditionChoiceBeforeTargetList" => {
            condition_route_after_fill(BattleSaveGatedSpellFill::ConditionChoice).1
        }
        "doFillTargetListAfterConditionChoice" => condition_then_target_list_route(),
        "doFillConditionSavingThrow" => condition_saving_throw_route(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doDiscoverAreaSaveDamage" => expected_area_discovery_route(),
        "doSubmitDamageBeforeSavingThrow" => {
            let mut route = expected_area_discovery_route();
            route.push(route_resolve_battle_subject(
                ReducerRouteSubjectFamily::SaveGatedSpell,
                ReducerRouteFillKind::RolledDice,
                vec![BattleHoleKind::SavingThrowOutcome],
                ReducerRouteOwnerGroup::HoleFrontier,
            ));
            route
        }
        "doFillAreaSaveFailed" => {
            let mut route = expected_area_discovery_route();
            route.push(route_resolve_battle_subject(
                ReducerRouteSubjectFamily::SaveGatedSpell,
                ReducerRouteFillKind::SavingThrowOutcome,
                vec![BattleHoleKind::RolledDice],
                ReducerRouteOwnerGroup::HoleFrontier,
            ));
            route
        }
        "doFillAreaDamageDice" => {
            let mut route = expected_area_discovery_route();
            route.push(route_resolve_battle_subject(
                ReducerRouteSubjectFamily::SaveGatedSpell,
                ReducerRouteFillKind::SavingThrowOutcome,
                vec![BattleHoleKind::RolledDice],
                ReducerRouteOwnerGroup::HoleFrontier,
            ));
            route.push(route_resolve_battle_subject(
                ReducerRouteSubjectFamily::SaveGatedSpell,
                ReducerRouteFillKind::RolledDice,
                Vec::new(),
                ReducerRouteOwnerGroup::HitPoint,
            ));
            route
        }
        "doDiscoverTargetListConditionChoice" => expected_condition_discovery_route(),
        "doFillTargetListBeforeConditionChoice" => {
            let mut route = expected_condition_discovery_route();
            route.push(route_resolve_battle_subject(
                ReducerRouteSubjectFamily::SaveGatedSpell,
                ReducerRouteFillKind::SpellTargetList,
                vec![BattleHoleKind::ConditionChoice],
                ReducerRouteOwnerGroup::HoleFrontier,
            ));
            route
        }
        "doFillConditionChoiceAfterTargetList" => {
            let mut route = expected_route("doFillTargetListBeforeConditionChoice");
            route.push(route_resolve_battle_subject(
                ReducerRouteSubjectFamily::SaveGatedSpell,
                ReducerRouteFillKind::ConditionChoice,
                vec![BattleHoleKind::SavingThrowOutcome],
                ReducerRouteOwnerGroup::HoleFrontier,
            ));
            route
        }
        "doFillConditionChoiceBeforeTargetList" => {
            let mut route = expected_condition_discovery_route();
            route.push(route_resolve_battle_subject(
                ReducerRouteSubjectFamily::SaveGatedSpell,
                ReducerRouteFillKind::ConditionChoice,
                vec![BattleHoleKind::SpellTargetList],
                ReducerRouteOwnerGroup::HoleFrontier,
            ));
            route
        }
        "doFillTargetListAfterConditionChoice" => {
            let mut route = expected_route("doFillConditionChoiceBeforeTargetList");
            route.push(route_resolve_battle_subject(
                ReducerRouteSubjectFamily::SaveGatedSpell,
                ReducerRouteFillKind::SpellTargetList,
                vec![BattleHoleKind::SavingThrowOutcome],
                ReducerRouteOwnerGroup::HoleFrontier,
            ));
            route
        }
        "doFillConditionSavingThrow" => {
            let mut route = expected_route("doFillTargetListAfterConditionChoice");
            route.push(route_resolve_battle_subject(
                ReducerRouteSubjectFamily::SaveGatedSpell,
                ReducerRouteFillKind::SavingThrowOutcome,
                Vec::new(),
                ReducerRouteOwnerGroup::HoleFrontier,
            ));
            route
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn projection_payload(state: &SaveGatedSpellOrderingState) -> String {
    let protocol_holes = protocol_holes(&state.protocol);
    [
        format!("qStage={}", stage_ref(state.stage)),
        format!("qHoles={}", joined_or_none(&protocol_holes)),
        format!(
            "qLastOrderingError={}",
            ordering_error_ref(state.last_ordering_error)
        ),
        format!("protocolResult={}", protocol_result_ref(&state.protocol)),
        format!("protocolHoles={}", joined_or_none(&protocol_holes)),
    ]
    .join("\n")
}

fn expected_area_discovery_route() -> Vec<ReducerRouteEvent> {
    vec![
        route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
        route_discover_battle_acts(
            ReducerRouteSubjectFamily::SaveGatedSpell,
            vec![BattleHoleKind::SavingThrowOutcome],
            ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
        ),
    ]
}

fn expected_condition_discovery_route() -> Vec<ReducerRouteEvent> {
    vec![
        route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
        route_discover_battle_acts(
            ReducerRouteSubjectFamily::SaveGatedSpell,
            vec![
                BattleHoleKind::SpellTargetList,
                BattleHoleKind::ConditionChoice,
            ],
            ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
        ),
    ]
}

fn stage_ref(stage: SaveGatedSpellFrontierStage) -> &'static str {
    match stage {
        SaveGatedSpellFrontierStage::ActSelection => "SaveGatedSpellActSelectionStage",
        SaveGatedSpellFrontierStage::TargetListAndConditionChoice => {
            "SaveGatedSpellTargetListAndConditionChoiceStage"
        }
        SaveGatedSpellFrontierStage::TargetList => "SaveGatedSpellTargetListStage",
        SaveGatedSpellFrontierStage::ConditionChoice => "SaveGatedSpellConditionChoiceStage",
        SaveGatedSpellFrontierStage::DamageSavingThrowOutcome => {
            "SaveGatedSpellDamageSavingThrowOutcomeStage"
        }
        SaveGatedSpellFrontierStage::ConditionSavingThrowOutcome => {
            "SaveGatedSpellConditionSavingThrowOutcomeStage"
        }
        SaveGatedSpellFrontierStage::DamageDice => "SaveGatedSpellDamageDiceStage",
        SaveGatedSpellFrontierStage::Resolved => "SaveGatedSpellResolvedStage",
    }
}

fn protocol_result_ref(protocol: &SaveGatedSpellOrderingProtocol) -> &'static str {
    match protocol {
        SaveGatedSpellOrderingProtocol::Init => "init",
        SaveGatedSpellOrderingProtocol::NeedsHoles(_) => "needsHoles",
        SaveGatedSpellOrderingProtocol::Resolved => "resolved",
    }
}

fn protocol_holes(protocol: &SaveGatedSpellOrderingProtocol) -> Vec<&'static str> {
    match protocol {
        SaveGatedSpellOrderingProtocol::NeedsHoles(holes) => holes
            .iter()
            .map(|hole| match hole {
                SaveGatedSpellHoleKind::SpellTargetList => "SpellTargetListHoleKind",
                SaveGatedSpellHoleKind::ConditionChoice => "ConditionChoiceHoleKind",
                SaveGatedSpellHoleKind::SavingThrowOutcome => "SavingThrowOutcomeHoleKind",
                SaveGatedSpellHoleKind::RolledDice => "RolledDiceHoleKind",
            })
            .collect(),
        SaveGatedSpellOrderingProtocol::Init | SaveGatedSpellOrderingProtocol::Resolved => {
            Vec::new()
        }
    }
}

fn ordering_error_ref(error: Option<SaveGatedSpellFillOrderingError>) -> &'static str {
    match error {
        Some(SaveGatedSpellFillOrderingError::TargetOrAreaRequired) => "targetOrAreaRequired",
        Some(SaveGatedSpellFillOrderingError::SavingThrowRequired) => "savingThrowRequired",
        None => "",
    }
}

fn joined_or_none(values: &[&str]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}

fn area_discovery_route() -> (BattleState, BattleSubject, Vec<ReducerRouteEvent>) {
    discovery_route(BattleSubjectKind::SaveGatedAreaDamage)
}

fn condition_discovery_route() -> (BattleState, BattleSubject, Vec<ReducerRouteEvent>) {
    discovery_route(BattleSubjectKind::SaveGatedTargetListConditionChoice)
}

fn discovery_route(
    subject_kind: BattleSubjectKind,
) -> (BattleState, BattleSubject, Vec<ReducerRouteEvent>) {
    let state = start_fighter_skeleton_battle();
    let act = discovered_save_gated_act(&state, subject_kind);
    let route = vec![
        route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
        route_discover_battle_acts(
            ReducerRouteSubjectFamily::SaveGatedSpell,
            act.holes,
            ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
        ),
    ];
    (state, act.subject, route)
}

fn area_route_after_fill(
    fill: BattleSaveGatedSpellFill,
) -> (BattleResolutionResult, Vec<ReducerRouteEvent>) {
    let (state, subject, route) = area_discovery_route();
    resolve_with_route(
        state,
        subject,
        fill,
        save_gated_route_fill(fill),
        ReducerRouteOwnerGroup::HoleFrontier,
        route,
    )
}

fn condition_route_after_fill(
    fill: BattleSaveGatedSpellFill,
) -> (BattleResolutionResult, Vec<ReducerRouteEvent>) {
    let (state, subject, route) = condition_discovery_route();
    resolve_with_route(
        state,
        subject,
        fill,
        save_gated_route_fill(fill),
        ReducerRouteOwnerGroup::HoleFrontier,
        route,
    )
}

fn area_damage_dice_route() -> Vec<ReducerRouteEvent> {
    let (result, route) = area_route_after_fill(BattleSaveGatedSpellFill::SavingThrowOutcome);
    let (state, subject) = needs_holes(result);
    resolve_with_route(
        state,
        subject,
        BattleSaveGatedSpellFill::DamageRoll,
        ReducerRouteFillKind::RolledDice,
        ReducerRouteOwnerGroup::HitPoint,
        route,
    )
    .1
}

fn target_list_then_condition_route() -> Vec<ReducerRouteEvent> {
    let (result, route) = condition_route_after_fill(BattleSaveGatedSpellFill::SpellTargetList);
    let (state, subject) = needs_holes(result);
    resolve_with_route(
        state,
        subject,
        BattleSaveGatedSpellFill::ConditionChoice,
        ReducerRouteFillKind::ConditionChoice,
        ReducerRouteOwnerGroup::HoleFrontier,
        route,
    )
    .1
}

fn condition_then_target_list_route() -> Vec<ReducerRouteEvent> {
    let (result, route) = condition_route_after_fill(BattleSaveGatedSpellFill::ConditionChoice);
    let (state, subject) = needs_holes(result);
    resolve_with_route(
        state,
        subject,
        BattleSaveGatedSpellFill::SpellTargetList,
        ReducerRouteFillKind::SpellTargetList,
        ReducerRouteOwnerGroup::HoleFrontier,
        route,
    )
    .1
}

fn condition_saving_throw_route() -> Vec<ReducerRouteEvent> {
    let (result, route) = condition_route_after_fill(BattleSaveGatedSpellFill::ConditionChoice);
    let (state, subject) = needs_holes(result);
    let (result, route) = resolve_with_route(
        state,
        subject,
        BattleSaveGatedSpellFill::SpellTargetList,
        ReducerRouteFillKind::SpellTargetList,
        ReducerRouteOwnerGroup::HoleFrontier,
        route,
    );
    let (state, subject) = needs_holes(result);
    resolve_with_route(
        state,
        subject,
        BattleSaveGatedSpellFill::SavingThrowOutcome,
        ReducerRouteFillKind::SavingThrowOutcome,
        ReducerRouteOwnerGroup::HoleFrontier,
        route,
    )
    .1
}

fn resolve_with_route(
    state: BattleState,
    subject: BattleSubject,
    fill: BattleSaveGatedSpellFill,
    route_fill: ReducerRouteFillKind,
    owner: ReducerRouteOwnerGroup,
    mut route: Vec<ReducerRouteEvent>,
) -> (BattleResolutionResult, Vec<ReducerRouteEvent>) {
    let result = resolve_battle_subject(state, subject, BattleFill::SaveGatedSpell(fill));
    let holes = match &result {
        BattleResolutionResult::NeedsHoles { holes, .. }
        | BattleResolutionResult::Invalid { holes, .. } => holes.clone(),
        BattleResolutionResult::Resolved { .. } => Vec::new(),
    };
    route.push(route_resolve_battle_subject(
        ReducerRouteSubjectFamily::SaveGatedSpell,
        route_fill,
        holes,
        owner,
    ));
    (result, route)
}

fn save_gated_route_fill(fill: BattleSaveGatedSpellFill) -> ReducerRouteFillKind {
    match fill {
        BattleSaveGatedSpellFill::SpellTargetList => ReducerRouteFillKind::SpellTargetList,
        BattleSaveGatedSpellFill::ConditionChoice => ReducerRouteFillKind::ConditionChoice,
        BattleSaveGatedSpellFill::SavingThrowOutcome => ReducerRouteFillKind::SavingThrowOutcome,
        BattleSaveGatedSpellFill::DamageRoll => ReducerRouteFillKind::RolledDice,
    }
}

fn discovered_area_save_damage() -> SaveGatedSpellOrderingState {
    let state = start_fighter_skeleton_battle();
    let act = discovered_save_gated_act(&state, BattleSubjectKind::SaveGatedAreaDamage);
    assert_eq!(
        act.subject.kind,
        BattleSubjectKind::SaveGatedAreaDamage,
        "save-gated area damage must be discovered through reducer"
    );
    discover_area_save_damage()
}

fn discovered_target_list_condition_choice() -> SaveGatedSpellOrderingState {
    let state = start_fighter_skeleton_battle();
    let act = discovered_save_gated_act(
        &state,
        BattleSubjectKind::SaveGatedTargetListConditionChoice,
    );
    assert_eq!(
        act.subject.kind,
        BattleSubjectKind::SaveGatedTargetListConditionChoice,
        "save-gated target/condition act must be discovered through reducer"
    );
    discover_target_list_condition_choice()
}

fn state_after_area_fill(fill: BattleSaveGatedSpellFill) -> SaveGatedSpellOrderingState {
    let state = start_fighter_skeleton_battle();
    let subject = discovered_save_gated_act(&state, BattleSubjectKind::SaveGatedAreaDamage).subject;
    project_save_gated_result(resolve_battle_subject(
        state,
        subject,
        BattleFill::SaveGatedSpell(fill),
    ))
}

fn state_after_area_damage_dice() -> SaveGatedSpellOrderingState {
    let (state, subject) =
        reducer_state_after_area_fill(BattleSaveGatedSpellFill::SavingThrowOutcome);
    project_save_gated_result(resolve_battle_subject(
        state,
        subject,
        BattleFill::SaveGatedSpell(BattleSaveGatedSpellFill::DamageRoll),
    ))
}

fn state_after_condition_path_fill(fill: BattleSaveGatedSpellFill) -> SaveGatedSpellOrderingState {
    let state = start_fighter_skeleton_battle();
    let subject = discovered_save_gated_act(
        &state,
        BattleSubjectKind::SaveGatedTargetListConditionChoice,
    )
    .subject;
    project_save_gated_result(resolve_battle_subject(
        state,
        subject,
        BattleFill::SaveGatedSpell(fill),
    ))
}

fn state_after_target_list_then_condition_choice() -> SaveGatedSpellOrderingState {
    let (state, subject) =
        reducer_state_after_condition_path_fill(BattleSaveGatedSpellFill::SpellTargetList);
    project_save_gated_result(resolve_battle_subject(
        state,
        subject,
        BattleFill::SaveGatedSpell(BattleSaveGatedSpellFill::ConditionChoice),
    ))
}

fn state_after_condition_choice_then_target_list() -> SaveGatedSpellOrderingState {
    let (state, subject) =
        reducer_state_after_condition_path_fill(BattleSaveGatedSpellFill::ConditionChoice);
    project_save_gated_result(resolve_battle_subject(
        state,
        subject,
        BattleFill::SaveGatedSpell(BattleSaveGatedSpellFill::SpellTargetList),
    ))
}

fn state_after_condition_saving_throw() -> SaveGatedSpellOrderingState {
    let (state, subject) =
        reducer_state_after_condition_path_fill(BattleSaveGatedSpellFill::ConditionChoice);
    let (state, subject) = needs_holes(resolve_battle_subject(
        state,
        subject,
        BattleFill::SaveGatedSpell(BattleSaveGatedSpellFill::SpellTargetList),
    ));
    project_save_gated_result(resolve_battle_subject(
        state,
        subject,
        BattleFill::SaveGatedSpell(BattleSaveGatedSpellFill::SavingThrowOutcome),
    ))
}

fn reducer_state_after_area_fill(fill: BattleSaveGatedSpellFill) -> (BattleState, BattleSubject) {
    let state = start_fighter_skeleton_battle();
    let subject = discovered_save_gated_act(&state, BattleSubjectKind::SaveGatedAreaDamage).subject;
    needs_holes(resolve_battle_subject(
        state,
        subject,
        BattleFill::SaveGatedSpell(fill),
    ))
}

fn reducer_state_after_condition_path_fill(
    fill: BattleSaveGatedSpellFill,
) -> (BattleState, BattleSubject) {
    let state = start_fighter_skeleton_battle();
    let subject = discovered_save_gated_act(
        &state,
        BattleSubjectKind::SaveGatedTargetListConditionChoice,
    )
    .subject;
    needs_holes(resolve_battle_subject(
        state,
        subject,
        BattleFill::SaveGatedSpell(fill),
    ))
}

fn discovered_save_gated_act(
    state: &BattleState,
    kind: BattleSubjectKind,
) -> crate::rules::battle_reducer_spine::AvailableBattleAct {
    discover_battle_acts(state)
        .into_iter()
        .find(|act| act.subject.kind == kind)
        .expect("save-gated diagnostic act should be discovered through reducer")
}

fn needs_holes(result: BattleResolutionResult) -> (BattleState, BattleSubject) {
    match result {
        BattleResolutionResult::NeedsHoles { state, subject, .. } => (state, subject),
        other => panic!("save-gated reducer branch should need holes, got {other:?}"),
    }
}

fn project_save_gated_result(result: BattleResolutionResult) -> SaveGatedSpellOrderingState {
    match result {
        BattleResolutionResult::NeedsHoles { state, .. }
        | BattleResolutionResult::Resolved { state }
        | BattleResolutionResult::Invalid { state, .. } => {
            save_gated_spell_ordering_from_battle(&state)
        }
    }
}
