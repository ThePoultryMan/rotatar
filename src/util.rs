macro_rules! arctex {
    ($value:expr) => {
        std::sync::Arc::new(std::sync::Mutex::new($value))
    };
}
pub(crate) use arctex;
