use std::ops::{Index,IndexMut};

#[derive(Clone)]
pub struct InfiniteTape<T: Clone> {
    pub vec: Vec<T>,
    default: T
}

impl<T: Clone> InfiniteTape<T> {
    pub fn new(default: T) -> InfiniteTape<T> {
        InfiniteTape {
            vec: Vec::new(),
            default: default
        }
    }

    // Map a position (in one dimension) from the infinite (isize range) space
    // to a natural number index (usize) for the underlying Vec.
    fn pos_to_index(pos: isize) -> usize {
        (if pos < 0 {
            (-pos*2) - 1
        } else {
            pos * 2
        }) as usize
    }
}

// Index into the grid at the given position.
impl<T: Clone> Index<isize> for InfiniteTape<T> {
    type Output = T;

    fn index<'a>(&'a self, pos: isize) -> &'a T {
        let index = Self::pos_to_index(pos);
        if index >= self.vec.len() {
            return &self.default
        }
        &self.vec[index]
    }
}

// Write into the grid at the given position.
impl<T: Clone> IndexMut<isize> for InfiniteTape<T> {
    fn index_mut<'a>(&'a mut self, pos: isize) -> &'a mut T {
        let index = Self::pos_to_index(pos);
        if index >= self.vec.len() {
            self.vec.resize(index+1, self.default.clone());
        }
        &mut self.vec[index]
    }
}
