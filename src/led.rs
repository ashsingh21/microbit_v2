use embedded_hal::digital::InputPin;
use microbit::{display::blocking::Display, pac::TIMER0};
use nrf52833_hal::Timer;
use rtt_target::rprintln;

pub struct LedGame {
    board: microbit::Board,
}

impl LedGame {
    pub fn try_new() -> Self {
        Self { board: microbit::Board::take().unwrap() }
    }

    pub fn run(mut self) -> ! {
        let timer = Timer::new(self.board.TIMER0);
        let display = Display::new(self.board.display_pins);

        let mut matrix = LedMatrix::new(display, timer);

        // bottom row is obstacle
        let obstacles = [
            (3,1)
        ];


        for obstacle in obstacles {
            matrix.turn_on(obstacle.0, obstacle.1);
            rprintln!("Obstacle at: {:?}", obstacle);
        }

        let start = (2, 2);
        let mut player = start;
        matrix.blink(player.0, player.1);

        loop {

            if let Ok(true) = self.board.buttons.button_a.is_low() {
                rprintln!("Button A pressed");
                // move the row
                let next_led_location = ((player.0 + 1) % 5, player.1);
                // check if the next led location is an obstacle
                let obstacle = obstacles.iter().find(|&&obstacle| obstacle == next_led_location);
                if obstacle.is_none() {
                    // turn off the current led
                    matrix.turn_off(player.0, player.1);
                    player = next_led_location;
                    matrix.blink(player.0, player.1);
                } else {
                    rprintln!("Player is on the obstacle");
                }
            }

            if let Ok(true) = self.board.buttons.button_b.is_low() {
                rprintln!("Button B pressed");
                // move the column
                let next_led_location = (player.0, (player.1 + 1) % 5);
                // check if the next led location is an obstacle
                let obstacle = obstacles.iter().find(|&&obstacle| obstacle == next_led_location);
                if obstacle.is_none() {
                    // turn off the current led
                    matrix.turn_off(player.0, player.1);
                    player = next_led_location;
                    matrix.blink(player.0, player.1);
                } else {
                    rprintln!("Player is on the obstacle");
                }
            }

            matrix.show(100);
            // add delay
            // loop_timer.delay_ms(100);
        }
    }
}


#[derive(Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum LedState {
    Blink{ toggle: bool },
    On,
    Off
}

pub struct Led {
    state: LedState
}

impl Led {
    /// Creates a new LED in the obstacle state.
    pub fn new_obstacle() -> Self {
        Self { state: LedState::On }
    }

    /// Creates a new LED in the off state.
    pub fn new() -> Self {
        Self { state: LedState::Off }
    }
}

pub struct LedMatrix {
    display: Display,
    timer: Timer<TIMER0>,
    leds: [[LedState; 5]; 5],
}

impl LedMatrix {
    pub fn new(display: Display, timer: Timer<TIMER0>) -> Self {
        Self {
            display,
            timer,
            leds: [[LedState::Off; 5]; 5],
        }
    }

    #[inline(always)]
    pub fn turn_on(&mut self, x: usize, y: usize) {
        self.set_state(x, y, LedState::On);
    }
    
    #[inline(always)]
    pub fn turn_off(&mut self, x: usize, y: usize) {
        self.set_state(x, y, LedState::Off);
    }

    #[inline(always)]
    pub fn blink(&mut self, x: usize, y: usize) {
        self.set_state(x, y, LedState::Blink { toggle: true });
    }

    #[inline(always)]
    pub fn set_state(&mut self, x: usize, y: usize, state: LedState) {
        if x < 5 && y < 5{
            self.leds[x][y] = state;
        } else {
            rprintln!("Invalid LED coordinates: ({}, {})", x, y);
        }
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        for i in 0..5 {
            for j in 0..5 {
                match self.leds[i][j] {
                    _ => {
                        self.leds[i][j] = LedState::Off;
                    }
                }
            }
        }
    }

    #[inline(always)]
    pub fn render(&mut self) -> [[u8; 5]; 5] {
        let mut leds: [[u8; 5]; 5] = [[0; 5]; 5];
        for i in 0..5 {
            for j in 0..5 {
                match self.leds[i][j] {
                    LedState::Blink { toggle } => {
                        leds[i][j] = toggle as u8;
                        self.leds[i][j] = LedState::Blink { toggle: !toggle };
                    },
                    LedState::On => leds[i][j] = 1,
                    LedState::Off => leds[i][j] = 0,
                }
            }
        }
        leds
    }

    #[inline(always)]
    pub fn show(&mut self, duration_ms: u32) {
        let leds = self.render();
        self.display.show(&mut self.timer, leds, duration_ms);
    }
}