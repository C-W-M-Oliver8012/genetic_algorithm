pub mod nn;
pub mod population;
pub mod frozen_lake;
pub mod snake;

use crate::population::Population;
use crate::snake::Snake;

fn main() {
	/*
	let mut population = Population::new(vec![14, 4], 50);

	while !population.get_has_solved() {
		population.calc_fitness();
		println!("Generation {}: af = {}", population.get_generation(), population.get_average_fitness());
		population.produce_new_gen();
	}

	println!("Solved in {} generations.", population.get_generation());
	population.play_best_individual();
	//population.print_best_individual();
	*/

	let mut snake = Snake::new();
	snake.play_game();
}