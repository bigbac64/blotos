use x86_64::instructions::port::Port;

pub(crate) struct SerialWriter;


fn dbg_write_str(s: &str) {
    let mut port: Port<u8> = Port::new(0x3F8);
    for byte in s.as_bytes() {
        unsafe { port.write(*byte); }
    }
}

impl core::fmt::Write for SerialWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        dbg_write_str(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! dbg_print {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        let _ = write!(crate::dbg_print::SerialWriter, $($arg)*);
    }}
}

#[macro_export]
macro_rules! dbg_println {
    ($($arg:tt)*) => { crate::dbg_print!("{}\n", format_args!($($arg)*)) }
}