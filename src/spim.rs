use display_interface::{DataFormat, DisplayError, WriteOnlyDataCommand};
use embedded_hal::digital::v2::OutputPin;

pub struct Spim<A, B, C, D>
    where A: OutputPin, B: OutputPin, C: OutputPin, D: OutputPin {
    dc: A,
    cs: B,
    clk: C,
    mosi: D,
}

impl<A, B, C, D> Spim<A, B, C, D>
    where A: OutputPin, B: OutputPin, C: OutputPin, D: OutputPin {
    pub fn new(dc: A, cs: B, clk: C, mosi: D) -> Spim<A, B, C, D>
    {
        Spim {
            dc,
            cs,
            clk,
            mosi,
        }
    }

    fn send_u8(&mut self, mut c: u8) {
        for _ in 0..8 {
            // 0b10000000
            if c & 0x80 == 0u8 { // 0x80 or 0x0
                let _ = self.mosi.set_low();
            } else {
                let _ = self.mosi.set_high();
            }
            let _ = self.clk.set_high();
            c <<= 1;
            let _ = self.clk.set_low();
        }
    }
    fn send_u16(&mut self, mut c: u16) {
        for _ in 0..16 {
            // 0b 1000 0000 0000 0000
            if c & 0x8000 == 0u16 {
                let _ = self.mosi.set_low();
            } else {
                let _ = self.mosi.set_high();
            }
            let _ = self.clk.set_high();
            c <<= 1;
            let _ = self.clk.set_low();
        }
    }
}

impl<A, B, C, D> WriteOnlyDataCommand for Spim<A, B, C, D> where A: OutputPin, B: OutputPin, C: OutputPin, D: OutputPin {
    fn send_commands(&mut self, cmd: DataFormat<'_>) -> Result<(), DisplayError> {
        if let DataFormat::U8Iter(u) = cmd {
            let _ = self.cs.set_low();
            let _ = self.dc.set_low();
            u.for_each(|c| {
                self.send_u8(c)
            });
            let _ = self.cs.set_high();
        }
        Ok(())
    }

    fn send_data(&mut self, buf: DataFormat<'_>) -> Result<(), DisplayError> {
        match buf {
            DataFormat::U8Iter(u) => {
                let _ = self.cs.set_low();
                let _ = self.dc.set_high();
                u.for_each(|c| {
                    self.send_u8(c)
                });
                let _ = self.cs.set_high();
            }
            DataFormat::U16BEIter(u) => {
                let _ = self.cs.set_low();
                let _ = self.dc.set_high();
                u.for_each(|c| {
                    self.send_u16(c)
                });
                let _ = self.cs.set_high();
            }
            _ => {}
        }
        Ok(())
    }
}