use core::cell::{RefCell};
use embassy_sync::blocking_mutex::Mutex;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_rp::pwm::{Config, Pwm};
use embassy_rp::interrupt;
use portable_atomic::{AtomicU32, Ordering};
use fixed::{FixedU16};

use embassy_rp::gpio;
use gpio::{Level, Output};

static COUNTER: AtomicU32 = AtomicU32::new(0);
static PWM: Mutex<CriticalSectionRawMutex, RefCell<Option<Pwm>>> = Mutex::new(RefCell::new(None));

pub struct Sound<'a> {
    debug_out: Output<'a>,
}

impl Sound<'_> {
    pub fn new(
        pin: embassy_rp::peripherals::PIN_1,
        debug_pin: embassy_rp::peripherals::PIN_2,
        pwm_slice: embassy_rp::peripherals::PWM_SLICE0 
    ) -> Self {
        let debug_out: Output<'_> = Output::new(debug_pin, Level::High );
        let pwm = embassy_rp::pwm::Pwm::new_output_b(pwm_slice, pin, Default::default());
        PWM.lock(|p| p.borrow_mut().replace(pwm));

        // PWM frequency is 62.5Mhz
        // Divided by 128, 268353
        // Top 65535,  4hz

        let mut config = Config::default();
        config.top = 65535;
        config.compare_b = config.top/2;
        config.divider= FixedU16::from_bits(4095);
        PWM.lock(|p| p.borrow_mut().as_mut().unwrap().set_config(&config));

        // Enable the interrupt for pwm slice 0
        embassy_rp::pac::PWM.inte().modify(|w| w.set_ch0(true));
        unsafe {
            cortex_m::peripheral::NVIC::unmask(interrupt::PWM_IRQ_WRAP);
        }

        Self {debug_out}
    }

    // Entirely for debugging.
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
    }
}

#[interrupt]
fn PWM_IRQ_WRAP() {
    critical_section::with(|cs| {
        PWM.borrow(cs).borrow_mut().as_mut().unwrap().clear_wrapped();
    });
    COUNTER.fetch_add(1, Ordering::Relaxed);
}
