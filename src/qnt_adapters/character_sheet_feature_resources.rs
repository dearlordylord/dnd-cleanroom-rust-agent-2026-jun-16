use crate::rules::feature_resources::{
    apply_lay_on_hands_resource, apply_uncanny_metabolism, can_apply_lay_on_hands_resource,
    can_convert_font_of_magic_slot_to_sorcery_points, can_create_font_of_magic_spell_slot,
    can_use_uncanny_metabolism, complete_long_rest_feature_resource_rest_state,
    complete_short_rest_feature_resource_rest_state, convert_font_of_magic_slot_to_sorcery_points,
    create_font_of_magic_spell_slot, font_of_magic_spell_slot_source_requires_choice,
    project_metamagic_shared_sorcery_points, FeatureResourceHitPoints, FeatureResourceRestState,
    MetamagicSharedSorceryPoints, ResourcePoolFacts,
};

use crate::rules::character_battle_handoff::{
    route_enter_battle_runtime, route_project_character_sheet_to_battle,
    route_reject_character_battle_handoff, CharacterBattleRouteEvent,
    CharacterBattleRouteFillFamily, CharacterBattleRouteHoleFamily, CharacterBattleRouteOwnerGroup,
    CharacterBattleRouteSubjectFamily,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SheetFeatureResourceFacts {
    pub source_current_hp: i16,
    pub target_current_hp: i16,
    pub temporary_hit_points: i16,
    pub target_poisoned: bool,
    pub lay_on_hands_capacity: i16,
    pub lay_on_hands_expended: i16,
    pub druid_wild_shape_expended: i16,
    pub monk_focus_expended: i16,
    pub sorcery_point_capacity: i16,
    pub sorcery_point_expended: i16,
    pub ordinary_level2_expended: i16,
    pub created_level3_capacity: i16,
    pub created_level3_expended: i16,
    pub uncanny_used_since_long_rest: bool,
    pub metamagic_known_options: u8,
    pub metamagic_shared_resource_expended: i16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SheetFeatureResourceWitness {
    pub last_result: &'static str,
    pub accepted: bool,
    pub message: &'static str,
    pub source_current_hp: i16,
    pub target_current_hp: i16,
    pub temporary_hit_points: i16,
    pub target_poisoned: bool,
    pub lay_on_hands_capacity: i16,
    pub lay_on_hands_expended: i16,
    pub druid_wild_shape_expended: i16,
    pub monk_focus_expended: i16,
    pub sorcery_point_capacity: i16,
    pub sorcery_point_expended: i16,
    pub ordinary_level2_expended: i16,
    pub created_level3_capacity: i16,
    pub created_level3_expended: i16,
    pub uncanny_used_since_long_rest: bool,
    pub metamagic_known_options: u8,
    pub metamagic_shared_resource_expended: i16,
    pub replay_index: u8,
}

pub const BRANCH_ACTIONS: [&str; 14] = [
    "doLayOnHandsRestoresHpAndRemovesPoisoned",
    "doRejectLayOnHandsOverspend",
    "doLongRestClearsLayOnHandsPool",
    "doShortRestRecoversUseCountPools",
    "doLongRestClearsPointPoolAndUseState",
    "doFontOfMagicSlotToPoints",
    "doRejectFontOfMagicAmbiguousSlotSource",
    "doFontOfMagicPointsToSlot",
    "doRejectFontOfMagicInsufficientPoints",
    "doShortRestPreservesUncannyUseState",
    "doLongRestClearsUncannyUseState",
    "doUncannyMetabolismRecoversFocusAndHeals",
    "doRejectUncannyMetabolismRepeatUse",
    "doMetamagicBridgeUsesSharedPointPool",
];

pub fn replay_observed_action(observed_action_taken: &str) -> SheetFeatureResourceWitness {
    match observed_action_taken {
        "doLayOnHandsRestoresHpAndRemovesPoisoned" => {
            lay_on_hands_restores_hp_and_removes_poisoned()
        }
        "doRejectLayOnHandsOverspend" => reject_lay_on_hands_overspend(),
        "doLongRestClearsLayOnHandsPool" => long_rest_clears_lay_on_hands_pool(),
        "doShortRestRecoversUseCountPools" => short_rest_recovers_use_count_pools(),
        "doLongRestClearsPointPoolAndUseState" => long_rest_clears_point_pool_and_use_state(),
        "doFontOfMagicSlotToPoints" => font_of_magic_slot_to_points(),
        "doRejectFontOfMagicAmbiguousSlotSource" => reject_font_of_magic_ambiguous_slot_source(),
        "doFontOfMagicPointsToSlot" => font_of_magic_points_to_slot(),
        "doRejectFontOfMagicInsufficientPoints" => reject_font_of_magic_insufficient_points(),
        "doShortRestPreservesUncannyUseState" => short_rest_preserves_uncanny_use_state(),
        "doLongRestClearsUncannyUseState" => long_rest_clears_uncanny_use_state(),
        "doUncannyMetabolismRecoversFocusAndHeals" => uncanny_metabolism_recovers_focus_and_heals(),
        "doRejectUncannyMetabolismRepeatUse" => reject_uncanny_metabolism_repeat_use(),
        "doMetamagicBridgeUsesSharedPointPool" => metamagic_bridge_uses_shared_point_pool(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> SheetFeatureResourceWitness {
    match observed_action_taken {
        "doLayOnHandsRestoresHpAndRemovesPoisoned" => {
            expected_lay_on_hands_restores_hp_and_removes_poisoned()
        }
        "doRejectLayOnHandsOverspend" => expected_reject_lay_on_hands_overspend(),
        "doLongRestClearsLayOnHandsPool" => expected_long_rest_clears_lay_on_hands_pool(),
        "doShortRestRecoversUseCountPools" => expected_short_rest_recovers_use_count_pools(),
        "doLongRestClearsPointPoolAndUseState" => {
            expected_long_rest_clears_point_pool_and_use_state()
        }
        "doFontOfMagicSlotToPoints" => expected_font_of_magic_slot_to_points(),
        "doRejectFontOfMagicAmbiguousSlotSource" => {
            expected_reject_font_of_magic_ambiguous_slot_source()
        }
        "doFontOfMagicPointsToSlot" => expected_font_of_magic_points_to_slot(),
        "doRejectFontOfMagicInsufficientPoints" => {
            expected_reject_font_of_magic_insufficient_points()
        }
        "doShortRestPreservesUncannyUseState" => expected_short_rest_preserves_uncanny_use_state(),
        "doLongRestClearsUncannyUseState" => expected_long_rest_clears_uncanny_use_state(),
        "doUncannyMetabolismRecoversFocusAndHeals" => {
            expected_uncanny_metabolism_recovers_focus_and_heals()
        }
        "doRejectUncannyMetabolismRepeatUse" => expected_reject_uncanny_metabolism_repeat_use(),
        "doMetamagicBridgeUsesSharedPointPool" => {
            expected_metamagic_bridge_uses_shared_point_pool()
        }
        action => panic!("unsupported expected mbt::actionTaken {action}"),
    }
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<CharacterBattleRouteEvent> {
    feature_resource_route_through_action(observed_action_taken)
}

pub fn expected_route(observed_action_taken: &str) -> Vec<CharacterBattleRouteEvent> {
    expected_feature_resource_route_through_action(observed_action_taken)
}

pub fn projection_payload(witness: &SheetFeatureResourceWitness) -> String {
    [
        format!("lastResult={}", witness.last_result),
        format!("accepted={}", witness.accepted),
        format!("message={}", witness.message),
        format!("sourceCurrentHp={}", witness.source_current_hp),
        format!("targetCurrentHp={}", witness.target_current_hp),
        format!("temporaryHitPoints={}", witness.temporary_hit_points),
        format!("targetPoisoned={}", witness.target_poisoned),
        format!("layOnHandsCapacity={}", witness.lay_on_hands_capacity),
        format!("layOnHandsExpended={}", witness.lay_on_hands_expended),
        format!(
            "druidWildShapeExpended={}",
            witness.druid_wild_shape_expended
        ),
        format!("monkFocusExpended={}", witness.monk_focus_expended),
        format!("sorceryPointCapacity={}", witness.sorcery_point_capacity),
        format!("sorceryPointExpended={}", witness.sorcery_point_expended),
        format!(
            "ordinaryLevel2Expended={}",
            witness.ordinary_level2_expended
        ),
        format!("createdLevel3Capacity={}", witness.created_level3_capacity),
        format!("createdLevel3Expended={}", witness.created_level3_expended),
        format!(
            "uncannyUsedSinceLongRest={}",
            witness.uncanny_used_since_long_rest
        ),
        format!("metamagicKnownOptions={}", witness.metamagic_known_options),
        format!(
            "metamagicSharedResourceExpended={}",
            witness.metamagic_shared_resource_expended
        ),
        format!("replayIndex={}", witness.replay_index),
    ]
    .join("\n")
}

fn lay_on_hands_restores_hp_and_removes_poisoned() -> SheetFeatureResourceWitness {
    let sheet = SheetFeatureResourceFacts {
        source_current_hp: 12,
        target_current_hp: 3,
        target_poisoned: true,
        lay_on_hands_capacity: 10,
        ..empty_feature_resource_facts()
    };
    let applied = apply_lay_on_hands_to_sheet(sheet, 10, 2, 1);

    record_projection(
        "lay-on-hands-restores-hp-and-removes-poisoned",
        true,
        "none",
        applied,
        1,
    )
}

fn reject_lay_on_hands_overspend() -> SheetFeatureResourceWitness {
    let sheet = SheetFeatureResourceFacts {
        source_current_hp: 6,
        target_current_hp: 6,
        target_poisoned: true,
        lay_on_hands_capacity: 5,
        ..empty_feature_resource_facts()
    };
    if can_apply_lay_on_hands_resource(lay_on_hands_pool(sheet), 1, 1) {
        panic!("Lay On Hands overspend witness unexpectedly accepted");
    }

    record_projection(
        "lay-on-hands-overspend-rejected",
        false,
        "Lay On Hands cannot spend more healing pool than remains.",
        sheet,
        2,
    )
}

fn long_rest_clears_lay_on_hands_pool() -> SheetFeatureResourceWitness {
    let sheet = SheetFeatureResourceFacts {
        source_current_hp: 11,
        target_current_hp: 11,
        lay_on_hands_capacity: 5,
        lay_on_hands_expended: 4,
        ..empty_feature_resource_facts()
    };

    record_projection(
        "long-rest-clears-lay-on-hands-pool",
        true,
        "none",
        complete_long_rest_feature_resources(sheet),
        3,
    )
}

fn expected_lay_on_hands_restores_hp_and_removes_poisoned() -> SheetFeatureResourceWitness {
    expected_record_projection(
        "lay-on-hands-restores-hp-and-removes-poisoned",
        true,
        "none",
        SheetFeatureResourceFacts {
            source_current_hp: 12,
            target_current_hp: 5,
            target_poisoned: false,
            lay_on_hands_capacity: 10,
            lay_on_hands_expended: 7,
            ..empty_feature_resource_facts()
        },
        1,
    )
}

fn expected_reject_lay_on_hands_overspend() -> SheetFeatureResourceWitness {
    expected_record_projection(
        "lay-on-hands-overspend-rejected",
        false,
        "Lay On Hands cannot spend more healing pool than remains.",
        SheetFeatureResourceFacts {
            source_current_hp: 6,
            target_current_hp: 6,
            target_poisoned: true,
            lay_on_hands_capacity: 5,
            ..empty_feature_resource_facts()
        },
        2,
    )
}

fn expected_long_rest_clears_lay_on_hands_pool() -> SheetFeatureResourceWitness {
    expected_record_projection(
        "long-rest-clears-lay-on-hands-pool",
        true,
        "none",
        SheetFeatureResourceFacts {
            source_current_hp: 11,
            target_current_hp: 11,
            lay_on_hands_capacity: 5,
            ..empty_feature_resource_facts()
        },
        3,
    )
}

fn expected_short_rest_recovers_use_count_pools() -> SheetFeatureResourceWitness {
    expected_record_projection(
        "short-rest-recovers-use-count-pools",
        true,
        "none",
        SheetFeatureResourceFacts {
            druid_wild_shape_expended: 1,
            ..empty_feature_resource_facts()
        },
        4,
    )
}

fn expected_long_rest_clears_point_pool_and_use_state() -> SheetFeatureResourceWitness {
    expected_record_projection(
        "long-rest-clears-point-pool-and-use-state",
        true,
        "none",
        SheetFeatureResourceFacts {
            sorcery_point_capacity: 2,
            ..empty_feature_resource_facts()
        },
        5,
    )
}

fn expected_font_of_magic_slot_to_points() -> SheetFeatureResourceWitness {
    expected_record_projection(
        "font-of-magic-slot-to-points",
        true,
        "none",
        SheetFeatureResourceFacts {
            sorcery_point_capacity: 3,
            sorcery_point_expended: 1,
            ordinary_level2_expended: 2,
            ..empty_feature_resource_facts()
        },
        6,
    )
}

fn expected_reject_font_of_magic_ambiguous_slot_source() -> SheetFeatureResourceWitness {
    expected_record_projection(
        "font-of-magic-ambiguous-slot-source-rejected",
        false,
        "Font of Magic conversion requires a Spell Slot source when ordinary and created Spell Slots are both available.",
        SheetFeatureResourceFacts {
            sorcery_point_capacity: 5,
            sorcery_point_expended: 5,
            created_level3_capacity: 1,
            ..empty_feature_resource_facts()
        },
        7,
    )
}

fn expected_font_of_magic_points_to_slot() -> SheetFeatureResourceWitness {
    expected_record_projection(
        "font-of-magic-points-to-slot",
        true,
        "none",
        SheetFeatureResourceFacts {
            sorcery_point_capacity: 5,
            sorcery_point_expended: 5,
            created_level3_capacity: 1,
            ..empty_feature_resource_facts()
        },
        8,
    )
}

fn expected_reject_font_of_magic_insufficient_points() -> SheetFeatureResourceWitness {
    expected_record_projection(
        "font-of-magic-insufficient-points-rejected",
        false,
        "Font of Magic Spell Slot creation requires enough unexpended Sorcery Points.",
        SheetFeatureResourceFacts {
            sorcery_point_capacity: 3,
            sorcery_point_expended: 1,
            ..empty_feature_resource_facts()
        },
        9,
    )
}

fn expected_short_rest_preserves_uncanny_use_state() -> SheetFeatureResourceWitness {
    expected_record_projection(
        "short-rest-preserves-uncanny-use-state",
        true,
        "none",
        SheetFeatureResourceFacts {
            uncanny_used_since_long_rest: true,
            ..empty_feature_resource_facts()
        },
        10,
    )
}

fn expected_long_rest_clears_uncanny_use_state() -> SheetFeatureResourceWitness {
    expected_record_projection(
        "long-rest-clears-uncanny-use-state",
        true,
        "none",
        empty_feature_resource_facts(),
        11,
    )
}

fn expected_uncanny_metabolism_recovers_focus_and_heals() -> SheetFeatureResourceWitness {
    expected_record_projection(
        "uncanny-metabolism-recovers-focus-and-heals",
        true,
        "none",
        SheetFeatureResourceFacts {
            source_current_hp: 14,
            temporary_hit_points: 3,
            uncanny_used_since_long_rest: true,
            ..empty_feature_resource_facts()
        },
        12,
    )
}

fn expected_reject_uncanny_metabolism_repeat_use() -> SheetFeatureResourceWitness {
    expected_record_projection(
        "uncanny-metabolism-repeat-use-rejected",
        false,
        "Uncanny Metabolism cannot be used again until a Long Rest.",
        SheetFeatureResourceFacts {
            source_current_hp: 14,
            temporary_hit_points: 3,
            uncanny_used_since_long_rest: true,
            ..empty_feature_resource_facts()
        },
        13,
    )
}

fn expected_metamagic_bridge_uses_shared_point_pool() -> SheetFeatureResourceWitness {
    expected_record_projection(
        "metamagic-bridge-uses-shared-point-pool",
        true,
        "none",
        SheetFeatureResourceFacts {
            sorcery_point_capacity: 5,
            sorcery_point_expended: 3,
            metamagic_known_options: 2,
            metamagic_shared_resource_expended: 3,
            ..empty_feature_resource_facts()
        },
        14,
    )
}

fn expected_record_projection(
    last_result: &'static str,
    accepted: bool,
    message: &'static str,
    sheet: SheetFeatureResourceFacts,
    replay_index: u8,
) -> SheetFeatureResourceWitness {
    record_projection(last_result, accepted, message, sheet, replay_index)
}

fn short_rest_recovers_use_count_pools() -> SheetFeatureResourceWitness {
    let sheet = SheetFeatureResourceFacts {
        druid_wild_shape_expended: 2,
        monk_focus_expended: 2,
        ..empty_feature_resource_facts()
    };

    record_projection(
        "short-rest-recovers-use-count-pools",
        true,
        "none",
        short_rest_feature_resources(sheet),
        4,
    )
}

fn long_rest_clears_point_pool_and_use_state() -> SheetFeatureResourceWitness {
    let sheet = SheetFeatureResourceFacts {
        sorcery_point_capacity: 2,
        sorcery_point_expended: 2,
        uncanny_used_since_long_rest: true,
        ..empty_feature_resource_facts()
    };

    record_projection(
        "long-rest-clears-point-pool-and-use-state",
        true,
        "none",
        complete_long_rest_feature_resources(sheet),
        5,
    )
}

fn font_of_magic_slot_to_points() -> SheetFeatureResourceWitness {
    let sheet = SheetFeatureResourceFacts {
        sorcery_point_capacity: 3,
        sorcery_point_expended: 3,
        ordinary_level2_expended: 1,
        ..empty_feature_resource_facts()
    };
    if !can_convert_font_of_magic_slot_to_sorcery_points(
        sorcery_point_pool(sheet),
        2,
        ordinary_level2_slot_pool(sheet),
        feature_resource_pool(0, 0),
    ) {
        panic!("Font of Magic slot-to-points witness unexpectedly rejected");
    }

    record_projection(
        "font-of-magic-slot-to-points",
        true,
        "none",
        convert_font_of_magic_level2_slot_to_points_on_sheet(sheet),
        6,
    )
}

fn reject_font_of_magic_ambiguous_slot_source() -> SheetFeatureResourceWitness {
    let sheet = SheetFeatureResourceFacts {
        sorcery_point_capacity: 5,
        sorcery_point_expended: 5,
        created_level3_capacity: 1,
        ..empty_feature_resource_facts()
    };
    if !font_of_magic_spell_slot_source_requires_choice(
        ordinary_level2_slot_pool(sheet),
        created_level3_slot_pool(sheet),
    ) {
        panic!("Font of Magic ambiguous-source witness unexpectedly had a single source");
    }

    record_projection(
        "font-of-magic-ambiguous-slot-source-rejected",
        false,
        "Font of Magic conversion requires a Spell Slot source when ordinary and created Spell Slots are both available.",
        sheet,
        7,
    )
}

fn font_of_magic_points_to_slot() -> SheetFeatureResourceWitness {
    let sheet = SheetFeatureResourceFacts {
        sorcery_point_capacity: 5,
        created_level3_capacity: 0,
        created_level3_expended: 0,
        ..empty_feature_resource_facts()
    };
    if !can_create_font_of_magic_spell_slot(
        sorcery_point_pool(sheet),
        created_level3_slot_pool(sheet),
        5,
        3,
    ) {
        panic!("Font of Magic points-to-slot witness unexpectedly rejected");
    }

    record_projection(
        "font-of-magic-points-to-slot",
        true,
        "none",
        create_font_of_magic_level3_slot_on_sheet(sheet),
        8,
    )
}

fn reject_font_of_magic_insufficient_points() -> SheetFeatureResourceWitness {
    let sheet = SheetFeatureResourceFacts {
        sorcery_point_capacity: 3,
        sorcery_point_expended: 1,
        ..empty_feature_resource_facts()
    };
    if can_create_font_of_magic_spell_slot(
        sorcery_point_pool(sheet),
        created_level3_slot_pool(sheet),
        3,
        2,
    ) {
        panic!("Font of Magic insufficient-points witness unexpectedly accepted");
    }

    record_projection(
        "font-of-magic-insufficient-points-rejected",
        false,
        "Font of Magic Spell Slot creation requires enough unexpended Sorcery Points.",
        sheet,
        9,
    )
}

fn short_rest_preserves_uncanny_use_state() -> SheetFeatureResourceWitness {
    let sheet = SheetFeatureResourceFacts {
        monk_focus_expended: 2,
        uncanny_used_since_long_rest: true,
        ..empty_feature_resource_facts()
    };

    record_projection(
        "short-rest-preserves-uncanny-use-state",
        true,
        "none",
        short_rest_feature_resources(sheet),
        10,
    )
}

fn long_rest_clears_uncanny_use_state() -> SheetFeatureResourceWitness {
    let sheet = SheetFeatureResourceFacts {
        monk_focus_expended: 2,
        uncanny_used_since_long_rest: true,
        ..empty_feature_resource_facts()
    };

    record_projection(
        "long-rest-clears-uncanny-use-state",
        true,
        "none",
        complete_long_rest_feature_resources(sheet),
        11,
    )
}

fn uncanny_metabolism_recovers_focus_and_heals() -> SheetFeatureResourceWitness {
    let sheet = SheetFeatureResourceFacts {
        source_current_hp: 8,
        temporary_hit_points: 3,
        monk_focus_expended: 2,
        ..empty_feature_resource_facts()
    };

    record_projection(
        "uncanny-metabolism-recovers-focus-and-heals",
        true,
        "none",
        use_uncanny_metabolism(sheet, 15, 2, 4),
        12,
    )
}

fn reject_uncanny_metabolism_repeat_use() -> SheetFeatureResourceWitness {
    let sheet = SheetFeatureResourceFacts {
        source_current_hp: 14,
        temporary_hit_points: 3,
        uncanny_used_since_long_rest: true,
        ..empty_feature_resource_facts()
    };
    if can_use_uncanny_metabolism(sheet.uncanny_used_since_long_rest) {
        panic!("Uncanny Metabolism repeat-use witness unexpectedly accepted");
    }

    record_projection(
        "uncanny-metabolism-repeat-use-rejected",
        false,
        "Uncanny Metabolism cannot be used again until a Long Rest.",
        sheet,
        13,
    )
}

fn metamagic_bridge_uses_shared_point_pool() -> SheetFeatureResourceWitness {
    let sheet = SheetFeatureResourceFacts {
        sorcery_point_capacity: 5,
        sorcery_point_expended: 3,
        metamagic_known_options: 2,
        ..empty_feature_resource_facts()
    };
    let projected = project_metamagic_shared_sorcery_points(MetamagicSharedSorceryPoints {
        sorcery_points: sorcery_point_pool(sheet),
        known_options: sheet.metamagic_known_options,
    })
    .expect("metamagic bridge witness uses a legal Sorcery Point pool");

    record_projection(
        "metamagic-bridge-uses-shared-point-pool",
        true,
        "none",
        SheetFeatureResourceFacts {
            metamagic_shared_resource_expended: projected.shared_resource_expended,
            metamagic_known_options: projected.known_options,
            ..sheet
        },
        14,
    )
}

fn empty_feature_resource_facts() -> SheetFeatureResourceFacts {
    SheetFeatureResourceFacts {
        source_current_hp: 0,
        target_current_hp: 0,
        temporary_hit_points: 0,
        target_poisoned: false,
        lay_on_hands_capacity: 0,
        lay_on_hands_expended: 0,
        druid_wild_shape_expended: 0,
        monk_focus_expended: 0,
        sorcery_point_capacity: 0,
        sorcery_point_expended: 0,
        ordinary_level2_expended: 0,
        created_level3_capacity: 0,
        created_level3_expended: 0,
        uncanny_used_since_long_rest: false,
        metamagic_known_options: 0,
        metamagic_shared_resource_expended: 0,
    }
}

fn feature_resource_route_through_action(
    observed_action_taken: &str,
) -> Vec<CharacterBattleRouteEvent> {
    build_feature_resource_route_through_action(observed_action_taken)
}

fn expected_feature_resource_route_through_action(
    observed_action_taken: &str,
) -> Vec<CharacterBattleRouteEvent> {
    match observed_action_taken {
        "doLayOnHandsRestoresHpAndRemovesPoisoned" => {
            expected_route_after_lay_on_hands_restores_hp_and_removes_poisoned()
        }
        "doRejectLayOnHandsOverspend" => expected_route_after_reject_lay_on_hands_overspend(),
        "doLongRestClearsLayOnHandsPool" => {
            expected_route_after_long_rest_clears_lay_on_hands_pool()
        }
        "doShortRestRecoversUseCountPools" => {
            expected_route_after_short_rest_recovers_use_count_pools()
        }
        "doLongRestClearsPointPoolAndUseState" => {
            expected_route_after_long_rest_clears_point_pool_and_use_state()
        }
        "doFontOfMagicSlotToPoints" => expected_route_after_font_of_magic_slot_to_points(),
        "doRejectFontOfMagicAmbiguousSlotSource" => {
            expected_route_after_reject_font_of_magic_ambiguous_slot_source()
        }
        "doFontOfMagicPointsToSlot" => expected_route_after_font_of_magic_points_to_slot(),
        "doRejectFontOfMagicInsufficientPoints" => {
            expected_route_after_reject_font_of_magic_insufficient_points()
        }
        "doShortRestPreservesUncannyUseState" => {
            expected_route_after_short_rest_preserves_uncanny_use_state()
        }
        "doLongRestClearsUncannyUseState" => {
            expected_route_after_long_rest_clears_uncanny_use_state()
        }
        "doUncannyMetabolismRecoversFocusAndHeals" => {
            expected_route_after_uncanny_metabolism_recovers_focus_and_heals()
        }
        "doRejectUncannyMetabolismRepeatUse" => {
            expected_route_after_reject_uncanny_metabolism_repeat_use()
        }
        "doMetamagicBridgeUsesSharedPointPool" => {
            expected_route_after_metamagic_bridge_uses_shared_point_pool()
        }
        action => panic!("unsupported expected route action {action}"),
    }
}

fn build_feature_resource_route_through_action(
    observed_action_taken: &str,
) -> Vec<CharacterBattleRouteEvent> {
    let mut route = initial_feature_resource_handoff_route();
    for action in BRANCH_ACTIONS {
        append_feature_resource_route_action(&mut route, action);
        if action == observed_action_taken {
            return route;
        }
    }
    panic!("unsupported route mbt::actionTaken {observed_action_taken}");
}

fn initial_feature_resource_handoff_route() -> Vec<CharacterBattleRouteEvent> {
    vec![route_project_character_sheet_to_battle(
        CharacterBattleRouteSubjectFamily::HandoffFeatureResourceProjectionRouteSubject,
        CharacterBattleRouteOwnerGroup::CharacterBattleSheetOwner,
    )]
}

fn append_feature_resource_route_action(
    route: &mut Vec<CharacterBattleRouteEvent>,
    observed_action_taken: &str,
) {
    match observed_action_taken {
        "doLayOnHandsRestoresHpAndRemovesPoisoned" => {
            append_accepted_feature_resource_with_hit_point_route(route);
        }
        "doRejectLayOnHandsOverspend"
        | "doRejectFontOfMagicInsufficientPoints"
        | "doRejectUncannyMetabolismRepeatUse" => {
            append_rejected_feature_resource_route(
                route,
                vec![CharacterBattleRouteHoleFamily::HandoffFeatureResourceProjectionHoleFamily],
            );
        }
        "doLongRestClearsLayOnHandsPool"
        | "doShortRestRecoversUseCountPools"
        | "doLongRestClearsPointPoolAndUseState"
        | "doShortRestPreservesUncannyUseState"
        | "doLongRestClearsUncannyUseState" => {
            append_accepted_feature_resource_route(route);
        }
        "doFontOfMagicSlotToPoints" | "doFontOfMagicPointsToSlot" => {
            append_accepted_spell_resource_route(route);
        }
        "doRejectFontOfMagicAmbiguousSlotSource" => {
            append_rejected_spell_resource_route(
                route,
                vec![
                    CharacterBattleRouteHoleFamily::HandoffSpellResourceProjectionHoleFamily,
                    CharacterBattleRouteHoleFamily::HandoffSettlementConflictHoleFamily,
                ],
            );
        }
        "doUncannyMetabolismRecoversFocusAndHeals" => {
            append_accepted_feature_resource_with_hit_point_route(route);
        }
        "doMetamagicBridgeUsesSharedPointPool" => {
            append_metamagic_battle_bridge_route(route);
        }
        action => panic!("unsupported route action {action}"),
    }
}

fn append_accepted_feature_resource_route(route: &mut Vec<CharacterBattleRouteEvent>) {
    route.push(route_project_character_sheet_to_battle(
        CharacterBattleRouteSubjectFamily::HandoffFeatureResourceProjectionRouteSubject,
        CharacterBattleRouteOwnerGroup::CharacterBattleResourceProjectionOwner,
    ));
}

fn append_accepted_feature_resource_with_hit_point_route(
    route: &mut Vec<CharacterBattleRouteEvent>,
) {
    append_accepted_feature_resource_route(route);
    route.push(route_project_character_sheet_to_battle(
        CharacterBattleRouteSubjectFamily::SheetToBattleInitRouteSubject,
        CharacterBattleRouteOwnerGroup::CharacterBattleSheetOwner,
    ));
}

fn append_rejected_feature_resource_route(
    route: &mut Vec<CharacterBattleRouteEvent>,
    holes: Vec<CharacterBattleRouteHoleFamily>,
) {
    route.push(route_reject_character_battle_handoff(
        CharacterBattleRouteSubjectFamily::HandoffFeatureResourceProjectionRouteSubject,
        CharacterBattleRouteFillFamily::HandoffResourceDeltaFill,
        holes,
        CharacterBattleRouteOwnerGroup::CharacterBattleResourceProjectionOwner,
    ));
}

fn append_accepted_spell_resource_route(route: &mut Vec<CharacterBattleRouteEvent>) {
    append_accepted_feature_resource_route(route);
    route.push(route_project_character_sheet_to_battle(
        CharacterBattleRouteSubjectFamily::HandoffResourceProjectionRouteSubject,
        CharacterBattleRouteOwnerGroup::CharacterBattleResourceProjectionOwner,
    ));
}

fn append_rejected_spell_resource_route(
    route: &mut Vec<CharacterBattleRouteEvent>,
    holes: Vec<CharacterBattleRouteHoleFamily>,
) {
    route.push(route_reject_character_battle_handoff(
        CharacterBattleRouteSubjectFamily::HandoffResourceProjectionRouteSubject,
        CharacterBattleRouteFillFamily::HandoffResourceDeltaFill,
        holes,
        CharacterBattleRouteOwnerGroup::CharacterBattleResourceProjectionOwner,
    ));
}

fn append_metamagic_battle_bridge_route(route: &mut Vec<CharacterBattleRouteEvent>) {
    append_accepted_feature_resource_route(route);
    route.push(route_enter_battle_runtime(
        CharacterBattleRouteSubjectFamily::HandoffBattleMutationRouteSubject,
        CharacterBattleRouteOwnerGroup::CharacterBattleRuntimeOwner,
    ));
    route.push(route_enter_battle_runtime(
        CharacterBattleRouteSubjectFamily::HandoffResourceProjectionRouteSubject,
        CharacterBattleRouteOwnerGroup::CharacterBattleRuntimeOwner,
    ));
}

fn expected_route_after_lay_on_hands_restores_hp_and_removes_poisoned(
) -> Vec<CharacterBattleRouteEvent> {
    vec![
        expected_initial_feature_resource_handoff_route_event(),
        expected_accepted_feature_resource_route_event(),
        expected_project_character_sheet_to_battle(
            CharacterBattleRouteSubjectFamily::SheetToBattleInitRouteSubject,
            CharacterBattleRouteOwnerGroup::CharacterBattleSheetOwner,
        ),
    ]
}

fn expected_route_after_reject_lay_on_hands_overspend() -> Vec<CharacterBattleRouteEvent> {
    let mut route = expected_route_after_lay_on_hands_restores_hp_and_removes_poisoned();
    route.push(expected_rejected_feature_resource_route_event(vec![
        CharacterBattleRouteHoleFamily::HandoffFeatureResourceProjectionHoleFamily,
    ]));
    route
}

fn expected_route_after_long_rest_clears_lay_on_hands_pool() -> Vec<CharacterBattleRouteEvent> {
    let mut route = expected_route_after_reject_lay_on_hands_overspend();
    route.push(expected_accepted_feature_resource_route_event());
    route
}

fn expected_route_after_short_rest_recovers_use_count_pools() -> Vec<CharacterBattleRouteEvent> {
    let mut route = expected_route_after_long_rest_clears_lay_on_hands_pool();
    route.push(expected_accepted_feature_resource_route_event());
    route
}

fn expected_route_after_long_rest_clears_point_pool_and_use_state() -> Vec<CharacterBattleRouteEvent>
{
    let mut route = expected_route_after_short_rest_recovers_use_count_pools();
    route.push(expected_accepted_feature_resource_route_event());
    route
}

fn expected_route_after_font_of_magic_slot_to_points() -> Vec<CharacterBattleRouteEvent> {
    let mut route = expected_route_after_long_rest_clears_point_pool_and_use_state();
    route.push(expected_accepted_feature_resource_route_event());
    route.push(expected_project_character_sheet_to_battle(
        CharacterBattleRouteSubjectFamily::HandoffResourceProjectionRouteSubject,
        CharacterBattleRouteOwnerGroup::CharacterBattleResourceProjectionOwner,
    ));
    route
}

fn expected_route_after_reject_font_of_magic_ambiguous_slot_source(
) -> Vec<CharacterBattleRouteEvent> {
    let mut route = expected_route_after_font_of_magic_slot_to_points();
    route.push(expected_rejected_spell_resource_route_event(vec![
        CharacterBattleRouteHoleFamily::HandoffSpellResourceProjectionHoleFamily,
        CharacterBattleRouteHoleFamily::HandoffSettlementConflictHoleFamily,
    ]));
    route
}

fn expected_route_after_font_of_magic_points_to_slot() -> Vec<CharacterBattleRouteEvent> {
    let mut route = expected_route_after_reject_font_of_magic_ambiguous_slot_source();
    route.push(expected_accepted_feature_resource_route_event());
    route.push(expected_project_character_sheet_to_battle(
        CharacterBattleRouteSubjectFamily::HandoffResourceProjectionRouteSubject,
        CharacterBattleRouteOwnerGroup::CharacterBattleResourceProjectionOwner,
    ));
    route
}

fn expected_route_after_reject_font_of_magic_insufficient_points() -> Vec<CharacterBattleRouteEvent>
{
    let mut route = expected_route_after_font_of_magic_points_to_slot();
    route.push(expected_rejected_feature_resource_route_event(vec![
        CharacterBattleRouteHoleFamily::HandoffFeatureResourceProjectionHoleFamily,
    ]));
    route
}

fn expected_route_after_short_rest_preserves_uncanny_use_state() -> Vec<CharacterBattleRouteEvent> {
    let mut route = expected_route_after_reject_font_of_magic_insufficient_points();
    route.push(expected_accepted_feature_resource_route_event());
    route
}

fn expected_route_after_long_rest_clears_uncanny_use_state() -> Vec<CharacterBattleRouteEvent> {
    let mut route = expected_route_after_short_rest_preserves_uncanny_use_state();
    route.push(expected_accepted_feature_resource_route_event());
    route
}

fn expected_route_after_uncanny_metabolism_recovers_focus_and_heals(
) -> Vec<CharacterBattleRouteEvent> {
    let mut route = expected_route_after_long_rest_clears_uncanny_use_state();
    route.push(expected_accepted_feature_resource_route_event());
    route.push(expected_project_character_sheet_to_battle(
        CharacterBattleRouteSubjectFamily::SheetToBattleInitRouteSubject,
        CharacterBattleRouteOwnerGroup::CharacterBattleSheetOwner,
    ));
    route
}

fn expected_route_after_reject_uncanny_metabolism_repeat_use() -> Vec<CharacterBattleRouteEvent> {
    let mut route = expected_route_after_uncanny_metabolism_recovers_focus_and_heals();
    route.push(expected_rejected_feature_resource_route_event(vec![
        CharacterBattleRouteHoleFamily::HandoffFeatureResourceProjectionHoleFamily,
    ]));
    route
}

fn expected_route_after_metamagic_bridge_uses_shared_point_pool() -> Vec<CharacterBattleRouteEvent>
{
    let mut route = expected_route_after_reject_uncanny_metabolism_repeat_use();
    route.push(expected_accepted_feature_resource_route_event());
    route.push(expected_enter_battle_runtime(
        CharacterBattleRouteSubjectFamily::HandoffBattleMutationRouteSubject,
        CharacterBattleRouteOwnerGroup::CharacterBattleRuntimeOwner,
    ));
    route.push(expected_enter_battle_runtime(
        CharacterBattleRouteSubjectFamily::HandoffResourceProjectionRouteSubject,
        CharacterBattleRouteOwnerGroup::CharacterBattleRuntimeOwner,
    ));
    route
}

fn expected_initial_feature_resource_handoff_route_event() -> CharacterBattleRouteEvent {
    expected_project_character_sheet_to_battle(
        CharacterBattleRouteSubjectFamily::HandoffFeatureResourceProjectionRouteSubject,
        CharacterBattleRouteOwnerGroup::CharacterBattleSheetOwner,
    )
}

fn expected_accepted_feature_resource_route_event() -> CharacterBattleRouteEvent {
    expected_project_character_sheet_to_battle(
        CharacterBattleRouteSubjectFamily::HandoffFeatureResourceProjectionRouteSubject,
        CharacterBattleRouteOwnerGroup::CharacterBattleResourceProjectionOwner,
    )
}

fn expected_rejected_feature_resource_route_event(
    holes: Vec<CharacterBattleRouteHoleFamily>,
) -> CharacterBattleRouteEvent {
    expected_reject_character_battle_handoff(
        CharacterBattleRouteSubjectFamily::HandoffFeatureResourceProjectionRouteSubject,
        CharacterBattleRouteFillFamily::HandoffResourceDeltaFill,
        holes,
        CharacterBattleRouteOwnerGroup::CharacterBattleResourceProjectionOwner,
    )
}

fn expected_rejected_spell_resource_route_event(
    holes: Vec<CharacterBattleRouteHoleFamily>,
) -> CharacterBattleRouteEvent {
    expected_reject_character_battle_handoff(
        CharacterBattleRouteSubjectFamily::HandoffResourceProjectionRouteSubject,
        CharacterBattleRouteFillFamily::HandoffResourceDeltaFill,
        holes,
        CharacterBattleRouteOwnerGroup::CharacterBattleResourceProjectionOwner,
    )
}

fn expected_project_character_sheet_to_battle(
    subject: CharacterBattleRouteSubjectFamily,
    owner: CharacterBattleRouteOwnerGroup,
) -> CharacterBattleRouteEvent {
    CharacterBattleRouteEvent::RouteProjectCharacterSheetToBattle { subject, owner }
}

fn expected_enter_battle_runtime(
    subject: CharacterBattleRouteSubjectFamily,
    owner: CharacterBattleRouteOwnerGroup,
) -> CharacterBattleRouteEvent {
    CharacterBattleRouteEvent::RouteEnterBattleRuntime { subject, owner }
}

fn expected_reject_character_battle_handoff(
    subject: CharacterBattleRouteSubjectFamily,
    fill: CharacterBattleRouteFillFamily,
    mut holes: Vec<CharacterBattleRouteHoleFamily>,
    owner: CharacterBattleRouteOwnerGroup,
) -> CharacterBattleRouteEvent {
    holes.sort();
    CharacterBattleRouteEvent::RouteRejectCharacterBattleHandoff {
        subject,
        fill,
        holes,
        owner,
    }
}

fn feature_resource_pool(capacity: i16, expended: i16) -> ResourcePoolFacts {
    ResourcePoolFacts { capacity, expended }
}

fn target_hit_point_facts(
    sheet: SheetFeatureResourceFacts,
    hit_point_maximum: i16,
) -> FeatureResourceHitPoints {
    FeatureResourceHitPoints {
        current_hit_points: sheet.target_current_hp,
        hit_point_maximum,
        temporary_hit_points: 0,
    }
}

fn source_hit_point_facts(
    sheet: SheetFeatureResourceFacts,
    hit_point_maximum: i16,
) -> FeatureResourceHitPoints {
    FeatureResourceHitPoints {
        current_hit_points: sheet.source_current_hp,
        hit_point_maximum,
        temporary_hit_points: sheet.temporary_hit_points,
    }
}

fn lay_on_hands_pool(sheet: SheetFeatureResourceFacts) -> ResourcePoolFacts {
    feature_resource_pool(sheet.lay_on_hands_capacity, sheet.lay_on_hands_expended)
}

fn sorcery_point_pool(sheet: SheetFeatureResourceFacts) -> ResourcePoolFacts {
    feature_resource_pool(sheet.sorcery_point_capacity, sheet.sorcery_point_expended)
}

fn created_level3_slot_pool(sheet: SheetFeatureResourceFacts) -> ResourcePoolFacts {
    feature_resource_pool(sheet.created_level3_capacity, sheet.created_level3_expended)
}

fn ordinary_level2_slot_pool(sheet: SheetFeatureResourceFacts) -> ResourcePoolFacts {
    feature_resource_pool(2, sheet.ordinary_level2_expended)
}

fn with_lay_on_hands_pool(
    sheet: SheetFeatureResourceFacts,
    pool: ResourcePoolFacts,
) -> SheetFeatureResourceFacts {
    SheetFeatureResourceFacts {
        lay_on_hands_capacity: pool.capacity,
        lay_on_hands_expended: pool.expended,
        ..sheet
    }
}

fn with_sorcery_point_pool(
    sheet: SheetFeatureResourceFacts,
    pool: ResourcePoolFacts,
) -> SheetFeatureResourceFacts {
    SheetFeatureResourceFacts {
        sorcery_point_capacity: pool.capacity,
        sorcery_point_expended: pool.expended,
        ..sheet
    }
}

fn with_created_level3_slot_pool(
    sheet: SheetFeatureResourceFacts,
    pool: ResourcePoolFacts,
) -> SheetFeatureResourceFacts {
    SheetFeatureResourceFacts {
        created_level3_capacity: pool.capacity,
        created_level3_expended: pool.expended,
        ..sheet
    }
}

fn feature_resource_rest_state(sheet: SheetFeatureResourceFacts) -> FeatureResourceRestState {
    FeatureResourceRestState {
        lay_on_hands_pool: lay_on_hands_pool(sheet),
        wild_shape_expended: sheet.druid_wild_shape_expended,
        monk_focus_expended: sheet.monk_focus_expended,
        sorcery_points: sorcery_point_pool(sheet),
        created_spell_slots: created_level3_slot_pool(sheet),
        uncanny_used_since_long_rest: sheet.uncanny_used_since_long_rest,
    }
}

fn with_feature_resource_rest_state(
    sheet: SheetFeatureResourceFacts,
    rest_state: FeatureResourceRestState,
) -> SheetFeatureResourceFacts {
    with_created_level3_slot_pool(
        with_sorcery_point_pool(
            with_lay_on_hands_pool(
                SheetFeatureResourceFacts {
                    druid_wild_shape_expended: rest_state.wild_shape_expended,
                    monk_focus_expended: rest_state.monk_focus_expended,
                    uncanny_used_since_long_rest: rest_state.uncanny_used_since_long_rest,
                    ..sheet
                },
                rest_state.lay_on_hands_pool,
            ),
            rest_state.sorcery_points,
        ),
        rest_state.created_spell_slots,
    )
}

fn apply_lay_on_hands_to_sheet(
    sheet: SheetFeatureResourceFacts,
    target_hit_point_maximum: i16,
    restored_hit_points: i16,
    removed_condition_count: i16,
) -> SheetFeatureResourceFacts {
    let applied = apply_lay_on_hands_resource(
        lay_on_hands_pool(sheet),
        target_hit_point_facts(sheet, target_hit_point_maximum),
        restored_hit_points,
        removed_condition_count,
    )
    .expect("Lay On Hands witness uses a legal healing pool spend");

    SheetFeatureResourceFacts {
        target_current_hp: applied.target_hit_points.current_hit_points,
        target_poisoned: if applied.condition_removed {
            false
        } else {
            sheet.target_poisoned
        },
        ..with_lay_on_hands_pool(sheet, applied.healing_pool)
    }
}

fn complete_long_rest_feature_resources(
    sheet: SheetFeatureResourceFacts,
) -> SheetFeatureResourceFacts {
    let rest_state =
        complete_long_rest_feature_resource_rest_state(feature_resource_rest_state(sheet))
            .expect("long rest witness uses a legal feature resource state");
    with_feature_resource_rest_state(sheet, rest_state)
}

fn short_rest_feature_resources(sheet: SheetFeatureResourceFacts) -> SheetFeatureResourceFacts {
    let rest_state =
        complete_short_rest_feature_resource_rest_state(feature_resource_rest_state(sheet))
            .expect("short rest witness uses a legal feature resource state");
    with_feature_resource_rest_state(sheet, rest_state)
}

fn use_uncanny_metabolism(
    sheet: SheetFeatureResourceFacts,
    maximum_hp: i16,
    monk_level: i16,
    martial_arts_roll: i16,
) -> SheetFeatureResourceFacts {
    let applied = apply_uncanny_metabolism(
        sheet.uncanny_used_since_long_rest,
        feature_resource_pool(2, sheet.monk_focus_expended),
        source_hit_point_facts(sheet, maximum_hp),
        monk_level,
        martial_arts_roll,
    )
    .expect("Uncanny Metabolism witness uses a legal Focus Point pool");

    SheetFeatureResourceFacts {
        source_current_hp: applied.hit_points.current_hit_points,
        temporary_hit_points: applied.hit_points.temporary_hit_points,
        monk_focus_expended: applied.focus_points.expended,
        uncanny_used_since_long_rest: applied.used_since_long_rest,
        ..sheet
    }
}

fn create_font_of_magic_level3_slot_on_sheet(
    sheet: SheetFeatureResourceFacts,
) -> SheetFeatureResourceFacts {
    let created = create_font_of_magic_spell_slot(
        sorcery_point_pool(sheet),
        created_level3_slot_pool(sheet),
        5,
        3,
    )
    .expect("Font of Magic witness uses enough Sorcery Points for a level 3 slot");
    with_created_level3_slot_pool(
        with_sorcery_point_pool(sheet, created.sorcery_points),
        created.created_spell_slot,
    )
}

fn convert_font_of_magic_level2_slot_to_points_on_sheet(
    sheet: SheetFeatureResourceFacts,
) -> SheetFeatureResourceFacts {
    let converted = convert_font_of_magic_slot_to_sorcery_points(
        sorcery_point_pool(sheet),
        2,
        ordinary_level2_slot_pool(sheet),
        feature_resource_pool(0, 0),
    )
    .expect("Font of Magic witness uses one unambiguous level 2 Spell Slot source");

    SheetFeatureResourceFacts {
        ordinary_level2_expended: converted.ordinary_spell_slot.expended,
        ..with_sorcery_point_pool(sheet, converted.sorcery_points)
    }
}

fn record_projection(
    last_result: &'static str,
    accepted: bool,
    message: &'static str,
    sheet: SheetFeatureResourceFacts,
    replay_index: u8,
) -> SheetFeatureResourceWitness {
    SheetFeatureResourceWitness {
        last_result,
        accepted,
        message,
        source_current_hp: sheet.source_current_hp,
        target_current_hp: sheet.target_current_hp,
        temporary_hit_points: sheet.temporary_hit_points,
        target_poisoned: sheet.target_poisoned,
        lay_on_hands_capacity: sheet.lay_on_hands_capacity,
        lay_on_hands_expended: sheet.lay_on_hands_expended,
        druid_wild_shape_expended: sheet.druid_wild_shape_expended,
        monk_focus_expended: sheet.monk_focus_expended,
        sorcery_point_capacity: sheet.sorcery_point_capacity,
        sorcery_point_expended: sheet.sorcery_point_expended,
        ordinary_level2_expended: sheet.ordinary_level2_expended,
        created_level3_capacity: sheet.created_level3_capacity,
        created_level3_expended: sheet.created_level3_expended,
        uncanny_used_since_long_rest: sheet.uncanny_used_since_long_rest,
        metamagic_known_options: sheet.metamagic_known_options,
        metamagic_shared_resource_expended: sheet.metamagic_shared_resource_expended,
        replay_index,
    }
}
