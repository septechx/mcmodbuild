#[macro_export]
macro_rules! string {
    ($s:literal) => {
        String::from($s)
    };
}
