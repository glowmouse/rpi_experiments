#![no_std]
#![no_main]

use hackernewyears::Stepper;

use defmt_rtt as _;
use panic_probe as _;

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = embassy_rp::init(Default::default());

    let mut stepper = Stepper::new(
        p.PIN_0,
        p.PIN_1,
        p.PIN_2,
        p.PIN_3,
    );

    let mut ticker = embassy_time::Ticker::every(embassy_time::Duration::from_micros(1000));
    loop {
        ticker.next().await;
        stepper.advance()
    }
}

