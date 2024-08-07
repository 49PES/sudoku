use rand::seq::SliceRandom;
use rand::thread_rng;

const ORDER: usize = 3;
const DIM: usize = ORDER * ORDER;

#[derive(Debug)]
struct Board {
    grid: [[Option<usize>; DIM]; DIM],
}

/// NOTE: Generalize this display method for values of ORDER that are not equal to 3
impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, j, v) in self.cells() {
            if j == 0 {
                writeln!(f)?;
                if i % ORDER == 0 {
                    writeln!(f, "—————————————————————————")?;
                }
                write!(f, "| ")?;
            }

            if let Some(v) = v {
                write!(f, "{} ", v)?;
            } else {
                write!(f, "_ ")?;
            }

            if j % ORDER == ORDER - 1 {
                write!(f, "| ")?;
            }
        }

        writeln!(f)?;
        writeln!(f, "—————————————————————————")?;

        Ok(())
    }
}
impl Board {
    /// Iterator over the cells of the board
    fn cells(&self) -> impl Iterator<Item = (usize, usize, Option<usize>)> + '_ {
        (0..DIM).flat_map(move |row| (0..DIM).map(move |col| (row, col, self.grid[row][col])))
    }

    fn get(&self, row: usize, col: usize) -> Option<usize> {
        self.grid[row][col]
    }

    fn set(&mut self, row: usize, col: usize, value: Option<usize>) {
        self.grid[row][col] = value;
    }
    /// Generate a permutation that preserves major and minor rows/cols
    /// (e.g. [0, 1, 2, 3, 4, 5, 6, 7, 8] -> [2, 1, 0, 6, 7, 8, 3, 4, 5])
    fn line_permutation() -> [usize; DIM] {
        let mut rng = thread_rng();
        let mut nums = [0; ORDER];
        for i in 0..ORDER {
            nums[i] = i;
        }
        nums.shuffle(&mut rng);

        let major = nums;

        let minor: [[usize; ORDER]; ORDER] = [0; ORDER].map(|_| {
            nums.shuffle(&mut rng);
            nums
        });

        let nums = major
            .iter()
            .zip(minor.iter())
            .flat_map(|(m, n)| n.iter().map(move |n| m * ORDER + n))
            .collect::<Vec<_>>();

        nums.try_into().unwrap()
    }

    // Permute 1-DIM values
    fn value_permutation() -> [usize; DIM] {
        let mut rng = thread_rng();
        let mut nums = [0_usize; DIM];
        for i in 0..DIM {
            nums[i] = i + 1;
        }
        nums.shuffle(&mut rng);
        nums
    }

    /// Generate a random permutation of the board that preserves the property of having a solution
    fn permute(&self) -> Board {
        let row_permutation = Board::line_permutation();
        let col_permutation = Board::line_permutation();
        let val_permutation = Board::value_permutation();

        let mut grid = [[None; DIM]; DIM];
        for (i, j, v) in self.cells() {
            grid[row_permutation[i]][col_permutation[j]] = v.map(|v| val_permutation[v - 1]);
        }
        Board { grid }
    }

    fn valid_moves(&self, row: usize, col: usize) -> Vec<Option<usize>> {
        let mut neighbors = [false; DIM];

        // Check across the row and col and toggle accordingly
        for i in 0..DIM {
            if let Some(v) = self.get(row, i) {
                neighbors[v - 1] = true;
            }
            if let Some(v) = self.get(i, col) {
                neighbors[v - 1] = true;
            }
        }

        // Round down towards the top-left corner of the sub-grid
        let major_row = ORDER * (row / ORDER);
        let major_col = ORDER * (col / ORDER);

        for r in major_row..major_row + ORDER {
            for c in major_col..major_col + ORDER {
                if let Some(v) = self.get(r, c) {
                    neighbors[v - 1] = true;
                }
            }
        }

        neighbors
            .iter()
            .enumerate()
            .filter(|(_, &is_neighbor)| !is_neighbor)
            .map(|(index, _)| Some(index + 1))
            .collect()
    }

    fn solve(&mut self) -> bool {
        self.solve_aux(0, 0)
    }

    fn solve_aux(&mut self, row: usize, col: usize) -> bool {
        // Have reached below the board (board has been solved)
        if row == DIM {
            return true;
        }
        // Wrap-around
        if col == DIM {
            return self.solve_aux(row + 1, 0);
        }
        // Occupied cell, iterate forwards
        if self.get(row, col).is_some() {
            return self.solve_aux(row, col + 1);
        }

        for valid_move in self.valid_moves(row, col) {
            self.set(row, col, valid_move);
            if self.solve_aux(row, col + 1) {
                return true;
            }
        }
        self.set(row, col, None);
        false
    }
}

fn main() {
    let board_base: [[usize; DIM]; DIM] = [
        [5, 3, 0, 0, 7, 0, 0, 0, 0],
        [6, 0, 0, 1, 9, 5, 0, 0, 0],
        [0, 9, 8, 0, 0, 0, 0, 6, 0],
        [8, 0, 0, 0, 6, 0, 0, 0, 3],
        [4, 0, 0, 8, 0, 3, 0, 0, 1],
        [7, 0, 0, 0, 2, 0, 0, 0, 6],
        [0, 6, 0, 0, 0, 0, 2, 8, 0],
        [0, 0, 0, 4, 1, 9, 0, 0, 5],
        [0, 0, 0, 0, 8, 0, 0, 7, 9],
    ];

    let option_board = board_base.map(|row| row.map(|v| if v == 0 { None } else { Some(v) }));
    let mut board = Board { grid: option_board };

    let mut generated_board = board.permute();
    println!("Original board: {}", board);
    println!("Generated board: {}", generated_board);

    board.solve();
    generated_board.solve();
    println!("Original board solved: {}", board);
    println!("Generated board solved: {}", generated_board);
}
