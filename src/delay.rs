use embedded_hal::blocking::delay::{DelayMs, DelayUs};

pub struct DelaySource{

}

impl DelayUs<usize> for DelaySource
{
    fn delay_us(&mut self, us: usize) {
        let usec = us * 144 ;
        unsafe { riscv::asm::delay(usec as u32) }
    }
}
impl DelayUs<u32> for DelaySource
{
    fn delay_us(&mut self, us: u32) {
        let usec = us * 144 ;
        unsafe { riscv::asm::delay(usec as u32) }
    }
}
impl DelayMs<u16> for DelaySource
{
    fn delay_ms(&mut self, ms: u16) {
        let msec:u32 = ms as u32 * 144_000;
        unsafe { riscv::asm::delay(msec) }
    }
}