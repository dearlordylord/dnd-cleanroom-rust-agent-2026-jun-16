#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DamageType {
    Fire,
    Force,
    Lightning,
    Necrotic,
    Piercing,
    Radiant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttackSpellProfile {
    ChillTouch,
    FireBolt,
    GuidingBolt,
    ShockingGrasp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SaveGatedSpellProfile {
    InflictWounds,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpellActiveEffect {
    HitPointRegainPrevented,
    NextAttackRollAgainstTargetAdvantage,
    OpportunityAttackDenied,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeActor {
    Fighter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpellObjectTarget {
    StarryWispObjectTarget,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpellInvocationKind {
    StarryWisp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpellShapeOutcome {
    ChillTouchHit,
    FireBoltHit,
    GuidingBoltHit,
    InflictWoundsFailedSave,
    InflictWoundsSuccessfulSave,
    ShockingGraspHit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpellShapeState {
    pub target_hit_points: i16,
    pub spell_slot_spent_this_turn: bool,
    pub level_one_slots_remaining: i16,
    pub active_effects: Vec<SpellActiveEffect>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AttackSpellHitFacts {
    pub spell: AttackSpellProfile,
    pub damage_roll: i16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SaveGatedSpellDamageFacts {
    pub spell: SaveGatedSpellProfile,
    pub damage_roll: i16,
    pub saving_throw_succeeded: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpellShapeProjection {
    pub state: SpellShapeState,
    pub outcome: SpellShapeOutcome,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EldritchBlastInvalidReason {
    StaleSubject,
    InvalidFill,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EldritchBlastProtocol {
    NeedsTargets,
    NeedsAttackRoll,
    NeedsDamageRoll,
    Resolved,
    Invalid(EldritchBlastInvalidReason),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EldritchBlastState {
    pub action_available: bool,
    pub target_hit_points: i16,
    pub resolved_beams: i16,
    pub protocol: EldritchBlastProtocol,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EldritchBlastAttackFacts {
    pub hit: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EldritchBlastDamageFacts {
    pub damage_roll: i16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StarryWispObjectInvalidReason {
    StaleSubject,
    InvalidFill,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StarryWispObjectProtocol {
    NeedsTarget,
    NeedsAttackRoll,
    NeedsDamageRoll,
    Resolved,
    Invalid(StarryWispObjectInvalidReason),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StarryWispObjectAttackFacts {
    pub hit: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StarryWispObjectDamageFacts {
    pub damage_roll: i16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObjectDamageOutcome {
    pub object: SpellObjectTarget,
    pub damage_type: DamageType,
    pub rolled_damage: i16,
    pub effective_damage: i16,
    pub prior_hit_points: i16,
    pub next_hit_points: i16,
    pub destroyed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObjectInvisibleRevealLightEmitter {
    pub source: RuntimeActor,
    pub source_spell: SpellInvocationKind,
    pub object: SpellObjectTarget,
    pub dim_light_radius_feet: i16,
    pub expires_at_actor: RuntimeActor,
    pub expires_at_round: i16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StarryWispObjectState {
    pub action_available: bool,
    pub object_damage: Option<ObjectDamageOutcome>,
    pub light_emitters: Vec<ObjectInvisibleRevealLightEmitter>,
    pub protocol: StarryWispObjectProtocol,
}

#[must_use]
pub fn spell_attack_damage_type(spell: AttackSpellProfile) -> DamageType {
    // QNT: cleanroom-input/qnt/shared-algebras/proofs/rule-core/
    // spell-attack-damage-projection-core.qnt.
    match spell {
        AttackSpellProfile::ChillTouch => DamageType::Necrotic,
        AttackSpellProfile::FireBolt => DamageType::Fire,
        AttackSpellProfile::GuidingBolt => DamageType::Radiant,
        AttackSpellProfile::ShockingGrasp => DamageType::Lightning,
    }
}

#[must_use]
pub fn save_gated_spell_damage_type(spell: SaveGatedSpellProfile) -> DamageType {
    // QNT: cleanroom-input/qnt/shared-algebras/proofs/rule-core/
    // spell-save-damage-projection-core.qnt.
    match spell {
        SaveGatedSpellProfile::InflictWounds => DamageType::Necrotic,
    }
}

#[must_use]
pub fn eldritch_blast_damage_type() -> DamageType {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-E-L.md
    // "Eldritch Blast".
    DamageType::Force
}

#[must_use]
pub fn eldritch_blast_beam_count(character_level: i16) -> Option<i16> {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-E-L.md
    // "Eldritch Blast", "Cantrip Upgrade".
    match character_level {
        1..=4 => Some(1),
        5..=10 => Some(2),
        11..=16 => Some(3),
        17.. => Some(4),
        _ => None,
    }
}

#[must_use]
pub fn eldritch_blast_initial_state() -> EldritchBlastState {
    EldritchBlastState {
        action_available: true,
        target_hit_points: 13,
        resolved_beams: 0,
        protocol: EldritchBlastProtocol::NeedsTargets,
    }
}

#[must_use]
pub fn starry_wisp_object_initial_state() -> StarryWispObjectState {
    StarryWispObjectState {
        action_available: true,
        object_damage: None,
        light_emitters: Vec::new(),
        protocol: StarryWispObjectProtocol::NeedsTarget,
    }
}

#[must_use]
pub fn fill_eldritch_blast_targets(state: EldritchBlastState) -> EldritchBlastState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-E-L.md
    // "Eldritch Blast" allows one creature or object target per beam.
    if !state.action_available || state.protocol != EldritchBlastProtocol::NeedsTargets {
        return invalid_eldritch_blast(state, EldritchBlastInvalidReason::InvalidFill);
    }

    EldritchBlastState {
        protocol: EldritchBlastProtocol::NeedsAttackRoll,
        ..state
    }
}

#[must_use]
pub fn fill_eldritch_blast_attack(
    state: EldritchBlastState,
    facts: EldritchBlastAttackFacts,
) -> EldritchBlastState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-E-L.md
    // "Eldritch Blast"; each beam has a separate ranged spell attack roll.
    if !state.action_available || state.protocol != EldritchBlastProtocol::NeedsAttackRoll {
        return invalid_eldritch_blast(state, EldritchBlastInvalidReason::InvalidFill);
    }

    if facts.hit {
        return EldritchBlastState {
            protocol: EldritchBlastProtocol::NeedsDamageRoll,
            ..state
        };
    }

    completed_eldritch_blast_beam(state, 0)
}

#[must_use]
pub fn fill_eldritch_blast_damage(
    state: EldritchBlastState,
    facts: EldritchBlastDamageFacts,
) -> EldritchBlastState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-E-L.md
    // "Eldritch Blast" deals 1d10 Force damage on a hit.
    if !state.action_available
        || state.protocol != EldritchBlastProtocol::NeedsDamageRoll
        || !(1..=10).contains(&facts.damage_roll)
    {
        return invalid_eldritch_blast(state, EldritchBlastInvalidReason::InvalidFill);
    }

    completed_eldritch_blast_beam(state, facts.damage_roll)
}

#[must_use]
pub fn reject_eldritch_blast_stale_after_resolved(state: EldritchBlastState) -> EldritchBlastState {
    EldritchBlastState {
        protocol: EldritchBlastProtocol::Invalid(EldritchBlastInvalidReason::StaleSubject),
        ..state
    }
}

#[must_use]
pub fn fill_starry_wisp_object_target(state: StarryWispObjectState) -> StarryWispObjectState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-S-Z.md
    // "Starry Wisp" targets one creature or object; QNT:
    // battle-runtime-starry-wisp-object.mbt.qnt.
    if !state.action_available || state.protocol != StarryWispObjectProtocol::NeedsTarget {
        return invalid_starry_wisp_object(state, StarryWispObjectInvalidReason::InvalidFill);
    }

    StarryWispObjectState {
        protocol: StarryWispObjectProtocol::NeedsAttackRoll,
        ..state
    }
}

#[must_use]
pub fn reject_starry_wisp_object_without_fact(
    state: StarryWispObjectState,
) -> StarryWispObjectState {
    invalid_starry_wisp_object(state, StarryWispObjectInvalidReason::InvalidFill)
}

#[must_use]
pub fn fill_starry_wisp_object_attack(
    state: StarryWispObjectState,
    facts: StarryWispObjectAttackFacts,
) -> StarryWispObjectState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-S-Z.md
    // "Starry Wisp" makes a ranged spell attack against the target.
    if !state.action_available || state.protocol != StarryWispObjectProtocol::NeedsAttackRoll {
        return invalid_starry_wisp_object(state, StarryWispObjectInvalidReason::InvalidFill);
    }

    if facts.hit {
        return StarryWispObjectState {
            protocol: StarryWispObjectProtocol::NeedsDamageRoll,
            ..state
        };
    }

    StarryWispObjectState {
        action_available: false,
        protocol: StarryWispObjectProtocol::Resolved,
        object_damage: None,
        light_emitters: Vec::new(),
    }
}

#[must_use]
pub fn fill_starry_wisp_object_damage(
    state: StarryWispObjectState,
    facts: StarryWispObjectDamageFacts,
) -> StarryWispObjectState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-S-Z.md
    // "Starry Wisp" deals 1d8 Radiant damage and emits Dim Light on a hit;
    // Rules-Glossary.md "Breaking Objects" supplies object HP semantics.
    if !state.action_available
        || state.protocol != StarryWispObjectProtocol::NeedsDamageRoll
        || !(1..=8).contains(&facts.damage_roll)
    {
        return invalid_starry_wisp_object(state, StarryWispObjectInvalidReason::InvalidFill);
    }

    StarryWispObjectState {
        action_available: false,
        object_damage: Some(starry_wisp_object_damage_outcome(facts.damage_roll)),
        light_emitters: vec![starry_wisp_object_dim_light_emitter()],
        protocol: StarryWispObjectProtocol::Resolved,
    }
}

#[must_use]
pub fn reject_starry_wisp_object_stale_after_resolved(
    state: StarryWispObjectState,
) -> StarryWispObjectState {
    StarryWispObjectState {
        protocol: StarryWispObjectProtocol::Invalid(StarryWispObjectInvalidReason::StaleSubject),
        ..state
    }
}

#[must_use]
pub fn resolve_spell_attack_hit(
    state: SpellShapeState,
    facts: AttackSpellHitFacts,
) -> SpellShapeProjection {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-A-D.md
    // "Chill Touch"; Descriptions-E-L.md "Fire Bolt" and "Guiding Bolt";
    // Descriptions-S-Z.md "Shocking Grasp".
    let mut state =
        spend_level_one_slot_if_required(state, attack_spell_requires_slot(facts.spell));
    state.target_hit_points = apply_damage(state.target_hit_points, facts.damage_roll);
    state.active_effects = attack_spell_hit_effects(facts.spell);

    SpellShapeProjection {
        state,
        outcome: attack_spell_outcome(facts.spell),
    }
}

#[must_use]
pub fn resolve_save_gated_spell_damage(
    state: SpellShapeState,
    facts: SaveGatedSpellDamageFacts,
) -> SpellShapeProjection {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-E-L.md
    // "Inflict Wounds"; QNT: spell-save-damage-projection-core.qnt.
    let mut state = spend_level_one_slot_if_required(state, true);
    let damage = if facts.saving_throw_succeeded {
        facts.damage_roll / 2
    } else {
        facts.damage_roll
    };
    state.target_hit_points = apply_damage(state.target_hit_points, damage);
    state.active_effects.clear();

    SpellShapeProjection {
        state,
        outcome: match facts.spell {
            SaveGatedSpellProfile::InflictWounds if facts.saving_throw_succeeded => {
                SpellShapeOutcome::InflictWoundsSuccessfulSave
            }
            SaveGatedSpellProfile::InflictWounds => SpellShapeOutcome::InflictWoundsFailedSave,
        },
    }
}

fn attack_spell_requires_slot(spell: AttackSpellProfile) -> bool {
    match spell {
        AttackSpellProfile::GuidingBolt => true,
        AttackSpellProfile::ChillTouch
        | AttackSpellProfile::FireBolt
        | AttackSpellProfile::ShockingGrasp => false,
    }
}

fn attack_spell_hit_effects(spell: AttackSpellProfile) -> Vec<SpellActiveEffect> {
    match spell {
        AttackSpellProfile::ChillTouch => vec![SpellActiveEffect::HitPointRegainPrevented],
        AttackSpellProfile::FireBolt => vec![],
        AttackSpellProfile::GuidingBolt => {
            vec![SpellActiveEffect::NextAttackRollAgainstTargetAdvantage]
        }
        AttackSpellProfile::ShockingGrasp => vec![SpellActiveEffect::OpportunityAttackDenied],
    }
}

fn attack_spell_outcome(spell: AttackSpellProfile) -> SpellShapeOutcome {
    match spell {
        AttackSpellProfile::ChillTouch => SpellShapeOutcome::ChillTouchHit,
        AttackSpellProfile::FireBolt => SpellShapeOutcome::FireBoltHit,
        AttackSpellProfile::GuidingBolt => SpellShapeOutcome::GuidingBoltHit,
        AttackSpellProfile::ShockingGrasp => SpellShapeOutcome::ShockingGraspHit,
    }
}

fn spend_level_one_slot_if_required(
    mut state: SpellShapeState,
    requires_slot: bool,
) -> SpellShapeState {
    if requires_slot && !state.spell_slot_spent_this_turn && state.level_one_slots_remaining > 0 {
        state.spell_slot_spent_this_turn = true;
        state.level_one_slots_remaining -= 1;
    }
    state
}

fn apply_damage(hit_points: i16, damage: i16) -> i16 {
    hit_points.saturating_sub(damage.max(0)).max(0)
}

fn completed_eldritch_blast_beam(state: EldritchBlastState, damage: i16) -> EldritchBlastState {
    let resolved_beams = state.resolved_beams + 1;
    let sequence_complete = resolved_beams >= 2;

    EldritchBlastState {
        action_available: !sequence_complete,
        target_hit_points: apply_damage(state.target_hit_points, damage),
        resolved_beams,
        protocol: if sequence_complete {
            EldritchBlastProtocol::Resolved
        } else {
            EldritchBlastProtocol::NeedsAttackRoll
        },
    }
}

fn invalid_eldritch_blast(
    state: EldritchBlastState,
    reason: EldritchBlastInvalidReason,
) -> EldritchBlastState {
    EldritchBlastState {
        protocol: EldritchBlastProtocol::Invalid(reason),
        ..state
    }
}

fn starry_wisp_object_damage_outcome(rolled_damage: i16) -> ObjectDamageOutcome {
    let prior_hit_points = 5;
    let effective_damage = rolled_damage;
    let next_hit_points = apply_damage(prior_hit_points, effective_damage);

    ObjectDamageOutcome {
        object: SpellObjectTarget::StarryWispObjectTarget,
        damage_type: DamageType::Radiant,
        rolled_damage,
        effective_damage,
        prior_hit_points,
        next_hit_points,
        destroyed: next_hit_points == 0,
    }
}

fn starry_wisp_object_dim_light_emitter() -> ObjectInvisibleRevealLightEmitter {
    ObjectInvisibleRevealLightEmitter {
        source: RuntimeActor::Fighter,
        source_spell: SpellInvocationKind::StarryWisp,
        object: SpellObjectTarget::StarryWispObjectTarget,
        dim_light_radius_feet: 10,
        expires_at_actor: RuntimeActor::Fighter,
        expires_at_round: 2,
    }
}

fn invalid_starry_wisp_object(
    state: StarryWispObjectState,
    reason: StarryWispObjectInvalidReason,
) -> StarryWispObjectState {
    StarryWispObjectState {
        protocol: StarryWispObjectProtocol::Invalid(reason),
        ..state
    }
}
