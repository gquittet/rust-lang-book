pub fn play() {
    // if else if else
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Ternary
    let condition = true;
    // 5 and 6 need to be the same type!
    // We can't do this : let number = if condition { 5 } else { "six" };
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

    // loop

    // loop countdown
    // a)
    let a: [u8; 3] = [3, 2, 1];

    for element in a.iter() {
        println!("{}!", element);
    }
    println!("LIFTOFF!!!");
    // b)
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
