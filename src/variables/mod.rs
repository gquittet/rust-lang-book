pub fn play() {
    // Variables are immutable. We can't do that
    // let x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);

    // Variables can be mutable
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Constant
    const MAX_POINTS: u32 = 100_000;
    println!("My constant is {}", MAX_POINTS);

    // Shadowing
    let w = 5;

    let w = w + 1;

    let w = w * 2;

    println!("The value of w is: {}", w);

    // We can do use another type because the second space in another variable.
    let spaces = "   ";
    println!("Spaces: '{}'", spaces);
    let spaces = spaces.len();
    println!("Spaces: {}", spaces);

    // We can't do that because spaces is a type mutable string. So, it can't be
    // an Integer.
    // let mut spaces = "   ";
    // spaces = spaces.len();
}
