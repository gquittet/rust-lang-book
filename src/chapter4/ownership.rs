use crate::util::chapter::Title;

pub fn play() {
    let title = Title {
        chapter: 4,
        name: "Ownership".to_string(),
    };
    title.print();
    // Stack and Heap
    // Stack (example: stack of plates)
    // - Last In First Out
    // - Elements in stack must have a fixed size
    // - Pushing data and access is faster than heap
    // Heap (example: stack of plates)
    // - Store element where there is enough space and return its pointer.
    // - Elements in the heap have a dynamic size
    // - Store data is slow because need to find a place and accessing is slower
    // than Stack because we need to follow the pointer.

    // Rules to always keep in mind
    // - Each value in Rust has a variable that is called its owner.
    // - There can only be one owner at a time.
    // - When the owner goes out of scope, the value will be dropped.
    // { // s not exist
    //      let s = "ici";
    // } // s is dropped
    // s exist only between this curly braces.
    println!("Just read the comments");
}
