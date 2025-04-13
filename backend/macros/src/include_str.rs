/// Includes a UTF-8 encoded file as a string
///
/// The file is located relative to `CARGO_MANIFEST_DIR`.
///
/// # Example
/// ```rust
/// use macros::include_str_from_crate_root;
///
/// let str: &str = include_str_from_crate_root!("/query/scheme.sql");
///
// todo: impl
// const PATH_PREFIX: &str = "/src";
// let str: &str = include_str_from_crate_root!("{}/query/scheme.sql", PATH_PREFIX);
/// ```
#[macro_export]
macro_rules! include_str_from_crate_root {
    // ($path:literal $(, $($arg:tt)*)?) => {
    // ($path:expr, $($args:tt)*) => {
    //     include_str!(concat!(env!("CARGO_MANIFEST_DIR"), format_args!($path, $($args)*)))
    // };
    ($path:expr) => {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), $path))
    };
}
