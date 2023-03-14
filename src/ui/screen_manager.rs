use eg::pixelcolor::Rgb565;
use eg::pixelcolor::RgbColor;
use eg::prelude::Point;
use eg::primitives::Primitive;
use eg::primitives::Rectangle;
use eg::Drawable;
use embedded_graphics as eg;
use embedded_graphics::primitives::PrimitiveStyleBuilder;
use wio::hal::clock::GenericClockController;
use wio::hal::delay::Delay;
use wio::pac::MCLK;
use wio::pac::SERCOM7;
use wio::prelude::*;
use wio::LCD;
use wio_terminal as wio;

/// Display
///
/// Implement the screen/display, i.e where we write/paint on the screen
///
/// Maybe do: struct Display<V: View>{} where view is the different view possible (wifi, games, ...)
pub struct Display {
    display: LCD,
}

impl Display {
    pub fn new(
        sets_display: wio::Display,
        sercom7: SERCOM7,
        clocks: &mut GenericClockController,
        mclk: &mut MCLK,
        delay: &mut Delay,
    ) -> Self {
        let (mut display, _backlight) = sets_display
            .init(clocks, sercom7, mclk, 58.mhz(), delay)
            .unwrap();

        Self { display }
    }

    pub fn clear(&mut self) {
        let style = PrimitiveStyleBuilder::new()
            .fill_color(Rgb565::BLACK)
            .build();
        let backdrop =
            Rectangle::with_corners(Point::new(0, 0), Point::new(320, 320)).into_styled(style);
        backdrop.draw(&mut self.display).ok().unwrap();
    }
}

/// View
///
/// Different type of view (wifi, game, ...)
#[derive(Default)]
enum View {
    #[default]
    Log,
    Wifi,
}

/// ScreenManager that control what to do depending of the `View`
pub struct ScreenManager {
    display: Display,
    // Current view
    curr_view: View,
}

impl ScreenManager {
    pub fn new(display: Display) -> Self {
        Self {
            display,
            curr_view: View::default(),
        }
    }

    pub fn add_screen(&mut self) -> Self {
        todo!()
    }
}

pub fn init(
    sets_display: wio::Display,
    sercom7: SERCOM7,
    clocks: &mut GenericClockController,
    mclk: &mut MCLK,
    delay: &mut Delay,
) -> ScreenManager {
    // init  the display as in the exemple
    let mut display = Display::new(sets_display, sercom7, clocks, mclk, delay);
    display.clear();
    ScreenManager::new(display)
}
