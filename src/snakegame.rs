use microbit::display::blocking::Display;
use nrf52833_hal::Timer;

use crate::led;


enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Snake {
    pub size: usize,
    pub start: (usize, usize),
    pub end: (usize, usize),
    pub direction: Direction,
}

impl Snake {
    pub fn new(start: (usize, usize)) -> Self {
        Self {
            start,
            end: start,
            size: 3,
            direction: Direction::Right,
        }
    }

    pub fn grow(&mut self) {
        self.size += 1;
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }
}

pub struct SnakeGame {
    pub led_matrix: led::LedMatrix,
    pub snake: Snake,
}

impl SnakeGame {
    pub fn new() -> Self {
        let board = microbit::board::Board::take().unwrap();
        let timer = Timer::new(board.TIMER0);
        let display = Display::new(board.display_pins);
        let led_matrix = led::LedMatrix::new(display, timer);

        let snake_start = (2, 2);

        Self {
            led_matrix,
            snake: Snake::new(snake_start),
        }
    }

    pub fn render_snake(&mut self) {
        for i in 0..self.snake.size {
            let x = self.snake.start.0 + i;
            let y = self.snake.start.1;
            self.led_matrix.turn_on(x, y);
        }
    }

    pub fn run(&mut self) -> ! {
        // let start = (2, 2);
        // let player = start;
        // self.led_matrix.turn_on(player.0, player.1);

        // Game loop
        loop {
            // Update snake position based on direction
            match self.snake.direction {
                Direction::Up => {}
                Direction::Down => {}
                Direction::Left => {}
                Direction::Right => {}
            }

            self.render_snake();
            // Render the game state
            self.led_matrix.show(100);
        }
    }
}

