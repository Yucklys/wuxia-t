pub mod tutorial;

use std::collections::HashSet;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Clone)]
pub enum GameSwitch {
    Tutorial,
}

#[derive(Deserialize, Clone)]
pub enum Event {
    Tutorial,
}

#[derive(Deserialize, Clone)]
pub enum EventActivation {
    Touch,
}

#[derive(Deserialize, Clone)]
pub struct GameEvent {
    pub activation: EventActivation,
    pub name: Event,
    #[serde(skip)]
    pub num_execute: usize,
    pub pos: (usize, usize),
    pub repeat: bool,
    pub when: HashSet<GameSwitch>,
}

impl GameEvent {
    pub fn is_active(&self, pos: (usize, usize), switches: &HashSet<GameSwitch>) -> bool {
        if self.when.is_subset(switches) {
            match self.activation {
                EventActivation::Touch => self.pos == pos,
            }
        } else {
            false
        }
    }
}
