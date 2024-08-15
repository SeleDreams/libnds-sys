#[panic_handler]
#[allow(unused_variables)]
fn panic(info: &core::panic::PanicInfo) -> ! {
    #[cfg(target_os = "nintendo_ds_arm9")]
    {
        use core::fmt::Write;

        struct ConsoleWriter;

        impl Write for ConsoleWriter {
            fn write_str(&mut self, s: &str) -> core::fmt::Result {
                // this is not a good way to do this but it should hopefully avoid heap allocations
                //
                // since there's no telling what state the heap is in during a panic,
                // it's best to not require any allocations for printing the panic message
                for c in s.bytes() {
                    unsafe {
                        crate::arm9_bindings::printf("%c\0".as_ptr() as *const core::ffi::c_char, c as core::ffi::c_int);
                    }
                }
                Ok(())
            }
        }

        unsafe {
            crate::arm9_bindings::consoleDemoInit();
        }

        let (file, line) = match info.location() {
            Some(loc) => (loc.file(), loc.line()),
            None => ("(unknown file)", 0),
        };

        let message = info.message();
        
        let _ = write!(ConsoleWriter, "PANIC: {message}\n at {file}:{line}");
    }
    loop {}
}
