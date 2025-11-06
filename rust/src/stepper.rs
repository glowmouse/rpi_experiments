use embassy_rp::gpio;
use gpio::{Level, Output};

pub struct Stepper<'a> {
    po: [Output<'a>; 4],
    state: usize
}

impl Stepper<'_> {
    pub fn new(
        pin_0: embassy_rp::peripherals::PIN_0,
        pin_1: embassy_rp::peripherals::PIN_1,
        pin_2: embassy_rp::peripherals::PIN_2,
        pin_3: embassy_rp::peripherals::PIN_3
    ) -> Self {
        let p0o: Output<'_> = Output::new(pin_0, Level::Low );
        let p1o: Output<'_> = Output::new(pin_1, Level::Low );
        let p2o: Output<'_> = Output::new(pin_2, Level::Low );
        let p3o: Output<'_> = Output::new(pin_3, Level::Low );

        let mut rval = Self{ 
            po: [p0o, p2o, p1o, p3o],
            state: 0
        };
        rval.advance();
        rval
    }

    pub fn advance( &mut self ) {
        self.state = (self.state + 1) % 8;

        let h0: usize= self.state/2;
        let h1: usize= ((self.state+1) % 8 )/2;

        for i in 0..4 {
            if i == h0 || i == h1 {
                self.po[i].set_high()
            }
            else {
                self.po[i].set_low()
            }
        }
    }
}

