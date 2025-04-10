#[macro_export]
macro_rules! fail {
    ($($arg:tt)+) => {
        use colored::Colorize;

        eprintln!("{}", format!($($arg)*).red());
        std::process::exit(1);
    };
}
