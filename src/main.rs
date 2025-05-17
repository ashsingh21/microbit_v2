#![no_std]
#![no_main]

mod microphone;
mod speaker;
mod led;
mod snakegame;
mod sensors;

use panic_rtt_target as _;
use rtt_target::{rtt_init_print, rprintln};

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Starting LED Game");
    
    let mut snake_game = snakegame::SnakeGame::new();
    snake_game.run();
 }




