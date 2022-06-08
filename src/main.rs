use std::io::stdin;
mod visitor;

fn main() {
    println!("Hello, what's your name?");
    let visitor_list = [
        visitor::Visitor::new("Bert", "hi"),
        visitor::Visitor::new("Shin", "Wsup"),
    ];
    let your_name = what_is_your_name();
    if let Some(visitor) = visitor_list.iter().find(|v| v.name == your_name) {
        visitor.greet_visitor();
    } else {
        println!("Sorry, {}. Your name in not in list.", your_name);
    }
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    your_name
        .trim()
        .to_lowercase()
}

