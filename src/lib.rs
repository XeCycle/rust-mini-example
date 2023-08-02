#![no_std]

mod panic_handler;

use linux_syscalls::*;

#[repr(transparent)]
struct InitFrame(u64);

impl InitFrame {
    unsafe fn get_args<'a>(initf: *const InitFrame) -> &'a [*const u8] {
        let argc = (*initf).0;
        let start = (initf as *const u8).offset(8) as *const *const u8;
        core::slice::from_raw_parts(start, argc as _)
    }
}

fn my_main(initf: *const InitFrame) -> Result<(), Errno> {
    let args = unsafe { InitFrame::get_args(initf) };
    for argp in args {
        let arg = unsafe { core::ffi::CStr::from_ptr(*argp as _) }.to_bytes();
        unsafe {
            syscall!([ro] Sysno::write, 1, arg.as_ptr(), arg.len())?;
        }
        let nl = b'\n';
        unsafe {
            syscall!([ro] Sysno::write, 1, &nl as *const _, 1)?;
        }
    }
    Ok(())
}

#[no_mangle]
extern "C" fn strlen(s: *const i8) -> usize {
    for i in 0.. {
        if 0 == unsafe { *s.offset(i as _) } {
            return i;
        }
    }
    usize::MAX
}

unsafe extern "C" fn my_main_noret(initf: *const InitFrame) -> ! {
    let status = my_main(initf).err().map(|e| e.into_raw()).unwrap_or(0);
    syscall!([!] Sysno::exit_group, status as u64)
}

core::arch::global_asm!(
    r".global _start
_start:
    xor %ebp, %ebp
    mov %rsp, %rdi
    jmp {main}",
    main=sym my_main_noret,
    options(att_syntax));
