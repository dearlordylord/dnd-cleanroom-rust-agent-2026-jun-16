#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DamageType {
    Fire,
    Force,
    Lightning,
    Necrotic,
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
