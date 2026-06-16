#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttackDamageDispositionOutcome {
    Initial,
    MeleeKnockOutAccepted,
    RangedKnockOutRejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttackKind {
    Melee,
    Ranged,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttackDamageDisposition {
    Ordinary,
    KnockOut,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreatureKind {
    PlayerCharacter,
    Monster,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreatureVitals {
    pub kind: CreatureKind,
    pub hit_points: i16,
    pub hit_point_maximum: i16,
    pub temporary_hit_points: i16,
    pub dead: bool,
    pub unconscious: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AttackDamageFacts {
    pub attack_kind: AttackKind,
    pub rolled_damage: i16,
    pub damage_modifier: i16,
    pub disposition: AttackDamageDisposition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PositiveHitPointDamageResult {
    pub vitals: CreatureVitals,
    pub damage_to_hit_points: i16,
    pub remaining_damage_at_zero: i16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AttackDamageDispositionState {
    pub outcome: AttackDamageDispositionOutcome,
    pub accepted: bool,
    pub target_hit_points: i16,
    pub target_unconscious: bool,
    pub target_dead: bool,
    pub knock_out_recovery_ends_when_hit_points_regained: bool,
    pub replay_index: i16,
}

pub const ATTACK_DAMAGE_TARGET_HIT_POINTS: i16 = 3;
pub const ATTACK_DAMAGE_TARGET_HIT_POINT_MAXIMUM: i16 = 12;
pub const ATTACK_DAMAGE_ROLLED_DAMAGE: i16 = 8;

#[must_use]
pub fn attack_damage_disposition_initial_state() -> AttackDamageDispositionState {
    AttackDamageDispositionState {
        outcome: AttackDamageDispositionOutcome::Initial,
        accepted: true,
        target_hit_points: ATTACK_DAMAGE_TARGET_HIT_POINTS,
        target_unconscious: false,
        target_dead: false,
        knock_out_recovery_ends_when_hit_points_regained: false,
        replay_index: 0,
    }
}

#[must_use]
pub fn resolve_melee_knock_out_disposition() -> AttackDamageDispositionState {
    let target_vitals = attack_damage_target_vitals();
    let facts = attack_damage_facts(AttackKind::Melee, AttackDamageDisposition::KnockOut);
    let damage_amount = damage_amount_before_hit_points(facts);
    let accepted = attack_damage_disposition_is_legal(target_vitals, damage_amount, facts);
    let damage_result = apply_resolved_damage_to_positive_hit_points(target_vitals, damage_amount);
    let (vitals, recovery) = apply_knock_out_disposition(damage_result);

    AttackDamageDispositionState {
        outcome: AttackDamageDispositionOutcome::MeleeKnockOutAccepted,
        accepted,
        target_hit_points: vitals.hit_points,
        target_unconscious: vitals.unconscious,
        target_dead: vitals.dead,
        knock_out_recovery_ends_when_hit_points_regained: recovery,
        replay_index: 1,
    }
}

#[must_use]
pub fn reject_ranged_knock_out_disposition() -> AttackDamageDispositionState {
    let target_vitals = attack_damage_target_vitals();
    let facts = attack_damage_facts(AttackKind::Ranged, AttackDamageDisposition::KnockOut);
    let damage_amount = damage_amount_before_hit_points(facts);
    let accepted = attack_damage_disposition_is_legal(target_vitals, damage_amount, facts);

    AttackDamageDispositionState {
        outcome: AttackDamageDispositionOutcome::RangedKnockOutRejected,
        accepted,
        target_hit_points: target_vitals.hit_points,
        target_unconscious: target_vitals.unconscious,
        target_dead: target_vitals.dead,
        knock_out_recovery_ends_when_hit_points_regained: false,
        replay_index: 2,
    }
}

#[must_use]
pub fn attack_damage_target_vitals() -> CreatureVitals {
    CreatureVitals {
        kind: CreatureKind::PlayerCharacter,
        hit_points: ATTACK_DAMAGE_TARGET_HIT_POINTS,
        hit_point_maximum: ATTACK_DAMAGE_TARGET_HIT_POINT_MAXIMUM,
        temporary_hit_points: 0,
        dead: false,
        unconscious: false,
    }
}

#[must_use]
pub fn attack_damage_facts(
    attack_kind: AttackKind,
    disposition: AttackDamageDisposition,
) -> AttackDamageFacts {
    AttackDamageFacts {
        attack_kind,
        rolled_damage: ATTACK_DAMAGE_ROLLED_DAMAGE,
        damage_modifier: 0,
        disposition,
    }
}

#[must_use]
pub fn damage_amount_before_hit_points(facts: AttackDamageFacts) -> i16 {
    (facts.rolled_damage + facts.damage_modifier).max(0)
}

#[must_use]
pub fn attack_damage_disposition_is_legal(
    vitals: CreatureVitals,
    damage_amount: i16,
    facts: AttackDamageFacts,
) -> bool {
    if facts.disposition == AttackDamageDisposition::Ordinary {
        return true;
    }

    facts.attack_kind == AttackKind::Melee
        && damage_amount > 0
        && apply_resolved_damage_to_positive_hit_points(vitals, damage_amount)
            .vitals
            .hit_points
            == 0
}

#[must_use]
pub fn apply_resolved_damage_to_positive_hit_points(
    vitals: CreatureVitals,
    raw_damage: i16,
) -> PositiveHitPointDamageResult {
    // RAW: cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md
    // "Damage Rolls" and "Knocking Out a Creature". QNT:
    // rule-core-attack-damage-disposition.mbt.qnt and
    // shared-algebras/proofs/rule-core/{attack-damage-composition,
    // hit-point-damage, hit-point-recovery}.qnt.
    if vitals.dead {
        return PositiveHitPointDamageResult {
            vitals,
            damage_to_hit_points: 0,
            remaining_damage_at_zero: 0,
        };
    }

    let resolved_damage = raw_damage.max(0);
    let absorbed_by_temporary_hit_points = resolved_damage.min(vitals.temporary_hit_points);
    let damage_to_hit_points = resolved_damage - absorbed_by_temporary_hit_points;
    let remaining_damage_at_zero = if damage_to_hit_points > vitals.hit_points {
        damage_to_hit_points - vitals.hit_points
    } else {
        0
    };
    let next_hit_points = (vitals.hit_points - damage_to_hit_points)
        .max(0)
        .min(vitals.hit_point_maximum);
    let drops_to_zero = vitals.hit_points > 0 && next_hit_points == 0;
    let instant_death = character_dies_from_massive_damage(vitals, damage_to_hit_points);

    PositiveHitPointDamageResult {
        vitals: CreatureVitals {
            hit_points: next_hit_points,
            temporary_hit_points: vitals.temporary_hit_points - absorbed_by_temporary_hit_points,
            unconscious: vitals.unconscious
                || (drops_to_zero
                    && vitals.kind == CreatureKind::PlayerCharacter
                    && !instant_death),
            dead: vitals.dead
                || (drops_to_zero && vitals.kind == CreatureKind::Monster)
                || instant_death,
            ..vitals
        },
        damage_to_hit_points,
        remaining_damage_at_zero,
    }
}

#[must_use]
pub fn apply_knock_out_disposition(
    damage_result: PositiveHitPointDamageResult,
) -> (CreatureVitals, bool) {
    if damage_result.vitals.hit_points != 0 || damage_result.damage_to_hit_points <= 0 {
        return (damage_result.vitals, false);
    }

    (
        CreatureVitals {
            hit_points: 1,
            dead: false,
            unconscious: true,
            ..damage_result.vitals
        },
        true,
    )
}

fn character_dies_from_massive_damage(vitals: CreatureVitals, damage_to_hit_points: i16) -> bool {
    vitals.kind == CreatureKind::PlayerCharacter
        && vitals.hit_points > 0
        && damage_to_hit_points > vitals.hit_points
        && damage_to_hit_points - vitals.hit_points >= vitals.hit_point_maximum
}
