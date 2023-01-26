pub const WARRIOR_BASE_STRENGTH: u32 = 10;
pub const WARRIOR_BASE_WISDOM: u32 = 2;
pub const WARRIOR_BASE_INTELLECT: u32 = 0;
pub const WARRIOR_BASE_AGILITY: u32 = 4;
pub const WARRIOR_BASE_STAMINA: u32 = 5;
pub const WARRIOR_BASE_STEALTH: u32 = 0;

pub const MAGE_BASE_STRENGTH: u32 = 2;
pub const MAGE_BASE_WISDOM: u32 = 10;
pub const MAGE_BASE_INTELLECT: u32 = 10;
pub const MAGE_BASE_AGILITY: u32 = 2;
pub const MAGE_BASE_STAMINA: u32 = 2;
pub const MAGE_BASE_STEALTH: u32 = 2;

pub const ROGUE_BASE_STRENGTH: u32 = 4;
pub const ROGUE_BASE_WISDOM: u32 = 2;
pub const ROGUE_BASE_INTELLECT: u32 = 2;
pub const ROGUE_BASE_AGILITY: u32 = 10;
pub const ROGUE_BASE_STAMINA: u32 = 5;
pub const ROGUE_BASE_STEALTH: u32 = 10;

pub const ARCHER_BASE_STRENGTH: u32 = 4;
pub const ARCHER_BASE_WISDOM: u32 = 2;
pub const ARCHER_BASE_INTELLECT: u32 = 2;
pub const ARCHER_BASE_AGILITY: u32 = 10;
pub const ARCHER_BASE_STAMINA: u32 = 5;
pub const ARCHER_BASE_STEALTH: u32 = 2;

pub const HEALER_BASE_STRENGTH: u32 = 2;
pub const HEALER_BASE_WISDOM: u32 = 10;
pub const HEALER_BASE_INTELLECT: u32 = 10;
pub const HEALER_BASE_AGILITY: u32 = 2;
pub const HEALER_BASE_STAMINA: u32 = 2;
pub const HEALER_BASE_STEALTH: u32 = 2;

#[derive(Debug, Copy, Clone)]
pub enum Class {
    Warrior,
    Mage,
    Rogue,
    Archer,
    Healer,
}

impl std::fmt::Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}