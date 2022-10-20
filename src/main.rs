#![deny(unsafe_code)]   //  Don't allow unsafe code in this file.
#![deny(warnings)]      //  If the Rust compiler generates a warning, stop the compilation with an error.
#![no_main]             //  Don't use the Rust standard bootstrap. We will provide our own.
#![no_std]              //  Don't use the Rust standard library. We are building a binary that can run on its own.

use core::fmt::Debug;

use cortex_m_rt::{entry, exception, ExceptionFrame};    //  Stack frame for exception handling.
use panic_semihosting as _;

use embedded_hal::digital::v2::OutputPin;
use stm32f1xx_hal::{delay::Delay, pac, prelude::*};

struct LedMatrix<ClockPin, DataPin, StrobePin> {
    clock: ClockPin,
    data: DataPin,
    strobe: StrobePin,
    height: u8,
    width: u8,
}

impl<ClockPin, DataPin, StrobePin> LedMatrix<ClockPin, DataPin, StrobePin> where
    ClockPin: OutputPin, ClockPin::Error: Debug,
    DataPin: OutputPin, DataPin::Error: Debug,
    StrobePin: OutputPin, StrobePin::Error: Debug,
{
    fn new(
        clock: ClockPin, data: DataPin, strobe: StrobePin,
        height: u8, width: u8,
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
        for _ in 0 .. self.width as u16 * self.height as u16 {
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
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut delay = Delay::new(cp.SYST, clocks);

    delay.delay_us(1u16);

    const HEIGHT: u8 = 8;
    const WIDTH: u8 = 72;

    let pixels: [u8; 576] = [255, 0, 255, 0, 255, 0, 255, 0, 0, 255, 0, 255, 0, 255, 0, 255, 255, 0, 255, 0, 255, 0, 255, 0, 0, 255, 0, 255, 0, 255, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 0, 255, 0, 0, 0, 0, 0, 255, 0, 255, 0, 255, 255, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 0, 0, 255, 0, 255, 0, 255, 255, 255, 255, 255, 0, 255, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 255, 255, 255, 255, 0, 255, 255, 255, 0, 255, 255, 0, 255, 255, 255, 255, 0, 255, 255, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 255, 255, 255, 255, 255, 255, 255, 0, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 255, 255, 255, 255, 255, 255, 255, 255, 0, 255, 0, 0, 255, 255, 255, 255, 255, 0, 255, 255, 0, 255, 255, 255, 255, 0, 255, 255, 0, 255, 255, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 255, 255, 0, 255, 255, 255, 255, 0, 255, 255, 0, 255, 255, 255, 255, 0, 255, 255, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 255, 255, 255, 255, 0, 0, 0, 255, 255, 255, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 255, 255, 255, 255, 0, 0, 255, 255, 255, 255, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 255, 255, 0, 255, 255, 255, 255, 0, 255, 255, 0, 255, 255, 255, 255, 0, 255, 255, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 255, 255, 255, 255, 255, 255, 255, 0, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 255, 255, 255, 255, 255, 255, 255, 0, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 255, 255, 0, 255, 0, 255, 255, 0, 255, 255, 0, 255, 0, 255, 255, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 0, 255, 255, 255, 255, 255, 255, 255, 0, 255, 255, 255, 255, 255, 255, 255, 0, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255];

    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let clock = gpioa.pa0.into_push_pull_output(&mut gpioa.crl);
    let data = gpioa.pa2.into_push_pull_output(&mut gpioa.crl);
    let strobe = gpioa.pa1.into_push_pull_output(&mut gpioa.crl);


    let mut matrix = LedMatrix::new(clock, data, strobe, HEIGHT, WIDTH);
    
    matrix.clear();
    let mut xpos = 144u16;

    loop {
        matrix.clear();

        let maxpos = if xpos < 72 {
            8*xpos
        } else {
            8*72
        };

        for pixel in pixels[0..maxpos as usize].iter() {
            if *pixel == 0 {
                matrix.pixel_on();
            } else {
                matrix.pixel_off();
            }
        }

        if xpos > 72 {
            for _ in 0 .. 8*(xpos-72) {
                matrix.pixel_off();
            }

        }

        matrix.show();
        xpos -= 1;
    }
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("Hard fault: {:#?}", ef);
}

#[exception]
fn DefaultHandler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}

