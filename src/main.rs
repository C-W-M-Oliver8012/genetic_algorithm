pub mod nn;
pub mod population;
pub mod frozen_lake;
pub mod snake;

use crate::population::Population;

fn main() {
	let mut population = Population::new(vec![14, 4], 50);
	let mut solved_generation = 0;
	let mut has_solved = false;

	while population.get_generation() < 100 {
		population.calc_fitness();
		println!("Generation {}: af = {}", population.get_generation(), population.get_average_fitness());
		population.produce_new_gen();
		if population.get_has_solved() && !has_solved {
			has_solved = true;
			solved_generation = population.get_generation();
		}
	}

	println!("Solved in {} generations.", solved_generation);
	population.play_best_individual();
	//population.print_best_individual();
}