#![allow(clippy::print_literal)]
use std::{env, process::exit};

fn main() {
    if matches!(
        env::var("CARGO_FEATURE_STD"),
        Err(env::VarError::NotPresent)
    ) && matches!(
        env::var("CARGO_FEATURE_LOG"),
        Err(env::VarError::NotPresent)
    ) {
        eprintln!(
            "Either the 'log' feature or the 'std' feature must be enabled, otherwise {}",
            "this crate will do nothing."
        );
        exit(1);
    }
}
