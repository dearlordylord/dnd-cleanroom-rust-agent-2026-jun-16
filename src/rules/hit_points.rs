#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HitPointMaximumFacts {
    pub starting_hit_point_die: i16,
    pub constitution_modifier: i16,
    pub fixed_higher_level_hit_point_dice: Vec<i16>,
    pub hit_point_maximum_bonus: i16,
    pub hit_point_maximum_reduction: i16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HitPointMaximumProjection {
    pub normal_hit_point_maximum: i16,
    pub effective_hit_point_maximum: i16,
    pub hit_dice_total: i16,
    pub hit_point_maximum_reduction: i16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HitPointMaximumError {
    IllegalFacts,
    HitDiceOverflow,
}

#[must_use]
pub fn fixed_higher_level_hit_point_gain(hit_point_die: i16, constitution_modifier: i16) -> i16 {
    // QNT: cleanroom-input/qnt/shared-algebras/proofs/rule-core/hit-point-maximum.qnt;
    // RAW: cleanroom-input/raw/srd-5.2.1/Character-Creation.md
    // "Gaining a Level" and "Fixed Hit Points by Class".
    let gain = (hit_point_die / 2) + 1 + constitution_modifier;
    gain.max(1)
}

#[must_use]
pub fn normal_hit_point_maximum(facts: &HitPointMaximumFacts) -> i16 {
    facts.starting_hit_point_die
        + facts.constitution_modifier
        + facts
            .fixed_higher_level_hit_point_dice
            .iter()
            .map(|die| fixed_higher_level_hit_point_gain(*die, facts.constitution_modifier))
            .sum::<i16>()
        + facts.hit_point_maximum_bonus
}

#[must_use]
pub fn effective_hit_point_maximum(facts: &HitPointMaximumFacts) -> i16 {
    normal_hit_point_maximum(facts) - facts.hit_point_maximum_reduction
}

#[must_use]
pub fn legal_hit_point_maximum_facts(facts: &HitPointMaximumFacts) -> bool {
    facts.starting_hit_point_die > 0
        && facts
            .fixed_higher_level_hit_point_dice
            .iter()
            .all(|die| *die > 0)
        && normal_hit_point_maximum(facts) > 0
        && facts.hit_point_maximum_reduction >= 0
        && facts.hit_point_maximum_reduction < normal_hit_point_maximum(facts)
}

pub fn hit_point_maximum_projection(
    facts: HitPointMaximumFacts,
) -> Result<HitPointMaximumProjection, HitPointMaximumError> {
    // QNT: cleanroom-input/qnt/shared-algebras/proofs/rule-core/hit-point-maximum.qnt;
    // RAW: Character-Creation.md "Level 1 Hit Points by Class",
    // "Fixed Hit Points by Class", and "Hit Points and Hit Point Dice".
    if !legal_hit_point_maximum_facts(&facts) {
        return Err(HitPointMaximumError::IllegalFacts);
    }

    let hit_dice_total = i16::try_from(facts.fixed_higher_level_hit_point_dice.len())
        .map_err(|_| HitPointMaximumError::HitDiceOverflow)?
        .checked_add(1)
        .ok_or(HitPointMaximumError::HitDiceOverflow)?;

    Ok(HitPointMaximumProjection {
        normal_hit_point_maximum: normal_hit_point_maximum(&facts),
        effective_hit_point_maximum: effective_hit_point_maximum(&facts),
        hit_dice_total,
        hit_point_maximum_reduction: facts.hit_point_maximum_reduction,
    })
}
