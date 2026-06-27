use crate::rules::character_creation::{
    apply_creation_retained_reference_operation, completed_fighter_creation_state, route_payload,
    CreationRetainedReferenceOperation, CreationRouteEvent,
};
use crate::rules::class_features::{
    level_one_order_projection, Ability, AbilityCheckBonus, Cantrip, ClassLevel, ClassOrderFeature,
    ClassOrderProjection, ClericDivineOrder, DruidPrimalOrder, OrderChoice, OrderSelection, Skill,
};

pub const BRANCH_ACTIONS: &[&str] = &[
    "doSelectClericProtectorOrder",
    "doSelectClericThaumaturgeOrder",
    "doSelectDruidMagicianOrder",
    "doSelectDruidWardenOrder",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderWitness {
    pub last_result: &'static str,
    pub selected_order_unit_id: &'static str,
    pub selected_order_option_id: &'static str,
    pub extra_cantrip_unit_id: &'static str,
    pub selected_order_option_count: u8,
    pub selected_suborder_class_choice_feature_count: u8,
    pub order_unit_ref_present: bool,
    pub extra_cantrip_unit_ref_present: bool,
    pub martial_weapon_proficiency_present: bool,
    pub heavy_armor_training_present: bool,
    pub medium_armor_training_present: bool,
    pub ability_check_bonus_kind: &'static str,
    pub ability_check_bonus_feature_count: u8,
    pub total_level: u8,
}

pub fn replay_observed_action(observed_action_taken: &str) -> OrderWitness {
    match observed_action_taken {
        "doSelectClericProtectorOrder" => cleric_protector_replay(),
        "doSelectClericThaumaturgeOrder" => cleric_thaumaturge_replay(),
        "doSelectDruidMagicianOrder" => druid_magician_replay(),
        "doSelectDruidWardenOrder" => druid_warden_replay(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn replay_observed_route(_observed_action_taken: &str) -> Vec<CreationRouteEvent> {
    apply_creation_retained_reference_operation(
        &completed_fighter_creation_state(),
        CreationRetainedReferenceOperation::RetainOnly,
    )
    .route
}

pub fn expected_route(observed_action_taken: &str) -> Vec<CreationRouteEvent> {
    replay_observed_route(observed_action_taken)
}

pub fn route_projection_payload(route: &[CreationRouteEvent]) -> String {
    route_payload(route)
}

pub fn expected_cleric_protector_witness() -> OrderWitness {
    OrderWitness {
        last_result: "clericProtector",
        selected_order_unit_id: "cleric_divine_order",
        selected_order_option_id: "protector",
        extra_cantrip_unit_id: "none",
        selected_order_option_count: 1,
        selected_suborder_class_choice_feature_count: 0,
        order_unit_ref_present: true,
        extra_cantrip_unit_ref_present: false,
        martial_weapon_proficiency_present: true,
        heavy_armor_training_present: true,
        medium_armor_training_present: true,
        ability_check_bonus_kind: "none",
        ability_check_bonus_feature_count: 0,
        total_level: 1,
    }
}

pub fn expected_cleric_thaumaturge_witness() -> OrderWitness {
    OrderWitness {
        last_result: "clericThaumaturge",
        selected_order_unit_id: "cleric_divine_order",
        selected_order_option_id: "thaumaturge",
        extra_cantrip_unit_id: "light",
        selected_order_option_count: 1,
        selected_suborder_class_choice_feature_count: 0,
        order_unit_ref_present: true,
        extra_cantrip_unit_ref_present: true,
        martial_weapon_proficiency_present: false,
        heavy_armor_training_present: false,
        medium_armor_training_present: true,
        ability_check_bonus_kind: "int_arcana_religion_wis_min1",
        ability_check_bonus_feature_count: 1,
        total_level: 1,
    }
}

pub fn expected_druid_magician_witness() -> OrderWitness {
    OrderWitness {
        last_result: "druidMagician",
        selected_order_unit_id: "druid_primal_order",
        selected_order_option_id: "magician",
        extra_cantrip_unit_id: "guidance",
        selected_order_option_count: 1,
        selected_suborder_class_choice_feature_count: 0,
        order_unit_ref_present: true,
        extra_cantrip_unit_ref_present: true,
        martial_weapon_proficiency_present: false,
        heavy_armor_training_present: false,
        medium_armor_training_present: false,
        ability_check_bonus_kind: "int_arcana_nature_wis_min1",
        ability_check_bonus_feature_count: 1,
        total_level: 1,
    }
}

pub fn expected_druid_warden_witness() -> OrderWitness {
    OrderWitness {
        last_result: "druidWarden",
        selected_order_unit_id: "druid_primal_order",
        selected_order_option_id: "warden",
        extra_cantrip_unit_id: "none",
        selected_order_option_count: 1,
        selected_suborder_class_choice_feature_count: 0,
        order_unit_ref_present: true,
        extra_cantrip_unit_ref_present: false,
        martial_weapon_proficiency_present: true,
        heavy_armor_training_present: false,
        medium_armor_training_present: true,
        ability_check_bonus_kind: "none",
        ability_check_bonus_feature_count: 0,
        total_level: 1,
    }
}

pub fn projection_payload(witness: &OrderWitness) -> String {
    [
        format!("lastResult={}", witness.last_result),
        format!("selectedOrderUnitId={}", witness.selected_order_unit_id),
        format!("selectedOrderOptionId={}", witness.selected_order_option_id),
        format!("extraCantripUnitId={}", witness.extra_cantrip_unit_id),
        format!(
            "selectedOrderOptionCount={}",
            witness.selected_order_option_count
        ),
        format!(
            "selectedSuborderClassChoiceFeatureCount={}",
            witness.selected_suborder_class_choice_feature_count
        ),
        format!("orderUnitRefPresent={}", witness.order_unit_ref_present),
        format!(
            "extraCantripUnitRefPresent={}",
            witness.extra_cantrip_unit_ref_present
        ),
        format!(
            "martialWeaponProficiencyPresent={}",
            witness.martial_weapon_proficiency_present
        ),
        format!(
            "heavyArmorTrainingPresent={}",
            witness.heavy_armor_training_present
        ),
        format!(
            "mediumArmorTrainingPresent={}",
            witness.medium_armor_training_present
        ),
        format!("abilityCheckBonusKind={}", witness.ability_check_bonus_kind),
        format!(
            "abilityCheckBonusFeatureCount={}",
            witness.ability_check_bonus_feature_count
        ),
        format!("totalLevel={}", witness.total_level),
    ]
    .join("\n")
}

fn cleric_protector_replay() -> OrderWitness {
    let observed = level_one_order_projection(OrderSelection::Cleric(ClericDivineOrder::Protector));
    order_projection_witness("clericProtector", &observed)
}

fn cleric_thaumaturge_replay() -> OrderWitness {
    let observed =
        level_one_order_projection(OrderSelection::Cleric(ClericDivineOrder::Thaumaturge {
            extra_cantrip: Cantrip::Light,
        }));
    order_projection_witness("clericThaumaturge", &observed)
}

fn druid_magician_replay() -> OrderWitness {
    let observed = level_one_order_projection(OrderSelection::Druid(DruidPrimalOrder::Magician {
        extra_cantrip: Cantrip::Guidance,
    }));
    order_projection_witness("druidMagician", &observed)
}

fn druid_warden_replay() -> OrderWitness {
    let observed = level_one_order_projection(OrderSelection::Druid(DruidPrimalOrder::Warden));
    order_projection_witness("druidWarden", &observed)
}

fn order_projection_witness(
    last_result: &'static str,
    projection: &ClassOrderProjection,
) -> OrderWitness {
    OrderWitness {
        last_result,
        selected_order_unit_id: order_feature_ref(projection.feature),
        selected_order_option_id: order_choice_ref(projection.selected_choice),
        extra_cantrip_unit_id: projection.extra_cantrip.map(cantrip_ref).unwrap_or("none"),
        selected_order_option_count: projection.selected_order_option_count,
        selected_suborder_class_choice_feature_count: projection
            .selected_suborder_class_choice_feature_count,
        order_unit_ref_present: true,
        extra_cantrip_unit_ref_present: projection.extra_cantrip.is_some(),
        martial_weapon_proficiency_present: projection.training.martial_weapon_proficiency,
        heavy_armor_training_present: projection.training.heavy_armor_training,
        medium_armor_training_present: projection.training.medium_armor_training,
        ability_check_bonus_kind: projection
            .ability_check_bonus
            .map(ability_check_bonus_ref)
            .unwrap_or("none"),
        ability_check_bonus_feature_count: u8::from(projection.ability_check_bonus.is_some()),
        total_level: class_level(projection.total_level),
    }
}

fn order_feature_ref(feature: ClassOrderFeature) -> &'static str {
    match feature {
        ClassOrderFeature::DivineOrder => "cleric_divine_order",
        ClassOrderFeature::PrimalOrder => "druid_primal_order",
    }
}

fn order_choice_ref(choice: OrderChoice) -> &'static str {
    match choice {
        OrderChoice::Protector => "protector",
        OrderChoice::Thaumaturge => "thaumaturge",
        OrderChoice::Magician => "magician",
        OrderChoice::Warden => "warden",
    }
}

fn cantrip_ref(cantrip: Cantrip) -> &'static str {
    match cantrip {
        Cantrip::Guidance => "guidance",
        Cantrip::Light => "light",
    }
}

fn ability_check_bonus_ref(bonus: AbilityCheckBonus) -> &'static str {
    match (
        bonus.check_ability,
        bonus.skills,
        bonus.bonus_ability,
        bonus.minimum_bonus,
    ) {
        (Ability::Intelligence, [Skill::Arcana, Skill::Religion], Ability::Wisdom, 1) => {
            "int_arcana_religion_wis_min1"
        }
        (Ability::Intelligence, [Skill::Arcana, Skill::Nature], Ability::Wisdom, 1) => {
            "int_arcana_nature_wis_min1"
        }
        _ => "unknown",
    }
}

fn class_level(level: ClassLevel) -> u8 {
    match level {
        ClassLevel::One => 1,
        ClassLevel::Two => 2,
    }
}
