use std::fmt;
use rand::Rng;



#[derive(Clone, Default)]
pub struct Board {
    pub queens: Vec<i32>
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str: String = String::new();

        for num in &self.queens[0..self.queens.len() - 1] {
            str.push_str(&num.to_string());
            str.push_str(", ");
        }

        str.push_str(&self.queens[self.queens.len() - 1].to_string());
        write!(f, "{}", str)
    }
}
pub fn fitness(board: &Board) -> f64 {
    let n = board.queens.len();
    let mut n_safe = 0;

    for i in 0..n {
        let row = board.queens[i];
        let mut safe = true;

        for j in 0..n {
            if j == i { continue };
            // Check if on same row
            if board.queens[j] == row {
                safe = false;
                break;
            }
            // Check diagonally
            let offset = ((i as i32) - (j as i32)).abs();
            if board.queens[j] == row + offset || board.queens[j] == row - offset {
                safe = false;
                break;
            }
        }
        if safe {
            n_safe += 1;
        }
    }

    return (n_safe as f64) / (n as f64);
}


// generate neighbors 
pub fn generate_neighboards(board: &Board) -> Vec<Board>{
    let mut rng = rand::thread_rng();
    let n = board.queens.len();
    let mut neighboards:Vec<Board> = vec![];

    for i in 0..n {
        let mut this_move: Board = board.clone();
        this_move.queens[i] = rng.gen_range(1..=8);
        neighboards.push(this_move);
        /*
        Encountered plateuing, changed random moves
        let row = board.queens[i];
        for amount in 1..(std::cmp::max(row - 1, (n as i32) - row)) {
            if row + amount < (n as i32) {
                this_move.queens[i] = row + amount;
                neighboards.push(this_move.clone());
            }
            if row > amount {
                this_move.queens[i] = row - amount;
                neighboards.push(this_move);
            }
        }
        */
    }
    return neighboards; 
}


