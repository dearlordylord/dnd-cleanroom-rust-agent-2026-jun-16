use crate::rules::battle_reducer_spine::{
    discover_generic_route_subject_observed, generic_route_subject_for_current_actor,
    resolve_battle_subject_observed, start_battle_observed, BattleEntrypointTrace,
    BattleGenericRouteFill, BattleResolutionRequest, BattleSetup, BattleState, BattleSubjectKind,
};

use super::battle_runtime_reducer_route::{
    observed_reducer_route, route_discover_battle_acts_from_route_holes,
    route_resolve_battle_subject_from_route_holes,
    route_resolve_battle_subject_without_fill_from_route_holes, route_start_battle,
    ReducerRouteEvent, ReducerRouteFillKind, ReducerRouteHoleKind, ReducerRouteOwnerGroup,
    ReducerRouteSubjectFamily,
};

pub const CONNECTOR_ACTIONS: [&str; 7] = [
    "doRouteAreaSavingThrowMixedOutcomes",
    "doRouteAttackHitBurstSavingThrowMixedOutcomes",
    "doRouteAttackMissBurstSavingThrowMixedOutcomes",
    "doRouteObjectAttackSecondaryProjection",
    "doRouteAttackHitConditionProjection",
    "doRouteSaveFailureNextAttackProjection",
    "doRouteChainedAttackMixedTargetOutcomes",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MixedTargetOutcomeRouteTargetFact {
    Primary,
    Secondary,
    Object,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MixedTargetOutcomeResolutionFact {
    AttackHit,
    AttackMiss,
    SavingThrowFailed,
    SavingThrowSucceeded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MixedTargetOutcomeDamageProjectionFact {
    Attack,
    FullSavingThrow,
    HalfSavingThrow,
    NoDamage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MixedTargetOutcomeSharedSpendFact {
    SharedSpellSlotSpend,
    SharedCantripActionSpend,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MixedTargetOutcomeSharedDamageRollFact {
    SharedSavingThrowDamageRoll,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MixedTargetOutcomeSecondaryProjectionFact {
    ObjectBoundary,
    Light,
    Condition,
    NextAttackRollMode,
    BurstSavingThrow,
    ChainedAttackTarget,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MixedTargetOutcomeRouteFact {
    Target(MixedTargetOutcomeRouteTargetFact),
    Resolution {
        target: MixedTargetOutcomeRouteTargetFact,
        outcome: MixedTargetOutcomeResolutionFact,
    },
    DamageProjection {
        target: MixedTargetOutcomeRouteTargetFact,
        damage: MixedTargetOutcomeDamageProjectionFact,
    },
    SharedSpend(MixedTargetOutcomeSharedSpendFact),
    SharedDamageRoll(MixedTargetOutcomeSharedDamageRollFact),
    SecondaryProjection {
        target: MixedTargetOutcomeRouteTargetFact,
        projection: MixedTargetOutcomeSecondaryProjectionFact,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MixedTargetOutcomeRouteWitness {
    pub surface: &'static str,
    pub facts: Vec<MixedTargetOutcomeRouteFact>,
}

pub fn replay_observed_action(observed_action_taken: &str) -> MixedTargetOutcomeRouteWitness {
    replay_observed_state_and_route(observed_action_taken).0
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    replay_observed_state_and_route(observed_action_taken).1
}

pub fn expected_witness(observed_action_taken: &str) -> MixedTargetOutcomeRouteWitness {
    MixedTargetOutcomeRouteWitness {
        surface: surface_ref(observed_action_taken),
        facts: expected_facts(observed_action_taken),
    }
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    let mut route = vec![route_start_battle(ReducerRouteOwnerGroup::ActionEconomy)];
    route.extend(expected_route_suffix(observed_action_taken));
    route
}

pub fn projection_payload(witness: &MixedTargetOutcomeRouteWitness) -> String {
    [
        format!("surface={}", witness.surface),
        format!("facts={}", facts_payload(&witness.facts)),
    ]
    .join("\n")
}

fn replay_observed_state_and_route(
    observed_action_taken: &str,
) -> (MixedTargetOutcomeRouteWitness, Vec<ReducerRouteEvent>) {
    let mut trace = BattleEntrypointTrace::default();
    let state = start_battle_observed(BattleSetup::standard(), &mut trace).state;
    let _state = match observed_action_taken {
        "doRouteAreaSavingThrowMixedOutcomes" => replay_route_steps(
            state,
            &mut trace,
            &[
                RouteStep::Discover(BattleSubjectKind::MixedTargetOutcomeInitialSavingThrow),
                RouteStep::Resolve(
                    BattleSubjectKind::MixedTargetOutcomeInitialSavingThrow,
                    BattleGenericRouteFill::SavingThrowOutcome,
                ),
                RouteStep::Resolve(
                    BattleSubjectKind::MixedTargetOutcomeDamageRoll,
                    BattleGenericRouteFill::RolledDice,
                ),
            ],
        ),
        "doRouteAttackHitBurstSavingThrowMixedOutcomes"
        | "doRouteAttackMissBurstSavingThrowMixedOutcomes" => replay_route_steps(
            state,
            &mut trace,
            &[
                RouteStep::Discover(BattleSubjectKind::MixedTargetOutcomeInitialTargetChoice),
                RouteStep::Resolve(
                    BattleSubjectKind::MixedTargetOutcomeInitialTargetChoice,
                    BattleGenericRouteFill::TargetChoice,
                ),
                RouteStep::Resolve(
                    BattleSubjectKind::MixedTargetOutcomeAttackRollToSavingThrow,
                    BattleGenericRouteFill::AttackRoll,
                ),
                RouteStep::Resolve(
                    BattleSubjectKind::MixedTargetOutcomeSavingThrowToDamage,
                    BattleGenericRouteFill::SavingThrowOutcome,
                ),
                RouteStep::Resolve(
                    BattleSubjectKind::MixedTargetOutcomeDamageRoll,
                    BattleGenericRouteFill::RolledDice,
                ),
            ],
        ),
        "doRouteObjectAttackSecondaryProjection" | "doRouteAttackHitConditionProjection" => {
            replay_route_steps(
                state,
                &mut trace,
                &[
                    RouteStep::Discover(BattleSubjectKind::MixedTargetOutcomeInitialTargetChoice),
                    RouteStep::Resolve(
                        BattleSubjectKind::MixedTargetOutcomeInitialTargetChoice,
                        BattleGenericRouteFill::TargetChoice,
                    ),
                    RouteStep::Resolve(
                        BattleSubjectKind::MixedTargetOutcomeAttackRollToDamage,
                        BattleGenericRouteFill::AttackRoll,
                    ),
                    RouteStep::Resolve(
                        BattleSubjectKind::MixedTargetOutcomeDamageRoll,
                        BattleGenericRouteFill::RolledDice,
                    ),
                    RouteStep::Resolve(
                        BattleSubjectKind::MixedTargetOutcomeProjection,
                        BattleGenericRouteFill::WithoutFill,
                    ),
                ],
            )
        }
        "doRouteSaveFailureNextAttackProjection" => replay_route_steps(
            state,
            &mut trace,
            &[
                RouteStep::Discover(BattleSubjectKind::MixedTargetOutcomeInitialTargetChoice),
                RouteStep::Resolve(
                    BattleSubjectKind::MixedTargetOutcomeTargetChoiceToSavingThrow,
                    BattleGenericRouteFill::TargetChoice,
                ),
                RouteStep::Resolve(
                    BattleSubjectKind::MixedTargetOutcomeSavingThrowToDamage,
                    BattleGenericRouteFill::SavingThrowOutcome,
                ),
                RouteStep::Resolve(
                    BattleSubjectKind::MixedTargetOutcomeDamageRoll,
                    BattleGenericRouteFill::RolledDice,
                ),
                RouteStep::Resolve(
                    BattleSubjectKind::MixedTargetOutcomeProjection,
                    BattleGenericRouteFill::WithoutFill,
                ),
            ],
        ),
        "doRouteChainedAttackMixedTargetOutcomes" => replay_route_steps(
            state,
            &mut trace,
            &[
                RouteStep::Discover(BattleSubjectKind::MixedTargetOutcomeInitialTargetChoice),
                RouteStep::Resolve(
                    BattleSubjectKind::MixedTargetOutcomeInitialTargetChoice,
                    BattleGenericRouteFill::TargetChoice,
                ),
                RouteStep::Resolve(
                    BattleSubjectKind::MixedTargetOutcomeAttackRollToDamage,
                    BattleGenericRouteFill::AttackRoll,
                ),
                RouteStep::Resolve(
                    BattleSubjectKind::MixedTargetOutcomeRolledDiceToTargetChoice,
                    BattleGenericRouteFill::RolledDice,
                ),
                RouteStep::Resolve(
                    BattleSubjectKind::MixedTargetOutcomeDamageRollToTargetChoice,
                    BattleGenericRouteFill::TargetChoice,
                ),
                RouteStep::Resolve(
                    BattleSubjectKind::MixedTargetOutcomeAttackRollToDamage,
                    BattleGenericRouteFill::AttackRoll,
                ),
                RouteStep::Resolve(
                    BattleSubjectKind::MixedTargetOutcomeDamageRoll,
                    BattleGenericRouteFill::RolledDice,
                ),
            ],
        ),
        action => panic!("unsupported mixed-target route action {action}"),
    };
    (
        MixedTargetOutcomeRouteWitness {
            surface: surface_ref(observed_action_taken),
            facts: expected_facts(observed_action_taken),
        },
        observed_reducer_route(
            &trace,
            &[ReducerRouteSubjectFamily::MixedTargetOutcomeSpell],
        ),
    )
}

#[derive(Debug, Clone, Copy)]
enum RouteStep {
    Discover(BattleSubjectKind),
    Resolve(BattleSubjectKind, BattleGenericRouteFill),
}

fn replay_route_steps(
    mut state: BattleState,
    trace: &mut BattleEntrypointTrace,
    steps: &[RouteStep],
) -> BattleState {
    for step in steps {
        match *step {
            RouteStep::Discover(kind) => {
                state = discover_generic_route_subject_observed(state, kind, trace).0;
            }
            RouteStep::Resolve(kind, fill) => {
                let subject = generic_route_subject_for_current_actor(&state, kind);
                state = resolve_battle_subject_observed(
                    state,
                    BattleResolutionRequest::generic_route(subject, fill)
                        .expect("mixed-target route subject should accept generic fill"),
                    trace,
                )
                .into_state();
            }
        }
    }
    state
}

fn expected_route_suffix(action: &str) -> Vec<ReducerRouteEvent> {
    match action {
        "doRouteAreaSavingThrowMixedOutcomes" => vec![
            discover(saving_throw_holes()),
            resolve(
                ReducerRouteFillKind::SavingThrowOutcome,
                rolled_dice_holes(),
            ),
            resolve(ReducerRouteFillKind::RolledDice, Vec::new()),
        ],
        "doRouteAttackHitBurstSavingThrowMixedOutcomes"
        | "doRouteAttackMissBurstSavingThrowMixedOutcomes" => vec![
            discover(target_choice_holes()),
            resolve(ReducerRouteFillKind::TargetChoice, attack_roll_holes()),
            resolve(ReducerRouteFillKind::AttackRoll, saving_throw_holes()),
            resolve(
                ReducerRouteFillKind::SavingThrowOutcome,
                rolled_dice_holes(),
            ),
            resolve(ReducerRouteFillKind::RolledDice, Vec::new()),
        ],
        "doRouteObjectAttackSecondaryProjection" | "doRouteAttackHitConditionProjection" => vec![
            discover(target_choice_holes()),
            resolve(ReducerRouteFillKind::TargetChoice, attack_roll_holes()),
            resolve(ReducerRouteFillKind::AttackRoll, rolled_dice_holes()),
            resolve(ReducerRouteFillKind::RolledDice, Vec::new()),
            resolve_without_fill(Vec::new()),
        ],
        "doRouteSaveFailureNextAttackProjection" => vec![
            discover(target_choice_holes()),
            resolve(ReducerRouteFillKind::TargetChoice, saving_throw_holes()),
            resolve(
                ReducerRouteFillKind::SavingThrowOutcome,
                rolled_dice_holes(),
            ),
            resolve(ReducerRouteFillKind::RolledDice, Vec::new()),
            resolve_without_fill(Vec::new()),
        ],
        "doRouteChainedAttackMixedTargetOutcomes" => vec![
            discover(target_choice_holes()),
            resolve(ReducerRouteFillKind::TargetChoice, attack_roll_holes()),
            resolve(ReducerRouteFillKind::AttackRoll, rolled_dice_holes()),
            resolve(ReducerRouteFillKind::RolledDice, target_choice_holes()),
            resolve(ReducerRouteFillKind::TargetChoice, attack_roll_holes()),
            resolve(ReducerRouteFillKind::AttackRoll, rolled_dice_holes()),
            resolve(ReducerRouteFillKind::RolledDice, Vec::new()),
        ],
        action => panic!("unsupported mixed-target route action {action}"),
    }
}

fn discover(holes: Vec<ReducerRouteHoleKind>) -> ReducerRouteEvent {
    route_discover_battle_acts_from_route_holes(
        ReducerRouteSubjectFamily::MixedTargetOutcomeSpell,
        holes,
        ReducerRouteOwnerGroup::SpellInvocation,
    )
}

fn resolve(fill: ReducerRouteFillKind, holes: Vec<ReducerRouteHoleKind>) -> ReducerRouteEvent {
    route_resolve_battle_subject_from_route_holes(
        ReducerRouteSubjectFamily::MixedTargetOutcomeSpell,
        fill,
        holes,
        ReducerRouteOwnerGroup::SpellInvocation,
    )
}

fn resolve_without_fill(holes: Vec<ReducerRouteHoleKind>) -> ReducerRouteEvent {
    route_resolve_battle_subject_without_fill_from_route_holes(
        ReducerRouteSubjectFamily::MixedTargetOutcomeSpell,
        holes,
        ReducerRouteOwnerGroup::SpellInvocation,
    )
}

fn target_choice_holes() -> Vec<ReducerRouteHoleKind> {
    vec![ReducerRouteHoleKind::TargetChoice]
}

fn attack_roll_holes() -> Vec<ReducerRouteHoleKind> {
    vec![ReducerRouteHoleKind::AttackRoll]
}

fn saving_throw_holes() -> Vec<ReducerRouteHoleKind> {
    vec![ReducerRouteHoleKind::SavingThrowOutcome]
}

fn rolled_dice_holes() -> Vec<ReducerRouteHoleKind> {
    vec![ReducerRouteHoleKind::RolledDice]
}

fn expected_facts(action: &str) -> Vec<MixedTargetOutcomeRouteFact> {
    use MixedTargetOutcomeDamageProjectionFact::{
        Attack, FullSavingThrow, HalfSavingThrow, NoDamage,
    };
    use MixedTargetOutcomeResolutionFact::{
        AttackHit, AttackMiss, SavingThrowFailed, SavingThrowSucceeded,
    };
    use MixedTargetOutcomeRouteFact::{
        DamageProjection, Resolution, SecondaryProjection, SharedDamageRoll, SharedSpend, Target,
    };
    use MixedTargetOutcomeRouteTargetFact::{Object, Primary, Secondary};
    use MixedTargetOutcomeSecondaryProjectionFact::{
        ChainedAttackTarget, Condition, Light, NextAttackRollMode, ObjectBoundary,
    };
    use MixedTargetOutcomeSharedDamageRollFact::SharedSavingThrowDamageRoll;
    use MixedTargetOutcomeSharedSpendFact::{SharedCantripActionSpend, SharedSpellSlotSpend};

    match action {
        "doRouteAreaSavingThrowMixedOutcomes" => vec![
            SharedSpend(SharedSpellSlotSpend),
            SharedDamageRoll(SharedSavingThrowDamageRoll),
            Target(Primary),
            Target(Secondary),
            Resolution {
                target: Primary,
                outcome: SavingThrowFailed,
            },
            Resolution {
                target: Secondary,
                outcome: SavingThrowSucceeded,
            },
            DamageProjection {
                target: Primary,
                damage: FullSavingThrow,
            },
            DamageProjection {
                target: Secondary,
                damage: HalfSavingThrow,
            },
        ],
        "doRouteAttackHitBurstSavingThrowMixedOutcomes" => attack_burst_facts(AttackHit, Attack),
        "doRouteAttackMissBurstSavingThrowMixedOutcomes" => {
            attack_burst_facts(AttackMiss, NoDamage)
        }
        "doRouteObjectAttackSecondaryProjection" => vec![
            SharedSpend(SharedCantripActionSpend),
            Target(Object),
            Resolution {
                target: Object,
                outcome: AttackHit,
            },
            DamageProjection {
                target: Object,
                damage: Attack,
            },
            SecondaryProjection {
                target: Object,
                projection: ObjectBoundary,
            },
            SecondaryProjection {
                target: Object,
                projection: Light,
            },
        ],
        "doRouteAttackHitConditionProjection" => vec![
            SharedSpend(SharedSpellSlotSpend),
            Target(Primary),
            Resolution {
                target: Primary,
                outcome: AttackHit,
            },
            DamageProjection {
                target: Primary,
                damage: Attack,
            },
            SecondaryProjection {
                target: Primary,
                projection: Condition,
            },
        ],
        "doRouteSaveFailureNextAttackProjection" => vec![
            SharedSpend(SharedCantripActionSpend),
            Target(Primary),
            Resolution {
                target: Primary,
                outcome: SavingThrowFailed,
            },
            DamageProjection {
                target: Primary,
                damage: FullSavingThrow,
            },
            SecondaryProjection {
                target: Primary,
                projection: NextAttackRollMode,
            },
        ],
        "doRouteChainedAttackMixedTargetOutcomes" => vec![
            SharedSpend(SharedSpellSlotSpend),
            Target(Primary),
            Target(Secondary),
            Resolution {
                target: Primary,
                outcome: AttackHit,
            },
            Resolution {
                target: Secondary,
                outcome: AttackHit,
            },
            DamageProjection {
                target: Primary,
                damage: Attack,
            },
            DamageProjection {
                target: Secondary,
                damage: Attack,
            },
            SecondaryProjection {
                target: Secondary,
                projection: ChainedAttackTarget,
            },
        ],
        action => panic!("unsupported mixed-target route action {action}"),
    }
}

fn attack_burst_facts(
    attack_outcome: MixedTargetOutcomeResolutionFact,
    attack_damage: MixedTargetOutcomeDamageProjectionFact,
) -> Vec<MixedTargetOutcomeRouteFact> {
    use MixedTargetOutcomeDamageProjectionFact::{FullSavingThrow, HalfSavingThrow};
    use MixedTargetOutcomeResolutionFact::{SavingThrowFailed, SavingThrowSucceeded};
    use MixedTargetOutcomeRouteFact::{
        DamageProjection, Resolution, SecondaryProjection, SharedDamageRoll, SharedSpend, Target,
    };
    use MixedTargetOutcomeRouteTargetFact::{Primary, Secondary};
    use MixedTargetOutcomeSecondaryProjectionFact::BurstSavingThrow;
    use MixedTargetOutcomeSharedDamageRollFact::SharedSavingThrowDamageRoll;
    use MixedTargetOutcomeSharedSpendFact::SharedSpellSlotSpend;

    vec![
        SharedSpend(SharedSpellSlotSpend),
        SharedDamageRoll(SharedSavingThrowDamageRoll),
        Target(Primary),
        Target(Secondary),
        Resolution {
            target: Primary,
            outcome: attack_outcome,
        },
        Resolution {
            target: Primary,
            outcome: SavingThrowFailed,
        },
        Resolution {
            target: Secondary,
            outcome: SavingThrowSucceeded,
        },
        DamageProjection {
            target: Primary,
            damage: attack_damage,
        },
        DamageProjection {
            target: Primary,
            damage: FullSavingThrow,
        },
        DamageProjection {
            target: Secondary,
            damage: HalfSavingThrow,
        },
        SecondaryProjection {
            target: Primary,
            projection: BurstSavingThrow,
        },
        SecondaryProjection {
            target: Secondary,
            projection: BurstSavingThrow,
        },
    ]
}

fn surface_ref(action: &str) -> &'static str {
    match action {
        "doRouteAreaSavingThrowMixedOutcomes" => "AreaSavingThrowMixedOutcomeRouteSurface",
        "doRouteAttackHitBurstSavingThrowMixedOutcomes" => "AttackHitBurstSavingThrowRouteSurface",
        "doRouteAttackMissBurstSavingThrowMixedOutcomes" => {
            "AttackMissBurstSavingThrowRouteSurface"
        }
        "doRouteObjectAttackSecondaryProjection" => "ObjectAttackProjectionRouteSurface",
        "doRouteAttackHitConditionProjection" => "AttackHitConditionProjectionRouteSurface",
        "doRouteSaveFailureNextAttackProjection" => "SaveFailureNextAttackProjectionRouteSurface",
        "doRouteChainedAttackMixedTargetOutcomes" => "ChainedAttackMixedTargetRouteSurface",
        action => panic!("unsupported mixed-target route action {action}"),
    }
}

fn facts_payload(facts: &[MixedTargetOutcomeRouteFact]) -> String {
    facts.iter().map(fact_ref).collect::<Vec<_>>().join(",")
}

fn fact_ref(fact: &MixedTargetOutcomeRouteFact) -> String {
    match *fact {
        MixedTargetOutcomeRouteFact::Target(target) => {
            format!("RouteMixedTargetOutcomeTarget({})", target_ref(target))
        }
        MixedTargetOutcomeRouteFact::Resolution { target, outcome } => format!(
            "RouteMixedTargetOutcomeResolution({}, {})",
            target_ref(target),
            resolution_ref(outcome)
        ),
        MixedTargetOutcomeRouteFact::DamageProjection { target, damage } => format!(
            "RouteMixedTargetOutcomeDamageProjection({}, {})",
            target_ref(target),
            damage_ref(damage)
        ),
        MixedTargetOutcomeRouteFact::SharedSpend(spend) => {
            format!("RouteMixedTargetOutcomeSharedSpend({})", spend_ref(spend))
        }
        MixedTargetOutcomeRouteFact::SharedDamageRoll(roll) => {
            format!(
                "RouteMixedTargetOutcomeSharedDamageRoll({})",
                damage_roll_ref(roll)
            )
        }
        MixedTargetOutcomeRouteFact::SecondaryProjection { target, projection } => format!(
            "RouteMixedTargetOutcomeSecondaryProjection({}, {})",
            target_ref(target),
            secondary_projection_ref(projection)
        ),
    }
}

fn target_ref(target: MixedTargetOutcomeRouteTargetFact) -> &'static str {
    match target {
        MixedTargetOutcomeRouteTargetFact::Primary => "PrimaryTarget",
        MixedTargetOutcomeRouteTargetFact::Secondary => "SecondaryTarget",
        MixedTargetOutcomeRouteTargetFact::Object => "ObjectTarget",
    }
}

fn resolution_ref(outcome: MixedTargetOutcomeResolutionFact) -> &'static str {
    match outcome {
        MixedTargetOutcomeResolutionFact::AttackHit => "AttackHitOutcome",
        MixedTargetOutcomeResolutionFact::AttackMiss => "AttackMissOutcome",
        MixedTargetOutcomeResolutionFact::SavingThrowFailed => "SavingThrowFailedOutcome",
        MixedTargetOutcomeResolutionFact::SavingThrowSucceeded => "SavingThrowSucceededOutcome",
    }
}

fn damage_ref(damage: MixedTargetOutcomeDamageProjectionFact) -> &'static str {
    match damage {
        MixedTargetOutcomeDamageProjectionFact::Attack => "AttackDamageProjection",
        MixedTargetOutcomeDamageProjectionFact::FullSavingThrow => {
            "FullSavingThrowDamageProjection"
        }
        MixedTargetOutcomeDamageProjectionFact::HalfSavingThrow => {
            "HalfSavingThrowDamageProjection"
        }
        MixedTargetOutcomeDamageProjectionFact::NoDamage => "NoDamageProjection",
    }
}

fn spend_ref(spend: MixedTargetOutcomeSharedSpendFact) -> &'static str {
    match spend {
        MixedTargetOutcomeSharedSpendFact::SharedSpellSlotSpend => "SharedSpellSlotSpend",
        MixedTargetOutcomeSharedSpendFact::SharedCantripActionSpend => "SharedCantripActionSpend",
    }
}

fn damage_roll_ref(roll: MixedTargetOutcomeSharedDamageRollFact) -> &'static str {
    match roll {
        MixedTargetOutcomeSharedDamageRollFact::SharedSavingThrowDamageRoll => {
            "SharedSavingThrowDamageRoll"
        }
    }
}

fn secondary_projection_ref(projection: MixedTargetOutcomeSecondaryProjectionFact) -> &'static str {
    match projection {
        MixedTargetOutcomeSecondaryProjectionFact::ObjectBoundary => "ObjectBoundaryProjection",
        MixedTargetOutcomeSecondaryProjectionFact::Light => "LightProjection",
        MixedTargetOutcomeSecondaryProjectionFact::Condition => "ConditionProjection",
        MixedTargetOutcomeSecondaryProjectionFact::NextAttackRollMode => {
            "NextAttackRollModeProjection"
        }
        MixedTargetOutcomeSecondaryProjectionFact::BurstSavingThrow => "BurstSavingThrowProjection",
        MixedTargetOutcomeSecondaryProjectionFact::ChainedAttackTarget => {
            "ChainedAttackTargetProjection"
        }
    }
}
