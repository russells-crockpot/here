macro_rules! make_log_rule {
    (($d:tt), $level:ident, $name:ident) => {
        /// Logs the line number and filename using `log::$level!`. Additionally, it can accept a
        /// string literal and format arguments like the `format!` macro.
        #[macro_export]
        macro_rules! $name {
            () => {
                ::log::$level!("{}:{:03}", file!(), line!());
            };
            ($msg:literal) => {
                ::log::$level!("({}:{:03}): {}", file!(), line!(), $msg);
            };
            ($msg:literal, $d($args:expr),+) => {
                ::log::$level!("({}:{:03}): {}", file!(), line!(), format!($msg, $d($args),+));
            }
        }
    };
    ($level:ident, $name:ident) => {
        make_log_rule!{($), $level, $name}
    };
}

make_log_rule! {trace, here_trace}
make_log_rule! {debug, here_debug}
make_log_rule! {info, here_info}
make_log_rule! {warn, here_warn}
make_log_rule! {error, here_error}
