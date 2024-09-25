#[derive(Debug, Clone)]
pub struct MyStruct {
    pub a: String,
    pub b: i32,
    pub c: f64,
}

#[derive(Debug)]
pub struct MyOtherStruct {
    pub a: String,
    pub b: MyStruct,
}

impl MyStruct {
    pub fn new() -> Self {
        Self {
            a: String::new(),
            b: 0,
            c: 0.0,
        }
    }
}

impl MyOtherStruct {
    pub fn new() -> Self {
        Self {
            a: String::new(),
            b: MyStruct::new(),
        }
    }

    pub fn say_only(&self, message: String) {
        println!("{}", message);
    }

    pub fn say(&self, p2: String) {
        println!("{}{}", self.b.a, p2);
    }

    pub fn say_with(&self, p1: MyStruct, p2: String) {
        println!("{}{}", p1.a, p2);
    }
}

include!("bindings.rs");
