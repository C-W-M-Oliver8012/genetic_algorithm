
pub enum Direction {
	Left,
	Right,
	Up,
	Down,
}

pub struct FrozenLake {
	lake: [[char; 6]; 6],
	has_won: bool,
	has_died: bool,
	player_x: usize,
	player_y: usize,
	num_moves: usize,
}

impl FrozenLake {
	pub fn new() -> FrozenLake {
		let mut fl = FrozenLake {
			lake: [['H'; 6]; 6],
			has_won: false,
			has_died: false,
			player_x: 1,
			player_y: 1,
			num_moves: 0,
		};

		fl.lake[0] = ['H', 'H', 'H', 'H', 'H', 'H'];
		fl.lake[1] = ['H', 'S', 'F', 'F', 'F', 'H'];
		fl.lake[2] = ['H', 'F', 'H', 'F', 'H', 'H'];
		fl.lake[3] = ['H', 'F', 'F', 'F', 'H', 'H'];
		fl.lake[4] = ['H', 'H', 'F', 'F', 'G', 'H'];
		fl.lake[5] = ['H', 'H', 'H', 'H', 'H', 'H'];

		fl
	}

	pub fn print_board(&self) {
		for i in 0..6 {
			for j in 0..6 {
				if i == self.player_y && j == self.player_x {
					print!("*, ");
				} else {
					print!("{}, ", self.lake[i][j]);
				}
			}
			println!("");
		}
		println!("");
	}

	pub fn print_fitness(&mut self) {
		for i in 0..6 {
			self.player_y = i;
			for j in 0..6 {
				self.player_x = j;
				let fitness: f64 = 8.0 - self.get_x_diff_from_g().abs() - self.get_y_diff_from_g().abs();
				print!("{}, ", fitness.exp2());
			}
			println!("");
		}
		println!("");
	}

	pub fn move_player(&mut self, direction: Direction) {
		if !self.has_died && !self.has_won {
			self.num_moves += 1;
			match direction {
				Direction::Up => self.player_y -= 1,
				Direction::Down => self.player_y += 1,
				Direction::Left => self.player_x -= 1,
				Direction::Right => self.player_x += 1,
			}

			if self.lake[self.player_y][self.player_x] == 'H' {
				self.has_died = true;
			} else if self.lake[self.player_y][self.player_x] == 'G' {
				self.has_won = true;
			}

			if self.num_moves == 50 {
				self.has_died = true;
			}
		}
	}

	pub fn is_game_over(&self) -> bool {
		self.has_won || self.has_died
	}

	pub fn get_num_moves(&self) -> usize {
		self.num_moves
	}

	pub fn get_x_diff_from_g(&self) -> f64 {
		self.player_x as f64 - 4.0
	}

	pub fn get_y_diff_from_g(&self) -> f64 {
		self.player_y as f64 - 4.0
	}

	pub fn get_left(&self) -> char {
		self.lake[self.player_y][self.player_x - 1]
	}

	pub fn get_right(&self) -> char {
		self.lake[self.player_y][self.player_x + 1]
	}

	pub fn get_up(&self) -> char {
		self.lake[self.player_y - 1][self.player_x]
	}

	pub fn get_down(&self) -> char {
		self.lake[self.player_y + 1][self.player_x]
	}
}