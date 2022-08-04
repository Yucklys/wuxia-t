use core::fmt;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Default, Serialize, Deserialize, Clone, Copy)]
pub struct Pos(usize, usize);

impl Pos {
    pub fn here(&self) -> (usize, usize) {
        (self.0, self.1)
    }

    pub fn move_down(&mut self, step: usize) {
        self.1 += step
    }
    pub fn move_left(&mut self, step: usize) {
        self.0 -= step
    }
    pub fn move_right(&mut self, step: usize) {
        self.0 += step
    }
    pub fn move_up(&mut self, step: usize) {
        self.1 -= step
    }

    pub fn new(x: usize, y: usize) -> Pos {
        Pos(x, y)
    }

    pub fn x(&self) -> usize {
        self.0
    }
    pub fn y(&self) -> usize {
        self.1
    }
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Attribute {
    pub con: usize,
    pub str: usize,
    pub dex: usize,
    pub int: usize,
    pub chr: usize,
}

impl Attribute {
    pub fn new(con: usize, str: usize, dex: usize, int: usize, chr: usize) -> Self {
        Self {
            con,
            str,
            dex,
            int,
            chr,
        }
    }

    pub fn human() -> Self {
        Self::new(10, 10, 10, 10, 10)
    }
}

#[derive(Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub enum Property {
    Jing(usize),
    Qi(usize),
    Shen(usize),
}

impl fmt::Display for Property {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Property::Jing(v) => v,
                Property::Qi(v) => v,
                Property::Shen(v) => v,
            }
        )
    }
}

impl Property {
    pub fn from_attrs(attr: Attribute) -> HashMap<String, Property> {
        HashMap::from([
            ("jing".to_string(), Property::calc_jing(&attr)),
            ("qi".to_string(), Property::calc_qi(&attr)),
            ("shen".to_string(), Property::calc_shen(&attr)),
        ])
    }

    pub fn calc_jing(attr: &Attribute) -> Property {
        let formula = |con: usize| -> usize { con * 5 };

        Property::Jing(formula(attr.con))
    }

    pub fn calc_qi(attr: &Attribute) -> Property {
        let formula = |con: usize| -> usize { con * 5 };

        Property::Qi(formula(attr.con))
    }

    pub fn calc_shen(attr: &Attribute) -> Property {
        let formula = |int: usize| -> usize { int * 5 };

        Property::Shen(formula(attr.int))
    }

    pub fn value(&self) -> f64 {
        match &self {
            Property::Jing(v) => v.to_owned() as f64,
            Property::Qi(v) => v.to_owned() as f64,
            Property::Shen(v) => v.to_owned() as f64,
        }
    }
}

#[derive(Default)]
pub struct WorldState {
    pub clock: Clock,
}

pub struct Clock {
    pub hour: Hour,
    pub subs: u16,
}

impl Default for Clock {
    fn default() -> Self {
        Self {
            hour: Hour::Zi,
            subs: 0,
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}",
            self.hour,
            match self.subs {
                0 => "初",
                1 => "正",
                _ => "",
            }
        )
    }
} // TODO apply time system
#[allow(dead_code)]
pub enum Hour {
    Zi,
    Chou,
    Yin,
    Mao,
    Chen,
    Si,
    Wu,
    Mo,
    Shen,
    You,
    Xu,
    Hai,
}

impl fmt::Display for Hour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Hour::Zi => "子",
                Hour::Chou => "丑",
                Hour::Yin => "寅",
                Hour::Mao => "卯",
                Hour::Chen => "辰",
                Hour::Si => "巳",
                Hour::Wu => "午",
                Hour::Mo => "未",
                Hour::Shen => "申",
                Hour::You => "酉",
                Hour::Xu => "戌",
                Hour::Hai => "亥",
            }
        )
    }
}
