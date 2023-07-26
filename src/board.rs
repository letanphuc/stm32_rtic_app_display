use fugit::RateExtU32;
use stm32f7xx_hal::gpio::{Floating, GpioExt, Input, Pin};
use stm32f7xx_hal::pac::{Peripherals, DMA2D, LTDC};
use stm32f7xx_hal::rcc::{HSEClock, HSEClockMode, RccExt};

pub struct DisplayPins {
    pub pe4: Pin<'E', 4, Input<Floating>>,
    pub pg12: Pin<'G', 12, Input<Floating>>,
    pub pi9: Pin<'I', 9, Input<Floating>>,
    pub pi10: Pin<'I', 10, Input<Floating>>,
    pub pi13: Pin<'I', 13, Input<Floating>>,
    pub pi14: Pin<'I', 14, Input<Floating>>,
    pub pi15: Pin<'I', 15, Input<Floating>>,
    pub pj0: Pin<'J', 0, Input<Floating>>,
    pub pj1: Pin<'J', 1, Input<Floating>>,
    pub pj2: Pin<'J', 2, Input<Floating>>,
    pub pj3: Pin<'J', 3, Input<Floating>>,
    pub pj4: Pin<'J', 4, Input<Floating>>,
    pub pj5: Pin<'J', 5, Input<Floating>>,
    pub pj6: Pin<'J', 6, Input<Floating>>,
    pub pj7: Pin<'J', 7, Input<Floating>>,
    pub pj8: Pin<'J', 8, Input<Floating>>,
    pub pj9: Pin<'J', 9, Input<Floating>>,
    pub pj10: Pin<'J', 10, Input<Floating>>,
    pub pj11: Pin<'J', 11, Input<Floating>>,
    pub pj13: Pin<'J', 13, Input<Floating>>,
    pub pj14: Pin<'J', 14, Input<Floating>>,
    pub pj15: Pin<'J', 15, Input<Floating>>,
    pub pk0: Pin<'K', 0, Input<Floating>>,
    pub pk1: Pin<'K', 1, Input<Floating>>,
    pub pk2: Pin<'K', 2, Input<Floating>>,
    pub pk4: Pin<'K', 4, Input<Floating>>,
    pub pk5: Pin<'K', 5, Input<Floating>>,
    pub pk6: Pin<'K', 6, Input<Floating>>,
    pub pk7: Pin<'K', 7, Input<Floating>>,

    pub hse_out: Pin<'H', 1, Input<Floating>>,
    pub enable: Pin<'I', 12, Input<Floating>>,
    pub backlight: Pin<'K', 3, Input<Floating>>,
}

pub struct Board {
    pub display_pins: DisplayPins,
    pub ltdc: LTDC,
    pub dma2d: DMA2D,
}

impl Board {
    pub fn new(p: Peripherals) -> Board {
        let gpioe = p.GPIOE.split();
        let gpiog = p.GPIOG.split();
        let gpioh = p.GPIOH.split();
        let gpioi = p.GPIOI.split();
        let gpioj = p.GPIOJ.split();
        let gpiok = p.GPIOK.split();

        let pe4 = gpioe.pe4; // LTCD_B0

        let pg12 = gpiog.pg12; // LTCD_B4

        let pi9 = gpioi.pi9; // LTCD_VSYNC
        let pi10 = gpioi.pi10; // LTCD_HSYNC
        let pi13 = gpioi.pi13; // ??
        let pi14 = gpioi.pi14; // LTCD_CLK
        let pi15 = gpioi.pi15; // LTCD_R0

        let pj0 = gpioj.pj0; // LTCD_R1
        let pj1 = gpioj.pj1; // LTCD_R2
        let pj2 = gpioj.pj2; // LTCD_R3
        let pj3 = gpioj.pj3; // LTCD_R4
        let pj4 = gpioj.pj4; // LTCD_R5
        let pj5 = gpioj.pj5; // LTCD_R6
        let pj6 = gpioj.pj6; // LTCD_R7
        let pj7 = gpioj.pj7; // LTCD_G0
        let pj8 = gpioj.pj8; // LTCD_G1
        let pj9 = gpioj.pj9; // LTCD_G2
        let pj10 = gpioj.pj10; // LTCD_G3
        let pj11 = gpioj.pj11; // LTCD_G4
        let pj13 = gpioj.pj13; // LTCD_B1
        let pj14 = gpioj.pj14; // LTCD_B2
        let pj15 = gpioj.pj15; // LTCD_B3

        let pk0 = gpiok.pk0; // LTCD_G5
        let pk1 = gpiok.pk1; // LTCD_G6
        let pk2 = gpiok.pk2; // LTCD_G7
        let pk4 = gpiok.pk4; // LTCD_B5
        let pk5 = gpiok.pk5; // LTCD_B6
        let pk6 = gpiok.pk6; // LTCD_D7
        let pk7 = gpiok.pk7; // LTCD_E

        let ph1 = gpioh.ph1;

        let display_pins = DisplayPins {
            pe4,
            pg12,
            pi9,
            pi10,
            pi13,
            pi14,
            pi15,
            pj0,
            pj1,
            pj2,
            pj3,
            pj4,
            pj5,
            pj6,
            pj7,
            pj8,
            pj9,
            pj10,
            pj11,
            pj13,
            pj14,
            pj15,
            pk0,
            pk1,
            pk2,
            pk4,
            pk5,
            pk6,
            pk7,
            hse_out: ph1,
            enable: gpioi.pi12,
            backlight: gpiok.pk3,
        };

        let rcc = p.RCC.constrain();
        let _clocks = rcc
            .cfgr
            .hse(HSEClock::new(25_000_000.Hz(), HSEClockMode::Bypass))
            .sysclk(216_000_000.Hz())
            .hclk(216_000_000.Hz())
            .freeze();

        Board {
            display_pins,
            ltdc: p.LTDC,
            dma2d: p.DMA2D,
        }
    }
}
