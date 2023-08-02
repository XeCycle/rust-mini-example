
#[panic_handler]
unsafe fn panic(_info: &core::panic::PanicInfo) -> ! {
    core::arch::asm!("int3", options(noreturn));
}
