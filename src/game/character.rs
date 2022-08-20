use std::{collections::HashMap, fmt};

use serde::{Deserialize, Serialize};
use tui::{backend::Backend, layout::Rect, Frame};

pub trait Character {
    fn draw_long_desc<B: Backend>(&self, f: &mut Frame<B>, area: Rect);
    fn draw_short_desc<B: Backend>(&self, f: &mut Frame<B>, area: Rect);

    fn symbol(&self) -> &str;

    fn move_left(&mut self);
    fn move_right(&mut self);
    fn move_up(&mut self);
    fn move_down(&mut self);

    fn get_pos(&self) -> (usize, usize);
    fn get_x(&self) -> usize;
    fn get_y(&self) -> usize;
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

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Property {
    props: HashMap<PropertyType, PropertyValue>,
}

impl Property {
    pub fn from_attrs(attr: Attribute) -> Self {
        let (max_jing, max_qi, max_shen) = (
            PropertyValue::calc_jing(&attr),
            PropertyValue::calc_qi(&attr),
            PropertyValue::calc_shen(&attr),
        );

        Self {
            props: HashMap::from([
                (PropertyType::Jing, max_jing),
                (PropertyType::Qi, max_qi),
                (PropertyType::Shen, max_shen),
                (PropertyType::MaxJing, max_jing),
                (PropertyType::MaxQi, max_qi),
                (PropertyType::MaxShen, max_shen),
            ]),
        }
    }

    pub fn get(&self, prop: &PropertyType) -> Option<&PropertyValue> {
        self.props.get(prop)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub enum PropertyType {
    Jing,
    Qi,
    Shen,
    MaxJing,
    MaxQi,
    MaxShen,
}

impl fmt::Display for PropertyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PropertyType::Jing => "精",
                PropertyType::Qi => "气",
                PropertyType::Shen => "神",
                PropertyType::MaxJing => "精上限",
                PropertyType::MaxQi => "气上限",
                PropertyType::MaxShen => "神上限",
            }
        )
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum PropertyValue {
    Number(f64),
}

impl fmt::Display for PropertyValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Number(v) => v,
            }
        )
    }
}

impl PropertyValue {
    pub fn calc_jing(attr: &Attribute) -> PropertyValue {
        let formula = |con: usize| -> f64 { con as f64 * 5.0 };

        PropertyValue::Number(formula(attr.con))
    }

    pub fn calc_qi(attr: &Attribute) -> PropertyValue {
        let formula = |con: usize| -> f64 { con as f64 * 5.0 };

        PropertyValue::Number(formula(attr.con))
    }

    pub fn calc_shen(attr: &Attribute) -> PropertyValue {
        let formula = |con: usize| -> f64 { con as f64 * 5.0 };

        PropertyValue::Number(formula(attr.int))
    }

    pub fn unwrap_number(&self) -> f64 {
        match self {
            Self::Number(v) => *v,
        }
    }
}
