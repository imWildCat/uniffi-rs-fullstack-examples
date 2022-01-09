
pub fn say_hello(name: String) {
    println!("Hello, {}!", name);
}

include!(concat!(env!("OUT_DIR"), "/hello.uniffi.rs"));
