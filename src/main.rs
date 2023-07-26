#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

use defmt_rtt as _;
use panic_semihosting as _;
use rtic::app;
mod board;
mod screen;

#[app(device = stm32f7xx_hal::pac, peripherals = true, dispatchers = [SPI3])]
mod app {
    use crate::{board::Board, screen::Stm32F7DiscoDisplay};
    use core::fmt::Write;
    use defmt::println;
    use embedded_graphics::{
        image::{Image, ImageRaw, ImageRawBE},
        mono_font::{iso_8859_14::FONT_10X20, MonoTextStyle},
        pixelcolor::Rgb565,
        prelude::*,
        text::Text,
    };
    use heapless::String;
    use profont::PROFONT_24_POINT;
    use rtic_monotonics::systick::*;
    use rtic_monotonics::Monotonic;
    use stm32f7xx_hal::ltdc::Layer;

    #[shared]
    struct Shared {
        display: Stm32F7DiscoDisplay<u16>,
    }

    #[local]
    struct Local {
        counter: u32,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        println!("Start of init func");

        let p = cx.device;
        let systick_mono_token = rtic_monotonics::create_systick_token!();
        Systick::start(cx.core.SYST, 36_000_000, systick_mono_token);

        let board = Board::new(p);

        let mut display = Stm32F7DiscoDisplay::new(board.ltdc, board.dma2d, board.display_pins);
        display.config_layer1();

        display.disable();
        display.backlight_on();

        display.controller.enable_layer(Layer::L1);
        display.controller.reload();

        display.enable();

        Text::new(
            "Hello,\nRust!",
            Point::new(200, 200),
            MonoTextStyle::new(&FONT_10X20, Rgb565::RED),
        )
        .draw(&mut display)
        .ok();

        let raw: ImageRawBE<Rgb565> = ImageRaw::new(include_bytes!("../assets/bear.bin"), 100);
        let image = Image::new(&raw, Point::zero());
        image.draw(&mut display).ok();

        update_number::spawn().ok();

        (Shared { display }, Local { counter: 0 })
    }

    #[task(shared=[display], local = [counter], priority = 1)]
    async fn update_number(mut cx: update_number::Context) {
        let style: MonoTextStyle<'_, Rgb565> =
            embedded_graphics::mono_font::MonoTextStyleBuilder::new()
                .font(&PROFONT_24_POINT)
                .text_color(Rgb565::GREEN)
                .background_color(Rgb565::BLACK)
                .build();

        println!("Task runs");
        let mut fps = 0;
        let mut start = Systick::now();

        loop {
            let mut str: String<32> = String::new();

            let c = *cx.local.counter;
            *cx.local.counter = (*cx.local.counter + 1) % 10000;
            let _ret = write!(&mut str, "{}", c);

            let text = Text::new(str.as_str(), Point::new(200, 100), style);

            let rec = embedded_graphics::primitives::Rectangle::new(
                Point::new(20, 20),
                Size::new(50, 40),
            )
            .into_styled(
                embedded_graphics::primitives::PrimitiveStyleBuilder::new()
                    .stroke_color(Rgb565::RED)
                    .stroke_width(5)
                    .fill_color(Rgb565::GREEN)
                    .build(),
            );

            cx.shared.display.lock(|d| {
                text.draw(d).ok();
                rec.draw(d).ok();
            });

            fps += 1;

            let n = Systick::now();
            let d = (n - start).to_millis();

            if d > 1000 {
                println!("fps = {}", fps as f32 * 1000_f32 / d as f32);
                start = n;
                fps = 0;
            }
        }
    }
}
