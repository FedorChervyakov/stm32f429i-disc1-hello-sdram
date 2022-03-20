#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use rtt_target::{rtt_init_print, rprintln};

use crate::hal::{
    gpio::Speed,
    pac::{CorePeripherals, Peripherals},
    prelude::*,
};



#[entry]
fn main() -> ! {
    rtt_init_print!();

    let cp = cortex_m::Peripherals::take().unwrap();
    let hp = hal::pac::Peripherals::take().unwrap();

    cortex_m::interrupt::free(|_cs| {
        // Split all the GPIO blocks we need
        let gpiob = hp.GPIOB.split();
        let gpioc = hp.GPIOC.split();
        let gpiod = hp.GPIOD.split();
        let gpioe = hp.GPIOE.split();
        let gpiof = hp.GPIOF.split();
        let gpiog = hp.GPIOG.split();

        // Setup clocks
        let rcc = hp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(180.MHz()).freeze();

        // Configure pins

        // Program FMC_SDCRx register
        // Program FMC_SDCR1 register (SDRAM, RBURST, RPIPE)
        // Program timings into the FMC_SDCRx register
        // Program TRP and TRC timings in the FMC_SDTR1 register
    });

    loop {
        // your code goes here
    }
}
