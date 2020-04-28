use crate::util::chapter::Title;

pub fn play() {
    let title = Title {
        chapter: 4,
        name: "Borrowing".to_string(),
    };
    title.print();
    // By default a reference is immutable like variables. Even the variable is
    // mutable, its reference by default is immutable. If we need to change it,
    // we have to change it as a mutable reference.
    mutable_reference();

    // One variable must have at most one mutable reference in the same
    // ownership.
    // Example A
    let mut s = String::from("Hello");
    {
        // r1 not the same ownership of r2 so it works
        let _r1 = &mut s;
    }
    let _r2 = &mut s;

    // Example B
    // This is not working because we have 2 mutable references of one variable
    // in the same ownership.
    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s;
    //
    // println!("{}, {}", r1, r2);

    // Example C
    // This is not working because we can't have immutable references and
    // mutable reference at the same time.
    // If we've immutable reference, we don’t expect the values to suddenly
    // change out from under them!
    // See "Example D" to get the solution.
    // let mut s = String::from("hello");
    //
    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM
    //
    // println!("{}, {}, and {}", r1, r2, r3);

    // Example D
    let mut s = String::from("me");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

    // Conclusion
    // - At any given time, you can have either one mutable reference or any
    // number of immutable references.
    // - References must always be valid.
}

fn mutable_reference() {
    let mut s = String::from("Hello");
    change(&mut s);
    println!("{}", s);
}

fn change(s: &mut String) {
    s.push_str(" world!");
}
