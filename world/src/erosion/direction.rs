
#[derive(Copy, Clone)]
pub enum Direction {
    TOP,
    RIGHT,
    BOTTOM,
    LEFT
}

pub struct DirectionIterator {
	index: u8
}

impl Default for DirectionIterator {
	fn default() -> Self {
		Self {
			index: 0
		}
	}
}

impl Iterator for DirectionIterator {
	type Item = Direction;

	fn next(&mut self) -> Option<Self::Item> {
		match self.index {
			i if i < 4 => { self.index += 1; Some(i.into()) },
			_ => None
		}
	}
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

impl Into<Direction> for u8 {
	fn into(self) -> Direction {
		match self {
			0 => Direction::TOP,
			1 => Direction::RIGHT,
			2 => Direction::BOTTOM,
			3 => Direction::LEFT,
			_ => unreachable!()
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

pub fn get_opposite_direction(dir: Direction) -> Direction {
    match dir {
        Direction::TOP => Direction::BOTTOM,
        Direction::RIGHT => Direction::LEFT,
        Direction::BOTTOM => Direction::TOP,
        Direction::LEFT => Direction::RIGHT
    }
}
