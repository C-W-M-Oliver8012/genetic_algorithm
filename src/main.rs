pub mod nn;
pub mod population;

use crate::population::Population;

fn main() {
	let mut population = Population::new(vec![2, 2, 2], 200);

	while !population.get_has_solved() {
		population.calc_fitness();
		population.produce_new_gen();
	}

	println!("Solved in {} generations.", population.get_generation());
	population.print_best_individual();
}