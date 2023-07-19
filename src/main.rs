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
    use stm32f7xx_hal::ltdc::{Layer, PixelFormat};
    use stm32f7xx_hal::prelude::*;

    const WIDTH: u16 = 480;
    const HEIGHT: u16 = 272;

    // Graphics framebuffer
    const FB_GRAPHICS_SIZE: usize = (WIDTH as usize) * (HEIGHT as usize);
    static mut FB_LAYER1: [u16; FB_GRAPHICS_SIZE] = [0; FB_GRAPHICS_SIZE];

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
        display
            .controller
            .config_layer(Layer::L1, unsafe { &mut FB_LAYER1 }, PixelFormat::RGB565);

        display.enable.set_low();
        display.backlight.set_high();

        display.controller.enable_layer(Layer::L1);
        display.controller.reload();

        display.enable.set_high();

        Text::new(
            "Hello,\nRust!",
            Point::new((WIDTH / 2).into(), (HEIGHT / 2).into()),
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
