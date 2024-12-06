#[derive(PartialEq, Debug, Copy, Clone, Hash, Eq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn rotate(&mut self) {
        match self {
            Direction::Up => *self = Direction::Right,
            Direction::Right => *self = Direction::Down,
            Direction::Down => *self = Direction::Left,
            Direction::Left => *self = Direction::Up,
        }
    }
    pub fn oposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone, Hash, Eq)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn new(pos: (usize, usize)) -> Self {
        Position {
            x: pos.0 as i32,
            y: pos.1 as i32,
        }
    }

    pub fn x(&self) -> usize {
        self.x as usize
    }

    pub fn y(&self) -> usize {
        self.y as usize
    }

    pub fn jump(&mut self, dir: &Direction) {
        match dir {
            Direction::Up => self.x -= 1,
            Direction::Right => self.y += 1,
            Direction::Down => self.x += 1,
            Direction::Left => self.y -= 1,
        }
    }

    pub fn is_out_of_scope(&self, x_size: usize, y_size: usize) -> bool {
        self.x < 0 || self.x >= x_size as i32 || self.y < 0 || self.y >= y_size as i32
    }
}
