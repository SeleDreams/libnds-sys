#![no_std]
#![feature(start)]
#![feature(const_fn_floating_point_arithmetic)]

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