use std::fmt::Display;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MoveParseError {
    NoWhitespace,
    InvalidDirection,
    InvalidAmount,
}
impl Display for MoveParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MoveParseError::NoWhitespace => "No whitespace between direction and amount",
                MoveParseError::InvalidDirection => "Direction invalid (must be U,D,L,R)",
                MoveParseError::InvalidAmount => "Amount is not a valid number",
            }
        )
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Move {
    pub direction: Direction,
    pub amount: u32,
}
impl TryFrom<&str> for Move {
    type Error = MoveParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let Some((dir_str, amount_str)) = value.split_once(' ') else {
            return Err(MoveParseError::NoWhitespace)
        };
        let Ok(amount) = amount_str.parse::<u32>() else {
            return Err(MoveParseError::InvalidAmount)
        };
        Ok(Move {
            direction: Direction::try_from(dir_str)?,
            amount,
        })
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl TryFrom<&str> for Direction {
    type Error = MoveParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "u" | "up" => Ok(Direction::Up),
            "d" | "down" => Ok(Direction::Down),
            "r" | "right" => Ok(Direction::Right),
            "l" | "left" => Ok(Direction::Left),
            _ => Err(MoveParseError::InvalidDirection),
        }
    }
}
