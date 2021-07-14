mod with_std {
    #[macro_export]
    /// Prints the line number and filename to stdout. Additionally, it can accept a string literal
    /// and format arguments like the `format!` macro.
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    macro_rules! here {
        () => {};
        ($msg:literal) => {};
        ($msg:literal, $($args:expr),+) => {};
    }

    #[macro_export]
    /// Prints the line number and filename to stderr. Additionally, it can accept a string literal
    /// and format arguments like the `format!` macro.
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    macro_rules! ehere {
        () => {};
        ($msg:literal) => {};
        ($msg:literal, $($args:expr),+) => {};
    }
}
