pub struct Visitor {
    pub name: String,
    greeting: String,
}

impl Visitor {
    pub fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
        }
    }

    pub fn greet_visitor(&self) {
        println!("{}, {}", self.greeting, self.name);
    }
}

