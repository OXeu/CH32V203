use alloc::fmt::format;
use core::fmt;
use core::fmt::Write;

use crate::display::SERIAL;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::writer::_print(format_args!($($arg)*)));
}
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    let mut serial = SERIAL.lock().take().unwrap();
    writeln!(serial, "{}", format(args)).unwrap();
    SERIAL.lock().replace(Some(serial));
}
