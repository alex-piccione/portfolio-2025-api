#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {{
        let timestamp = $crate::utils::logging::timestamp();
        let message = format!("{} [DEBUG] {}\n", timestamp, format_args!($($arg)*));
        let mut stdout = std::io::stdout();
        let _ = std::io::Write::write_all(&mut stdout, message.as_bytes());
        let _ = std::io::Write::flush(&mut stdout);
    }};
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        let timestamp = $crate::utils::logging::timestamp();
        let message = format!("{} [INFO] {}\n", timestamp, format_args!($($arg)*));
        let mut stdout = std::io::stdout();
        let _ = std::io::Write::write_all(&mut stdout, message.as_bytes());
        let _ = std::io::Write::flush(&mut stdout);
    }};
}


#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {{
        let timestamp = $crate::utils::logging::timestamp();
        let message = format!("{} [WARN] {}\n", timestamp, format_args!($($arg)*));
        let mut stderr = std::io::stderr();
        let _ = std::io::Write::write_all(&mut stderr, message.as_bytes());
        let _ = std::io::Write::flush(&mut stderr);
    }};
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        let timestamp = $crate::utils::logging::timestamp();
        let message = format!("{} [ERROR] {}\n", timestamp, format_args!($($arg)*));
        let mut stderr = std::io::stderr();
        let _ = std::io::Write::write_all(&mut stderr, message.as_bytes());
        let _ = std::io::Write::flush(&mut stderr);
    }};
}


pub fn timestamp() -> String {
    crate::utils::datetime::now()
        .format("%M:%S%.3f").to_string()  // MM:SS.sss to have only significant time part
}