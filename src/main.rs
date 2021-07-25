pub mod nn;
pub mod population;
pub mod frozen_lake;

use crate::population::Population;

fn main() {
	let mut population = Population::new(vec![14, 12, 12, 12, 8, 4], 50);

	while !population.get_has_solved() {
		population.calc_fitness();
		println!("Generation {}: af = {}", population.get_generation(), population.get_average_fitness());
		population.produce_new_gen();
	}

	println!("Solved in {} generations.", population.get_generation());
	population.play_best_individual();
	//population.print_best_individual();
}