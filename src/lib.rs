#![no_std]
#![feature(start)]
#![feature(const_fn_floating_point_arithmetic)]

use alloc::fmt;
use alloc::vec::Vec;
extern crate alloc;

#[allow(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    improper_ctypes,
    dead_code
)]
pub mod irq_bindings;
pub mod memory_allocator;
pub mod panic;
#[cfg(target_os = "nintendo_ds_arm7")]
#[allow(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    improper_ctypes,
    dead_code
)]
pub mod arm7_bindings;

#[cfg(target_os = "nintendo_ds_arm7")]
#[allow(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    improper_ctypes,
    dead_code
)]
pub mod dswifi7;

#[cfg(target_os = "nintendo_ds_arm7")]
#[allow(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    improper_ctypes,
    dead_code
)]
pub mod maxmod7;

#[cfg(target_os = "nintendo_ds_arm7")]
#[allow(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    improper_ctypes,
    dead_code
)]
pub mod bios_registers;

#[allow(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    improper_ctypes,
    dead_code
)]
#[cfg(target_os = "nintendo_ds_arm9")]
pub mod arm9_bindings;

#[cfg(target_os = "nintendo_ds_arm9")]
#[allow(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    improper_ctypes,
    dead_code
)]
pub mod background_registers;
#[cfg(target_os = "nintendo_ds_arm9")]
#[allow(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    improper_ctypes,
    dead_code
)]
pub mod video_registers;

#[cfg(target_os = "nintendo_ds_arm7")]
mod atomics;
#[cfg(target_os = "nintendo_ds_arm9")]
mod atomics;
#[cfg(target_os = "nintendo_ds_arm7")]
#[no_mangle]
pub static __debugger_unit: c_char = 0;

pub struct Buffer {
    buf: Vec<u8>,
}

impl Buffer {
    pub fn new() -> Self
    {
        Self {
            buf: Vec::new()
        }
    }
    pub fn buf(&mut self) -> &mut Vec<u8> {
        &mut self.buf
    }
}

impl fmt::Write for Buffer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.buf.push(byte);
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let mut writer = libnds_sys::Buffer::new();
        let _ = write!(&mut writer, "{}", format_args!($($arg)*));
        unsafe {
            printf("%s\0".as_ptr() as *const core::ffi::c_char, writer.buf().as_ptr() as *const core::ffi::c_char);
        }
    });
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let mut writer = libnds_sys::Buffer::new();
        let _ = write!(&mut writer, "{}\n", format_args!($($arg)*));
        unsafe {
            printf("%s\0".as_ptr() as *const core::ffi::c_char, writer.buf().as_ptr() as *const core::ffi::c_char);
        }
    });
}