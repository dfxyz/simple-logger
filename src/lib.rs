#[macro_export]
macro_rules! clear_prev_line {
    () => {
        print!("\x1b[1A\x1b[2K");
    };
}

#[macro_export]
macro_rules! clear_prev_lines {
    ($n:expr) => {
        for _ in 0..$n {
            print!("\x1b[1A\x1b[2K");
        }
    };
}

#[macro_export]
macro_rules! info {
    ($($tt:tt)+) => {
        println!("\x1b[1;32minfo:\x1b[0m {}", format_args!($($tt)+));
    };
}

#[macro_export]
macro_rules! warn {
    ($($tt:tt)+) => {
        println!("\x1b[1;33mwarning:\x1b[0m {}", format_args!($($tt)+));
    };
}

#[macro_export]
macro_rules! error {
    ($($tt:tt)+) => {
        println!("\x1b[1;31merror:\x1b[0m {}", format_args!($($tt)+));
    };
}

#[macro_export]
macro_rules! custom_red {
    ($title:ident: $($tt:tt)+) => {
        println!("\x1b[1;31m{}:\x1b[0m {}", stringify!($title), format_args!($($tt)+));
    };
}

#[macro_export]
macro_rules! custom_green {
    ($title:ident: $($tt:tt)+) => {
        println!("\x1b[1;32m{}:\x1b[0m {}", stringify!($title), format_args!($($tt)+));
    };
}

#[macro_export]
macro_rules! custom_yellow {
    ($title:ident: $($tt:tt)+) => {
        println!("\x1b[1;33m{}:\x1b[0m {}", stringify!($title), format_args!($($tt)+));
    };
}

#[macro_export]
macro_rules! custom_blue {
    ($title:ident: $($tt:tt)+) => {
        println!("\x1b[1;34m{}:\x1b[0m {}", stringify!($title), format_args!($($tt)+));
    };
}

#[macro_export]
macro_rules! custom_magenta {
    ($title:ident: $($tt:tt)+) => {
        println!("\x1b[1;35m{}:\x1b[0m {}", stringify!($title), format_args!($($tt)+));
    };
}

#[macro_export]
macro_rules! custom_cyan {
    ($title:ident: $($tt:tt)+) => {
        println!("\x1b[1;36m{}:\x1b[0m {}", stringify!($title), format_args!($($tt)+));
    };
}

#[cfg(test)]
#[ignore]
#[test]
fn test() {
    info!("this is an info message");
    warn!("this is a warning message");
    error!("this is an error message");
    
    custom_red!(TITLE: "this is a custom red message");
    custom_green!(TITLE: "this is a custom green message");
    custom_yellow!(TITLE: "this is a custom yellow message");
    custom_blue!(TITLE: "this is a custom blue message");
    custom_magenta!(TITLE: "this is a custom magenta message");
    custom_cyan!(TITLE: "this is a custom cyan message");
    
    let n = 1;
    info!("this line will be overwritten");
    clear_prev_line!();
    custom_red!(OVERRIDE: "{} line(s) has been overwritten", n);
    
    let n = 3;
    info!("this line will be overwritten");
    warn!("this line will be overwritten");
    error!("this line will be overwritten");
    clear_prev_lines!(n);
    custom_red!(OVERRIDE: "{} line(s) has been overwritten", n);
}