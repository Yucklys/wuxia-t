use core::fmt;

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
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
