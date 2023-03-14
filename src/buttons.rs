use cortex_m::interrupt::{free as disable_interrupts, CriticalSection};
use cortex_m::peripheral::NVIC;
use heapless::consts::U256;
use heapless::spsc::Queue;
use wio_terminal as wio;

use wio::hal::clock::GenericClockController;
use wio::pac::*;
use wio::{button_interrupt, pac::EIC, ButtonController, ButtonEvent, ButtonPins};

static mut BUTTON_CTRLR: Option<ButtonController> = None;
static mut BUTTON_QUEUE: Queue<ButtonEvent, U256> = Queue(heapless::i::Queue::new());

/// Initializes the button controler.
///
/// # Errors
/// .
pub fn init(
    sets_buttons: ButtonPins,
    eic: EIC,
    clocks: &mut GenericClockController,
    mclk: &mut MCLK,
    nvic: &mut NVIC,
) {
    let button_ctrlr = sets_buttons.init(eic, clocks, mclk);
    disable_interrupts(|_| unsafe {
        button_ctrlr.enable(nvic);
        BUTTON_CTRLR = Some(button_ctrlr);
    });
}

button_interrupt!(
    BUTTON_CTRLR,
    unsafe fn on_button_event(_cs: &CriticalSection, event: ButtonEvent) {
        // if event.down {
        log::info!("Button down {event:?}");
        let mut q = BUTTON_QUEUE.split().0;
        q.enqueue(event).ok();
        // }
    }
);
