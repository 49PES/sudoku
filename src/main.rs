use rand::seq::SliceRandom;
use rand::thread_rng;

const ORDER: usize = 3;
const DIM: usize = ORDER * ORDER;

#[derive(Debug)]
struct Board {
    grid: [[Option<usize>; DIM]; DIM],
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "—————————————————————————")?;
        for i in 0..DIM {
            for j in 0..DIM {
                if j == 0 {
                    write!(f, "| ")?;
                }
                if let Some(v) = self.grid[i][j] {
                    write!(f, "{} ", v)?;
                } else {
                    write!(f, "_ ")?;
                }

                if j % ORDER == ORDER - 1 {
                    write!(f, "| ")?;
                }
            }
            writeln!(f)?;

            if i % ORDER == ORDER - 1 {
                writeln!(f, "—————————————————————————")?;
            }
        }
        Ok(())
    }
}
impl Board {
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
        for i in 0..DIM {
            for j in 0..DIM {
                grid[row_permutation[i]][col_permutation[j]] =
                    self.grid[i][j].map(|v| val_permutation[v - 1]);
            }
        }
        Board { grid }
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
    let board = Board { grid: option_board };

    println!("{}", board);
    println!("{}", board.permute());
    println!("{}", board.permute());
    println!("{}", board.permute());
    println!("{}", board);
}
