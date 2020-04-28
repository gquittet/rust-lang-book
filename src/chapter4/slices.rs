use crate::util::chapter::Title;

pub fn play() {
    let title = Title {
        chapter: 4,
        name: "Slices".to_string(),
    };
    title.print();

    // You can make the same thing with an array.
    let s = String::from("hello world");
    println!("let s = {}", s);

    let hello = &s[0..5];
    println!("&s[0..5] -> {}", hello);
    let hello = &s[..5];
    println!("&s[..5] -> {}", hello);
    let world = &s[6..11];
    println!("&s[6..11] -> {}", world);
    let world = &s[6..];
    println!("&s[6..] -> {}", world);
    let s = &s[..];
    println!("&s[..] -> {}", s);
}
