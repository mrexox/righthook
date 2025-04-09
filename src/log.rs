#[macro_export]
macro_rules! error {
    ($($arg:tt)+) => {
        use colored::Colorize;

        eprintln!("{}", format!($($arg)*).red());
        std::process::exit(1);
    };
}
