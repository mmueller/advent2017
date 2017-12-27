// Cartesian grid structures and utilities.

#[derive(Clone,Copy,Debug)]
pub enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    pub fn turn_right(&self) -> Dir {
        match self {
            &Dir::Up    => Dir::Right,
            &Dir::Right => Dir::Down,
            &Dir::Down  => Dir::Left,
            &Dir::Left  => Dir::Up,
        }
    }

    pub fn turn_left(&self) -> Dir {
        match self {
            &Dir::Up    => Dir::Left,
            &Dir::Right => Dir::Up,
            &Dir::Down  => Dir::Right,
            &Dir::Left  => Dir::Down,
        }
    }

    pub fn reverse(&self) -> Dir {
        match self {
            &Dir::Up    => Dir::Down,
            &Dir::Right => Dir::Left,
            &Dir::Down  => Dir::Up,
            &Dir::Left  => Dir::Right,
        }
    }
}

#[derive(Clone,Copy,Debug)]
pub struct IPos {
    pub row: isize,
    pub col: isize
}

impl IPos {
    pub fn new(row: isize, col: isize) -> Self {
        IPos {
            row: row,
            col: col
        }
    }

    pub fn origin() -> Self {
        IPos {
            row: 0,
            col: 0
        }
    }

    pub fn neighbor(&self, dir: Dir) -> IPos {
        match dir {
            Dir::Up    => IPos::new(self.row-1, self.col),
            Dir::Right => IPos::new(self.row,   self.col+1),
            Dir::Down  => IPos::new(self.row+1, self.col),
            Dir::Left  => IPos::new(self.row,   self.col-1),
        }
    }
}

macro_rules! ipos {
    ($row:expr,$col:expr) => (
        IPos::new($row, $col)
    )
}
