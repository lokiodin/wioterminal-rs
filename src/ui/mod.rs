mod display;
pub mod screen_manager;

pub use screen_manager::*;

use wio::hal::clock::GenericClockController;
use wio::hal::delay::Delay;
use wio::pac::MCLK;
use wio::pac::SERCOM7;
// use wio::prelude::*;
use wio_terminal as wio;

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
