pub fn play() {
    // Scalar Types

    // By default the float type is a 64bits float because modern processor deal
    // it with the same speed as 32bits.
    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32

    // Boolean are coded on 1 bit
    let _t = true;
    let _f: bool = false; // with explicit type annotation

    // addition
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;

    // remainder
    let _remainder = 43 % 5;

    // Character are coded on 4bits
    let _c = 'z';
    let _z = 'ℤ';
    let _heart_eyed_cat = '😻';


    // Compound Types
    // Compound types can group multiple values into one type. Rust has two
    // primitive compound types: tuples and arrays.

    // Tuple
    let _tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);

    let (_x, y, _z) = tup;

    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let _five_hundred = x.0;

    let _six_point_four = x.1;

    let _one = x.2;

    // Array
    // Array size is static. To use dynamic array check out Vector.
    println!("\nArray:");
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    println!("months -> {:?}", months);
    // a = [3, 3, 3, 3, 3];
    let a = [3; 5];
    println!("[3; 5] -> {:?}", a);

    // - i32 is the type of each element in the array
    // - 5 is the size of the array
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    println!("b -> {:?}", b);
}
