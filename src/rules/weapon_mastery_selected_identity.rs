#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeaponMasteryRuntimeOutcome {
    Init,
    NeedsHoles,
    Resolved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeaponMasteryRuntimeHole {
    WitnessProtocol,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeaponMasteryProperty {
    Sap,
    Topple,
    Cleave,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeaponMasteryRuntimeProtocol {
    Init(Vec<WeaponMasteryRuntimeHole>),
    NeedsHoles(Vec<WeaponMasteryRuntimeHole>),
    Resolved,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeaponMasterySelectedIdentityState {
    pub primary_target_hit_points: i16,
    pub second_target_hit_points: i16,
    pub action_available: bool,
    pub primary_target_has_sap_effect: bool,
    pub primary_target_prone: bool,
    pub cleave_used: bool,
    pub outcome: WeaponMasteryRuntimeOutcome,
    pub protocol: WeaponMasteryRuntimeProtocol,
}

pub const WEAPON_MASTERY_TARGET_INITIAL_HIT_POINTS: i16 = 13;
pub const WEAPON_MASTERY_SAMPLE_WEAPON_DAMAGE: i16 = 4;

#[must_use]
pub fn weapon_mastery_selected_identity_initial_state() -> WeaponMasterySelectedIdentityState {
    projection(WeaponMasteryProjectionFacts {
        primary_target_hit_points: WEAPON_MASTERY_TARGET_INITIAL_HIT_POINTS,
        second_target_hit_points: WEAPON_MASTERY_TARGET_INITIAL_HIT_POINTS,
        action_available: true,
        primary_target_has_sap_effect: false,
        primary_target_prone: false,
        cleave_used: false,
        outcome: WeaponMasteryRuntimeOutcome::Init,
    })
}

#[must_use]
pub fn resolve_sap_mastery_property_hit() -> WeaponMasterySelectedIdentityState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Equipment.md
    // "Mastery Properties" / "Sap"; QNT:
    // battle-runtime-weapon-mastery-selected-identity.mbt.qnt.
    projection(WeaponMasteryProjectionFacts {
        primary_target_hit_points: damage_after_weapon_mastery_hit(
            WEAPON_MASTERY_TARGET_INITIAL_HIT_POINTS,
            WEAPON_MASTERY_SAMPLE_WEAPON_DAMAGE,
        ),
        second_target_hit_points: WEAPON_MASTERY_TARGET_INITIAL_HIT_POINTS,
        action_available: false,
        primary_target_has_sap_effect: true,
        primary_target_prone: false,
        cleave_used: false,
        outcome: WeaponMasteryRuntimeOutcome::Resolved,
    })
}

#[must_use]
pub fn resolve_topple_mastery_property_failed_saving_throw() -> WeaponMasterySelectedIdentityState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Equipment.md
    // "Mastery Properties" / "Topple"; QNT:
    // battle-runtime-weapon-mastery-selected-identity.mbt.qnt.
    projection(WeaponMasteryProjectionFacts {
        primary_target_hit_points: WEAPON_MASTERY_TARGET_INITIAL_HIT_POINTS,
        second_target_hit_points: WEAPON_MASTERY_TARGET_INITIAL_HIT_POINTS,
        action_available: true,
        primary_target_has_sap_effect: false,
        primary_target_prone: true,
        cleave_used: false,
        outcome: WeaponMasteryRuntimeOutcome::NeedsHoles,
    })
}

#[must_use]
pub fn resolve_cleave_mastery_property_second_target_hit() -> WeaponMasterySelectedIdentityState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Equipment.md
    // "Mastery Properties" / "Cleave"; QNT:
    // battle-runtime-weapon-mastery-selected-identity.mbt.qnt.
    let damaged_hit_points = damage_after_weapon_mastery_hit(
        WEAPON_MASTERY_TARGET_INITIAL_HIT_POINTS,
        WEAPON_MASTERY_SAMPLE_WEAPON_DAMAGE,
    );
    projection(WeaponMasteryProjectionFacts {
        primary_target_hit_points: damaged_hit_points,
        second_target_hit_points: damaged_hit_points,
        action_available: false,
        primary_target_has_sap_effect: false,
        primary_target_prone: false,
        cleave_used: true,
        outcome: WeaponMasteryRuntimeOutcome::Resolved,
    })
}

#[must_use]
pub fn damage_after_weapon_mastery_hit(hit_points: i16, damage: i16) -> i16 {
    (hit_points - damage).max(0)
}

struct WeaponMasteryProjectionFacts {
    primary_target_hit_points: i16,
    second_target_hit_points: i16,
    action_available: bool,
    primary_target_has_sap_effect: bool,
    primary_target_prone: bool,
    cleave_used: bool,
    outcome: WeaponMasteryRuntimeOutcome,
}

fn projection(facts: WeaponMasteryProjectionFacts) -> WeaponMasterySelectedIdentityState {
    let protocol = match facts.outcome {
        WeaponMasteryRuntimeOutcome::Init => {
            WeaponMasteryRuntimeProtocol::Init(vec![WeaponMasteryRuntimeHole::WitnessProtocol])
        }
        WeaponMasteryRuntimeOutcome::NeedsHoles => WeaponMasteryRuntimeProtocol::NeedsHoles(vec![
            WeaponMasteryRuntimeHole::WitnessProtocol,
        ]),
        WeaponMasteryRuntimeOutcome::Resolved => WeaponMasteryRuntimeProtocol::Resolved,
    };
    WeaponMasterySelectedIdentityState {
        primary_target_hit_points: facts.primary_target_hit_points,
        second_target_hit_points: facts.second_target_hit_points,
        action_available: facts.action_available,
        primary_target_has_sap_effect: facts.primary_target_has_sap_effect,
        primary_target_prone: facts.primary_target_prone,
        cleave_used: facts.cleave_used,
        outcome: facts.outcome,
        protocol,
    }
}
