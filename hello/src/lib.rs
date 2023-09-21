uniffi::include_scaffolding!("hello");
// This is interesting. Because we're supposed to use setup_scaffolding!() at the top.
// Please refer to <https://mozilla.github.io/uniffi-rs/proc_macro/index.html>
// I found this sample at: https://github.com/MathieuTricoire/convex-rs-ffi/tree/90fb36ea3dec16b05a8e4f47aa032987b2727122
// uniffi::setup_scaffolding!();
use async_std::future::{pending, timeout};

#[uniffi::export(callback_interface)]
pub trait GreetingDelegate: Send + Sync {
    fn greeting_called(&self, to: String);
}

pub struct GreetingLogger {
    delegate: Box<dyn GreetingDelegate>,
}

impl GreetingLogger {
    pub fn new(delegate: Box<dyn GreetingDelegate>) -> Self {
        Self { delegate }
    }

    pub fn greeting_called(&self, to: String) {
        self.delegate.greeting_called(to)
    }
}

static LOGGER_INSTANCE: once_cell::sync::OnceCell<GreetingLogger> =
    once_cell::sync::OnceCell::new();

#[uniffi::export]
pub fn set_logging_delegate(delegate: Box<dyn GreetingDelegate>) {
    let logger = GreetingLogger::new(delegate);
    let result = LOGGER_INSTANCE.set(logger);
    if result.is_err() {
        panic!("Logger already set");
    }
}

#[uniffi::export]
pub fn rust_greeting(to: String) -> String {
    if let Some(logger) = LOGGER_INSTANCE.get() {
        logger.greeting_called(to.clone());
    }
    return format!("Hello, {}!", to);
}

#[uniffi::export]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[uniffi::export]
pub async fn say_after(ms: u64, who: String) -> String {
    let never = pending::<()>();
    timeout(std::time::Duration::from_millis(ms), never)
        .await
        .unwrap_err();
    format!("Hello, {who}!")
}
