use crate::class::*;

pub struct Character {
    name: String,
    class: Class,
    level: u32,
    health: u32,
    mana: u32,
    max_health: u32,
    strength: u32,
    wisdom: u32,
    intellect: u32,
    agility: u32,
    stamina: u32,
    stealth: u32,
}

impl Character {
    pub fn new(name: &str, class: Class) -> Character {
        let mut character = Character {
            name: String::from(name),
            class,
            level: 0,
            health: 100,
            max_health: 100,
            mana: 100,
            strength: 0,
            wisdom: 0,
            intellect: 0,
            agility: 0,
            stamina: 0,
            stealth: 0,
        };

        match class {
            Class::Warrior => {
                character.strength += WARRIOR_BASE_STRENGTH;
                character.wisdom += WARRIOR_BASE_WISDOM;
                character.intellect += WARRIOR_BASE_INTELLECT; 
                character.agility += WARRIOR_BASE_AGILITY;
                character.stamina += WARRIOR_BASE_STAMINA;
                character.stealth += WARRIOR_BASE_STEALTH;
            }
            Class::Mage => {
                character.strength += MAGE_BASE_STRENGTH;
                character.wisdom += MAGE_BASE_WISDOM;
                character.intellect += MAGE_BASE_INTELLECT;
                character.agility += MAGE_BASE_AGILITY;
                character.stamina += MAGE_BASE_STAMINA;
                character.stealth += MAGE_BASE_STEALTH;
            }
            Class::Rogue => {
                character.strength += ROGUE_BASE_STRENGTH;
                character.wisdom += ROGUE_BASE_WISDOM;
                character.intellect += ROGUE_BASE_INTELLECT;
                character.agility += ROGUE_BASE_AGILITY;
                character.stamina += ROGUE_BASE_STAMINA;
                character.stealth += ROGUE_BASE_STEALTH;
            }
            Class::Archer => {
                character.strength += ARCHER_BASE_STRENGTH;
                character.wisdom += ARCHER_BASE_WISDOM;
                character.intellect += ARCHER_BASE_INTELLECT;
                character.agility += ARCHER_BASE_AGILITY;
                character.stamina += ARCHER_BASE_STAMINA;
                character.stealth += ARCHER_BASE_STEALTH;
            }
            Class::Healer => {
                character.strength += HEALER_BASE_STRENGTH;
                character.wisdom += HEALER_BASE_WISDOM;
                character.intellect += HEALER_BASE_INTELLECT;
                character.agility += HEALER_BASE_AGILITY;
                character.stamina += HEALER_BASE_STAMINA;
                character.stealth += HEALER_BASE_STEALTH;
            }
        }

        return character;
    }

    pub fn set_level(&mut self, level: u32) {
        self.level = level;
    }

    pub fn is_damaged(&mut self, damage: u32) {
        if damage > self.health {
            self.health = 0;
        } else {
            self.health -= damage;
        }
    }

    pub fn is_healed(&mut self, health: u32) {
        if self.health + health > self.max_health {
            self.health = self.max_health;
        } else {
            self.health += health;
        }
    }

    pub fn values(&self) -> String {
        format!(
            "Name;{};Class;{};Level;{};Health;{};Mana;{};STR;{};WIS;{};INT;{};AGL;{};STA;{};STH;{};",
            self.name, self.class, self.level, self.health, self.mana, self.strength, self.wisdom, self.intellect, self.agility, self.stamina, self.stealth
        )
    }
}
