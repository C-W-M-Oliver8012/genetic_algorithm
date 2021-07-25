use crate::nn::NN;
use rand;

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
                        let mut fitness: f64 = 1.0;

                        let mut outputs = self.population[i].feed_forward(vec![0.0, 0.0]).unwrap();
                        if outputs[0] > outputs[1] {
                                fitness += 1.0;
                        }

                        outputs = self.population[i].feed_forward(vec![0.0, 1.0]).unwrap();
                        if outputs[1] > outputs[0] {
                                fitness += 1.0;
                        }

                        outputs = self.population[i].feed_forward(vec![1.0, 0.0]).unwrap();
                        if outputs[1] > outputs[0] {
                                fitness += 1.0;
                        }

                        outputs = self.population[i].feed_forward(vec![1.0, 1.0]).unwrap();
                        if outputs[0] > outputs[1] {
                                fitness += 1.0;
                        }

                        self.pop_fitness[i] = fitness.exp2();

                        if fitness.exp2() > self.best_fitness {
                                self.best_fitness = fitness.exp2();
                        }

                        // solved the problem
                        if self.pop_fitness[i] == 32.0 {
                                self.has_solved = true;
                                self.best_individual = self.population[i].clone();
                        }
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