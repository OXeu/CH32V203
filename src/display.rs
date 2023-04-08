use core::cell::{RefCell};
use ch32v20x_hal as hal;
use ch32v20x_hal::gpio::{Alternate, Output, Pin};

use hal::serial::UartTx;
use spin::{Lazy, Mutex};
use st7789::ST7789;
use crate::spim::Spim;


pub static DISPLAY: Lazy<Mutex<RefCell<Option<Display>>>> = Lazy::new(|| Mutex::new(RefCell::new(None)));
pub static SERIAL: Lazy<Mutex<RefCell<Option<Serial>>>> = Lazy::new(|| Mutex::new(RefCell::new(None)));

pub type Display = ST7789<Spim<Pin<'B', 10, Output>, Pin<'B', 12, Output>, Pin<'B', 13, Output>, Pin<'B', 15, Output>>, Pin<'B', 11, Output>, Pin<'B', 0, Output>>;
pub type Serial = UartTx<ch32v20x_hal::pac::USART1, Pin<'B', 6, Alternate>>;
