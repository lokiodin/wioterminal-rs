use embedded_graphics as eg;

use eg::pixelcolor::Rgb565;
use eg::prelude::Point;
use eg::prelude::*;
use eg::primitives::{PrimitiveStyleBuilder, Rectangle};

use wio::hal::clock::GenericClockController;
use wio::hal::delay::Delay;
use wio::pac::MCLK;
use wio::prelude::*;
use wio::{pac::SERCOM7, Display};
use wio_terminal as wio;

struct Screen {
    display: wio::LCD,
}

pub fn init(
    sets_display: Display,
    sercom7: SERCOM7,
    clocks: &mut GenericClockController,
    mclk: &mut MCLK,
    delay: &mut Delay,
) {
    let (mut display, _backlight) = sets_display
        .init(clocks, sercom7, mclk, 58.mhz(), delay)
        .unwrap();
    clear(&mut display);
}

fn clear(display: &mut wio::LCD) {
    let style = PrimitiveStyleBuilder::new()
        .fill_color(Rgb565::BLACK)
        .build();
    let backdrop =
        Rectangle::with_corners(Point::new(0, 0), Point::new(320, 320)).into_styled(style);
    backdrop.draw(display).ok().unwrap();
}
