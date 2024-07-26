macro_rules! dprintln {
    ($($arg:tt)*) => (#[cfg(feature = "debug_verbose")] println!($($arg)*));
}

pub(crate) use dprintln;