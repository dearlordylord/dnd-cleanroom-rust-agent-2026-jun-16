#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleCoreComponentOwner {
    AbilitySkillCommand,
    SpellProcedureProfile,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleCoreComponentRouteEventKind {
    ParseInput,
    AdmitInput,
    Call,
    ProjectResult,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuleCoreComponentRouteEvent {
    pub owner: RuleCoreComponentOwner,
    pub kind: RuleCoreComponentRouteEventKind,
}

pub fn rule_core_component_route(
    owner: RuleCoreComponentOwner,
) -> Vec<RuleCoreComponentRouteEvent> {
    vec![
        RuleCoreComponentRouteEvent {
            owner,
            kind: RuleCoreComponentRouteEventKind::ParseInput,
        },
        RuleCoreComponentRouteEvent {
            owner,
            kind: RuleCoreComponentRouteEventKind::AdmitInput,
        },
        RuleCoreComponentRouteEvent {
            owner,
            kind: RuleCoreComponentRouteEventKind::Call,
        },
        RuleCoreComponentRouteEvent {
            owner,
            kind: RuleCoreComponentRouteEventKind::ProjectResult,
        },
    ]
}

pub fn rule_core_component_route_event_ref(event: &RuleCoreComponentRouteEvent) -> &'static str {
    match (event.kind, event.owner) {
        (
            RuleCoreComponentRouteEventKind::ParseInput,
            RuleCoreComponentOwner::AbilitySkillCommand,
        ) => "RuleCoreComponentParseInput(RuleCoreAbilitySkillCommandOwner)",
        (
            RuleCoreComponentRouteEventKind::AdmitInput,
            RuleCoreComponentOwner::AbilitySkillCommand,
        ) => "RuleCoreComponentAdmitInput(RuleCoreAbilitySkillCommandOwner)",
        (RuleCoreComponentRouteEventKind::Call, RuleCoreComponentOwner::AbilitySkillCommand) => {
            "RuleCoreComponentCall(RuleCoreAbilitySkillCommandOwner)"
        }
        (
            RuleCoreComponentRouteEventKind::ProjectResult,
            RuleCoreComponentOwner::AbilitySkillCommand,
        ) => "RuleCoreComponentProjectResult(RuleCoreAbilitySkillCommandOwner)",
        (
            RuleCoreComponentRouteEventKind::ParseInput,
            RuleCoreComponentOwner::SpellProcedureProfile,
        ) => "RuleCoreComponentParseInput(RuleCoreSpellProcedureProfileOwner)",
        (
            RuleCoreComponentRouteEventKind::AdmitInput,
            RuleCoreComponentOwner::SpellProcedureProfile,
        ) => "RuleCoreComponentAdmitInput(RuleCoreSpellProcedureProfileOwner)",
        (RuleCoreComponentRouteEventKind::Call, RuleCoreComponentOwner::SpellProcedureProfile) => {
            "RuleCoreComponentCall(RuleCoreSpellProcedureProfileOwner)"
        }
        (
            RuleCoreComponentRouteEventKind::ProjectResult,
            RuleCoreComponentOwner::SpellProcedureProfile,
        ) => "RuleCoreComponentProjectResult(RuleCoreSpellProcedureProfileOwner)",
    }
}
