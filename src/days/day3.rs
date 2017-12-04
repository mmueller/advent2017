const TARGET: usize = 347991;

pub fn solve() {
    // Part 1: If 1 is the origin (x=0, y=0), then distance is trivially
    // calculated by spiraling out to our target number.
    let mut pos: (isize, isize) = (0, 0);
    // Considering right and down positive, these mark the lower right corner
    // of the square we're traversing. When we reach the lower right corner,
    // we have to grow the bounds again.
    let mut bounds: (isize, isize) = (0, 0);
    for i in 1..TARGET+1 {
        if i == TARGET {
            break;
        }
        pos = next_pos(pos, bounds);
        if pos.0 > bounds.0 {
            bounds = (bounds.0+1, bounds.1+1)
        }
    }

    println!("{} occurs at ({}, {}), dist: {}",
             TARGET, pos.0, pos.1, pos.0.abs()+pos.1.abs());

    build_pt2_matrix();
}

fn next_pos(pos: (isize, isize), bounds: (isize, isize)) -> (isize, isize) {
    // Bottom edge
    if pos.1 == bounds.1 {
        // May exceed bounds, caller should update if necessary
        (pos.0+1, pos.1)
    // Right edge
    } else if pos.0 == bounds.0 {
        if pos.1 > -bounds.1 {
            (pos.0, pos.1-1)
        } else {
            (pos.0-1, pos.1)
        }
    // Top edge
    } else if pos.1 == -bounds.1 {
        if pos.0 > -bounds.0 {
            (pos.0-1, pos.1)
        } else {
            (pos.0, pos.1+1)
        }
    // Left edge
    } else /* if pos.0 == -bounds.0 */ {
        if pos.1 < bounds.1 {
            (pos.0, pos.1+1)
        } else {
            (pos.0+1, pos.1)
        }
    }
}

// Implement an indexing scheme that allows us to map negative & positive
// indexes into positions in the range 0..∞.
fn pos_to_index(pos: isize) -> usize {
    let result = if pos < 0 {
                     (-pos*2) - 1
                 } else {
                     pos * 2
                 };
    result as usize
}

// I never needed the inverse, but for reference it would look like this:
#[allow(dead_code)]
fn index_to_pos(index: usize) -> isize {
    let sindex = index as isize;
    if sindex % 2 == 1 {
        -(sindex+1)/2
    } else {
        sindex/2
    }
}

fn sum_of_neighbors(matrix: &Vec<Vec<usize>>, pos: (isize, isize)) -> usize {
    let offsets: [(isize, isize); 8] = [
        (-1, -1), ( 0, -1), ( 1, -1),
        (-1,  0),           ( 1,  0),
        (-1,  1), ( 0,  1), ( 1,  1)
    ];
    let mut result: usize = 0;
    for offset in offsets.into_iter() {
        let neighbor_pos = (pos.0 + offset.0, pos.1 + offset.1);
        let neighbor_index = (pos_to_index(neighbor_pos.0),
                              pos_to_index(neighbor_pos.1));
        if matrix.len() > neighbor_index.1 &&
           matrix[neighbor_index.1].len() > neighbor_index.0 {
            result += matrix[neighbor_index.1][neighbor_index.0];
        }
    }
    result
}

fn build_pt2_matrix() {
    let mut matrix: Vec<Vec<usize>> = Vec::new();
    matrix.push(Vec::new());
    matrix[0].push(1);
    let mut value: usize = 1;
    let mut pos: (isize, isize) = (0, 0);
    let mut bounds: (isize, isize) = (0, 0);
    while value <= TARGET {
        pos = next_pos(pos, bounds);
        if pos.0 > bounds.0 {
            bounds = (bounds.0+1, bounds.1+1);
        }
        value = sum_of_neighbors(&matrix, pos);
        let next_index = (pos_to_index(pos.0), pos_to_index(pos.1));
        while matrix.len() <= next_index.1 {
            matrix.push(Vec::new());
        }
        if matrix[next_index.1].len() <= next_index.0 {
            matrix[next_index.1].resize(next_index.0+1, 0);
        }
        matrix[next_index.1][next_index.0] = value;
    }
    println!("First value greater than target: {}", value);
}
