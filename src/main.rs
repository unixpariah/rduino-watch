#![no_std]
#![no_main]

use display_interface_spi::SPIInterfaceNoCS;
use arduino_hal::{spi, Spi, Delay};
use embedded_graphics::{pixelcolor::{RgbColor, Rgb565}, draw_target::DrawTarget};
use mipidsi::Builder;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let (spi, _) = Spi::new(
        dp.SPI,
        pins.d13.into_output(),
        pins.d11.into_output(),
        pins.d12.into_pull_up_input(),
        pins.d10.into_output(),
        spi::Settings::default(),
    );

    let mut delay = Delay::new();
    let di = SPIInterfaceNoCS::new(spi, pins.d9.into_output());
    let mut display = Builder::st7789(di).init(&mut delay, Some(pins.d8.into_output())).unwrap();
    display.clear(Rgb565::BLUE).unwrap();

    loop {
    }
}
