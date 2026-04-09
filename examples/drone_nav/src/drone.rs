use gen_fsm::{Context, State};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DroneState {
    Cruise,
    Avoid,
    Search,
    Emergency,
}

impl State for DroneState {
    const COUNT: usize = 4;

    fn to_index(&self) -> usize {
        *self as usize
    }

    fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::Cruise),
            1 => Some(Self::Avoid),
            2 => Some(Self::Search),
            3 => Some(Self::Emergency),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DroneContext {
    Clear,
    ObstacleNear,
    ObstacleFar,
    DeadEnd,
}

impl Context for DroneContext {
    const COUNT: usize = 4;

    fn to_index(&self) -> usize {
        *self as usize
    }

    fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::Clear),
            1 => Some(Self::ObstacleNear),
            2 => Some(Self::ObstacleFar),
            3 => Some(Self::DeadEnd),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub const ALL: [Direction; 4] = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];

    pub fn apply(&self, row: usize, col: usize, max_row: usize, max_col: usize) -> Option<(usize, usize)> {
        match self {
            Direction::Up if row > 0 => Some((row - 1, col)),
            Direction::Down if row + 1 < max_row => Some((row + 1, col)),
            Direction::Left if col > 0 => Some((row, col - 1)),
            Direction::Right if col + 1 < max_col => Some((row, col + 1)),
            _ => None,
        }
    }
}
