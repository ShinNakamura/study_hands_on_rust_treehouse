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
    action: VisitorAction,
    age: i8,
}

impl Visitor {
    pub fn new(name: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action: action,
            age: age,
        }
    }

    pub fn greet_visitor(&self) {
        match &self.action {
            VisitorAction::Accept => println!("Welcome, {}", self.name),
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome, {}", self.name);
                println!("{note}");
                if self.age < 21 {
                    println!("Do not serve alcohol to {}", self.name);
                }
            },
            VisitorAction::Probation => println!("{} is a probationary mamber", self.name),
            VisitorAction::Refuse => println!("Do not allow {} in!", self.name),
        }
    }
}

