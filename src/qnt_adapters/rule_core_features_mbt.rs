#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum FeatureActionGrant {
    NoActionSurgeActionGrant,
    ActionSurgeActionAvailable,
    ActionSurgeActionSpent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum FeatureProfileProtocol {
    Init,
    NeedsHoles(&'static [&'static str]),
    Resolved,
    Invalid(FeatureProfileInvalidReason),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum FeatureProfileInvalidReason {
    StaleSubject,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuleCoreFeatureProfileState {
    pub action_available: bool,
    pub bonus_action_available: bool,
    pub reaction_available: bool,
    pub feature_uses_remaining: i16,
    pub action_surge_grant: FeatureActionGrant,
    pub actor_hit_points: i16,
    pub target_hit_points: i16,
    pub dash_bonus_feet: i16,
    pub disengaged: bool,
    pub hidden: bool,
    pub rage_active: bool,
    pub reckless_active: bool,
    pub incoming_attack_advantage: bool,
    pub sneak_attack_used_this_turn: bool,
    pub last_damage_amount: i16,
    pub ability_check_boosted_total: i16,
    pub ability_check_boosted_succeeded: bool,
    pub actor_armor_class: i16,
    pub critical: bool,
    pub pending_reaction: bool,
    pub protocol: FeatureProfileProtocol,
    pub component_route: Vec<RuleCoreComponentRouteEvent>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleCoreComponentRouteEvent {
    ParseInput(&'static str),
    AdmitInput(&'static str),
    Call(&'static str),
    ProjectResult(&'static str),
}

const FEATURE_OWNER: &str = "RuleCoreFeatureProfileSemanticsOwner";
const BASELINE_ARMOR_CLASS: i16 = 10;

pub const BRANCH_ACTIONS: [&str; 22] = [
    "doActionSurgeActivate",
    "doActionSurgeSpendAttack",
    "doActionSurgeRejectTwice",
    "doDiscoverSecondWind",
    "doResolveSecondWindLow",
    "doResolveSecondWindHigh",
    "doTacticalMindConvertedSuccess",
    "doTacticalMindStillFailed",
    "doCunningDash",
    "doCunningDisengage",
    "doCunningHide",
    "doRageActivateAndDamage",
    "doRecklessAttack",
    "doSneakAttack",
    "doFrenzy",
    "doImprovedCritical",
    "doCuttingWordsDamage",
    "doDeflectAttacksDamageReduction",
    "doDefenseArmorClass",
    "doArcheryAttackRollBonus",
    "doSavageAttackerDamage",
    "doZeroHitPointReplacement",
];

pub fn replay_observed_action(observed_action_taken: &str) -> RuleCoreFeatureProfileState {
    assert_supported_branch_action(observed_action_taken);
    RuleCoreFeatureProfileState {
        component_route: qnt_rule_core_feature_profile_component_route(),
        ..base_projection_without_component_route()
    }
}

fn assert_supported_branch_action(observed_action_taken: &str) {
    match observed_action_taken {
        "doActionSurgeActivate"
        | "doActionSurgeSpendAttack"
        | "doActionSurgeRejectTwice"
        | "doDiscoverSecondWind"
        | "doResolveSecondWindLow"
        | "doResolveSecondWindHigh"
        | "doTacticalMindConvertedSuccess"
        | "doTacticalMindStillFailed"
        | "doCunningDash"
        | "doCunningDisengage"
        | "doCunningHide"
        | "doRageActivateAndDamage"
        | "doRecklessAttack"
        | "doSneakAttack"
        | "doFrenzy"
        | "doImprovedCritical"
        | "doCuttingWordsDamage"
        | "doDeflectAttacksDamageReduction"
        | "doDefenseArmorClass"
        | "doArcheryAttackRollBonus"
        | "doSavageAttackerDamage"
        | "doZeroHitPointReplacement" => {}
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn qnt_component_route_witness() -> Vec<RuleCoreComponentRouteEvent> {
    qnt_rule_core_feature_profile_component_route()
}

fn base_projection_without_component_route() -> RuleCoreFeatureProfileState {
    RuleCoreFeatureProfileState {
        action_available: true,
        bonus_action_available: true,
        reaction_available: true,
        feature_uses_remaining: 1,
        action_surge_grant: FeatureActionGrant::NoActionSurgeActionGrant,
        actor_hit_points: 12,
        target_hit_points: 12,
        dash_bonus_feet: 0,
        disengaged: false,
        hidden: false,
        rage_active: false,
        reckless_active: false,
        incoming_attack_advantage: false,
        sneak_attack_used_this_turn: false,
        last_damage_amount: 0,
        ability_check_boosted_total: 0,
        ability_check_boosted_succeeded: false,
        actor_armor_class: BASELINE_ARMOR_CLASS,
        critical: false,
        pending_reaction: false,
        protocol: FeatureProfileProtocol::Init,
        component_route: Vec::new(),
    }
}

fn qnt_rule_core_feature_profile_component_route() -> Vec<RuleCoreComponentRouteEvent> {
    vec![
        RuleCoreComponentRouteEvent::ParseInput(FEATURE_OWNER),
        RuleCoreComponentRouteEvent::AdmitInput(FEATURE_OWNER),
        RuleCoreComponentRouteEvent::Call(FEATURE_OWNER),
        RuleCoreComponentRouteEvent::ProjectResult(FEATURE_OWNER),
    ]
}

pub fn projection_payload(state: &RuleCoreFeatureProfileState) -> String {
    let holes = protocol_holes(&state.protocol);
    [
        format!("qActionAvailable={}", state.action_available),
        format!("qBonusActionAvailable={}", state.bonus_action_available),
        format!("qReactionAvailable={}", state.reaction_available),
        format!("qFeatureUsesRemaining={}", state.feature_uses_remaining),
        format!(
            "qActionSurgeGrant={}",
            action_surge_grant_ref(state.action_surge_grant)
        ),
        format!("qActorHp={}", state.actor_hit_points),
        format!("qTargetHp={}", state.target_hit_points),
        format!("qDashBonusFeet={}", state.dash_bonus_feet),
        format!("qDisengaged={}", state.disengaged),
        format!("qHidden={}", state.hidden),
        format!("qRageActive={}", state.rage_active),
        format!("qRecklessActive={}", state.reckless_active),
        format!(
            "qIncomingAttackAdvantage={}",
            state.incoming_attack_advantage
        ),
        format!(
            "qSneakAttackUsedThisTurn={}",
            state.sneak_attack_used_this_turn
        ),
        format!("qLastDamageAmount={}", state.last_damage_amount),
        format!(
            "qAbilityCheckBoostedTotal={}",
            state.ability_check_boosted_total
        ),
        format!(
            "qAbilityCheckBoostedSucceeded={}",
            state.ability_check_boosted_succeeded
        ),
        format!("qActorArmorClass={}", state.actor_armor_class),
        format!("qCritical={}", state.critical),
        format!("qHoles={}", joined_or_none(holes)),
        format!("qPendingReaction={}", state.pending_reaction),
        format!("protocolResult={}", protocol_result_ref(&state.protocol)),
        format!(
            "protocolInvalidReason={}",
            protocol_invalid_reason_ref(&state.protocol)
        ),
        format!("protocolHoles={}", joined_or_none(holes)),
        format!(
            "qComponentRoute={}",
            component_route_ref(&state.component_route)
        ),
    ]
    .join("\n")
}

fn action_surge_grant_ref(grant: FeatureActionGrant) -> &'static str {
    match grant {
        FeatureActionGrant::NoActionSurgeActionGrant => "NoActionSurgeActionGrant",
        FeatureActionGrant::ActionSurgeActionAvailable => "ActionSurgeActionAvailable",
        FeatureActionGrant::ActionSurgeActionSpent => "ActionSurgeActionSpent",
    }
}

fn protocol_result_ref(protocol: &FeatureProfileProtocol) -> &'static str {
    match protocol {
        FeatureProfileProtocol::Init => "init",
        FeatureProfileProtocol::NeedsHoles(_) => "needsHoles",
        FeatureProfileProtocol::Resolved => "resolved",
        FeatureProfileProtocol::Invalid(_) => "invalid",
    }
}

fn protocol_invalid_reason_ref(protocol: &FeatureProfileProtocol) -> &'static str {
    match protocol {
        FeatureProfileProtocol::Invalid(FeatureProfileInvalidReason::StaleSubject) => {
            "WStaleSubject"
        }
        FeatureProfileProtocol::Init
        | FeatureProfileProtocol::NeedsHoles(_)
        | FeatureProfileProtocol::Resolved => "",
    }
}

fn protocol_holes(protocol: &FeatureProfileProtocol) -> &'static [&'static str] {
    match protocol {
        FeatureProfileProtocol::NeedsHoles(holes) => holes,
        FeatureProfileProtocol::Init
        | FeatureProfileProtocol::Resolved
        | FeatureProfileProtocol::Invalid(_) => &[],
    }
}

fn component_route_ref(route: &[RuleCoreComponentRouteEvent]) -> String {
    route
        .iter()
        .map(|event| match event {
            RuleCoreComponentRouteEvent::ParseInput(owner) => {
                format!("RuleCoreComponentParseInput({owner})")
            }
            RuleCoreComponentRouteEvent::AdmitInput(owner) => {
                format!("RuleCoreComponentAdmitInput({owner})")
            }
            RuleCoreComponentRouteEvent::Call(owner) => {
                format!("RuleCoreComponentCall({owner})")
            }
            RuleCoreComponentRouteEvent::ProjectResult(owner) => {
                format!("RuleCoreComponentProjectResult({owner})")
            }
        })
        .collect::<Vec<_>>()
        .join(">")
}

fn joined_or_none(values: &[&str]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}
