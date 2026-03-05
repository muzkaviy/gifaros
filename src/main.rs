#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::global_asm;

// İşlemci başladığında stack pointer'ı ayarlar ve main'e atlar
global_asm!(
    ".section .text._start",
    ".global _start",
    "_start:",
    "mov x0, #0x0000",      // Adresin alt kısmı
    "movk x0, #0x4008, lsl #16", // Adresin üst kısmı (0x40080000 yapar)
    "mov sp, x0",
    "bl main",
    "b ." // Olduğu yere dallan (sonsuz döngü)
);

#[no_mangle]
pub extern "C" fn main() -> ! {
    let uart = 0x0900_0000 as *mut u8;
    let msg = b"GIFAROS M4 PRO: SISTEM AKTIF!\n";

    for &byte in msg {
        unsafe {
            core::ptr::write_volatile(uart, byte);
        }
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}