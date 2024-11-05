use core::cell::{Cell, RefCell};
use embassy_sync::blocking_mutex::Mutex;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_rp::pwm::{Config, Pwm};
use embassy_rp::interrupt;
use portable_atomic::{AtomicU32, Ordering};

use embassy_rp::gpio;
use gpio::{Level, Output};

static COUNTER: AtomicU32 = AtomicU32::new(0);
static PWM: Mutex<CriticalSectionRawMutex, RefCell<Option<Pwm>>> = Mutex::new(RefCell::new(None));

pub struct Interrupt<'a> {
    debug_out: Output<'a>,
    //state: u32
}

impl Interrupt<'_> {
    pub fn new(
        pin: embassy_rp::peripherals::PIN_1,
        debug_pin: embassy_rp::peripherals::PIN_2,
        pwm_slice: embassy_rp::peripherals::PWM_SLICE0 
    ) -> Self {
        let debug_out: Output<'_> = Output::new(debug_pin, Level::High );
        let pwm = embassy_rp::pwm::Pwm::new_output_b(pwm_slice, pin, Default::default());
        PWM.lock(|p| p.borrow_mut().replace(pwm));

        // Enable the interrupt for pwm slice 0
        embassy_rp::pac::PWM.inte().modify(|w| w.set_ch0(true));
        unsafe {
            cortex_m::peripheral::NVIC::unmask(interrupt::PWM_IRQ_WRAP);
        }

        //Self { /*data,*/ state:0 }
        Self {debug_out}
    }

    pub fn update(&mut self)
    {
        let counter = COUNTER.load(Ordering::Relaxed);

        match counter % 2 {
            0 => {
                self.debug_out.set_high();
            }
            1..=u32::MAX => {
                self.debug_out.set_low();
            }
        }

        //self.debug_out.set_low();
        /*
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
        */
    }
}

#[interrupt]
fn PWM_IRQ_WRAP() {
    critical_section::with(|cs| {
        //let mut adc = ADC.borrow(cs).borrow_mut();
        //let (adc, p26) = adc.as_mut().unwrap();
        //let val = adc.blocking_read(p26).unwrap();
        //ADC_VALUES.try_send(val).ok();

        // Clear the interrupt, so we don't immediately re-enter this irq handler
        PWM.borrow(cs).borrow_mut().as_mut().unwrap().clear_wrapped();
    });
    COUNTER.fetch_add(1, Ordering::Relaxed);
}


