
#[derive(Copy, Clone)]
pub enum Direction {
    TOP,
    RIGHT,
    BOTTOM,
    LEFT
}

impl Into<u8> for Direction {
    fn into(self) -> u8 {
        match self {
            Direction::TOP => 0,
            Direction::RIGHT => 1,
            Direction::BOTTOM => 2,
            Direction::LEFT => 3,
        }
    }
}

impl Into<usize> for Direction {
    fn into(self) -> usize {
        match self {
            Direction::TOP => 0,
            Direction::RIGHT => 1,
            Direction::BOTTOM => 2,
            Direction::LEFT => 3,
        }
    }
}

pub fn get_neighbour_pos(pos: &[i32; 2], dir: Direction) -> [i32; 2] {
    match dir {
        Direction::TOP => [pos[0], pos[1] - 1],
        Direction::RIGHT => [pos[0] + 1, pos[1]],
        Direction::BOTTOM => [pos[0], pos[1] + 1],
        Direction::LEFT => [pos[0] - 1, pos[1]]
    }
}

fn get_opposite_direction(dir: Direction) -> Direction {
    match dir {
        Direction::TOP => Direction::BOTTOM,
        Direction::RIGHT => Direction::LEFT,
        Direction::BOTTOM => Direction::TOP,
        Direction::LEFT => Direction::RIGHT
    }
}