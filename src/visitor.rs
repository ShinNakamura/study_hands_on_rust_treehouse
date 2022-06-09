#[derive(Debug)]
pub enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation, // 保護観察
}

#[derive(Debug)]
pub struct Visitor {
    pub name: String, // 名前でリストに入っているかチェックするのでパブリックにしてる
    greeting: String,
    action: VisitorAction,
}

impl Visitor {
    pub fn new(name: &str, greeting: &str, action: VisitorAction) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
            action: action,
        }
    }

    pub fn greet_visitor(&self) {
        println!("{}, {}", self.greeting, self.name);
    }
}

