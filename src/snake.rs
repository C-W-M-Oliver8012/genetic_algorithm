use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::time::{Duration, Instant};
use rand;

pub enum SnakeDirection {
        None,
        Left,
        Right,
        Up,
        Down,
}

#[derive(Copy, Clone)]
struct Position {
        pub x: usize,
        pub y: usize,
}

pub struct Snake {
        grid: [[char; 10]; 10],
        direction: SnakeDirection,
        snake: Vec<Position>,
        food: Position,
        has_won: bool,
        has_died: bool,
        score: usize,
}

impl Snake {
        pub fn new() -> Snake {
                let mut snake = Snake {
                        grid: [[' '; 10]; 10],
                        direction: SnakeDirection::None,
                        snake: Vec::new(),
                        food: Position { x: 7, y: 3 },
                        has_won: false,
                        has_died: false,
                        score: 3,
                };

                snake.snake.push(Position { x: 4, y: 4, });
                snake.snake.push(Position { x: 4, y: 5, });
                snake.snake.push(Position { x: 4, y: 6, });

                snake.grid[0] = ['W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W'];
                snake.grid[1] = ['W', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', 'W'];
                snake.grid[2] = ['W', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', 'W'];
                snake.grid[3] = ['W', ' ', ' ', ' ', ' ', ' ', ' ', 'A', ' ', 'W'];
                snake.grid[4] = ['W', ' ', ' ', ' ', 'S', ' ', ' ', ' ', ' ', 'W'];
                snake.grid[5] = ['W', ' ', ' ', ' ', 'S', ' ', ' ', ' ', ' ', 'W'];
                snake.grid[6] = ['W', ' ', ' ', ' ', 'S', ' ', ' ', ' ', ' ', 'W'];
                snake.grid[7] = ['W', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', 'W'];
                snake.grid[8] = ['W', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', 'W'];
                snake.grid[9] = ['W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W'];

                snake
        }

        pub fn set_direction(&mut self, direction: SnakeDirection) {
                self.direction = direction;
        }

        pub fn is_game_over(&self) -> bool {
                self.has_won || self.has_died
        }

        pub fn print_grid(&self) {
                for i in 0..10 {
                        for j in 0..10 {
                                print!("{}", self.grid[i][j]);
                        }
                        println!("");
                }
                println!("");
        }

        pub fn move_snake(&mut self) {
                if !self.has_won && !self.has_died {
                        let mut new_head_pos: Position = self.snake[0];
                        match self.direction {
                                SnakeDirection::Left => new_head_pos.x -= 1,
                                SnakeDirection::Right => new_head_pos.x += 1,
                                SnakeDirection::Up => new_head_pos.y -= 1,
                                SnakeDirection::Down => new_head_pos.y += 1,
                                _ => return,
                        }

                        // ran into the wall or your tail
                        if self.grid[new_head_pos.y][new_head_pos.x] == 'W' {
                                self.has_died = true;
                                return;
                        } else if self.grid[new_head_pos.y][new_head_pos.x] == 'S' {
                                self.has_died = true;
                                return;
                        }

                        // got an apple
                        if self.grid[new_head_pos.y][new_head_pos.x] == 'A' {
                                self.score += 1;
                                let y = self.snake[self.snake.len() - 1].y;
                                let x = self.snake[self.snake.len() - 1].x;
                                self.snake.push(Position { x: x, y: y, });
                                self.grid[y][x] = 'S';

                                if self.score < 64 {
                                        loop {
                                                let food_x = rand::random::<usize>() % 8 + 1;
                                                let food_y = rand::random::<usize>() % 8 + 1;
        
                                                if self.grid[food_y][food_x] == ' ' {
                                                        self.food = Position { x: food_x, y: food_y };
                                                        self.grid[food_y][food_x] = 'A';
                                                        break;
                                                }
                                        }
                                } else {
                                        self.has_won = true;
                                }
                        }

                        // update body in grid
                        let mut previous_position: Position = self.snake[0];
                        for i in 0..self.snake.len() {
                                if i == 0 {
                                        self.snake[i] = new_head_pos;
                                } else {
                                        // undraw previous tail position
                                        if i == self.snake.len() - 1 {
                                                self.grid[self.snake[i].y][self.snake[i].x] = ' ';
                                        }
                                        let current_position = self.snake[i];
                                        self.snake[i] = previous_position;
                                        previous_position = current_position;
                                }
                                self.grid[self.snake[i].y][self.snake[i].x] = 'S';
                        }
                }
        }

        pub fn play_game(&mut self) {
                let sdl_context = sdl2::init().unwrap();
                let video_subsystem = sdl_context.video().unwrap();

                let window = video_subsystem.window("Snake", 320, 320)
                        .build()
                        .unwrap();
                
                let mut canvas = window.into_canvas().build().unwrap();
                let mut event_pump = sdl_context.event_pump().unwrap();

                let mut screen: [[Rect; 8]; 8] = [[Rect::new(0, 0, 0, 0); 8]; 8];

                for i in 0..8 {
                        for j in 0..8 {
                                screen[i][j] = Rect::new(j as i32 * 40, i as i32 * 40, 40, 40);
                        }
                }

                let mut move_frame = Instant::now();

                'running: loop {
                        let time_frame = Instant::now();

                        for event in event_pump.poll_iter() {
                                match event {
                                        Event::Quit {..} => break 'running,
                                        Event::KeyDown { keycode: Some(Keycode::I),.. } => self.set_direction(SnakeDirection::Up),
                                        Event::KeyDown { keycode: Some(Keycode::K),.. } => self.set_direction(SnakeDirection::Down),
                                        Event::KeyDown { keycode: Some(Keycode::J),.. } => self.set_direction(SnakeDirection::Left),
                                        Event::KeyDown { keycode: Some(Keycode::L),.. } => self.set_direction(SnakeDirection::Right),
                                        _ => {},
                                }
                        }

                        if move_frame.elapsed() > Duration::new(0, 200000000) {
                                move_frame = Instant::now();
                                self.move_snake();
                        }

                        for i in 1..9 {
                                for j in 1..9 {
                                        if self.grid[i][j] == ' ' {
                                                canvas.set_draw_color(Color::RGB(0, 0, 0));
                                        } else if self.grid[i][j] == 'S' {
                                                canvas.set_draw_color(Color::RGB(0, 255, 0));
                                        } else if self.grid[i][j] == 'A' {
                                                canvas.set_draw_color(Color::RGB(255, 0, 0));
                                        }
                                        canvas.fill_rect(screen[i - 1][j - 1]).unwrap();
                                        canvas.draw_rect(screen[i - 1][j - 1]).unwrap();
                                }
                        }

                        canvas.present();
                        match Duration::new(0, 16666670).checked_sub(time_frame.elapsed()) {
                                Some(t) => std::thread::sleep(t),
                                None => {},
                        }
                }
                println!("Score: {}", self.score);
        }
}