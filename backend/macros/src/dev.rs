#[macro_export]
macro_rules! dev_consume {
    ($($var:ident),+) => {
        $(let _ = $var;)*
    };
}
