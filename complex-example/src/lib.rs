pub type StrVec = Vec<String>;

#[derive(Debug, Clone)]
pub struct MyStruct {
    pub a: String,
    pub b: i32,
    pub c: f64,
    pub people: StrVec,
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
            people: Vec::new(),
        }
    }

    pub fn say_to_all(&self, msg: String) {
        println!(
            "Attention, {}:\n>> {}",
            self.people
                .iter()
                .map(|it| format!("{it}"))
                .collect::<Vec<_>>()
                .join(", "),
            msg
        );
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
