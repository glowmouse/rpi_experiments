//! Draw a square, circle and triangle on a 128x32px display.
//!
//! This example is for the STM32F103 "Blue Pill" board using I2C1.
//!
//! Wiring connections are as follows for a CRIUS-branded display:
//!
//! ```
//!      Display -> Blue Pill
//! (black)  GND -> GND
//! (red)    +5V -> VCC
//! (yellow) SDA -> PB7
//! (green)  SCL -> PB6
//! ```
//!
//! Run on a Blue Pill with `cargo run --bin graphics_i2c_128x32`.

#![no_std]
#![no_main]

//use cortex_m::asm::nop;
use cortex_m_rt::entry;
use defmt_rtt as _;
use panic_probe as _;

#[entry]
fn main() -> ! {
    let p = embassy_rp::init(Default::default());
    let mut display = hackernewyears::Display::new(
        p.I2C0, p.PIN_17, // SCLR
        p.PIN_16, // SDA
    );

    let mut leds = hackernewyears::LEDs::new(
        p.PIN_11,
        p.PIN_12,
        p.PIN_13,
    );
    leds.set( 1, 1, true );
    leds.set( 1, 3, true );
    leds.set( 3, 1, true );
    leds.set( 3, 3, true );

    display.update();

    loop {
        leds.update();
    }
}
