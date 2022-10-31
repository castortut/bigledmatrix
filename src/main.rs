#![deny(warnings)]      //  If the Rust compiler generates a warning, stop the compilation with an error.
#![no_main]             //  Don't use the Rust standard bootstrap. We will provide our own.
#![no_std]              //  Don't use the Rust standard library. We are building a binary that can run on its own.

mod usb;
mod matrix;

use cortex_m_rt::{entry, exception, ExceptionFrame};    //  Stack frame for exception handling.
use panic_semihosting as _;
use cortex_m_semihosting::hprintln;

use stm32f1xx_hal::{pac, prelude::*};

use usb::{UsbSerial, UsbBus};
use matrix::LedMatrix;

const HELP_MESSAGE: &str = "
.h - show this help\r
.0 - select row 0\r
.1 - select row 1\r
.s - strobe active row\r
.c - clear active row\r
.i - toggle instant strobe\r
.q - set quiet (high speed) mode\r
\r
anything else will be interpreted as data to active row\r
use '..' to enter a literal '.'-byte as data\r
";

#[derive(Copy, Clone)]
enum Row {
    Row0,
    Row1,
}
use Row::*;

#[entry]
fn main() -> ! {
    hprintln!("Starting");

    // Init chip
    let _cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr
        .use_hse(8.MHz())
        .sysclk(48.MHz())
        .hclk(48.MHz())
        .pclk1(24.MHz())
        .freeze(&mut flash.acr);


    // Init GPIO
    let mut gpioa = dp.GPIOA.split();
    let clock0 = gpioa.pa0.into_push_pull_output(&mut gpioa.crl);
    let strobe0 = gpioa.pa1.into_push_pull_output(&mut gpioa.crl);
    let data0 = gpioa.pa2.into_push_pull_output(&mut gpioa.crl);

    let clock1 = gpioa.pa5.into_push_pull_output(&mut gpioa.crl);
    let strobe1 = gpioa.pa6.into_push_pull_output(&mut gpioa.crl);
    let data1 = gpioa.pa7.into_push_pull_output(&mut gpioa.crl);

    // Init USB
    let usb_bus = UsbBus::new(&clocks, dp.USB, gpioa.pa12, gpioa.pa11, &mut gpioa.crh);
    let mut usb_serial = UsbSerial::new(&usb_bus);

    // Create LED matrices
    const HEIGHT: u16 = 8;
    const WIDTH: u16 = 72;

    let mut matrix0 = LedMatrix::new(clock0, data0, strobe0, HEIGHT, WIDTH);
    matrix0.clear();
    matrix0.show();
    let mut matrix1 = LedMatrix::new(clock1, data1, strobe1, HEIGHT, WIDTH);
    matrix1.clear();
    matrix1.show();

    hprintln!("*");

    const CONTROL: u8 = '.' as u8;
    const ROW0: u8 = '0' as u8;
    const ROW1: u8 = '1' as u8;
    const SHOW: u8 = 's' as u8;
    const CLEAR: u8 = 'c' as u8;
    const HELP: u8 = 'h' as u8;
    const INSTANT: u8 = 'i' as u8;
    const QUIET: u8 = 'q' as u8;

    let mut command_mode = false;
    let mut active_row = Row0;
    let mut instant_strobe = false;

    loop {
        if usb_serial.poll() {
            let mut buf = [0u8; 64];
            if let Ok(count) = usb_serial.read(&mut buf) {
                for i in 0..count {
                    let byte = buf[i];

                    match (command_mode, byte, active_row) {
                        // Escape double-CONTROL
                        (true, CONTROL, Row0) => {
                            matrix0.push_row(byte);
                            if instant_strobe {
                                matrix0.show();
                            }
                            command_mode = false;
                        },
                        (true, CONTROL, Row1) => {
                            matrix1.push_row(byte);
                            command_mode = false;
                        },
                        (true, HELP, _) => {
                            usb_serial.write_str(&HELP_MESSAGE);
                            command_mode = false;
                        },
                        (true, ROW0, _) => {
                            active_row = Row0;
                            usb_serial.write_str("Switching to row 0\r\n");
                            command_mode = false;
                        },
                        (true, ROW1, _) => {
                            active_row = Row1;
                            usb_serial.write_str("Switching to row 1\r\n");
                            command_mode = false;
                        },
                        (true, SHOW, _) => {
                            matrix0.show();
                            usb_serial.write_str("Showing\r\n");
                            command_mode = false;
                        },
                        (true, CLEAR, _) => {
                            matrix0.clear();
                            usb_serial.write_str("Clearing\r\n");
                            command_mode = false;
                        },
                        (true, INSTANT, _) => {
                            if instant_strobe {
                                usb_serial.write_str("Disabling instant strobe\r\n");
                                instant_strobe = false;
                            } else {
                                usb_serial.write_str("Enabling instant strobe\r\n");
                                instant_strobe = true;
                            }
                            command_mode = false;
                        },
                        (true, QUIET, _) => {
                            usb_serial.set_quiet(true);
                            command_mode = false;
                        },
                        (true, _, _) => {
                            usb_serial.write_str("Invalid command character\r\n");
                            command_mode = false;
                        }
                        (false, CONTROL, _) => command_mode = true,
                        (false, _, Row0) => {
                            matrix0.push_row(byte);
                            if instant_strobe {
                                matrix0.show();
                            }
                        },
                        (false, _, Row1) => matrix1.push_row(byte),
                    };
                }
            }
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

