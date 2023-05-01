use winit::event::*;
use rand::Rng;

use crate::render::new;

pub const GRID_SIZE: [i32;2] = [20, 20];
pub const DEFAULT_POS: [u16;2] = [(GRID_SIZE[0] / 2) as u16, (GRID_SIZE[1] / 2) as u16];
const FRAME_TIME: f32 = 0.01;
const MOVE_TIME: f32 = 1.0;

pub struct GameState {
    pub board: [[Option<Cell>; GRID_SIZE[1] as usize]; GRID_SIZE[0] as usize],
    pub snake_pos: [usize; 2],
    apple_pos: [usize; 2],
    tail: Vec<[usize; 2]>,
    dir: [i8; 2],
    time: f32,
    paused: bool,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Cell {
    Head,
    Tail,
    Apple
}

impl GameState {
    pub fn new() -> Self {
        let mut board = [[None; GRID_SIZE[1] as usize]; GRID_SIZE[0] as usize];

        let snake_pos = GameState::random_position();
        let apple_pos = GameState::random_position();

        board[snake_pos[0] as usize][snake_pos[1] as usize] = Some(Cell::Head);
        board[apple_pos[0] as usize][apple_pos[1] as usize] = Some(Cell::Apple);

        GameState {
            board,
            dir: [0,0],
            time: 0.0,
            snake_pos,
            paused: false,
            tail: vec![],
            apple_pos
        }
    }

    pub fn update(&mut self) {
        if self.paused {return}

        self.time += FRAME_TIME;

        if self.time > MOVE_TIME {
            self.move_snake();
            self.time = 0.0;
        }
    }

    fn move_snake(&mut self) {
        if self.dir == [0, 0] { return }

        self.board[self.snake_pos[0]][self.snake_pos[1]] = None;

        if self.tail.len() > 0 {
            for tail in self.tail.iter() {
                self.board[tail[0]][tail[1]] = None;
            }
            self.tail = self.tail.iter().zip(self.tail.iter().skip(1)).map(|(&a, _)| a).collect::<Vec<_>>();
            self.tail.insert(0, self.snake_pos);
            for tail in self.tail.iter() {
                self.board[tail[0]][tail[1]] = Some(Cell::Tail);
            }
        }

        let new_pos = [(self.snake_pos[0] as i8 + self.dir[0]), (self.snake_pos[1] as i8 + self.dir[1])];
        
        if new_pos[0] < 0 || new_pos[0] >= GRID_SIZE[0] as i8 || new_pos[1] < 0 || new_pos[1] >= GRID_SIZE[1] as i8 {
            self.reset_game();
            return
        }

        let new_pos = [new_pos[0] as usize, new_pos[1] as usize];

        if self.board[new_pos[0]][new_pos[1]] != None {
            if self.apple_pos == new_pos { 
                self.tail.push(self.snake_pos);
                self.random_apple(); 
            } else {
                self.reset_game();
                return
            }
        }
        self.snake_pos = new_pos;
        self.board[self.snake_pos[0]][self.snake_pos[1]] = Some(Cell::Head);
    }

    fn random_apple(&mut self) {
        self.board[self.apple_pos[0]][self.apple_pos[1]] = None;
        self.apple_pos = GameState::random_position();
        self.board[self.apple_pos[0]][self.apple_pos[1]] = Some(Cell::Apple);
    }

    fn reset_game(&mut self) {
        self.dir = [0, 0];
        self.board = [[None; GRID_SIZE[1] as usize]; GRID_SIZE[0] as usize];

        self.snake_pos = GameState::random_position();
        self.apple_pos = GameState::random_position();

        self.board[self.snake_pos[0] as usize][self.snake_pos[1] as usize] = Some(Cell::Head);
        self.board[self.apple_pos[0] as usize][self.apple_pos[1] as usize] = Some(Cell::Apple);

        for tail in self.tail.iter() {
            self.board[tail[0]][tail[1]] = None;
        }

        println!("Your score was: {}", self.tail.len());
        self.tail.clear();
    }

    fn random_position() -> [usize; 2] {
        [rand::thread_rng().gen_range(0..GRID_SIZE[0]) as usize, rand::thread_rng().gen_range(0..GRID_SIZE[1]) as usize]
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Space),
                        ..
                    },
                ..
            } => {
                self.paused = !self.paused;
                return true;
            }
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Down),
                        ..
                    },
                ..
            } => {
                if self.dir != [0, 1] { self.dir = [0, -1]; }
                return true;
            }
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Up),
                        ..
                    },
                ..
            } => {
                if self.dir != [0, -1] { self.dir = [0, 1]; }
                return true;
            }
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Left),
                        ..
                    },
                ..
            } => {
                if self.dir != [1, 0] { self.dir = [-1, 0]; }
                return true;
            }
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Right),
                        ..
                    },
                ..
            } => {
                if self.dir != [-1, 0] { self.dir = [1, 0]; }
                return true;
            }
            _ => { }
        }
        false
    }
}
