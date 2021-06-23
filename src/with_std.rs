#[macro_export]
/// Prints the line number and filename to stdout. Additionally, it can accept a string literal and
/// format arguments like the `format!` macro.
macro_rules! here {
    () => {
        println!("{}:{:03}", file!(), line!());
    };
    ($msg:literal) => {
        println!("({}:{:03}): {}", file!(), line!(), $msg);
    };
    ($msg:literal, $($args:expr),+) => {
        println!("({}:{:03}): {}", file!(), line!(), format!($msg, $d($args),+));
    }
}

#[macro_export]
/// Prints the line number and filename to stderr. Additionally, it can accept a string literal and
/// format arguments like the `format!` macro.
macro_rules! ehere {
    () => {
        eprintln!("{}:{:03}", file!(), line!());
    };
    ($msg:literal) => {
        eprintln!("({}:{:03}): {}", file!(), line!(), $msg);
    };
    ($msg:literal, $($args:expr),+) => {
        eprintln!("({}:{:03}): {}", file!(), line!(), format!($msg, $d($args),+));
    }
}
