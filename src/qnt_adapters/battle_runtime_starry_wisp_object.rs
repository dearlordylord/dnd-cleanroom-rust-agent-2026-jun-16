use crate::rules::spell_shapes::{
    fill_starry_wisp_object_attack, fill_starry_wisp_object_damage, fill_starry_wisp_object_target,
    reject_starry_wisp_object_stale_after_resolved, reject_starry_wisp_object_without_fact,
    starry_wisp_object_initial_state, DamageType, ObjectDamageOutcome, RuntimeActor,
    SpellInvocationKind, SpellObjectTarget, StarryWispObjectAttackFacts,
    StarryWispObjectDamageFacts, StarryWispObjectInvalidReason, StarryWispObjectProtocol,
    StarryWispObjectState,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StarryWispObjectWitness {
    pub action_available: bool,
    pub object_damage: &'static str,
    pub object_damage_type: &'static str,
    pub object_damage_rolled: i16,
    pub object_damage_effective: i16,
    pub object_damage_prior_hp: i16,
    pub object_damage_next_hp: i16,
    pub object_destroyed: bool,
    pub light_emitter_count: usize,
    pub light_emitter_source: &'static str,
    pub light_emitter_source_spell: &'static str,
    pub light_emitter_object: &'static str,
    pub light_emitter_dim_radius_feet: i16,
    pub light_emitter_expires_at_actor: &'static str,
    pub light_emitter_expires_at_round: i16,
    pub protocol_result: &'static str,
    pub protocol_invalid_reason: &'static str,
    pub protocol_holes: Vec<&'static str>,
}

pub const BRANCH_ACTIONS: [&str; 7] = [
    "doFillObjectTarget",
    "doRejectObjectWithoutFact",
    "doFillObjectAttackRollMiss",
    "doFillObjectAttackRollHit",
    "doFillObjectDamageLow",
    "doFillObjectDamageHigh",
    "doRejectStaleAfterResolved",
];

pub fn replay_observed_action(observed_action_taken: &str) -> StarryWispObjectWitness {
    match observed_action_taken {
        "doFillObjectTarget" => witness_from_state(fill_object_target()),
        "doRejectObjectWithoutFact" => witness_from_state(reject_object_without_fact()),
        "doFillObjectAttackRollMiss" => witness_from_state(fill_object_attack_roll_miss()),
        "doFillObjectAttackRollHit" => witness_from_state(fill_object_attack_roll_hit()),
        "doFillObjectDamageLow" => witness_from_state(fill_object_damage_low()),
        "doFillObjectDamageHigh" => witness_from_state(fill_object_damage_high()),
        "doRejectStaleAfterResolved" => witness_from_state(reject_stale_after_resolved()),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> StarryWispObjectWitness {
    replay_observed_action(observed_action_taken)
}

pub fn projection_payload(witness: &StarryWispObjectWitness) -> String {
    [
        format!("qActionAvailable={}", witness.action_available),
        format!("qObjectDamage={}", witness.object_damage),
        format!("qObjectDamageType={}", witness.object_damage_type),
        format!("qObjectDamageRolled={}", witness.object_damage_rolled),
        format!("qObjectDamageEffective={}", witness.object_damage_effective),
        format!("qObjectDamagePriorHp={}", witness.object_damage_prior_hp),
        format!("qObjectDamageNextHp={}", witness.object_damage_next_hp),
        format!("qObjectDestroyed={}", witness.object_destroyed),
        format!("qLightEmitterCount={}", witness.light_emitter_count),
        format!("qLightEmitterSource={}", witness.light_emitter_source),
        format!(
            "qLightEmitterSourceSpell={}",
            witness.light_emitter_source_spell
        ),
        format!("qLightEmitterObject={}", witness.light_emitter_object),
        format!(
            "qLightEmitterDimRadiusFeet={}",
            witness.light_emitter_dim_radius_feet
        ),
        format!(
            "qLightEmitterExpiresAtActor={}",
            witness.light_emitter_expires_at_actor
        ),
        format!(
            "qLightEmitterExpiresAtRound={}",
            witness.light_emitter_expires_at_round
        ),
        format!("qHoles={}", joined_or_none(&witness.protocol_holes)),
        format!("protocolResult={}", witness.protocol_result),
        format!("protocolInvalidReason={}", witness.protocol_invalid_reason),
        format!("protocolHoles={}", joined_or_none(&witness.protocol_holes)),
    ]
    .join("\n")
}

fn fill_object_target() -> StarryWispObjectState {
    fill_starry_wisp_object_target(starry_wisp_object_initial_state())
}

fn reject_object_without_fact() -> StarryWispObjectState {
    reject_starry_wisp_object_without_fact(starry_wisp_object_initial_state())
}

fn fill_object_attack_roll_miss() -> StarryWispObjectState {
    fill_starry_wisp_object_attack(
        fill_object_target(),
        StarryWispObjectAttackFacts { hit: false },
    )
}

fn fill_object_attack_roll_hit() -> StarryWispObjectState {
    fill_starry_wisp_object_attack(
        fill_object_target(),
        StarryWispObjectAttackFacts { hit: true },
    )
}

fn fill_object_damage_low() -> StarryWispObjectState {
    fill_starry_wisp_object_damage(
        fill_object_attack_roll_hit(),
        StarryWispObjectDamageFacts { damage_roll: 4 },
    )
}

fn fill_object_damage_high() -> StarryWispObjectState {
    fill_starry_wisp_object_damage(
        fill_object_attack_roll_hit(),
        StarryWispObjectDamageFacts { damage_roll: 6 },
    )
}

fn reject_stale_after_resolved() -> StarryWispObjectState {
    reject_starry_wisp_object_stale_after_resolved(fill_object_damage_high())
}

fn witness_from_state(state: StarryWispObjectState) -> StarryWispObjectWitness {
    let light_emitter = state.light_emitters.first().copied();
    StarryWispObjectWitness {
        action_available: state.action_available,
        object_damage: object_damage_ref(state.object_damage),
        object_damage_type: state
            .object_damage
            .map_or("", |damage| damage_type_ref(damage.damage_type)),
        object_damage_rolled: state.object_damage.map_or(0, |damage| damage.rolled_damage),
        object_damage_effective: state
            .object_damage
            .map_or(0, |damage| damage.effective_damage),
        object_damage_prior_hp: state
            .object_damage
            .map_or(0, |damage| damage.prior_hit_points),
        object_damage_next_hp: state
            .object_damage
            .map_or(0, |damage| damage.next_hit_points),
        object_destroyed: state.object_damage.is_some_and(|damage| damage.destroyed),
        light_emitter_count: state.light_emitters.len(),
        light_emitter_source: light_emitter.map_or("", |emitter| actor_ref(emitter.source)),
        light_emitter_source_spell: light_emitter
            .map_or("", |emitter| spell_invocation_ref(emitter.source_spell)),
        light_emitter_object: light_emitter.map_or("", |emitter| object_target_ref(emitter.object)),
        light_emitter_dim_radius_feet: light_emitter
            .map_or(0, |emitter| emitter.dim_light_radius_feet),
        light_emitter_expires_at_actor: light_emitter
            .map_or("", |emitter| actor_ref(emitter.expires_at_actor)),
        light_emitter_expires_at_round: light_emitter.map_or(0, |emitter| emitter.expires_at_round),
        protocol_result: protocol_result_ref(state.protocol),
        protocol_invalid_reason: protocol_invalid_reason_ref(state.protocol),
        protocol_holes: protocol_holes(state.protocol),
    }
}

fn object_damage_ref(damage: Option<ObjectDamageOutcome>) -> &'static str {
    match damage {
        Some(_) => "SomeObjectDamage",
        None => "NoObjectDamage",
    }
}

fn damage_type_ref(damage_type: DamageType) -> &'static str {
    match damage_type {
        DamageType::Radiant => "Radiant",
        DamageType::Fire => "Fire",
        DamageType::Force => "Force",
        DamageType::Lightning => "Lightning",
        DamageType::Necrotic => "Necrotic",
        DamageType::Piercing => "Piercing",
    }
}

fn actor_ref(actor: RuntimeActor) -> &'static str {
    match actor {
        RuntimeActor::Fighter => "Fighter",
    }
}

