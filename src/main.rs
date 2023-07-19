#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

use defmt_rtt as _;
use panic_semihosting as _;
use rtic::app;
mod board;
mod screen;

#[app(device = stm32f7xx_hal::pac, peripherals = true)]
mod app {
    use crate::{board::Board, screen::Stm32F7DiscoDisplay};
    use defmt::println;
    use embedded_graphics::{
        image::{Image, ImageRaw, ImageRawBE},
        mono_font::{iso_8859_14::FONT_10X20, MonoTextStyle},
        pixelcolor::Rgb565,
        prelude::*,
        text::Text,
    };
    use stm32f7xx_hal::ltdc::Layer;

    #[shared]
    struct Shared {
        display: Stm32F7DiscoDisplay<u16>,
    }

    #[local]
    struct Local {}

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        println!("Start of init func");

        let p = cx.device;

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

        (Shared { display }, Local {})
    }
}
