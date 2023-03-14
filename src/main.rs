#![no_std]
#![no_main]

// use embedded_graphics as eg;
// use panic_halt as _;
use panic_persist as _;
use wio_terminal as wio;

use wioterminal::{buttons, cli, ui, usb_serial, usb_serial_log, wifi};

use wio::hal::clock::GenericClockController;
use wio::hal::delay::Delay;

use wio::pac::{CorePeripherals, Peripherals};

use wio::entry;
use wio::prelude::*;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut core = CorePeripherals::take().unwrap();

    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut delay = Delay::new(core.SYST, &mut clocks);

    let sets = wio::Pins::new(peripherals.PORT).split();

    let mut user_led = sets.user_led.into_push_pull_output();
    user_led.set_low().unwrap();
    let nvic = &mut core.NVIC;

    // Initialize USB.
    let usb_allocator = sets
        .usb
        .usb_allocator(peripherals.USB, &mut clocks, &mut peripherals.MCLK);
    usb_serial::init(nvic, usb_allocator);

    // Initialize logger on usb serial port;
    usb_serial_log::init().unwrap();

    // Wait for us to have the terminal open. If actived, absolutely need to write bytes to serial ...
    // while !usb_serial::user_present() {
    //     // delay.delay_ms(20u8);

    //     cortex_m::asm::wfi();
    // }

    // initialize the CLI
    log::info!("Initialize USBSerial/CLI");
    let (cli_producer, mut cli_bytes_to_write) = unsafe { cli::CLI_OUTPUT_QUEUE.split() };
    let mut cli = cli::Cli::new(cli_producer);

    let mut serial_read_queue = usb_serial::get_serial_input_pipe();

    // Check if there was a panic message, if so, send to UART
    if let Some(msg) = panic_persist::get_panic_message_bytes() {
        log::error!("panic from previous boot:");
        let mut bytes_written = 0;
        // Write it out in chunks, waiting for USB interrupt handler to run in between before trying to shove more bytes
        while bytes_written != msg.len() {
            let chunk_written = usb_serial::get(|usbserial| usbserial.write(&msg[bytes_written..]));
            bytes_written += chunk_written;
            cortex_m::asm::wfi();
        }
    }

    // Initialize button controler
    log::info!("Initialize button controler");
    buttons::init(
        sets.buttons,
        peripherals.EIC,
        &mut clocks,
        &mut peripherals.MCLK,
        nvic,
    );

    // Set up the display so we can print out APs.
    log::info!("Initialize the screen");
    // TODO: with via screen manager that manage all screens (wifi, snake game for ex, ...)

    let display = ui::init(
        sets.display,
        peripherals.SERCOM7,
        &mut clocks,
        &mut peripherals.MCLK,
        &mut delay,
    );

    // Initialize the wifi peripheral.
    log::info!("Initialize WiFi");
    wifi::init(
        sets.wifi,
        peripherals.SERCOM0,
        &mut clocks,
        &mut peripherals.MCLK,
        &mut delay,
        nvic,
    );

    log::info!("Entering loop");
    loop {
        // handle our CLI
        while let Some(new_byte) = cli_bytes_to_write.dequeue() {
            usb_serial::get(|usbserial| usbserial.write(&[new_byte]));
        }
        while let Some(new_byte) = serial_read_queue.dequeue() {
            cli.input_from_serial(new_byte);
        }

        user_led.toggle().ok();
        delay.delay_ms(200u8);
    }
}

// use core::panic::PanicInfo;
// #[inline(never)]
// #[panic_handler]
// fn panic(info: &PanicInfo) -> ! {
//     disable_interrupts(|cs| {
//         println!("panic: {}", info);
//     });

//     loop {}
// }

// // -------------------------------------------------
// //              SCREEN/TERMINAL
// // -------------------------------------------------

// fn write<'a, T: Into<&'a str>>(display: &mut wio::LCD, text: T, pos: Point) {
//     Text::with_baseline(
//         text.into(),
//         pos,
//         MonoTextStyle::new(&FONT_6X12, Rgb565::WHITE),
//         Baseline::Top,
//     )
//     .draw(display)
//     .ok()
//     .unwrap();
// }

// fn write_with_clear<'a, T: Into<&'a str>>(
//     display: &mut wio::LCD,
//     text: T,
//     num_clear: i32,
//     pos: Point,
// ) {
//     let style = PrimitiveStyleBuilder::new()
//         .fill_color(Rgb565::BLACK)
//         .build();
//     Rectangle::with_corners(pos, Point::new(pos.x + (6 * num_clear), pos.y + 12))
//         .into_styled(style)
//         .draw(display)
//         .ok()
//         .unwrap();

//     Text::with_baseline(
//         text.into(),
//         pos,
//         MonoTextStyle::new(&FONT_6X12, Rgb565::WHITE),
//         Baseline::Top,
//     )
//     .draw(display)
//     .ok()
//     .unwrap();
// }
