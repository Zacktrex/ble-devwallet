use embedded_graphics::{
    mono_font::{
        MonoTextStyle,
        ascii::{
            // FONT_6X10, FONT_9X18_BOLD,
             FONT_10X20},

    },
    prelude::*,
    text::Text,
};

use epd_waveshare::{color::Color, epd1in54_v2::Epd1in54, graphics::Display, prelude::*};

use embedded_hal_bus::spi::{ExclusiveDevice, NoDelay};

use esp_hal::{
    delay::Delay,
    gpio::{Input, Output},
    spi::master::Spi,
};

pub fn init_display(
    spi: Spi<'_, esp_hal::Blocking>,
    cs: Output<'_>,
    dc: Output<'_>,
    rst: Output<'_>,
    busy: Input<'_>,
) {
    let mut delay = Delay::new();

    let mut spi_device = ExclusiveDevice::new(spi, cs, NoDelay).unwrap();

    let mut epd = Epd1in54::new(&mut spi_device, busy, dc, rst, &mut delay, None).unwrap();

    let mut display = Display::<200, 200, false, 5000, Color>::default();

    display.clear(Color::White).unwrap();

    let style = MonoTextStyle::new(&FONT_10X20, Color::Black);

    Text::new("ESP32-C3", Point::new(10, 20), style)
        .draw(&mut display)
        .unwrap();

    Text::new("BLE Device", Point::new(10, 40), style)
        .draw(&mut display)
        .unwrap();

    Text::new("Wallet", Point::new(10, 60), style)
        .draw(&mut display)
        .unwrap();

    epd.update_frame(&mut spi_device, display.buffer(), &mut delay)
        .unwrap();

    epd.display_frame(&mut spi_device, &mut delay).unwrap();
}
