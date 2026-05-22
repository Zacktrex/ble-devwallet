use embedded_graphics::{
    mono_font::{
        MonoTextStyle,
        ascii::{FONT_8X13_BOLD, FONT_10X20},
    },
    prelude::*,
    primitives::{Circle, Line, PrimitiveStyle, Rectangle},
    text::Text,
};

use embedded_hal_bus::spi::{ExclusiveDevice, NoDelay};

use epd_waveshare::{color::Color, epd1in54_v2::Epd1in54, graphics::Display, prelude::*};

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

    draw_wallet_ui(&mut display);

    epd.update_frame(&mut spi_device, display.buffer(), &mut delay)
        .unwrap();

    epd.display_frame(&mut spi_device, &mut delay).unwrap();
}

fn draw_wallet_ui(display: &mut Display<200, 200, false, 5000, Color>) {
    display.clear(Color::White).unwrap();

    let big = MonoTextStyle::new(&FONT_10X20, Color::Black);
    let small = MonoTextStyle::new(&FONT_8X13_BOLD, Color::Black);

    // OUTER BORDER

    Rectangle::new(Point::new(0, 0), Size::new(199, 199))
        .into_styled(PrimitiveStyle::with_stroke(Color::Black, 2))
        .draw(display)
        .unwrap();

    Rectangle::new(Point::new(4, 4), Size::new(191, 191))
        .into_styled(PrimitiveStyle::with_stroke(Color::Black, 1))
        .draw(display)
        .unwrap();

    // TOP LINES

    Line::new(Point::new(25, 18), Point::new(85, 18))
        .into_styled(PrimitiveStyle::with_stroke(Color::Black, 1))
        .draw(display)
        .unwrap();

    Line::new(Point::new(115, 18), Point::new(175, 18))
        .into_styled(PrimitiveStyle::with_stroke(Color::Black, 1))
        .draw(display)
        .unwrap();

    // BLE ICON PLACEHOLDER

    Text::new("B", Point::new(95, 22), small)
        .draw(display)
        .unwrap();

    // TIME

    Text::new("14:42", Point::new(73, 50), big)
        .draw(display)
        .unwrap();

    // BLE STATUS

    Line::new(Point::new(20, 68), Point::new(55, 68))
        .into_styled(PrimitiveStyle::with_stroke(Color::Black, 1))
        .draw(display)
        .unwrap();

    Line::new(Point::new(145, 68), Point::new(180, 68))
        .into_styled(PrimitiveStyle::with_stroke(Color::Black, 1))
        .draw(display)
        .unwrap();

    Circle::new(Point::new(15, 68), 4)
        .into_styled(PrimitiveStyle::with_fill(Color::Black))
        .draw(display)
        .unwrap();

    Circle::new(Point::new(180, 68), 4)
        .into_styled(PrimitiveStyle::with_fill(Color::Black))
        .draw(display)
        .unwrap();

    Text::new("BLE CONNECTED", Point::new(50, 82), small)
        .draw(display)
        .unwrap();

    // MIDDLE SECTION

    Line::new(Point::new(0, 88), Point::new(92, 88))
        .into_styled(PrimitiveStyle::with_stroke(Color::Black, 2))
        .draw(display)
        .unwrap();

    Line::new(Point::new(108, 88), Point::new(199, 88))
        .into_styled(PrimitiveStyle::with_stroke(Color::Black, 2))
        .draw(display)
        .unwrap();

    Line::new(Point::new(92, 88), Point::new(100, 96))
        .into_styled(PrimitiveStyle::with_stroke(Color::Black, 2))
        .draw(display)
        .unwrap();

    Line::new(Point::new(108, 88), Point::new(100, 96))
        .into_styled(PrimitiveStyle::with_stroke(Color::Black, 2))
        .draw(display)
        .unwrap();

    // MONTHLY

    Text::new("THIS MONTH", Point::new(55, 118), small)
        .draw(display)
        .unwrap();

    Text::new("12,450", Point::new(30, 140), big)
        .draw(display)
        .unwrap();

    for y in (102..145).step_by(10) {
        Circle::new(Point::new(18, y), 2)
            .into_styled(PrimitiveStyle::with_fill(Color::Black))
            .draw(display)
            .unwrap();

        Circle::new(Point::new(178, y), 2)
            .into_styled(PrimitiveStyle::with_fill(Color::Black))
            .draw(display)
            .unwrap();
    }

    // BOTTOM SECTION

    Line::new(Point::new(0, 155), Point::new(92, 155))
        .into_styled(PrimitiveStyle::with_stroke(Color::Black, 2))
        .draw(display)
        .unwrap();

    Line::new(Point::new(108, 155), Point::new(199, 155))
        .into_styled(PrimitiveStyle::with_stroke(Color::Black, 2))
        .draw(display)
        .unwrap();

    Line::new(Point::new(92, 155), Point::new(100, 150))
        .into_styled(PrimitiveStyle::with_stroke(Color::Black, 2))
        .draw(display)
        .unwrap();

    Line::new(Point::new(108, 155), Point::new(100, 150))
        .into_styled(PrimitiveStyle::with_stroke(Color::Black, 2))
        .draw(display)
        .unwrap();

    // CONTACT

    Text::new("IF FOUND", Point::new(70, 170), small)
        .draw(display)
        .unwrap();

    Text::new("98765 43210", Point::new(55, 190), small)
        .draw(display)
        .unwrap();
}
