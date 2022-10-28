#![deny(warnings)]      //  If the Rust compiler generates a warning, stop the compilation with an error.

use cortex_m::asm::delay;

use stm32f1xx_hal::pac::USB;
use usb_device::{prelude::*, class_prelude::UsbBusAllocator};
use usbd_serial::{SerialPort, USB_CLASS_CDC};

use stm32f1xx_hal::{usb, rcc::Clocks};

pub struct UsbBus {
    pub bus: UsbBusAllocator<usb::UsbBus<usb::Peripheral>>,
}

impl UsbBus {
    pub fn new(
        clocks: &Clocks,
        usb: USB,
        usb_dp: stm32f1xx_hal::gpio::Pin<stm32f1xx_hal::gpio::Input<stm32f1xx_hal::gpio::Floating>, stm32f1xx_hal::gpio::CRH, 'A', 12_u8>,
        usb_dm: stm32f1xx_hal::gpio::Pin<stm32f1xx_hal::gpio::Input<stm32f1xx_hal::gpio::Floating>, stm32f1xx_hal::gpio::CRH, 'A', 11_u8>,
        crh: &mut stm32f1xx_hal::gpio::Cr<stm32f1xx_hal::gpio::CRH, 'A'>,
    ) -> UsbBus {
        assert!(clocks.usbclk_valid());

        // BluePill board has a pull-up resistor on the D+ line.
        // Pull the D+ pin down to send a RESET condition to the USB bus.
        // This forced reset is needed only for development, without it host
        // will not reset your device when you upload new firmware.
        let mut usb_dp = usb_dp.into_push_pull_output(crh);
        usb_dp.set_low();
        delay(clocks.sysclk().raw() / 100);

        let usb_dm = usb_dm.into_floating_input(crh);
        let usb_dp = usb_dp.into_floating_input(crh);

        let usb = usb::Peripheral {
            usb,
            pin_dm: usb_dm,
            pin_dp: usb_dp,
        };

        UsbBus {
            bus: usb::UsbBus::new(usb)
        }
    }
}

pub struct UsbSerial<'a> {
    dev: UsbDevice<'a, usb::UsbBus<usb::Peripheral>>,
    serial: SerialPort<'a, usb::UsbBus<usb::Peripheral>>,
}

impl<'a> UsbSerial<'a> {
    pub fn new(
        mybus: &'a UsbBus
    ) -> UsbSerial {
        let usb_serial = SerialPort::new(&mybus.bus);

        // https://github.com/obdev/v-usb/blob/master/usbdrv/USB-IDs-for-free.txt
        let usb_dev = UsbDeviceBuilder::new(&mybus.bus, UsbVidPid(0x16c0, 0x05e1))
            .manufacturer("Castor https://avaruuskerho.fi")
            .product("LED Matrix 72x16")
            .serial_number("1")
            .device_class(USB_CLASS_CDC)
            .build();

        UsbSerial {
            dev: usb_dev,
            serial: usb_serial,
        }
    }

    pub fn poll(&mut self) -> bool {
        self.dev.poll(&mut [&mut self.serial])
    }

    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, UsbError>{
        self.serial.read(buf)

    }

    pub fn write(&mut self, buf: &[u8]) -> Result<usize, UsbError> {
        self.serial.write(buf)
    }

    pub fn write_str(&mut self, message: &str) -> Result<usize, UsbError> {
        let bytes = message.as_bytes();
        let mut bytes_remaining = message.len();
        let mut bytes_written = 0;
        while bytes_remaining > 0 {
            let range_start = bytes_written;
            let range_end = if bytes_remaining > 32 {
                bytes_written + 32
            } else {
                bytes_written + bytes_remaining
            };
            let result = self.write(&bytes[range_start..range_end]);
            if let Ok(written) = result {
                bytes_written += written;
                bytes_remaining -= written;
            } else {
                return result;
            }
            delay(500);
        }

        Ok(bytes_written)

    }
}

