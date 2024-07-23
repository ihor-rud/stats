use anyhow::anyhow;
use thiserror::Error;

#[derive(Debug, Clone, Copy)]
pub enum WindowSize {
    E1,
    E2,
    E3,
    E4,
    E5,
    E6,
    E7,
    E8,
}

impl From<WindowSize> for usize {
    fn from(value: WindowSize) -> Self {
        match value {
            WindowSize::E1 => 10usize.pow(1),
            WindowSize::E2 => 10usize.pow(2),
            WindowSize::E3 => 10usize.pow(3),
            WindowSize::E4 => 10usize.pow(4),
            WindowSize::E5 => 10usize.pow(5),
            WindowSize::E6 => 10usize.pow(6),
            WindowSize::E7 => 10usize.pow(7),
            WindowSize::E8 => 10usize.pow(8),
        }
    }
}

impl TryFrom<u8> for WindowSize {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(WindowSize::E1),
            2 => Ok(WindowSize::E2),
            3 => Ok(WindowSize::E3),
            4 => Ok(WindowSize::E4),
            5 => Ok(WindowSize::E5),
            6 => Ok(WindowSize::E6),
            7 => Ok(WindowSize::E7),
            8 => Ok(WindowSize::E8),
            _ => Err(anyhow!("value should be in range [1, 8]")),
        }
    }
}

#[derive(Debug)]
pub struct Stats {
    pub min: f64,
    pub max: f64,
    pub last: f64,
    pub avg: f64,
    pub var: f64,
}

#[derive(Debug, Error)]
pub enum StatsError {
    #[error("Name not found")]
    NameNotFound,
    #[error("Window doesn't have enough data")]
    WindowIsNotFilled,
}
