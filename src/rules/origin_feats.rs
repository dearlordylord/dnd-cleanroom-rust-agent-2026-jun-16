#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Background {
    Criminal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OriginFeat {
    Alert,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OriginFeatProjection {
    pub background: Background,
    pub origin_feat: OriginFeat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlertInitiativeState {
    Absent,
    Present,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InitiativeHandoffFacts {
    pub dexterity_modifier: i16,
    pub proficiency_bonus: i16,
    pub alert_initiative: AlertInitiativeState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InitiativeHandoffProjection {
    pub initiative_score: i16,
}

#[must_use]
pub fn criminal_origin_feat_projection() -> OriginFeatProjection {
    // RAW: cleanroom-input/raw/srd-5.2.1/Character-Origins.md
    // "Backgrounds" and "Criminal"; QNT:
    // cleanroom-input/qnt/character-battle-runtime/
    // character-battle-origin-feat-selected-identity.mbt.qnt.
    OriginFeatProjection {
        background: Background::Criminal,
        origin_feat: OriginFeat::Alert,
    }
}

#[must_use]
pub fn initiative_handoff_projection(facts: InitiativeHandoffFacts) -> InitiativeHandoffProjection {
    // RAW: cleanroom-input/raw/srd-5.2.1/Rules-Glossary.md "Initiative";
    // cleanroom-input/raw/srd-5.2.1/Feats.md "Alert";
    // QNT: character-battle-origin-feat-selected-identity.mbt.qnt.
    let alert_bonus = match facts.alert_initiative {
        AlertInitiativeState::Absent => 0,
        AlertInitiativeState::Present => facts.proficiency_bonus,
    };

    InitiativeHandoffProjection {
        initiative_score: 10 + facts.dexterity_modifier + alert_bonus,
    }
}
