#[macro_export]
macro_rules! println {
    ($fmt:expr $(, $arg:tt)*) => ($crate::io::with_stdout(|h| {
        use std::io::Write;
        write!(h, concat!($fmt, "\n") $(, $arg)*).unwrap();
    }))
}

#[macro_export]
macro_rules! eprintln {
    ($fmt:expr $(, $arg:tt)*) => ($crate::io::with_stderr(|h| {
        use std::io::Write;
        write!(h, concat!($fmt, "\n") $(, $arg)*).unwrap();
    }))
}

#[macro_export]
macro_rules! main {
    ($body:block) => {
        #[no_mangle]
        pub extern "C" fn __app_main() {
            { $body }
        }
    }
}
