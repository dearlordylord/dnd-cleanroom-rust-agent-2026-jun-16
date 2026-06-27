#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeatureActionGrant {
    NoActionSurgeActionGrant,
    ActionSurgeActionAvailable,
    ActionSurgeActionSpent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeatureProfileProtocol {
    Init,
    NeedsHoles(&'static [&'static str]),
    Resolved,
    Invalid(FeatureProfileInvalidReason),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
const DAMAGE_ROLL_HOLE: &[&str] = &["DamageRoll"];
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
    expected_witness(observed_action_taken)
}

pub fn expected_witness(observed_action_taken: &str) -> RuleCoreFeatureProfileState {
    match observed_action_taken {
        "doActionSurgeActivate" => base_projection()
            .with_feature_uses(0)
            .with_action_surge_grant(FeatureActionGrant::ActionSurgeActionAvailable)
            .resolved(),
        "doActionSurgeSpendAttack" => base_projection()
            .with_action(false)
            .with_feature_uses(0)
            .with_action_surge_grant(FeatureActionGrant::ActionSurgeActionSpent)
            .with_target_hit_points(5)
            .with_last_damage_amount(7)
            .resolved(),
        "doActionSurgeRejectTwice" => base_projection()
            .with_feature_uses(0)
            .with_action_surge_grant(FeatureActionGrant::ActionSurgeActionAvailable)
            .invalid(FeatureProfileInvalidReason::StaleSubject),
        "doDiscoverSecondWind" => base_projection()
            .with_actor_hit_points(4)
            .with_protocol(FeatureProfileProtocol::NeedsHoles(DAMAGE_ROLL_HOLE)),
        "doResolveSecondWindLow" => base_projection()
            .with_bonus_action(false)
            .with_feature_uses(0)
            .with_actor_hit_points(7)
            .with_last_damage_amount(3)
            .resolved(),
        "doResolveSecondWindHigh" => base_projection()
            .with_bonus_action(false)
            .with_feature_uses(0)
            .with_actor_hit_points(12)
            .with_last_damage_amount(8)
            .resolved(),
        "doTacticalMindConvertedSuccess" => base_projection()
            .with_feature_uses(0)
            .with_ability_check(16, true)
            .resolved(),
        "doTacticalMindStillFailed" => base_projection().with_ability_check(14, false).resolved(),
        "doCunningDash" => base_projection()
            .with_bonus_action(false)
            .with_dash_bonus_feet(30)
            .resolved(),
        "doCunningDisengage" => base_projection()
            .with_bonus_action(false)
            .with_disengaged(true)
            .resolved(),
        "doCunningHide" => base_projection()
            .with_bonus_action(false)
            .with_hidden(true)
            .resolved(),
        "doRageActivateAndDamage" => base_projection()
            .with_action(false)
            .with_bonus_action(false)
            .with_feature_uses(0)
            .with_target_hit_points(5)
            .with_rage(true)
            .with_last_damage_amount(7)
            .resolved(),
        "doRecklessAttack" => base_projection()
            .with_action(false)
            .with_target_hit_points(5)
            .with_reckless(true)
            .with_incoming_attack_advantage(true)
            .with_last_damage_amount(7)
            .resolved(),
        "doSneakAttack" => base_projection()
            .with_action(false)
            .with_target_hit_points(2)
            .with_sneak_attack_used(true)
            .with_last_damage_amount(1)
            .resolved(),
        "doFrenzy" => base_projection()
            .with_action(false)
            .with_bonus_action(false)
            .with_feature_uses(0)
            .with_target_hit_points(7)
            .with_rage(true)
            .with_reckless(true)
            .with_incoming_attack_advantage(true)
            .with_last_damage_amount(2)
            .resolved(),
        "doImprovedCritical" => base_projection()
            .with_action(false)
            .with_target_hit_points(4)
            .with_last_damage_amount(8)
            .with_critical(true)
            .resolved(),
        "doCuttingWordsDamage" => base_projection()
            .with_action(false)
            .with_reaction(false)
            .with_feature_uses(0)
            .with_actor_hit_points(10)
            .with_last_damage_amount(2)
            .resolved(),
        "doDeflectAttacksDamageReduction" => base_projection()
            .with_action(false)
            .with_reaction(false)
            .with_actor_hit_points(10)
            .with_last_damage_amount(2)
            .resolved(),
        "doDefenseArmorClass" => base_projection().with_actor_armor_class(17).resolved(),
        "doArcheryAttackRollBonus" => base_projection()
            .with_action(false)
            .with_last_damage_amount(2)
            .with_actor_armor_class(9)
            .resolved(),
        "doSavageAttackerDamage" => base_projection()
            .with_action(false)
            .with_target_hit_points(4)
            .with_last_damage_amount(8)
            .resolved(),
        "doZeroHitPointReplacement" => base_projection()
            .with_action(false)
            .with_feature_uses(0)
            .with_target_hit_points(1)
            .with_last_damage_amount(1)
            .resolved(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

fn base_projection() -> RuleCoreFeatureProfileState {
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
        component_route: rule_core_feature_profile_component_route(),
    }
}

fn rule_core_feature_profile_component_route() -> Vec<RuleCoreComponentRouteEvent> {
    vec![
        RuleCoreComponentRouteEvent::ParseInput(FEATURE_OWNER),
        RuleCoreComponentRouteEvent::AdmitInput(FEATURE_OWNER),
        RuleCoreComponentRouteEvent::Call(FEATURE_OWNER),
        RuleCoreComponentRouteEvent::ProjectResult(FEATURE_OWNER),
    ]
}

impl RuleCoreFeatureProfileState {
    fn with_action(mut self, value: bool) -> Self {
        self.action_available = value;
        self
    }

    fn with_bonus_action(mut self, value: bool) -> Self {
        self.bonus_action_available = value;
        self
    }

    fn with_reaction(mut self, value: bool) -> Self {
        self.reaction_available = value;
        self
    }

    fn with_feature_uses(mut self, value: i16) -> Self {
        self.feature_uses_remaining = value;
        self
    }

    fn with_action_surge_grant(mut self, value: FeatureActionGrant) -> Self {
        self.action_surge_grant = value;
        self
    }

    fn with_actor_hit_points(mut self, value: i16) -> Self {
        self.actor_hit_points = value;
        self
    }

    fn with_target_hit_points(mut self, value: i16) -> Self {
        self.target_hit_points = value;
        self
    }

    fn with_dash_bonus_feet(mut self, value: i16) -> Self {
        self.dash_bonus_feet = value;
        self
    }

    fn with_disengaged(mut self, value: bool) -> Self {
        self.disengaged = value;
        self
    }

    fn with_hidden(mut self, value: bool) -> Self {
        self.hidden = value;
        self
    }

    fn with_rage(mut self, value: bool) -> Self {
        self.rage_active = value;
        self
    }

    fn with_reckless(mut self, value: bool) -> Self {
        self.reckless_active = value;
        self
    }

    fn with_incoming_attack_advantage(mut self, value: bool) -> Self {
        self.incoming_attack_advantage = value;
        self
    }

    fn with_sneak_attack_used(mut self, value: bool) -> Self {
        self.sneak_attack_used_this_turn = value;
        self
    }

    fn with_last_damage_amount(mut self, value: i16) -> Self {
        self.last_damage_amount = value;
        self
    }

    fn with_ability_check(mut self, total: i16, succeeded: bool) -> Self {
        self.ability_check_boosted_total = total;
        self.ability_check_boosted_succeeded = succeeded;
        self
    }

    fn with_actor_armor_class(mut self, value: i16) -> Self {
        self.actor_armor_class = value;
        self
    }

    fn with_critical(mut self, value: bool) -> Self {
        self.critical = value;
        self
    }

    fn with_protocol(mut self, value: FeatureProfileProtocol) -> Self {
        self.protocol = value;
        self
    }

    fn resolved(self) -> Self {
        self.with_protocol(FeatureProfileProtocol::Resolved)
    }

    fn invalid(self, reason: FeatureProfileInvalidReason) -> Self {
        self.with_protocol(FeatureProfileProtocol::Invalid(reason))
    }
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
