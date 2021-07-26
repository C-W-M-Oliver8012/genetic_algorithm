use crate::nn::NN;
use crate::frozen_lake::{FrozenLake, Direction};
use rand;
use std::{thread, time};

pub struct Population {
	population: Vec<NN>,
	pop_fitness: Vec<f64>,
	pop_size: usize,
	generation: usize,
	best_individual: NN,
	best_fitness: f64,
	has_solved: bool,
}

impl Population {
	pub fn new(nn_info: Vec<usize>, pop_size: usize) -> Population {
		let mut population = Population {
			population: Vec::new(),
			pop_fitness: Vec::new(),
			pop_size: pop_size,
			generation: 0,
			best_individual: NN::new(nn_info.clone()),
			best_fitness: 0.0,
			has_solved: false,
		};

		for _i in 0..pop_size {
			population.population.push(NN::new(nn_info.clone()));
		}

		for _i in 0..pop_size {
			population.pop_fitness.push(0.0);
		}

		population
	}

	pub fn get_has_solved(&self) -> bool {
		self.has_solved
	}

	pub fn get_generation(&self) -> usize {
		self.generation
	}

	pub fn get_average_fitness(&self) -> f64 {
		let mut total_fitness = 0.0;

		for i in 0..self.pop_size {
			total_fitness += self.pop_fitness[i];
		}

		total_fitness / self.pop_size as f64
	}

	pub fn print_best_individual(&self) {
		self.best_individual.print_nodes();
		self.best_individual.print_connections();
	}

	pub fn calc_fitness(&mut self) {
		for i in 0..self.pop_size {
			let mut frozen_lake = FrozenLake::new();
			while !frozen_lake.is_game_over() {
				let left = frozen_lake.get_left();
				let right = frozen_lake.get_right();
				let up = frozen_lake.get_up();
				let down = frozen_lake.get_down();

				let mut inputs = Vec::new();

				inputs.push((left == 'F' || left == 'S') as i64 as f64); // input 1
				inputs.push((left == 'H') as i64 as f64); // input 2
				inputs.push((left == 'G') as i64 as f64); //input 3
				inputs.push((right == 'F' || right == 'S') as i64 as f64); // input 4
				inputs.push((right == 'H') as i64 as f64); // input 5
				inputs.push((right == 'G') as i64 as f64); // input 6
				inputs.push((up == 'F' || up == 'S') as i64 as f64); // input 7
				inputs.push((up == 'H') as i64 as f64); // input 8
				inputs.push((up == 'G') as i64 as f64); // input 9
				inputs.push((down == 'F' || down == 'S') as i64 as f64); // input 10
				inputs.push((down == 'H') as i64 as f64); // input 11
				inputs.push((down == 'G') as i64 as f64); // input 12
				inputs.push(frozen_lake.get_x_diff_from_g()); // input 13
				inputs.push(frozen_lake.get_y_diff_from_g()); // input 14

				let outputs = self.population[i].feed_forward(inputs).unwrap();
				let up_d = outputs[0];
				let down_d = outputs[1];
				let left_d = outputs[2];
				let right_d = outputs[3];

				let mut direction = Direction::Left;

				if up_d >= down_d && up_d >= left_d && up_d >= right_d {
					direction = Direction::Up;
				} else if down_d >= up_d && down_d >= left_d && down_d >= right_d {
					direction = Direction::Down;
				} else if left_d >= up_d && left_d >= down_d && left_d >= right_d {
					direction = Direction::Left;
				} else if right_d >= up_d && right_d >= down_d && right_d >= left_d {
					direction = Direction::Right;
				}

				frozen_lake.move_player(direction);
			}
			let mut fitness: f64 = 8.0 - frozen_lake.get_x_diff_from_g().abs() - frozen_lake.get_y_diff_from_g().abs();
			fitness = fitness.exp2() + frozen_lake.get_num_moves() as f64;

			self.pop_fitness[i] = fitness;

			if fitness > self.best_fitness {
				self.best_fitness = fitness;
				self.best_individual = self.population[i].clone();
			}

			if fitness >= 256.0 {
				self.has_solved = true;
			}
		}
	}

	pub fn play_best_individual(&mut self) {
		let mut frozen_lake = FrozenLake::new();
		frozen_lake.print_board();
		while !frozen_lake.is_game_over() {
			let left = frozen_lake.get_left();
			let right = frozen_lake.get_right();
			let up = frozen_lake.get_up();
			let down = frozen_lake.get_down();

			let mut inputs = Vec::new();

			inputs.push((left == 'F' || left == 'S') as i64 as f64); // input 1
			inputs.push((left == 'H') as i64 as f64); // input 2
			inputs.push((left == 'G') as i64 as f64); //input 3
			inputs.push((right == 'F' || right == 'S') as i64 as f64); // input 4
			inputs.push((right == 'H') as i64 as f64); // input 5
			inputs.push((right == 'G') as i64 as f64); // input 6
			inputs.push((up == 'F' || up == 'S') as i64 as f64); // input 7
			inputs.push((up == 'H') as i64 as f64); // input 8
			inputs.push((up == 'G') as i64 as f64); // input 9
			inputs.push((down == 'F' || down == 'S') as i64 as f64); // input 10
			inputs.push((down == 'H') as i64 as f64); // input 11
			inputs.push((down == 'G') as i64 as f64); // input 12
			inputs.push(frozen_lake.get_x_diff_from_g()); // input 13
			inputs.push(frozen_lake.get_y_diff_from_g()); // input 14

			let outputs = self.best_individual.feed_forward(inputs).unwrap();
			let up_d = outputs[0];
			let down_d = outputs[1];
			let left_d = outputs[2];
			let right_d = outputs[3];

			let mut direction = Direction::Left;

			if up_d >= down_d && up_d >= left_d && up_d >= right_d {
				direction = Direction::Up;
			} else if down_d >= up_d && down_d >= left_d && down_d >= right_d {
				direction = Direction::Down;
			} else if left_d >= up_d && left_d >= down_d && left_d >= right_d {
				direction = Direction::Left;
			} else if right_d >= up_d && right_d >= down_d && right_d >= left_d {
				direction = Direction::Right;
			}

			frozen_lake.move_player(direction);
			thread::sleep(time::Duration::new(2, 0));
			frozen_lake.print_board();
		}
	}

	pub fn produce_new_gen(&mut self) {
		self.generation += 1;
		let mut new_population: Vec<NN> = Vec::new();

		for _i in 0..self.pop_size {
			let mut parent1: usize = 0;
			let mut parent2: usize = 0;
			let mut found_parent1 = false;
			let mut found_parent2 = false;

			while !found_parent1 {
				let random_index = rand::random::<usize>() % self.pop_size;
				let accept_reject = rand::random::<f64>() * self.best_fitness;

				if accept_reject < self.pop_fitness[random_index] {
					found_parent1 = true;
					parent1 = random_index;
				}
			}

			while !found_parent2 {
				let random_index = rand::random::<usize>() % self.pop_size;
				let accept_reject = rand::random::<f64>() * self.best_fitness;

				if accept_reject < self.pop_fitness[random_index] {
					found_parent2 = true;
					parent2 = random_index;
				}
			}

			new_population.push(self.population[parent1].crossover(self.population[parent2].clone()));
		}
		self.population = new_population;
	}
}