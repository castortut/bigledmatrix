#![deny(warnings)]      //  If the Rust compiler generates a warning, stop the compilation with an error.

use core::{marker::PhantomData, cell::Cell};

use cortex_m::asm::delay;

use stm32f1xx_hal::pac::USB;
use usb_device::{prelude::*, class_prelude::UsbBusAllocator};
use usbd_serial::{SerialPort, USB_CLASS_CDC};

use stm32f1xx_hal::{usb::{Peripheral, UsbBus}, rcc::Clocks};

// Create a custom wrapper that in an unsafe way
// wraps unsync contents as sync
struct ForceSync<T>(T);
unsafe impl<T> Sync for ForceSync<T> {}

// We want this to be 'static, but that requires Sync. Force sync but guarantee
// that this will only be called from one thread.
static mut BUS: Option<ForceSync<UsbBusAllocator<UsbBus<Peripheral>>>> = None;

pub struct UsbSerial {
    dev: UsbDevice<'static, UsbBus<Peripheral>>,
    serial: SerialPort<'static, UsbBus<Peripheral>>,

    // This will force the struct to be !Sync, while also providing
    // the only interface to use the "Sync but not really Sync" BUS
    _make_unsync: PhantomData<Cell<()>>,
}

impl UsbSerial {
    pub unsafe fn new(
        clocks: &Clocks,
        usb: USB,
        usb_dp: stm32f1xx_hal::gpio::Pin<stm32f1xx_hal::gpio::Input<stm32f1xx_hal::gpio::Floating>, stm32f1xx_hal::gpio::CRH, 'A', 12_u8>,
        usb_dm: stm32f1xx_hal::gpio::Pin<stm32f1xx_hal::gpio::Input<stm32f1xx_hal::gpio::Floating>, stm32f1xx_hal::gpio::CRH, 'A', 11_u8>,
        crh: &mut stm32f1xx_hal::gpio::Cr<stm32f1xx_hal::gpio::CRH, 'A'>,
    ) -> UsbSerial {
        assert!(BUS.is_none());
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

        let usb = Peripheral {
            usb,
            pin_dm: usb_dm,
            pin_dp: usb_dp,
        };

        BUS = Some(ForceSync(UsbBus::new(usb)));
        let usb_serial = SerialPort::new(&BUS.as_ref().unwrap().0);

        // https://github.com/obdev/v-usb/blob/master/usbdrv/USB-IDs-for-free.txt
        let usb_dev = UsbDeviceBuilder::new(&BUS.as_ref().unwrap().0, UsbVidPid(0x16c0, 0x05e1))
            .manufacturer("Castor https://avaruuskerho.fi")
            .product("LED Matrix 72x16")
            .serial_number("1")
            .device_class(USB_CLASS_CDC)
            .build();

        UsbSerial {
            dev: usb_dev,
            serial: usb_serial,
            _make_unsync: PhantomData::default(),
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
}

