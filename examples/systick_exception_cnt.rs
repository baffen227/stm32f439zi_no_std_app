#![deny(unsafe_code)]
#![no_main]
#![no_std]

// Print panic message to probe console
// use panic_probe as _;
use panic_semihosting as _;

use core::fmt::Write;

use cortex_m::peripheral::{syst, Peripherals};

use cortex_m_rt::{entry, exception};

use cortex_m_semihosting::hio::{self, HostStream};

use stm32f4xx_hal as _;

#[allow(clippy::empty_loop)]
#[entry]
fn main() -> ! {
	let peripherals = Peripherals::take().unwrap();
	let mut systick = peripherals.SYST;

	systick.set_clock_source(syst::SystClkSource::Core);
	systick.set_reload(12_000_000);
	systick.clear_current();
	systick.enable_counter();
	systick.enable_interrupt();

	// while !systick.has_wrapped() {
	//    let val = peripheral::SYST::get_current();
	//    hprintln!("Hello, world! {}", val);
	//}

	loop {}
}

#[exception]
fn SysTick() {
	static mut COUNT: u32 = 0;
	static mut STDOUT: Option<HostStream> = None;

	*COUNT += 1;

	if STDOUT.is_none() {
		*STDOUT = hio::hstdout().ok();
	}

	if let Some(hoststream) = STDOUT.as_mut() {
		write!(hoststream, "{}", *COUNT).ok();
	}
}
