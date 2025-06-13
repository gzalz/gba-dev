#![no_std]
#![no_main]

mod device;
mod game_loop;
mod input;
mod object;

use gba::prelude::*;

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
  #[cfg(debug_assertions)]
  if let Ok(mut logger) = MgbaBufferedLogger::try_new(MgbaMessageLevel::Fatal) {
    use core::fmt::Write;
    writeln!(logger, "{info}").ok();
  }
  loop {}
}

#[no_mangle]
extern "C" fn main() -> ! {
  device::init();
  game_loop::start();
  loop {}
}
