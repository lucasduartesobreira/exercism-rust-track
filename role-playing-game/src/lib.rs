// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

const REVIVE_HEALTH: u32 = 100;
const REVIVE_MANA: u32 = 100;

impl Player {
    pub fn revive(&self) -> Option<Player> {
        match self.health {
            0 => Some(Player {
                health: REVIVE_HEALTH,
                mana: self.mana.map(|_| REVIVE_MANA),
                level: self.level,
            }),
            _ => None,
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(mana) if mana > mana_cost => {
                self.mana.replace(mana - mana_cost);
                2 * mana_cost
            }
            Some(_) => 0,
            None => {
                self.health = u32::checked_sub(self.health, mana_cost).unwrap_or(0);
                0
            }
        }
    }
}
