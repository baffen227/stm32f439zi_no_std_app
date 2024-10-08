#![no_main]
#![no_std]

use panic_halt as _;

use core::{fmt::Write, ptr};

use cortex_m_rt::{entry, exception, ExceptionFrame};
use cortex_m_semihosting::hio;

use stm32f4xx_hal as _;

#[entry]
fn main() -> ! {
	// read a nonexistent memory location
	unsafe {
		ptr::read_volatile(0x3FFF_FFFE as *const u32);
	}

	loop {}
}

// NOTE: HardFault was not triggered, why ?

#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
	if let Ok(mut hstdout) = hio::hstdout() {
		writeln!(hstdout, "{:#?}", ef).ok();
	}

	loop {}
}