fn spell_invocation_ref(spell: SpellInvocationKind) -> &'static str {
    match spell {
        SpellInvocationKind::StarryWisp => "StarryWisp",
    }
}

fn object_target_ref(object: SpellObjectTarget) -> &'static str {
    match object {
        SpellObjectTarget::StarryWispObjectTarget => "StarryWispObjectTarget",
    }
}

fn protocol_result_ref(protocol: StarryWispObjectProtocol) -> &'static str {
    match protocol {
        StarryWispObjectProtocol::NeedsTarget
        | StarryWispObjectProtocol::NeedsAttackRoll
        | StarryWispObjectProtocol::NeedsDamageRoll => "needsHoles",
        StarryWispObjectProtocol::Resolved => "resolved",
        StarryWispObjectProtocol::Invalid(_) => "invalid",
    }
}

fn protocol_invalid_reason_ref(protocol: StarryWispObjectProtocol) -> &'static str {
    match protocol {
        StarryWispObjectProtocol::Invalid(StarryWispObjectInvalidReason::StaleSubject) => {
            "WStaleSubject"
        }
        StarryWispObjectProtocol::Invalid(StarryWispObjectInvalidReason::InvalidFill) => {
            "WInvalidFill"
        }
        StarryWispObjectProtocol::NeedsTarget
        | StarryWispObjectProtocol::NeedsAttackRoll
        | StarryWispObjectProtocol::NeedsDamageRoll
        | StarryWispObjectProtocol::Resolved => "",
    }
}

fn protocol_holes(protocol: StarryWispObjectProtocol) -> Vec<&'static str> {
    match protocol {
        StarryWispObjectProtocol::NeedsTarget
        | StarryWispObjectProtocol::Invalid(StarryWispObjectInvalidReason::InvalidFill) => {
            vec!["TargetChoice", "ObjectTargetChoice"]
        }
        StarryWispObjectProtocol::NeedsAttackRoll => vec!["AttackRoll"],
        StarryWispObjectProtocol::NeedsDamageRoll => vec!["SpellDamageRoll"],
        StarryWispObjectProtocol::Resolved
        | StarryWispObjectProtocol::Invalid(StarryWispObjectInvalidReason::StaleSubject) => {
            Vec::new()
        }
    }
}

fn joined_or_none(values: &[&str]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}
