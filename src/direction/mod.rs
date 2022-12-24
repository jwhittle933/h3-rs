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
            Direction::NumDigits => 7,
            Direction::PentagonSkippedDigit => 1,
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
        Into::<Direction>::into(digit) as u64
    }
}
