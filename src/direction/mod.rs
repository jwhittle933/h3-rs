#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Direction {
    Center,
    KAxes,
    JAxes,
    JKAxes,
    IAxes,
    IKAxes,
    IJAxes,
    Invalid,
    NumDigits,
    PentagonSkippedDigit,
}

impl Direction {
    /// Rotates indexing digit 60-degrees counter-clockwise.
    pub fn rotate_60_ccw(self) -> Direction {
        match self {
            Self::KAxes => Self::IKAxes,
            Self::IKAxes => Self::IAxes,
            Self::IAxes => Self::IJAxes,
            Self::IJAxes => Self::JAxes,
            Self::JAxes => Self::JKAxes,
            Self::JKAxes => Self::KAxes,
            _ => self,
        }
    }

    /// Rotates indexing digit 60-degrees clockwise.
    pub fn rotate_60_cw(self) -> Direction {
        match self {
            Self::KAxes => Self::JKAxes,
            Self::JKAxes => Self::JAxes,
            Self::JAxes => Self::IJAxes,
            Self::IJAxes => Self::IAxes,
            Self::IAxes => Self::IKAxes,
            Self::IKAxes => Self::KAxes,
            _ => self,
        }
    }
}

impl From<usize> for Direction {
    fn from(digit: usize) -> Self {
        match digit {
            0 => Self::Center,
            1 => Self::KAxes,
            2 => Self::JAxes,
            3 => Self::JKAxes,
            4 => Self::IAxes,
            5 => Self::IKAxes,
            6 => Self::IJAxes,
            _ => Self::Invalid,
        }
    }
}

impl From<Direction> for usize {
    fn from(digit: Direction) -> Self {
        match digit {
            Direction::Center => 0,
            Direction::KAxes => 1,
            Direction::JAxes => 2,
            Direction::JKAxes => 3,
            Direction::IAxes => 4,
            Direction::IKAxes => 5,
            Direction::IJAxes => 6,
            Direction::Invalid => 7,
            Direction::NumDigits => Direction::Invalid as usize,
            Direction::PentagonSkippedDigit => Direction::KAxes as usize,
        }
    }
}

impl From<u64> for Direction {
    fn from(digit: u64) -> Self {
        Into::<Direction>::into(digit as usize)
    }
}

impl From<Direction> for u64 {
    fn from(digit: Direction) -> Self {
        Into::<usize>::into(digit) as u64
    }
}
