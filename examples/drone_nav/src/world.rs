#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Obstacle,
    Start,
    Goal,
}

#[derive(Debug, Clone)]
pub struct World {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Vec<Cell>>,
    pub start: (usize, usize),
    pub goal: (usize, usize),
}

impl World {
    pub fn test_map() -> Self {
        let width = 20;
        let height = 12;
        let mut grid = vec![vec![Cell::Empty; width]; height];

        let obstacles = [
            (2, 3), (2, 4), (3, 3),
            (1, 7), (1, 8), (2, 6), (2, 7), (2, 8),
            (3, 6),
            (3, 12), (4, 11), (4, 12), (4, 13),
            (5, 12),
            (6, 3), (6, 4), (7, 3),
            (6, 9), (6, 10), (7, 9), (7, 10),
            (8, 6), (8, 7), (9, 7),
        ];

        for (r, c) in &obstacles {
            if *r < height && *c < width {
                grid[*r][*c] = Cell::Obstacle;
            }
        }

        let start = (1, 1);
        let goal = (10, 17);

        grid[start.0][start.1] = Cell::Start;
        grid[goal.0][goal.1] = Cell::Goal;

        Self {
            width,
            height,
            grid,
            start,
            goal,
        }
    }

    pub fn is_walkable(&self, row: usize, col: usize) -> bool {
        if row >= self.height || col >= self.width {
            return false;
        }
        self.grid[row][col] != Cell::Obstacle
    }

    pub fn manhattan_distance(a: (usize, usize), b: (usize, usize)) -> f64 {
        ((a.0 as isize - b.0 as isize).abs() + (a.1 as isize - b.1 as isize).abs()) as f64
    }
}
