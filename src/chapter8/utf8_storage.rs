use crate::util::chapter::Title;

pub fn play() {
    let title = Title {
        chapter: 8,
        name: "UTF-8 Storage Text".to_string(),
    };
    title.print();

    // 2 types of string
    // - str -> immutable
    // - String -> mutable

    // Creating new empty String
    let mut _s = String::new();

    // Creating String from literal str
    let _s = "This is a string".to_string();
    // We can also use String::from
    let _s = String::from("This is a string");

    // Concatenation
    // Appending String to a String
    // push_str take a slice because we won't to take the ownership.
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);

    let mut s1 = String::from("Pa");
    let s2 = "ris";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    // Appending char to a String
    // push
    let mut s = String::from("lo");
    s.push('l');
    println!("s is {}", s);

    // Appending with the '+' operation
    // The '+' operator use the 'add' method whose signature looks something like
    // this: fn add(self, s: &str) -> String {
    let s1 = String::from("New");
    let s2 = String::from("York");
    let s = s1 + "-" + &s2;
    println!("s -> {}", s);

    // Strings Internal Representation
    // String are wrappers around Vec<u8>
    let hello = "Bonjour";
    let answer = &hello[0..4];
    println!("{}", answer);
    // You can't get a char from a String because of encoding problems.
    // https://doc.rust-lang.org/book/ch08-02-strings.html#internal-representation
    // The below line won't compile.
    // let answer = &hello[0];

    // Iterating over Strings
    for c in "Hello World!".chars() {
        println!("{}", c);
    }
}
