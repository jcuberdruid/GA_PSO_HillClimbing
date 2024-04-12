#![allow(dead_code)]
#![allow(unused_variables)]

use shuffle::shuffler::Shuffler;
use shuffle::irs::Irs;
use rand::rngs::mock::StepRng;
use rand::Rng;
use tokio::time::{timeout, Duration};
use std::time::Instant;

mod board;
mod hillclimb;
mod geneticAlgorithm;
pub mod pso;

#[tokio::main]
async fn main() {
    //let mut queens: Vec<i32> = [1, 2, 3, 4, 5, 6, 7, 8].to_vec();
    let mut queens: Vec<i32> = Vec::new();
    for i in 0..100 {
        queens.push(i);
    }

    let mut rng: StepRng = StepRng::new(2, 13);
    let mut irs: Irs<i32> = Irs::default();
    let _ = irs.shuffle(&mut queens, &mut rng);

    let test_board = board::Board{queens};
    //println!("{}", board::fitness(&test_board));



    let possible_moves = board::generate_neighboards(&test_board);
    //println!("{}",possible_moves.len());

    //hillclimb::hillclimb(test_board);
    //geneticAlgorithm::genetic_algorithm(test_board, 5000, 200,  3,  0.05);
   // pso::pso(0.7288994, 0.5, 0.5, 20000, 8);
    let mut n = 0;
    let mut start = Instant::now(); // Start timing
    let max_duration = Duration::from_secs(15); 

    let mut hill_climb_time: Vec<u128> = Vec::new();
    let mut ga_time: Vec<u128> = Vec::new();
    let mut pso_time: Vec<u128> = Vec::new();

    for n in 4..=6 {
        println!("#############################################");
        println!("# Beginning n = {}",n);
        println!("#############################################");
        start = Instant::now();
        let n1 = n.clone();
        let n2 = n.clone();
        let n3 = n.clone();
        match timeout(max_duration, tokio::task::spawn_blocking(move || {
            println!("######## Hillclimb ########");
            let mut queens: Vec<i32> = Vec::new();
            for i in 0..n1 {
                queens.push(i);
            }
        
            let mut rng: StepRng = StepRng::new(2, 13);
            let mut irs: Irs<i32> = Irs::default();
            let _ = irs.shuffle(&mut queens, &mut rng);    
            let test_board = board::Board{queens: queens.clone()};        
            hillclimb::hillclimb(test_board.clone())
        })).await {
            Ok(_) => {
                hill_climb_time.push(start.elapsed().as_millis());
            },
            Err(_) => {
                hill_climb_time.push(max_duration.as_millis());
            },    
        };
        
        start = Instant::now();
        match timeout(max_duration, tokio::task::spawn_blocking(move || {
            println!("######## GA ########");
            let mut queens: Vec<i32> = Vec::new();
            for i in 0..n2 {
                queens.push(i);
            }
        
            let mut rng: StepRng = StepRng::new(2, 13);
            let mut irs: Irs<i32> = Irs::default();
            let _ = irs.shuffle(&mut queens, &mut rng);    
            let test_board = board::Board{queens: queens.clone()};
            geneticAlgorithm::genetic_algorithm(test_board, 100000, 1000,  2,  0.1);
        })).await {
            Ok(_) => {
                ga_time.push(start.elapsed().as_millis());
            },
            Err(_) => {
                ga_time.push(max_duration.as_millis());
            },    
        };

        println!("######## PSO ########");
        start = Instant::now();
        match timeout(max_duration, tokio::task::spawn_blocking(move || {
            pso::pso(0.7288994, 0.5, 0.5, 10000, n3);
        })).await {
            Ok(_) => {
                pso_time.push(start.elapsed().as_millis());
            },
            Err(_) => {
                pso_time.push(max_duration.as_millis());
            },    
        };
        println!("######## Timing for n={} ########", n);
        println!("Hill climb times: {:?}", hill_climb_time);
        println!("Genetic algorithm times: {:?}", ga_time);
        println!("Particle swarm optimization times: {:?}", pso_time);
    }
}
