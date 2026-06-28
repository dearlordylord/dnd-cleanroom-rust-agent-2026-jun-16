use super::battle_runtime_reducer_route::{
    reducer_route_payload, route_discover_battle_acts_from_route_holes,
    route_resolve_battle_subject_from_route_holes,
    route_resolve_battle_subject_without_fill_from_route_holes, route_start_battle,
    ReducerRouteEvent, ReducerRouteFillKind, ReducerRouteHoleKind, ReducerRouteOwnerGroup,
    ReducerRouteSubjectFamily,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetamagicRouteStage {
    ResourceGovernorRouted,
    ResourceGovernorRejected,
    BonusActionCastingTimeRouted,
    PriorLevelOnePlusSpellRejected,
    SavingThrowProtectionRouted,
    SavingThrowRollModeRouted,
    DamageTypeSubstitutionRouted,
    EffectiveSpellLevelRouted,
    SpellRangeProjectionRouted,
    SpellDurationProjectionRouted,
    SpellComponentProjectionRouted,
    MissedSpellAttackRerollRouted,
    DamageDiceRerollRouted,
}

pub const BRANCH_ACTIONS: [&str; 13] = [
    "doRouteMetamagicResourceGovernor",
    "doRejectMetamagicResourceGovernor",
    "doRouteBonusActionCastingTime",
    "doRejectPriorLevelOnePlusSpell",
    "doRouteSavingThrowProtection",
    "doRouteSavingThrowRollMode",
    "doRouteDamageTypeSubstitution",
    "doRouteEffectiveSpellLevel",
    "doRouteSpellRangeProjection",
    "doRouteSpellDurationProjection",
    "doRouteSpellComponentProjection",
    "doRouteMissedSpellAttackReroll",
    "doRouteDamageDiceReroll",
];

pub fn replay_observed_action(observed_action_taken: &str) -> MetamagicRouteStage {
    stage_for_action(observed_action_taken)
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    expected_route(observed_action_taken)
}

pub fn expected_witness(observed_action_taken: &str) -> MetamagicRouteStage {
    stage_for_action(observed_action_taken)
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doRouteMetamagicResourceGovernor" | "doRejectMetamagicResourceGovernor" => {
            metamagic_resource_governor_route()
        }
        "doRouteBonusActionCastingTime" => route_bonus_action_casting_time(),
        "doRejectPriorLevelOnePlusSpell" => reject_prior_level_one_plus_spell_route(),
        "doRouteSavingThrowProtection" => route_saving_throw_protection(),
        "doRouteSavingThrowRollMode" => route_saving_throw_roll_mode(),
        "doRouteDamageTypeSubstitution" => route_damage_type_substitution(),
        "doRouteEffectiveSpellLevel" => route_effective_spell_level(),
        "doRouteSpellRangeProjection" => route_spell_range_projection(),
        "doRouteSpellDurationProjection" => route_spell_duration_projection(),
        "doRouteSpellComponentProjection" => route_spell_component_projection(),
        "doRouteMissedSpellAttackReroll" => route_missed_spell_attack_reroll(),
        "doRouteDamageDiceReroll" => route_damage_dice_reroll(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn projection_payload(stage: MetamagicRouteStage, route: &[ReducerRouteEvent]) -> String {
    format!(
        "qStage={}\n{}",
        stage_ref(stage),
        reducer_route_payload(route)
    )
}

fn stage_for_action(observed_action_taken: &str) -> MetamagicRouteStage {
    match observed_action_taken {
        "doRouteMetamagicResourceGovernor" => MetamagicRouteStage::ResourceGovernorRouted,
        "doRejectMetamagicResourceGovernor" => MetamagicRouteStage::ResourceGovernorRejected,
        "doRouteBonusActionCastingTime" => MetamagicRouteStage::BonusActionCastingTimeRouted,
        "doRejectPriorLevelOnePlusSpell" => MetamagicRouteStage::PriorLevelOnePlusSpellRejected,
        "doRouteSavingThrowProtection" => MetamagicRouteStage::SavingThrowProtectionRouted,
        "doRouteSavingThrowRollMode" => MetamagicRouteStage::SavingThrowRollModeRouted,
        "doRouteDamageTypeSubstitution" => MetamagicRouteStage::DamageTypeSubstitutionRouted,
        "doRouteEffectiveSpellLevel" => MetamagicRouteStage::EffectiveSpellLevelRouted,
        "doRouteSpellRangeProjection" => MetamagicRouteStage::SpellRangeProjectionRouted,
        "doRouteSpellDurationProjection" => MetamagicRouteStage::SpellDurationProjectionRouted,
        "doRouteSpellComponentProjection" => MetamagicRouteStage::SpellComponentProjectionRouted,
        "doRouteMissedSpellAttackReroll" => MetamagicRouteStage::MissedSpellAttackRerollRouted,
        "doRouteDamageDiceReroll" => MetamagicRouteStage::DamageDiceRerollRouted,
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

fn initial_route() -> Vec<ReducerRouteEvent> {
    vec![route_start_battle(
        ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
    )]
}

fn metamagic_resource_governor_route() -> Vec<ReducerRouteEvent> {
    let subject = ReducerRouteSubjectFamily::MetamagicSpellGovernor;
    initial_route()
        .into_iter()
        .chain([
            discover(subject, no_holes(), ReducerRouteOwnerGroup::FeatureResource),
            resolve_without_fill(subject, no_holes(), ReducerRouteOwnerGroup::FeatureResource),
        ])
        .collect()
}

fn route_bonus_action_casting_time() -> Vec<ReducerRouteEvent> {
    let subject = ReducerRouteSubjectFamily::MetamagicBonusActionCastingTime;
    initial_route()
        .into_iter()
        .chain([
            discover(
                subject,
                target_choice_holes(),
                ReducerRouteOwnerGroup::FeatureResource,
            ),
            resolve_without_fill(
                subject,
                target_choice_holes(),
                ReducerRouteOwnerGroup::ActionEconomy,
            ),
            resolve_without_fill(
                subject,
                target_choice_holes(),
                ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
            ),
            resolve(
                subject,
                ReducerRouteFillKind::TargetChoice,
                attack_roll_holes(),
                ReducerRouteOwnerGroup::TargetSelection,
            ),
            resolve(
                subject,
                ReducerRouteFillKind::AttackRoll,
                damage_roll_holes(),
                ReducerRouteOwnerGroup::SpellAttackProcedure,
            ),
            resolve_without_fill(subject, no_holes(), ReducerRouteOwnerGroup::TurnBoundary),
        ])
        .collect()
}

fn reject_prior_level_one_plus_spell_route() -> Vec<ReducerRouteEvent> {
    let subject = ReducerRouteSubjectFamily::MetamagicBonusActionCastingTime;
    initial_route()
        .into_iter()
        .chain([
            discover(subject, no_holes(), ReducerRouteOwnerGroup::FeatureResource),
            resolve_without_fill(subject, no_holes(), ReducerRouteOwnerGroup::TurnBoundary),
        ])
        .collect()
}

fn route_saving_throw_protection() -> Vec<ReducerRouteEvent> {
    let subject = ReducerRouteSubjectFamily::MetamagicSavingThrowProtection;
    initial_route()
        .into_iter()
        .chain([
            discover(
                subject,
                saving_throw_holes(),
                ReducerRouteOwnerGroup::FeatureResource,
            ),
            resolve(
                subject,
                ReducerRouteFillKind::SavingThrowOutcome,
                damage_roll_holes(),
                ReducerRouteOwnerGroup::SavingThrowOutcome,
            ),
            resolve_without_fill(
                subject,
                damage_roll_holes(),
                ReducerRouteOwnerGroup::DamageAdjustment,
            ),
            resolve_without_fill(subject, no_holes(), ReducerRouteOwnerGroup::FeatureResource),
        ])
        .collect()
}

fn route_saving_throw_roll_mode() -> Vec<ReducerRouteEvent> {
    let subject = ReducerRouteSubjectFamily::MetamagicSavingThrowRollMode;
    initial_route()
        .into_iter()
        .chain([
            discover(
                subject,
                saving_throw_holes(),
                ReducerRouteOwnerGroup::FeatureResource,
            ),
            resolve_without_fill(
                subject,
                saving_throw_holes(),
                ReducerRouteOwnerGroup::SavingThrowRollMode,
            ),
            resolve(
                subject,
                ReducerRouteFillKind::SavingThrowOutcome,
                no_holes(),
                ReducerRouteOwnerGroup::SavingThrowOutcome,
            ),
            resolve_without_fill(
                subject,
                no_holes(),
                ReducerRouteOwnerGroup::ConditionLifecycle,
            ),
        ])
        .collect()
}

fn route_damage_type_substitution() -> Vec<ReducerRouteEvent> {
    let subject = ReducerRouteSubjectFamily::MetamagicDamageTypeSubstitution;
    initial_route()
        .into_iter()
        .chain([
            discover(
                subject,
                damage_type_holes(),
                ReducerRouteOwnerGroup::FeatureResource,
            ),
            resolve(
                subject,
                ReducerRouteFillKind::DamageTypeChoice,
                damage_roll_holes(),
                ReducerRouteOwnerGroup::DamageType,
            ),
            resolve(
                subject,
                ReducerRouteFillKind::RolledDice,
                no_holes(),
                ReducerRouteOwnerGroup::DamageRoll,
            ),
            resolve_without_fill(subject, no_holes(), ReducerRouteOwnerGroup::HitPoint),
        ])
        .collect()
}

fn route_effective_spell_level() -> Vec<ReducerRouteEvent> {
    let subject = ReducerRouteSubjectFamily::MetamagicEffectiveSpellLevel;
    initial_route()
        .into_iter()
        .chain([
            discover(
                subject,
                target_list_holes(),
                ReducerRouteOwnerGroup::FeatureResource,
            ),
            resolve_without_fill(
                subject,
                target_list_holes(),
                ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
            ),
            resolve(
                subject,
                ReducerRouteFillKind::SpellTargetList,
                no_holes(),
                ReducerRouteOwnerGroup::TargetSelection,
            ),
        ])
        .collect()
}

fn route_spell_range_projection() -> Vec<ReducerRouteEvent> {
    let subject = ReducerRouteSubjectFamily::MetamagicSpellRangeProjection;
    initial_route()
        .into_iter()
        .chain([
            discover(
                subject,
                target_choice_holes(),
                ReducerRouteOwnerGroup::FeatureResource,
            ),
            resolve_without_fill(
                subject,
                target_choice_holes(),
                ReducerRouteOwnerGroup::ObjectTargetBoundary,
            ),
            resolve(
                subject,
                ReducerRouteFillKind::TargetChoice,
                no_holes(),
                ReducerRouteOwnerGroup::TargetSelection,
            ),
        ])
        .collect()
}

fn route_spell_duration_projection() -> Vec<ReducerRouteEvent> {
    let subject = ReducerRouteSubjectFamily::MetamagicSpellDurationProjection;
    initial_route()
        .into_iter()
        .chain([
            discover(subject, no_holes(), ReducerRouteOwnerGroup::FeatureResource),
            resolve_without_fill(subject, no_holes(), ReducerRouteOwnerGroup::ActiveEffect),
            resolve_without_fill(subject, no_holes(), ReducerRouteOwnerGroup::Concentration),
        ])
        .collect()
}

fn route_spell_component_projection() -> Vec<ReducerRouteEvent> {
    let subject = ReducerRouteSubjectFamily::MetamagicSpellComponentProjection;
    initial_route()
        .into_iter()
        .chain([
            discover(subject, no_holes(), ReducerRouteOwnerGroup::FeatureResource),
            resolve_without_fill(
                subject,
                no_holes(),
                ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
            ),
        ])
        .collect()
}

fn route_missed_spell_attack_reroll() -> Vec<ReducerRouteEvent> {
    let subject = ReducerRouteSubjectFamily::MetamagicMissedSpellAttackReroll;
    initial_route()
        .into_iter()
        .chain([
            discover(
                subject,
                attack_roll_holes(),
                ReducerRouteOwnerGroup::FeatureResource,
            ),
            resolve(
                subject,
                ReducerRouteFillKind::AttackRoll,
                attack_roll_holes(),
                ReducerRouteOwnerGroup::AttackRoll,
            ),
            resolve(
                subject,
                ReducerRouteFillKind::AttackRoll,
                damage_roll_holes(),
                ReducerRouteOwnerGroup::AttackRoll,
            ),
            resolve_without_fill(subject, no_holes(), ReducerRouteOwnerGroup::FeatureResource),
        ])
        .collect()
}

fn route_damage_dice_reroll() -> Vec<ReducerRouteEvent> {
    let subject = ReducerRouteSubjectFamily::MetamagicDamageDiceReroll;
    initial_route()
        .into_iter()
        .chain([
            discover(
                subject,
                damage_roll_holes(),
                ReducerRouteOwnerGroup::FeatureResource,
            ),
            resolve(
                subject,
                ReducerRouteFillKind::RolledDice,
                damage_roll_holes(),
                ReducerRouteOwnerGroup::DamageRoll,
            ),
            resolve(
                subject,
                ReducerRouteFillKind::RolledDice,
                no_holes(),
                ReducerRouteOwnerGroup::DamageRoll,
            ),
            resolve_without_fill(subject, no_holes(), ReducerRouteOwnerGroup::HitPoint),
        ])
        .collect()
}

fn discover(
    subject: ReducerRouteSubjectFamily,
    holes: Vec<ReducerRouteHoleKind>,
    owner: ReducerRouteOwnerGroup,
) -> ReducerRouteEvent {
    route_discover_battle_acts_from_route_holes(subject, holes, owner)
}

fn resolve(
    subject: ReducerRouteSubjectFamily,
    fill: ReducerRouteFillKind,
    holes: Vec<ReducerRouteHoleKind>,
    owner: ReducerRouteOwnerGroup,
) -> ReducerRouteEvent {
    route_resolve_battle_subject_from_route_holes(subject, fill, holes, owner)
}

fn resolve_without_fill(
    subject: ReducerRouteSubjectFamily,
    holes: Vec<ReducerRouteHoleKind>,
    owner: ReducerRouteOwnerGroup,
) -> ReducerRouteEvent {
    route_resolve_battle_subject_without_fill_from_route_holes(subject, holes, owner)
}

fn no_holes() -> Vec<ReducerRouteHoleKind> {
    Vec::new()
}

fn attack_roll_holes() -> Vec<ReducerRouteHoleKind> {
    vec![ReducerRouteHoleKind::AttackRoll]
}

fn damage_roll_holes() -> Vec<ReducerRouteHoleKind> {
    vec![ReducerRouteHoleKind::RolledDice]
}

fn damage_type_holes() -> Vec<ReducerRouteHoleKind> {
    vec![ReducerRouteHoleKind::DamageTypeChoice]
}

fn saving_throw_holes() -> Vec<ReducerRouteHoleKind> {
    vec![ReducerRouteHoleKind::SavingThrowOutcome]
}

fn target_choice_holes() -> Vec<ReducerRouteHoleKind> {
    vec![ReducerRouteHoleKind::TargetChoice]
}

fn target_list_holes() -> Vec<ReducerRouteHoleKind> {
    vec![ReducerRouteHoleKind::SpellTargetList]
}

fn stage_ref(stage: MetamagicRouteStage) -> &'static str {
    match stage {
        MetamagicRouteStage::ResourceGovernorRouted => "ResourceGovernorRouted",
        MetamagicRouteStage::ResourceGovernorRejected => "ResourceGovernorRejected",
        MetamagicRouteStage::BonusActionCastingTimeRouted => "BonusActionCastingTimeRouted",
        MetamagicRouteStage::PriorLevelOnePlusSpellRejected => "PriorLevelOnePlusSpellRejected",
        MetamagicRouteStage::SavingThrowProtectionRouted => "SavingThrowProtectionRouted",
        MetamagicRouteStage::SavingThrowRollModeRouted => "SavingThrowRollModeRouted",
        MetamagicRouteStage::DamageTypeSubstitutionRouted => "DamageTypeSubstitutionRouted",
        MetamagicRouteStage::EffectiveSpellLevelRouted => "EffectiveSpellLevelRouted",
        MetamagicRouteStage::SpellRangeProjectionRouted => "SpellRangeProjectionRouted",
        MetamagicRouteStage::SpellDurationProjectionRouted => "SpellDurationProjectionRouted",
        MetamagicRouteStage::SpellComponentProjectionRouted => "SpellComponentProjectionRouted",
        MetamagicRouteStage::MissedSpellAttackRerollRouted => "MissedSpellAttackRerollRouted",
        MetamagicRouteStage::DamageDiceRerollRouted => "DamageDiceRerollRouted",
    }
}
