use crate::board;

pub fn hillclimb(initial_state: board::Board) { 
    let mut current_state = initial_state.clone();

    while board::fitness(&current_state) < 1.0 {
        let mut current_fitness = board::fitness(&current_state);
        let neighboards: Vec<board::Board> = board::generate_neighboards(&current_state);
        let neighboards_fitness: Vec<_> = neighboards.iter().map(board::fitness).collect();
        let n_neighbors = neighboards.len();
        for i in 0..n_neighbors {
            if neighboards_fitness[i] >= current_fitness {
                current_state = neighboards[i].clone();
                current_fitness = neighboards_fitness[i];
            }
        }
    }

    println!("Solution found {}", current_state)
}
