use crate::board;
use rand::Rng;


#[derive(Clone, Default)]
pub struct Particle {
    pub position: Vec<f64>,
    pub velocity: Vec<f64>,
    pub best_position: Vec<f64>,
	pub best_fitness: f64
}

impl Particle {
	pub fn update_velocity(
		&mut self,
		w: f64,
		c1: f64,
		c2: f64,
		global_best: &Particle
	) {
		self.velocity = self.velocity.iter()
		.zip(self.position.iter())
		.zip(self.best_position.iter())
		.zip(global_best.position.iter())
		.map(|(((velocity, position), best_position), global_best_position)| {
			w * velocity
			+ c1 * rand::thread_rng().gen_range(0.0..=1.0) * (best_position - position)
			+ c2 * rand::thread_rng().gen_range(0.0..=1.0) * (global_best_position - position)
		}).collect();
	}

	pub fn update_position(&mut self) {
		self.position = self.position.iter()
		.zip(self.velocity.iter())
		.map(|(position, velocity)| {
			position + velocity
		}).collect()
	}

	pub fn initialize_random(n: i32) -> Particle {
		let mut position: Vec<f64> = Vec::new();
		let mut velocity: Vec<f64> = Vec::new();
		let mut best_position: Vec<f64> = Vec::new();
		for _ in 0..n {
			position.push(rand::thread_rng().gen_range(0.0..(n as f64)));
			velocity.push(rand::thread_rng().gen_range(0.0..=1.0));
			best_position.push(0.0)
		}
		Particle { position, velocity, best_position, best_fitness: 0.0 }
	}
	pub fn fitness(&self) -> f64 {
		let n = self.position.len();
		let mut n_safe = 0;
	
		for i in 0..n {
			let row = self.position[i] as i32;

			if row < 0 || row >= (n as i32) {
				return 0.0;
			}

			let mut safe = true;
	
			for j in 0..n {
				if j == i { continue };
				// Check if on same row
				if self.position[j] as i32 == row {
					safe = false;
					break;
				}
				// Check diagonally
				let offset = ((i as i32) - (j as i32)).abs();
				if self.position[j] as i32 == row + offset || self.position[j] as i32 == row - offset {
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
}

pub fn pso(w: f64, c1: f64, c2:f64, population: i32, n: i32) { 
	let mut global_best: Particle = Particle::initialize_random(n);
	let mut swarm: Vec<Particle> = Vec::new();

	for _ in 0..population { // initialize random sawrm
		swarm.push(Particle::initialize_random(n));	
	}

	loop {
		for i in 0..swarm.len() {  // calculate fitness
			let fitness = swarm[i].fitness();
			if fitness > swarm[i].best_fitness {
				swarm[i].best_fitness = fitness;
				swarm[i].best_position = swarm[i].position.clone();
				if fitness > global_best.best_fitness {
					global_best = swarm[i].clone();
				}
			}
			swarm[i].update_velocity(w, 
				c1,
				c2,
				&global_best
			);
			swarm[i].update_position();
		}

		if global_best.best_fitness == 1.0 {
			let queens: Vec<i32> = (
				global_best.best_position.iter().map(|x| {
					*x as i32
				})
			).collect();
			println!("Solution found: {:?}", queens);
			break;
		}
	}
}
