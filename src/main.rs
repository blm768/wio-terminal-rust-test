#![no_main]
#![no_std]

use panic_halt as _;

use core::convert::Infallible;

use atsamd_hal as hal;
use hal::clock::GenericClockController;
use hal::common::pad::PadPin;
use hal::common::time::Hertz;
use hal::delay::Delay;
use hal::sercom::{SPIMaster7, UART2};
use hal::target_device as pac;
use hal::time::MegaHertz;
use pac::{CorePeripherals, Peripherals};

use cortex_m_rt::entry;

use embedded_graphics::fonts::Font6x8;
use embedded_graphics::pixelcolor::{Rgb565, RgbColor};
use embedded_graphics::style::TextStyle;
use embedded_graphics::DrawTarget;

use embedded_hal::digital::v2::OutputPin;
use embedded_hal::serial::{Read, Write};
use embedded_hal::spi::MODE_0;

use ili9341::{Ili9341, Orientation};

use nb::block;

use wio_terminal::Pins;

use wio_terminal_rust_test::terminal::TerminalEmulator;

fn run() -> Result<(), Infallible> {
    let mut core = CorePeripherals::take().unwrap();
    let mut peripherals = Peripherals::take().unwrap();
    let mut pins = Pins::new(peripherals.PORT);

    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );
    let gclk0 = clocks.gclk0();

    // TODO: it would be nice if SercomXPadY::new only accepted pins that are configured correctly for SERCOM use.
    let spi: SPIMaster7<
        hal::sercom::Sercom7Pad2<_>,
        hal::sercom::Sercom7Pad3<_>,
        hal::sercom::Sercom7Pad1<_>,
    > = SPIMaster7::new(
        &clocks.sercom7_core(&gclk0).unwrap(),
        MegaHertz(48),
        MODE_0,
        peripherals.SERCOM7,
        &mut peripherals.MCLK,
        (
            pins.lcd_miso.into_pad(&mut pins.port),
            pins.lcd_mosi.into_pad(&mut pins.port),
            pins.lcd_sck.into_pad(&mut pins.port),
        ),
    );

    let reset = pins.lcd_reset.into_push_pull_output(&mut pins.port);
    let cs = pins.lcd_cs.into_push_pull_output(&mut pins.port);
    let dc = pins.lcd_dc.into_push_pull_output(&mut pins.port);
    let mut delay = Delay::new(core.SYST, &mut clocks);
    let mut display = Ili9341::new_spi(spi, cs, dc, reset, &mut delay).unwrap();
    core.SYST = delay.free();

    display
        .set_orientation(Orientation::LandscapeFlipped)
        .unwrap();
    let mut backlight = pins.lcd_backlight.into_push_pull_output(&mut pins.port);
    backlight.set_high().unwrap();
    display.clear(Rgb565::WHITE).unwrap();

    let text_style = TextStyle::new(Font6x8, Rgb565::BLACK);
    let mut term = TerminalEmulator::new(display, text_style);
    term.put_str("Hello, world!\r\n").unwrap();

    let mut uart: UART2<hal::sercom::Sercom2Pad1<_>, hal::sercom::Sercom2Pad0<_>, (), ()> =
        UART2::new(
            &clocks.sercom2_core(&gclk0).unwrap(),
            Hertz(115200),
            peripherals.SERCOM2,
            &mut peripherals.MCLK,
            (
                pins.rxd.into_pad(&mut pins.port),
                pins.txd.into_pad(&mut pins.port),
            ),
        );
    let (mut tx, mut rx) = uart.split();

    let mut led = pins.user_led.into_push_pull_output(&mut pins.port);
    led.set_low().unwrap();
    loop {
        block!(tx.write(b'a')).unwrap();
        if let Ok(b) = rx.read() {
            if b == b'b' {
                led.set_high().unwrap();
            }
            term.put_char(b.into()).unwrap();
        }
    }
}

#[entry]
unsafe fn main() -> ! {
    run().unwrap_or_else(|_: Infallible| unreachable!());
    loop {}
}
