use crate::util::chapter::Title;
// use std::fs::File;
// use std::io::ErrorKind;

pub fn play() {
    let title = Title {
        chapter: 9,
        name: "Recoverable Errors with Result".to_string(),
    };
    title.print();

    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    // Example opening a file
    // let f = File::open("hello.txt");
    // let _f = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error),
    // };

    // Match different errors
    // let f = File::open("hello.txt");
    // let _f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error)
    //         }
    //     },
    // };

    // Other way
    // let _f = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {:?}", error);
    //         })
    //     } else {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // });

    // Shortcut
    // let _f = File::open("hello.txt").expect("Failed to open hello.txt");

    // We can use the ? operator
    // The function needs to return a Result to use ? inside it.
    // let mut s = String::new();
    // File::open("hello.txt")?.read_to_string(&mut s)?;
}
