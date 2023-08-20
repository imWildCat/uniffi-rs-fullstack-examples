uniffi::setup_scaffolding!();
// This must be put at the top


pub fn rust_greeting(to: String) -> String {
    return format!("Hello, {}!", to);
}

// uniffi::include_scaffolding!("hello");
// We don't need this because of uniffi::setup_scaffolding()
// Please refer to <https://mozilla.github.io/uniffi-rs/proc_macro/index.html>
