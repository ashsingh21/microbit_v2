use arrayvec::ArrayVec;
use embedded_hal::digital::InputPin;
use microbit::display::blocking::Display;
use nrf52833_hal::Timer;
use rtt_target::rprintln;

use crate::led;


pub enum Direction {
    Down,
    Right,
}

const TOTAL_LED : usize = 25;

pub struct Snake {
    pub body: ArrayVec<(usize, usize), TOTAL_LED>,
    pub direction: Direction,
}

impl Snake {
    pub fn new(start: (usize, usize), direction: Direction, size: usize) -> Self {
        let mut body: ArrayVec<(usize, usize), TOTAL_LED> = ArrayVec::new();

        body.push(start);

        match direction {
            Direction::Down => {
                for i in 1..size {
                    body.push(((start.0 + i) % 5, start.1));
                }
            }
            Direction::Right => {
                for i in 1..size {
                    body.push((start.0, (start.1 + i) % 5));
                }
            }
        }

        Self {
            body: body,
            direction: Direction::Down,
        }
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn move_snake(&mut self) {
        if self.body.len() == 0 {
            panic!("Snake body is empty");
        }

        // wrap around the matrix
        match self.direction {
            Direction::Down => {
                let head = self.body[0];
                let new_head =  ((head.0 + 1) % 5, head.1);

                self.body.insert(0, new_head);
                self.body.pop();
            },
            Direction::Right => {
                let head = self.body[0];
                let new_head = (head.0, (head.1 + 1) % 5);

                self.body.insert(0, new_head);
                self.body.pop();
            }
        }
    }
}

pub struct SnakeGame {
    pub snake: Snake,
    pub board: microbit::Board,
}

impl SnakeGame {
    pub fn new() -> Self {
        let board = microbit::board::Board::take().unwrap();

        Self {
            snake: Snake::new((0, 2), Direction::Down, 3),
            board,
        }
    }

    pub fn run(mut self) -> ! {

        let timer = Timer::new(self.board.TIMER0);
        let display = Display::new(self.board.display_pins);

        let mut led_matrix = led::LedMatrix::new(display, timer);
        // Game loop
        loop {

            if let Ok(true) = self.board.buttons.button_a.is_low() {
                rprintln!("Button A pressed");
                self.snake.set_direction(Direction::Down);
            }
            if let Ok(true) = self.board.buttons.button_b.is_low() {
                rprintln!("Button B pressed");
                self.snake.set_direction(Direction::Right);
            }

            match self.snake.direction {
                Direction::Down => {
                    self.snake.direction = Direction::Down;
                }
                Direction::Right => {
                    self.snake.direction = Direction::Right;
                }
            }

            self.snake.move_snake();
            Self::render_snake(&self.snake, &mut led_matrix);
            
            led_matrix.show(100);
        }
    }

    fn render_snake(snake: &Snake, led_matrix: &mut led::LedMatrix) {
        // Clear the matrix
        led_matrix.clear(); // TODO this will remove the food too

        // Draw the snake
        for (x, y) in snake.body.iter() {
            led_matrix.turn_on(*x, *y);
        }
    }
}

