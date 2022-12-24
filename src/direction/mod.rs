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
            4 => Direction::IAxes,
            5 => Direction::IKAxes,
            6 => Direction::IJAxes,
            _ => Direction::Invalid,
            // Direction::NumDigits => 7,
            // Direction::PentagonSkippedDigit => 1,
        }
    }
}

impl From<u64> for Direction {
    fn from(digit: u64) -> Self {
        Into::<Direction>::into(digit as usize)
    }
}
