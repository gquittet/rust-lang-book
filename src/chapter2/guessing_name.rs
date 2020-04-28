use crate::util::chapter::Title;
use std::io;

pub fn play() {
    let title = Title {
        chapter: 2,
        name: "Hello".to_string(),
    };
    title.print();

    println!("Please input your name.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("👋 Hello {}", guess);
}
