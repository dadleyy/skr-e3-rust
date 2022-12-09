#![deny(warnings)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m as _;
use cortex_m_rt as rt;
use panic_halt as _;
use stm32g0xx_hal as hal;

use hal::prelude::*;
use hal::rcc::Config;
use hal::stm32;
use rt::entry;

#[entry]
fn main() -> ! {
  let dp = stm32::Peripherals::take().expect("cannot take peripherals");
  let mut rcc = dp.RCC.freeze(Config::pll());
  let mut delay = dp.TIM15.delay(&mut rcc);
  let gpioa = dp.GPIOA.split(&mut rcc);
  let gpiob = dp.GPIOB.split(&mut rcc);

  let mut led = gpioa.pa8.into_push_pull_output();

  let mut step = gpiob.pb13.into_push_pull_output();
  let mut dir = gpiob.pb12.into_push_pull_output();
  let mut en = gpiob.pb14.into_push_pull_output();

  en.set_low().unwrap();
  dir.set_high().unwrap();

  let mut current_dir = true;

  loop {
    for _ in 0..(200 * 8) {
      led.set_high().unwrap();
      step.set_high().unwrap();
      delay.delay_ms(1u16);
      step.set_low().unwrap();
      led.set_low().unwrap();
      delay.delay_ms(1u16);
    }

    if current_dir {
      current_dir = false;
      dir.set_low().unwrap();
    } else {
      current_dir = false;
      dir.set_high().unwrap();
    }

    delay.delay(hal::time::Second(2));
  }
}
