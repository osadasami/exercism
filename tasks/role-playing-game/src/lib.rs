// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::cmp::min;

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    fn new(level: u32) -> Self {
        Self {
            health: 100,
            level,
            mana: if level < 10 { None } else { Some(100) },
        }
    }

    pub fn revive(&self) -> Option<Self> {
        match self.health {
            0 => Some(Self::new(self.level)),
            _ => None,
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(mana) => {
                if mana >= mana_cost {
                    self.mana = Some(mana - mana_cost);
                    2 * mana_cost
                } else {
                    0
                }
            }
            None => {
                self.health = self.health - min(self.health, mana_cost);
                0
            }
        }
    }
}
