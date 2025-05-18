use cortex_m_rt::entry;
use embedded_hal::i2c::I2c;


use nrf52833_hal::{saadc::Time, Timer};
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;

use microbit::{
    hal::twim,
    pac::twim0::frequency::FREQUENCY_A,
};

use lsm303agr::{
    AccelOutputDataRate, Lsm303agr,
};

const ACCELEROMETER_ADDR: u8 = 0b0011001;
const MAGNETOMETER_ADDR: u8 = 0b0011110;

const ACCELEROMETER_ID_REG: u8 = 0x0f;
const MAGNETOMETER_ID_REG: u8 = 0x4f;

pub enum Orientation {
    Up,
    Down,
    Left,
    Right,
    Unknown,
}

pub fn run() -> ! {
    let board = microbit::Board::take().unwrap();

    let i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };

    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();

    let mut delay = Timer::new(board.TIMER0);

    sensor.set_accel_mode_and_odr(&mut delay, lsm303agr::AccelMode::HighResolution, AccelOutputDataRate::Hz10).unwrap();

    loop {
        if sensor.accel_status().expect("failed to get accelerometer status").xyz_new_data() {
            let data = sensor.acceleration().expect("Failed to read acceleration data");

            let x = data.x_mg();

            if x > 0 {
                rprintln!("Left");
            }
            else if x < 0 {
                rprintln!("Right");
            }

            rprintln!("Acceleration: x {} y {} z {}", data.x_mg(), data.y_mg(), data.z_mg());
        }
    }
}