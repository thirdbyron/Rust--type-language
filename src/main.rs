use std::io;

fn main() {
    println!("Type the language you are studying right now");

    let mut lang: String = String::new();

    io::stdin().read_line(&mut lang).expect("Wrong typying");

    let lang: &str = &lang.trim().to_lowercase();

    match lang {
        "rust" => println!("Rust is a very good programming language!"),
        _ => println!("Good for you! Have you ever heard about Rust?"),
    }
}
