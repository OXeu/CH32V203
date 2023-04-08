#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
extern crate alloc;

use alloc::string::ToString;
use core::fmt::Write;
use core::panic::PanicInfo;

use ch32v20x_hal as hal;
use ch32v20x_hal::gpio::{Output, Pin};
use ch32v20x_hal::serial::{TxPin, UartTx};
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::Drawable;
use embedded_graphics::geometry::Point;
use embedded_graphics::image::{Image, ImageRawLE};
use embedded_graphics::mono_font::ascii::FONT_6X10;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::{Rgb565, RgbColor};
use embedded_graphics::text::Text;
use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayUs;
use good_memory_allocator::SpinLockedAllocator;
use hal::gpio::GpioExt;
use hal::pac;
use hal::prelude::*;
use riscv as _;
use riscv_rt::entry;
use st7789::{Orientation, ST7789};

use crate::allocator::init_heap;
use crate::delay::DelaySource;
use crate::display::{DISPLAY, SERIAL, Serial};
use crate::spim::Spim;

mod spim;
mod delay;
mod allocator;
mod writer;
mod traits;
mod display;


#[global_allocator]
static ALLOCATOR: SpinLockedAllocator = SpinLockedAllocator::empty();

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut serial = SERIAL.lock().take().unwrap();
    write!(serial, "{}\n\r", info).unwrap();
    SERIAL.lock().replace(Some(serial));
    loop {}
}

// use crate::display::ser;

#[entry]
fn main() -> ! {
    init_heap();

    let peripherals = pac::Peripherals::take().unwrap();
    let rcc = peripherals.RCC.constrain();
    let clocks = rcc.config.sysclk_144m_via_hsi().freeze();

    let gpiob = peripherals.GPIOB.split();
    let tx_pin = gpiob.pb6.into_alternate();
    let config = hal::serial::Config::default();
    let mut serial: Serial = UartTx::new(peripherals.USART1, tx_pin, &clocks, config);
    let str = "Hello, World!\n\r".to_string();
    write!(&mut serial, "{}", str).unwrap();


    SERIAL.lock().replace(Some(serial));
    let serial_option = SERIAL.lock().take();
    if let Some(mut ser) = serial_option {
        write!(ser, "reachable\n\r").unwrap();
        SERIAL.lock().replace(Some(ser));
    }

    let gpioa = peripherals.GPIOA.split();
    let mut led = gpioa.pa15.into_push_pull_output();
    let dc = gpiob.pb10.into_push_pull_output(); // Data/Clock
    let cs = gpiob.pb12.into_push_pull_output(); // CS
    let clk = gpiob.pb13.into_push_pull_output(); // Clock
    let mosi = gpiob.pb15.into_push_pull_output(); // Mosi
    let reset = gpiob.pb11.into_push_pull_output(); // Reset
    let di = Spim::new(dc, cs, clk, mosi);
    let mut display = ST7789::new(di, Some(reset), None::<Pin<'B', 0, Output>>, 240, 240);
    let mut delay = DelaySource {};
    display.init(&mut delay).unwrap();
    display.set_orientation(Orientation::Landscape).unwrap();
    display.clear(Rgb565::BLACK).unwrap();
    let txt = "Hello,Rust embedded!";
    let style = MonoTextStyle::new(&FONT_6X10, Rgb565::WHITE);
    let _ = Text::new(txt, Point::new(20, 30), style).draw(&mut display);
    DISPLAY.lock().replace(Some(display));

    let mut _display = DISPLAY.lock().take().unwrap();
    let raw_image_data = ImageRawLE::new(include_bytes!("../assets/ferris.raw"), 86);
    let ferris = Image::new(&raw_image_data, Point::new(77, 77));
    let _ = ferris.draw(&mut _display);
    DISPLAY.lock().replace(Some(_display));

    let mut delay = DelaySource {};
    loop {
        delay.delay_us(1_000_000_usize);
        // println!("Hello,World");
        // panic!("Hell Panic");
        led.toggle()
    }
}