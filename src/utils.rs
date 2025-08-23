#[macro_export]
macro_rules! red {
    ($($args:tt)*) => {
        format!("\x1b[1;31m{}\x1b[0m", format!($($args)*))
    };
}

#[macro_export]
macro_rules! green {
    ($($args:tt)*) => {
        format!("\x1b[1;32m{}\x1b[0m", format!($($args)*))
    };
}

#[macro_export]
macro_rules! yellow {
    ($($args:tt)*) => {
        format!("\x1b[1;33m{}\x1b[0m", format!($($args)*))
    };
}

#[macro_export]
macro_rules! blue {
    ($($args:tt)*) => {
        format!("\x1b[1;34m{}\x1b[0m", format!($($args)*))
    };
}

#[macro_export]
macro_rules! purple {
    ($($args:tt)*) => {
        format!("\x1b[1;35m{}\x1b[0m", format!($($args)*))
    };
}
