#![deny(warnings)]      //  If the Rust compiler generates a warning, stop the compilation with an error.
#![no_main]             //  Don't use the Rust standard bootstrap. We will provide our own.
#![no_std]              //  Don't use the Rust standard library. We are building a binary that can run on its own.

use core::fmt::Debug;

use cortex_m_rt::{entry, exception, ExceptionFrame};    //  Stack frame for exception handling.
use panic_semihosting as _;
use cortex_m_semihosting::hprintln;

use embedded_hal::digital::v2::OutputPin;
use stm32f1xx_hal::{pac, prelude::*};

struct LedMatrix<ClockPin, DataPin, StrobePin> {
    clock: ClockPin,
    data: DataPin,
    strobe: StrobePin,
    // Make size an u16 to prevent overflow on multiplication and avoid
    // lots of casting
    height: u16,
    width: u16,
}

impl<ClockPin, DataPin, StrobePin> LedMatrix<ClockPin, DataPin, StrobePin> where
    ClockPin: OutputPin, ClockPin::Error: Debug,
    DataPin: OutputPin, DataPin::Error: Debug,
    StrobePin: OutputPin, StrobePin::Error: Debug,
{
    fn new(
        clock: ClockPin, data: DataPin, strobe: StrobePin,
        height: u16, width: u16,
    ) -> LedMatrix<ClockPin, DataPin, StrobePin> {
        LedMatrix {
            clock,
            data,
            strobe,
            height,
            width,
        }
    }

    fn pulse_clock(&mut self) {
        self.clock.set_high().unwrap();
        self.clock.set_low().unwrap();
    }

    fn show(&mut self) {
        self.strobe.set_high().unwrap();
        self.strobe.set_low().unwrap();
    }

    fn clear(&mut self) {
        self.data.set_low().unwrap();
        for _ in 0 .. self.width * self.height {
            self.pulse_clock();
        }
    }

    fn pixel_on(&mut self) {
        self.data.set_high().unwrap();
        self.pulse_clock();
    }

    fn pixel_off(&mut self) {
        self.data.set_low().unwrap();
        self.pulse_clock();
    }

}

#[entry]
fn main() -> ! {
    hprintln!("Starting");

    // Init chip
    let _cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let _clocks = rcc.cfgr
        .use_hse(8.MHz())
        .sysclk(72.MHz())
        .hclk(72.MHz())
        .pclk1(36.MHz())
        .freeze(&mut flash.acr);

    // Init GPIO
    let mut gpioa = dp.GPIOA.split();
    let clock0 = gpioa.pa0.into_push_pull_output(&mut gpioa.crl);
    let strobe0 = gpioa.pa1.into_push_pull_output(&mut gpioa.crl);
    let data0 = gpioa.pa2.into_push_pull_output(&mut gpioa.crl);

    let clock1 = gpioa.pa5.into_push_pull_output(&mut gpioa.crl);
    let strobe1 = gpioa.pa6.into_push_pull_output(&mut gpioa.crl);
    let data1 = gpioa.pa7.into_push_pull_output(&mut gpioa.crl);

    hprintln!("Init OK!");

    // LED matrix properties
    const HEIGHT: u16 = 8;
    const WIDTH: u16 = 72;

    // Hello world
    let pixels: [u8; 576] = [255, 0, 255, 0, 255, 0, 255, 0, 0, 255, 0, 255, 0, 255, 0, 255, 255, 0, 255, 0, 255, 0, 255, 0, 0, 255, 0, 255, 0, 255, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 0, 255, 0, 0, 0, 0, 0, 255, 0, 255, 0, 255, 255, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 0, 0, 255, 0, 255, 0, 255, 255, 255, 255, 255, 0, 255, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 255, 255, 255, 255, 0, 255, 255, 255, 0, 255, 255, 0, 255, 255, 255, 255, 0, 255, 255, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 255, 255, 255, 255, 255, 255, 255, 0, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 255, 255, 255, 255, 255, 255, 255, 255, 0, 255, 0, 0, 255, 255, 255, 255, 255, 0, 255, 255, 0, 255, 255, 255, 255, 0, 255, 255, 0, 255, 255, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 255, 255, 0, 255, 255, 255, 255, 0, 255, 255, 0, 255, 255, 255, 255, 0, 255, 255, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 255, 255, 255, 255, 0, 0, 0, 255, 255, 255, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 255, 255, 255, 255, 0, 0, 255, 255, 255, 255, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 255, 255, 0, 255, 255, 255, 255, 0, 255, 255, 0, 255, 255, 255, 255, 0, 255, 255, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 255, 255, 255, 255, 255, 255, 255, 0, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 255, 255, 255, 255, 255, 255, 255, 0, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 255, 255, 0, 255, 0, 255, 255, 0, 255, 255, 0, 255, 0, 255, 255, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 0, 255, 255, 255, 255, 255, 255, 255, 0, 255, 255, 255, 255, 255, 255, 255, 0, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255];

    let mut matrix0 = LedMatrix::new(clock0, data0, strobe0, HEIGHT, WIDTH);
    let mut matrix1 = LedMatrix::new(clock1, data1, strobe1, HEIGHT, WIDTH);

    // Start a full display width outside of draw area
    const INITXPOS: u16 = 2 * WIDTH;
    let mut xpos = INITXPOS;

    loop {
        matrix0.clear();
        matrix1.clear();

        // Leading space for scroller
        let maxpos = if xpos < WIDTH {
            HEIGHT*xpos
        } else {
            HEIGHT*WIDTH
        };

        for pixel in pixels[0..maxpos as usize].iter() {
            if *pixel == 0 {
                matrix0.pixel_on();
                matrix1.pixel_off();
            } else {
                matrix0.pixel_off();
                matrix1.pixel_on();
            }
        }

        // Trailing space for scroller
        if xpos > WIDTH {
            for _ in 0 .. HEIGHT*(xpos-WIDTH) {
                matrix0.pixel_off();
                matrix1.pixel_on();
            }
        }

        matrix0.show();
        matrix1.show();
        xpos -= 1;

        if xpos == 0 {
            xpos = INITXPOS;
        }
    }
}

#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("Hard fault: {:#?}", ef);
}

#[exception]
unsafe fn DefaultHandler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}

