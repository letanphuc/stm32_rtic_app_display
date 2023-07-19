use embedded_graphics::{
    geometry::Size,
    pixelcolor::{Rgb565, RgbColor},
    prelude::{DrawTarget, OriginDimensions},
    Pixel,
};

use stm32f7xx_hal::gpio::{Output, Pin};
use stm32f7xx_hal::{
    gpio::Speed,
    ltdc::{DisplayConfig, DisplayController, Layer, PixelFormat, SupportedWord},
    pac::{DMA2D, LTDC},
    prelude::*,
    rcc::{HSEClock, HSEClockMode},
};

use crate::board::DisplayPins;

/// STM32F7-DISCO board display
pub const DISCO_SCREEN_CONFIG: DisplayConfig = DisplayConfig {
    active_width: 480,
    active_height: 272,
    h_back_porch: 13,
    h_front_porch: 30,
    h_sync: 41,
    v_back_porch: 2,
    v_front_porch: 2,
    v_sync: 10,
    frame_rate: 60,
    h_sync_pol: false,
    v_sync_pol: false,
    no_data_enable_pol: false,
    pixel_clock_pol: false,
};

const FB_GRAPHICS_SIZE: usize =
    (DISCO_SCREEN_CONFIG.active_width as usize) * (DISCO_SCREEN_CONFIG.active_height as usize);
static mut FB_LAYER1: [u16; FB_GRAPHICS_SIZE] = [0; FB_GRAPHICS_SIZE];

pub struct Stm32F7DiscoDisplay<T: 'static + SupportedWord> {
    pub controller: DisplayController<T>,
    enable_pin: Pin<'I', 12, Output>,
    backlight_pin: Pin<'K', 3, Output>,
}

impl<T: 'static + SupportedWord> Stm32F7DiscoDisplay<T> {
    pub fn new(ltdc: LTDC, dma2d: DMA2D, pins: DisplayPins) -> Stm32F7DiscoDisplay<T> {
        let controller = DisplayController::new(
            ltdc,
            dma2d,
            PixelFormat::RGB565,
            DISCO_SCREEN_CONFIG,
            Some(&HSEClock::new(25_000_000.Hz(), HSEClockMode::Bypass)),
        );

        pins.pe4.into_alternate::<14>().set_speed(Speed::VeryHigh);
        pins.pg12.into_alternate::<14>().set_speed(Speed::VeryHigh);
        pins.pi9.into_alternate::<14>().set_speed(Speed::VeryHigh);
        pins.pi10.into_alternate::<14>().set_speed(Speed::VeryHigh);
        pins.pi13.into_alternate::<14>().set_speed(Speed::VeryHigh);
        pins.pi14.into_alternate::<14>().set_speed(Speed::VeryHigh);
        pins.pi15.into_alternate::<14>().set_speed(Speed::VeryHigh);
        pins.pj0.into_alternate::<14>().set_speed(Speed::VeryHigh);
        pins.pj1.into_alternate::<14>().set_speed(Speed::VeryHigh);
        pins.pj2.into_alternate::<14>().set_speed(Speed::VeryHigh);
        pins.pj3.into_alternate::<14>().set_speed(Speed::VeryHigh);
        pins.pj4.into_alternate::<14>().set_speed(Speed::VeryHigh);
        pins.pj5.into_alternate::<14>().set_speed(Speed::VeryHigh);
        pins.pj6.into_alternate::<14>().set_speed(Speed::VeryHigh);
        pins.pj7.into_alternate::<14>().set_speed(Speed::VeryHigh);
        pins.pj8.into_alternate::<14>().set_speed(Speed::VeryHigh);
        pins.pj9.into_alternate::<14>().set_speed(Speed::VeryHigh);
        pins.pj10.into_alternate::<14>().set_speed(Speed::VeryHigh);
        pins.pj11.into_alternate::<14>().set_speed(Speed::VeryHigh);
        pins.pj13.into_alternate::<14>().set_speed(Speed::VeryHigh);
        pins.pj14.into_alternate::<14>().set_speed(Speed::VeryHigh);
        pins.pj15.into_alternate::<14>().set_speed(Speed::VeryHigh);
        pins.pk0.into_alternate::<14>().set_speed(Speed::VeryHigh);
        pins.pk1.into_alternate::<14>().set_speed(Speed::VeryHigh);
        pins.pk2.into_alternate::<14>().set_speed(Speed::VeryHigh);
        pins.pk4.into_alternate::<14>().set_speed(Speed::VeryHigh);
        pins.pk5.into_alternate::<14>().set_speed(Speed::VeryHigh);
        pins.pk6.into_alternate::<14>().set_speed(Speed::VeryHigh);
        pins.pk7.into_alternate::<14>().set_speed(Speed::VeryHigh);

        let _ = pins.hse_out.into_push_pull_output();
        let enable_pin = pins.enable.into_push_pull_output();
        let backlight_pin = pins.backlight.into_push_pull_output();

        Stm32F7DiscoDisplay {
            controller,
            enable_pin,
            backlight_pin,
        }
    }
    pub fn enable(&mut self) {
        self.enable_pin.set_high();
    }
    pub fn disable(&mut self) {
        self.enable_pin.set_low();
    }
    pub fn backlight_on(&mut self) {
        self.backlight_pin.set_high();
    }
    pub fn backlight_off(&mut self) {
        self.backlight_pin.set_low();
    }
}

impl Stm32F7DiscoDisplay<u16> {
    pub fn config_layer1(&mut self) {
        self.controller
            .config_layer(Layer::L1, unsafe { &mut FB_LAYER1 }, PixelFormat::RGB565);
    }
}

impl OriginDimensions for Stm32F7DiscoDisplay<u16> {
    fn size(&self) -> Size {
        Size::new(480, 272)
    }
}

impl DrawTarget for Stm32F7DiscoDisplay<u16> {
    type Error = core::convert::Infallible;
    type Color = Rgb565;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            let value: u16 = (color.b() as u16 & 0x1F)
                | ((color.g() as u16 & 0x3F) << 5)
                | ((color.r() as u16 & 0x1F) << 11);
            self.controller
                .draw_pixel(Layer::L1, coord.x as usize, coord.y as usize, value);
        }

        Ok(())
    }
}
