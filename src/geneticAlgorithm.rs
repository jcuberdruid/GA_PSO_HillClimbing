use crate::board;
use rand::Rng;

pub fn generate_random_queens(initial_board: board::Board, population: i32) -> Vec<board::Board> {
    let n = initial_board.queens.len();
    let mut boards: Vec<board::Board> = Vec::new();

    for _ in 0..population {
        let mut temp_initial = initial_board.clone();
        for x in 0..temp_initial.queens.len() {
            temp_initial.queens[x] = rand::thread_rng().gen_range(0..n) as i32;
        }
        boards.push(temp_initial.clone());
        //println!("{:#?}",temp_initial.queens);
    }
    return boards;
}
// Corrected cross_over
pub fn cross_over(board_1: &board::Board, board_2: &board::Board) -> board::Board {
    let mut child = board_1.clone();
    let mut rng = rand::thread_rng();
    child.queens.iter_mut().enumerate().for_each(|(i, queen)| {
        if rng.gen_bool(0.5) { // 50% chance
            *queen = board_2.queens[i];
        }
    });
    child
}
fn replace_if_larger_than_min(vec: &mut Vec<f64>, value: f64, genome_indices_vec: &mut Vec<i32>, index: i32){
    if let Some(min) = vec.iter().min_by(|a, b| a.partial_cmp(b).unwrap()) {
        if value > *min {
            if let Some(pos) = vec.iter().position(|x| x == min) {
                //println!("{}",pos);
                vec[pos] = value;
                genome_indices_vec[pos] = index;
            }
        }
    }
}

// Optimized average calculation
fn average(numbers: &[f64]) -> f64 {
    numbers.iter().sum::<f64>() / numbers.len() as f64
}

pub fn genetic_algorithm(initial_state: board::Board, pop_size: i32, cross_over_pop: i32, max_stagnant_itr: i32, mutation_rate: f64) { 
    //population size, Cross over pool %, mutation rate, max_stagnant_itr
    let n = initial_state.queens.len();
    let mut genomes: Vec<board::Board> = generate_random_queens(initial_state, pop_size);
    
    let mut last_avg_fitness:f64 = 0.0; 
    let mut stagnant_itr_count = 0;
    let mut fitness_of_genomes: Vec<f64> = Vec::new(); 
    let mut iterations_taken = 0; 
    loop {
        // get fitness and find the most fit
        let mut most_fit_genomes_fitnesses: Vec<f64> = vec![0.0; cross_over_pop.try_into().unwrap()];
        let mut most_fit_genomes_indices: Vec<i32> = vec![0; cross_over_pop.try_into().unwrap()];

        for i in 0..pop_size {
            fitness_of_genomes.push(board::fitness(&genomes[i as usize]));
            replace_if_larger_than_min(&mut most_fit_genomes_fitnesses, fitness_of_genomes[i as usize], &mut most_fit_genomes_indices, i as i32);
        }
        //mutate? 
	let new_avg_fitness = average(&most_fit_genomes_fitnesses);
        let diff = (new_avg_fitness - last_avg_fitness).abs();
        let average = (new_avg_fitness + last_avg_fitness) / 2.0;
        let tolerance = average * 0.05;
        if diff <= tolerance {
           stagnant_itr_count += 1;
           if stagnant_itr_count == max_stagnant_itr {
                    //mutate 
                    for i in 0..cross_over_pop {
                        let mutate = (rand::thread_rng().gen_range(0..100) as f64)/100.0; 
                        
                        //println!("mutation? {} (should be if > 5)", mutate);
                        if mutate > mutation_rate {
                            let rand_queen = rand::thread_rng().gen_range(0..n) as i32;
                            genomes[i as usize].queens[rand_queen as usize] = rand::thread_rng().gen_range(0..n) as i32;
                        }
                    }
                    last_avg_fitness = 0.0;
                    stagnant_itr_count = 0;
           } 
        } 
       last_avg_fitness = new_avg_fitness;
        //cross over most fit: 
        let mut new_genomes: Vec<board::Board> = Vec::new(); 
        while new_genomes.len() <= pop_size as usize {
            let rand_1 = rand::thread_rng().gen_range(0..cross_over_pop) as i32;
            let rand_2 = rand::thread_rng().gen_range(0..cross_over_pop) as i32;
            let child_board: board::Board = cross_over(&genomes[most_fit_genomes_indices[rand_1 as usize] as usize], &genomes[most_fit_genomes_indices[rand_2 as usize] as usize]);
            new_genomes.push(child_board);
        } 
        genomes = new_genomes;
        fitness_of_genomes = Vec::new();
        println!("End of iteration, stagnant iter {}, avg best fitness {}", stagnant_itr_count, new_avg_fitness);
        iterations_taken += 1;     
        if new_avg_fitness == 1.0 {
            break;
        }
    }
    println!("Solution Found! in {} iterations", iterations_taken);
}
