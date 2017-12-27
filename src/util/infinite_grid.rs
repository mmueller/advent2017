use util::grid::IPos;
use std::ops::{Index,IndexMut};

/* An infinite 2-dimensional grid where every position has default value until
 * it is set otherwise. */
#[derive(Clone)]
pub struct InfiniteGrid<T: Clone> {
    rows: Vec<Vec<T>>,
    default: T
}

impl<T: Clone> InfiniteGrid<T> {
    pub fn new(default: T) -> InfiniteGrid<T> {
        InfiniteGrid {
            rows: Vec::new(),
            default: default
        }
    }

    // Ensure the underlying vectors have min_rows and min_cols capacity.
    fn ensure_capacity(&mut self, min_rows: usize, min_cols: usize) {
        let orig_num_rows = self.rows.len();
        if orig_num_rows < min_rows {
            self.rows.reserve(min_rows - orig_num_rows);
            for _ in orig_num_rows..min_rows {
                self.rows.push(Vec::with_capacity(min_cols));
            }
        }
        for row in 0..min_rows {
            if self.rows[row].len() < min_cols {
                self.rows[row].resize(min_cols, self.default.clone());
            }
        }
    }

    // Map a position (in one dimension) from the infinite (isize range) space
    // to a natural number index (usize) for the underlying Vec.
    fn pos_to_index(pos: isize) -> usize {
        let result = if pos < 0 {
                         (-pos*2) - 1
                     } else {
                         pos * 2
                     };
        result as usize
    }
}

// Index into the grid at the position (row, col).
impl<T: Clone> Index<IPos> for InfiniteGrid<T> {
    type Output = T;

    fn index<'a>(&'a self, pos: IPos) -> &'a T {
        let row_index = Self::pos_to_index(pos.row);
        let col_index = Self::pos_to_index(pos.col);
        if self.rows.len() <= row_index ||
           self.rows[row_index].len() <= col_index {
            return &self.default
        }
        &self.rows[row_index][col_index]
    }
}

// Write into the grid at the position (row, col).
impl<T: Clone> IndexMut<IPos> for InfiniteGrid<T> {
    fn index_mut<'a>(&'a mut self, pos: IPos) -> &'a mut T {
        let row_index = Self::pos_to_index(pos.row);
        let col_index = Self::pos_to_index(pos.col);
        self.ensure_capacity(row_index+1, col_index+1);
        &mut self.rows[row_index][col_index]
    }
}

#[cfg(test)]
mod tests {
    use super::InfiniteGrid;

    #[test]
    fn can_be_constructed_with_int() {
        let grid: InfiniteGrid<isize> = InfiniteGrid::new(0);
        assert!(grid.default == 0);
    }

    #[test]
    fn returns_default_for_out_of_bounds() {
        let grid: InfiniteGrid<isize> = InfiniteGrid::new(5);
        assert!(grid[ipos!(-1000, 545)] == 5);
    }

    #[test]
    fn set_values_and_read_them_back() {
        let mut grid: InfiniteGrid<char> = InfiniteGrid::new('x');
        grid[ipos!(10, 10)]  = 'a';
        grid[ipos!(-5, -9)]  = 'b';
        grid[ipos!(0, 0)]    = 'c';
        grid[ipos!(14, -14)] = 'd';
        grid[ipos!(-14, 14)] = 'e';

        assert!(grid[ipos!(1, 1)]    == 'x');
        assert!(grid[ipos!(10, 10)]  == 'a');
        assert!(grid[ipos!(-5, -9)]  == 'b');
        assert!(grid[ipos!(0, 0)]    == 'c');
        assert!(grid[ipos!(14, -14)] == 'd');
        assert!(grid[ipos!(-14, 14)] == 'e');
        
        // overwrite
        grid[ipos!(0, 0)] = 'z';
        assert!(grid[ipos!(0, 0)] == 'z');
    }
}
