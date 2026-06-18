#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreatureSize {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
    Gargantuan,
}

impl CreatureSize {
    #[must_use]
    pub const fn rank(self) -> i16 {
        match self {
            CreatureSize::Tiny => 0,
            CreatureSize::Small => 1,
            CreatureSize::Medium => 2,
            CreatureSize::Large => 3,
            CreatureSize::Huge => 4,
            CreatureSize::Gargantuan => 5,
        }
    }

    #[must_use]
    pub const fn is_medium_or_smaller(self) -> bool {
        self.rank() <= CreatureSize::Medium.rank()
    }
}
