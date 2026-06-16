#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScalarBuffScenarioResult {
    Init,
    ShieldOfFaith,
    Longstrider,
    SpiderClimb,
    Aid,
    FalseLife,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScalarBuffProtocol {
    Init,
    Resolved,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScalarBuffActiveEffectsState {
    pub affected_armor_class: i16,
    pub affected_speed_feet: i16,
    pub affected_climb_speed_feet: i16,
    pub affected_hit_point_maximum: i16,
    pub affected_hit_points: i16,
    pub affected_temporary_hit_points: i16,
    pub armor_class_bonus_active: bool,
    pub speed_delta_active: bool,
    pub special_speed_grant_active: bool,
    pub hit_point_maximum_increase_active: bool,
    pub caster_concentrating: bool,
    pub scenario_result: ScalarBuffScenarioResult,
    pub protocol: ScalarBuffProtocol,
}

#[must_use]
pub fn scalar_buff_active_effects_initial_state() -> ScalarBuffActiveEffectsState {
    base_state(ScalarBuffScenarioResult::Init, ScalarBuffProtocol::Init)
}

#[must_use]
pub fn resolve_shield_of_faith_scalar_buff() -> ScalarBuffActiveEffectsState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-S-Z.md
    // "Shield of Faith"; QNT:
    // battle-runtime-scalar-buff-active-effects.mbt.qnt.
    let mut state = resolved_state(ScalarBuffScenarioResult::ShieldOfFaith);
    state.affected_armor_class = 12;
    state.armor_class_bonus_active = true;
    state.caster_concentrating = true;
    state
}

#[must_use]
pub fn resolve_longstrider_scalar_buff() -> ScalarBuffActiveEffectsState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-E-L.md
    // "Longstrider"; QNT:
    // battle-runtime-scalar-buff-active-effects.mbt.qnt.
    let mut state = resolved_state(ScalarBuffScenarioResult::Longstrider);
    state.affected_speed_feet = 40;
    state.speed_delta_active = true;
    state
}

#[must_use]
pub fn resolve_spider_climb_scalar_buff() -> ScalarBuffActiveEffectsState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-S-Z.md
    // "Spider Climb"; QNT:
    // battle-runtime-scalar-buff-active-effects.mbt.qnt.
    let mut state = resolved_state(ScalarBuffScenarioResult::SpiderClimb);
    state.affected_climb_speed_feet = 30;
    state.special_speed_grant_active = true;
    state.caster_concentrating = true;
    state
}

#[must_use]
pub fn resolve_aid_scalar_buff() -> ScalarBuffActiveEffectsState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-A-D.md
    // "Aid"; QNT: battle-runtime-scalar-buff-active-effects.mbt.qnt.
    let mut state = resolved_state(ScalarBuffScenarioResult::Aid);
    state.affected_hit_point_maximum = 17;
    state.affected_hit_points = 17;
    state.hit_point_maximum_increase_active = true;
    state
}

#[must_use]
pub fn resolve_false_life_scalar_buff() -> ScalarBuffActiveEffectsState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-E-L.md
    // "False Life"; QNT: battle-runtime-scalar-buff-active-effects.mbt.qnt.
    let mut state = resolved_state(ScalarBuffScenarioResult::FalseLife);
    state.affected_temporary_hit_points = 9;
    state
}

#[must_use]
pub fn stutter_scalar_buff_active_effect() -> ScalarBuffActiveEffectsState {
    resolve_false_life_scalar_buff()
}

fn resolved_state(scenario_result: ScalarBuffScenarioResult) -> ScalarBuffActiveEffectsState {
    base_state(scenario_result, ScalarBuffProtocol::Resolved)
}

fn base_state(
    scenario_result: ScalarBuffScenarioResult,
    protocol: ScalarBuffProtocol,
) -> ScalarBuffActiveEffectsState {
    ScalarBuffActiveEffectsState {
        affected_armor_class: 10,
        affected_speed_feet: 30,
        affected_climb_speed_feet: 0,
        affected_hit_point_maximum: 12,
        affected_hit_points: 12,
        affected_temporary_hit_points: 0,
        armor_class_bonus_active: false,
        speed_delta_active: false,
        special_speed_grant_active: false,
        hit_point_maximum_increase_active: false,
        caster_concentrating: false,
        scenario_result,
        protocol,
    }
}
