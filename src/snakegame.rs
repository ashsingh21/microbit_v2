use arrayvec::ArrayVec;
use embedded_hal::digital::InputPin;
use microbit::display::blocking::Display;
use nrf52833_hal::Timer;
use rtt_target::rprintln;

use crate::led;


enum Direction {
    Down,
    Right,
}

struct Snake {
    pub body: ArrayVec<(usize, usize), 25>,
    pub direction: Direction,
}

impl Snake {
    pub fn new(start: (usize, usize), direction: Direction, size: u8) -> Self {

        let head = (start.0, start.1);
        let mut body = ArrayVec::new();

        match direction {
            Direction::Down => {
                for i in 0..size {
                    body.push(((head.0 + i as usize) % 5, head.1));
                }
            }
            Direction::Right => {
                for i in 0..size {
                    body.push((head.0, (head.1 + i as usize) % 5));
                }
            }
        }

        Self {
            body,
            direction: direction,
        }
    }

    pub fn move_snake(&mut self) {
        // Move the snake in the current direction
        let head = *self.body.first().expect("Snake body is empty");


        // button pressed

        let new_head = match self.direction {
            Direction::Down => ((head.0 + 1) % 5, head.1),
            Direction::Right => (head.0, (head.1 + 1) % 5),
        };

        // Add new head to the snake body
        self.grow(new_head);
        self.body.pop(); // Remove the tail
    }

    pub fn grow(&mut self, (x, y): (usize, usize)) {
        self.body.insert(0, (x, y));
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }
}

pub struct SnakeGame {
    pub snake: Snake,
    pub food: [(usize, usize); 2],
    board: microbit::Board,
}

impl SnakeGame {
    pub fn new() -> Self {
        let board = microbit::board::Board::take().unwrap();

        let snake_start = (2, 2);

        // initial food positions
        // You can change these positions to any valid coordinates on the 5x5 grid
        let food = [
            (0, 0), // Food position 1
            (4, 4), // Food position 2
        ];

        Self {
            board,
            snake: Snake::new(snake_start, Direction::Down, 2),
            food,
        }
    }

    pub fn render_game_state(led_matrix: &mut led::LedMatrix, snake: &Snake, food: &[(usize, usize)]) {
        // Clear the display
        led_matrix.clear();

        // Render the food
        for (x, y) in food {
            led_matrix.blink(*x, *y);
        }

        // Render the snake
        for (x, y) in &snake.body {
            led_matrix.turn_on(*x, *y);
        }
    }

    pub fn run(mut self) -> ! {
        let timer = Timer::new(self.board.TIMER0);
        let display = Display::new(self.board.display_pins);
        let mut led_matrix = led::LedMatrix::new(display, timer);

        let mut snake = self.snake;

        // Game loop
        loop {
            if self.board.buttons.button_a.is_low().unwrap() {
                rprintln!("Button A pressed");
                snake.set_direction(Direction::Down);
            }

            if self.board.buttons.button_b.is_low().unwrap() {
                rprintln!("Button B pressed");
                snake.set_direction(Direction::Right);
            }

            snake.move_snake();


            Self::render_game_state(&mut led_matrix, &snake, &self.food);
            Self::show(&mut led_matrix, 300)
        }
    }

    pub fn show(led_matrix: &mut led::LedMatrix, delay_ms: u32) {
        led_matrix.show(delay_ms);
    }
}

