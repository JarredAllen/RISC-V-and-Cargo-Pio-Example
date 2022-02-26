#![no_std]

use riscv::asm::wfi;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
extern "C" fn main() -> i32 {
    loop {
        unsafe { wfi() }
    }
}
