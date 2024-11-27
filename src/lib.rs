//! Feel free to create helpers

#[macro_export]
macro_rules! import_file {
    ($path:literal) => {
        const FILE: &str = include_str!($path);
    };
}
