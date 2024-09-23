pub struct HelloWorld;

impl HelloWorld {
    pub fn hello() -> String {
        "Hello, world!".into()
    }
}

include!("bindings.rs");
