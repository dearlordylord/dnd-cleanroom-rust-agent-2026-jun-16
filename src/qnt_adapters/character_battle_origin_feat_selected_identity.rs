use crate::rules::origin_feats::{
    criminal_origin_feat_projection, initiative_handoff_projection, AlertInitiativeState,
    Background, InitiativeHandoffFacts, OriginFeat,
};

use crate::rules::character_battle_handoff::{
    route_enter_battle_runtime, route_project_character_sheet_to_battle, CharacterBattleRouteEvent,
    CharacterBattleRouteOwnerGroup, CharacterBattleRouteSubjectFamily,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OriginFeatHandoffWitness {
    pub last_result: &'static str,
    pub origin_feat_unit_id: &'static str,
    pub background_unit_id: &'static str,
    pub initiative_score: i16,
}

pub const BRANCH_ACTIONS: [&str; 2] = [
    "doFinalizeCriminalAlertOriginFeat",
    "doProjectAlertInitiativeHandoff",
];

pub fn replay_observed_action(observed_action_taken: &str) -> OriginFeatHandoffWitness {
    match observed_action_taken {
        "doFinalizeCriminalAlertOriginFeat" => finalize_criminal_alert_origin_feat(),
        "doProjectAlertInitiativeHandoff" => project_alert_initiative_handoff(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> OriginFeatHandoffWitness {
    match observed_action_taken {
        "doFinalizeCriminalAlertOriginFeat" => expected_finalize_criminal_alert_origin_feat(),
        "doProjectAlertInitiativeHandoff" => expected_project_alert_initiative_handoff(),
        action => panic!("unsupported expected mbt::actionTaken {action}"),
    }
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<CharacterBattleRouteEvent> {
    match observed_action_taken {
        "doFinalizeCriminalAlertOriginFeat" => route_after_finalize_criminal_alert_origin_feat(),
        "doProjectAlertInitiativeHandoff" => route_after_project_alert_initiative_handoff(),
        action => panic!("unsupported route mbt::actionTaken {action}"),
    }
}

pub fn expected_route(observed_action_taken: &str) -> Vec<CharacterBattleRouteEvent> {
    match observed_action_taken {
        "doFinalizeCriminalAlertOriginFeat" => {
            expected_route_after_finalize_criminal_alert_origin_feat()
        }
        "doProjectAlertInitiativeHandoff" => {
            expected_route_after_project_alert_initiative_handoff()
        }
        action => panic!("unsupported expected route mbt::actionTaken {action}"),
    }
}

pub fn projection_payload(witness: &OriginFeatHandoffWitness) -> String {
    [
        format!("lastResult={}", witness.last_result),
        format!("originFeatUnitId={}", witness.origin_feat_unit_id),
        format!("backgroundUnitId={}", witness.background_unit_id),
        format!("initiativeScore={}", witness.initiative_score),
    ]
    .join("\n")
}

fn finalize_criminal_alert_origin_feat() -> OriginFeatHandoffWitness {
    let projection = criminal_origin_feat_projection();

    OriginFeatHandoffWitness {
        last_result: "criminal-alert-origin-feat",
        origin_feat_unit_id: origin_feat_ref(projection.origin_feat),
        background_unit_id: background_ref(projection.background),
        initiative_score: 0,
    }
}

fn project_alert_initiative_handoff() -> OriginFeatHandoffWitness {
    let origin_projection = criminal_origin_feat_projection();
    let initiative_projection = initiative_handoff_projection(InitiativeHandoffFacts {
        dexterity_modifier: 4,
        proficiency_bonus: 2,
        alert_initiative: AlertInitiativeState::Present,
    });

    OriginFeatHandoffWitness {
        last_result: "alert-initiative-handoff",
        origin_feat_unit_id: origin_feat_ref(origin_projection.origin_feat),
        background_unit_id: background_ref(origin_projection.background),
        initiative_score: initiative_projection.initiative_score,
    }
}

fn expected_finalize_criminal_alert_origin_feat() -> OriginFeatHandoffWitness {
    OriginFeatHandoffWitness {
        last_result: "criminal-alert-origin-feat",
        origin_feat_unit_id: "alert",
        background_unit_id: "background_criminal",
        initiative_score: 0,
    }
}

fn expected_project_alert_initiative_handoff() -> OriginFeatHandoffWitness {
    OriginFeatHandoffWitness {
        last_result: "alert-initiative-handoff",
        origin_feat_unit_id: "alert",
        background_unit_id: "background_criminal",
        initiative_score: 16,
    }
}

fn route_after_finalize_criminal_alert_origin_feat() -> Vec<CharacterBattleRouteEvent> {
    let mut route = Vec::new();
    append_retain_origin_feat_route(&mut route);
    route
}

fn route_after_project_alert_initiative_handoff() -> Vec<CharacterBattleRouteEvent> {
    let mut route = route_after_finalize_criminal_alert_origin_feat();
    append_project_alert_initiative_handoff_route(&mut route);
    route
}

fn expected_route_after_finalize_criminal_alert_origin_feat() -> Vec<CharacterBattleRouteEvent> {
    vec![expected_project_character_sheet_to_battle(
        CharacterBattleRouteSubjectFamily::HandoffSelectedReferenceRouteSubject,
        CharacterBattleRouteOwnerGroup::CharacterBattleBuildProjectionOwner,
    )]
}

fn expected_route_after_project_alert_initiative_handoff() -> Vec<CharacterBattleRouteEvent> {
    vec![
        expected_project_character_sheet_to_battle(
            CharacterBattleRouteSubjectFamily::HandoffSelectedReferenceRouteSubject,
            CharacterBattleRouteOwnerGroup::CharacterBattleBuildProjectionOwner,
        ),
        expected_project_character_sheet_to_battle(
            CharacterBattleRouteSubjectFamily::HandoffSelectedReferenceRouteSubject,
            CharacterBattleRouteOwnerGroup::CharacterBattleInitProjectionOwner,
        ),
        expected_enter_battle_runtime(
            CharacterBattleRouteSubjectFamily::HandoffSelectedReferenceRouteSubject,
            CharacterBattleRouteOwnerGroup::CharacterBattleRuntimeOwner,
        ),
    ]
}

fn append_retain_origin_feat_route(route: &mut Vec<CharacterBattleRouteEvent>) {
    route.push(route_project_character_sheet_to_battle(
        CharacterBattleRouteSubjectFamily::HandoffSelectedReferenceRouteSubject,
        CharacterBattleRouteOwnerGroup::CharacterBattleBuildProjectionOwner,
    ));
}

fn append_project_alert_initiative_handoff_route(route: &mut Vec<CharacterBattleRouteEvent>) {
    route.push(route_project_character_sheet_to_battle(
        CharacterBattleRouteSubjectFamily::HandoffSelectedReferenceRouteSubject,
        CharacterBattleRouteOwnerGroup::CharacterBattleInitProjectionOwner,
    ));
    route.push(route_enter_battle_runtime(
        CharacterBattleRouteSubjectFamily::HandoffSelectedReferenceRouteSubject,
        CharacterBattleRouteOwnerGroup::CharacterBattleRuntimeOwner,
    ));
}

fn expected_project_character_sheet_to_battle(
    subject: CharacterBattleRouteSubjectFamily,
    owner: CharacterBattleRouteOwnerGroup,
) -> CharacterBattleRouteEvent {
    CharacterBattleRouteEvent::RouteProjectCharacterSheetToBattle { subject, owner }
}

fn expected_enter_battle_runtime(
    subject: CharacterBattleRouteSubjectFamily,
    owner: CharacterBattleRouteOwnerGroup,
) -> CharacterBattleRouteEvent {
    CharacterBattleRouteEvent::RouteEnterBattleRuntime { subject, owner }
}

fn origin_feat_ref(origin_feat: OriginFeat) -> &'static str {
    match origin_feat {
        OriginFeat::Alert => "alert",
    }
}

fn background_ref(background: Background) -> &'static str {
    match background {
        Background::Criminal => "background_criminal",
    }
}
