#[macro_export]
macro_rules! arctex {
    ($value:expr) => {
        std::sync::Arc::new(std::sync::Mutex::new($value))
    };
}

#[macro_export]
macro_rules! interval {
    ($duration:expr, $block:block) => {
        let mut interval = tokio::time::interval($duration);
        loop {
            interval.tick().await;
            $block
        }
    };
}
