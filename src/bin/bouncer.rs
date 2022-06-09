use std::io::stdin;
use treehouse::visitor::{Visitor, VisitorAction};

fn main() {
    println!("Hello, what's your name?(leave empty and press ENTER to quit)");
    let mut visitor_list = vec![
        Visitor::new("Bert", VisitorAction::Accept, 50),
        Visitor::new("Shin", 
                      VisitorAction::AcceptWithNote{note: "V.I.P".to_string()},
                      44),
        Visitor::new("Jr", 
                      VisitorAction::AcceptWithNote{note: "V.I.P son".to_string()},
                      12),
        Visitor::new("DJ", VisitorAction::Probation, 30),
        Visitor::new("Fred", VisitorAction::Refuse, 30),
    ];
    loop {
        let name = what_is_your_name();
        if name.is_empty() {
            println!("The final list of visitors:");
            println!("{:#?}", visitor_list);
            break;
        }
        if let Some(visitor) = visitor_list.iter().find(|v| v.name == name) {
            visitor.greet_visitor();
        } else {
            println!("Sorry, {}. Your name in not in list.", name);
            // 最初は追い返すが、新しい友達としてリスト登録してあげる
            visitor_list.push(
                Visitor::new(&name,
                            VisitorAction::AcceptWithNote{note: "New friend".to_string()},
                            21));
        }
    }
}

fn what_is_your_name() -> String {
    let mut name = String::new();
    stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    name
        .trim()
        .to_lowercase()
}

