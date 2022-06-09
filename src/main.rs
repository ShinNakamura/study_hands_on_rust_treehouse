use std::io::stdin;
mod visitor;
// ↑ このファイルが`src`直下にある`main.rs`なのでこのように
// `mod visitor;` という書き方になっている。
// もし、これが `src/bin/(コマンドの名前).rs` に移動したとすると
// `use treehouse::visior;` となる。

fn main() {
    println!("Hello, what's your name?(leave empty and press ENTER to quit)");
    let mut visitor_list = vec![
        visitor::Visitor::new("Bert", "hi", visitor::VisitorAction::Accept, 50),
        visitor::Visitor::new("Shin", "Wasup", 
                              visitor::VisitorAction::AcceptWithNote{note: "*".to_string()},
                              44),
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
            visitor_list.push(visitor::Visitor::new(&name, "Hello, New friend",
                                                    visitor::VisitorAction::Accept,
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

