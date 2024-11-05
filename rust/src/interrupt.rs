use embassy_rp::gpio;
use gpio::{Level, Output};

pub struct Interrupt<'a> {
    data: Output<'a>,
    state: u32
}

impl Interrupt<'_> {
    pub fn new(
        pin: embassy_rp::peripherals::PIN_10,
    ) -> Self {
        let data: Output<'_> = Output::new(pin, Level::High );

        Self { data, state:0 }
    }

    pub fn update(&mut self)
    {
        let cycle = self.state % 2;

        match cycle {
            0 => {
                self.data.set_low();
            }
            1..=u32::MAX => {
                self.data.set_high();
            }
        }

        self.state = self.state + 1;
    }
}

