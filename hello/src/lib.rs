pub fn rust_greeting(to: String) -> String {
    return format!("Hello, {}!", to);
}

include!(concat!(env!("OUT_DIR"), "/hello.uniffi.rs"));
