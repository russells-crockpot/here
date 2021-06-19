//! One of the oldest and still most commonly used debugging techniques looks something like this:
//!
//! ```ignore
//! // Do something
//! println!("here");
//! // Do something else
//! ```
//!
//! Or if you're feeling real fancy:
//!
//! ```ignore
//! // Do something
//! println!("here 1");
//! // Do something else
//! println!("here 2");
//! // Do yet another thing
//! ```
//!
//! It's rather crude, but it's effective... as long as you can find where you put the print
//! statements.
//!
//! Well, [`here`] is here to help! Usage is simple:
//!
//! ```ignore
//! here!(); // prints "filename:lineno" to stdout
//! ```
//!
//! What's that? You want to print to stderr instead of stdout? Then just use [`ehere`]. It's the
//! same great taste as [`here`], but with stderr!
//!
//! Oh? So you've decided to use the wonderful `log` crate? Don't worry, we've got you covered
//! there, too. With the optional `log` feature enabled, you get five new macros! One for each log
//! level!
//!
//! ```ignore
//! here_trace!(); // log a trace message of "filename:lineno"
//! here_debug!(); // log a debug message of "filename:lineno"
//! here_info!(); // log a info message of "filename:lineno"
//! here_warn!(); // log a warn message of "filename:lineno"
//! here_error!(); // log a error message of "filename:lineno"
//! ```
//!
//! Huh? You want add more information? Don't worry we've got you covered. Each `here` macro can
//! accept a format string and arguments just like the `format!` macro!
//!
//! ```ignore
//! here!("A message."); // prints "(filename:lineno): A message."
//! here!("Something: 0x{:02x}", 16);  // prints "(filename:lineno): Something: 0x0f"
//! ```

#[cfg(feature = "log")]
mod log_rules;

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
