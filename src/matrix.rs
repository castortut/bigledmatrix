use core::fmt::Debug;
use embedded_hal::digital::v2::OutputPin;

pub struct LedMatrix<ClockPin, DataPin, StrobePin> {
    clock: ClockPin,
    data: DataPin,
    strobe: StrobePin,
    // Make size an u16 to prevent overflow on multiplication and avoid
    // lots of casting
    height: u16,
    width: u16,
}

#[allow(dead_code)]
impl<ClockPin, DataPin, StrobePin> LedMatrix<ClockPin, DataPin, StrobePin> where
    ClockPin: OutputPin, ClockPin::Error: Debug,
    DataPin: OutputPin, DataPin::Error: Debug,
    StrobePin: OutputPin, StrobePin::Error: Debug,
{
    pub fn new(
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

    pub fn pulse_clock(&mut self) {
        self.clock.set_high().unwrap();
        self.clock.set_low().unwrap();
    }

    pub fn show(&mut self) {
        self.strobe.set_high().unwrap();
        self.strobe.set_low().unwrap();
    }

    pub fn clear(&mut self) {
        self.data.set_low().unwrap();
        for _ in 0 .. self.width * self.height {
            self.pulse_clock();
        }
    }

    pub fn pixel_on(&mut self) {
        self.data.set_high().unwrap();
        self.pulse_clock();
    }

    pub fn pixel_off(&mut self) {
        self.data.set_low().unwrap();
        self.pulse_clock();
    }

    pub fn push_row(&mut self, row: u8) {
        for i in (0..=7).rev() {
            if (row & (1 << i)) != 0 {
                self.pixel_on();
            } else {
                self.pixel_off();
            }
            self.pulse_clock();
        }
    }
}

