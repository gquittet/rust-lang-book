use crate::util::chapter::Title;

pub fn play() {
    let title = Title {
        chapter: 8,
        name: "Storing list of values with Vectors".to_string(),
    };
    title.print();

    // Create a new Vector
    let _v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3, 4];
    println!("vec![1, 2, 3, 4] -> {:?}", v);

    // Access element
    let third: &i32 = &v[2];
    println!("&v[2] -> {}", third);
    match v.get(2) {
        Some(third) => println!("The third element is: {}", third),
        None => println!("There's no third element."),
    }

    // By default a Vector is immutable and we can't push any data on it
    // The bottom line won't work
    // v.push(2);

    // Iterating of Vector
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    // We can also iterate over mutable references to each element in a mutable
    // vector in order to make changes to all the elements.
    let mut v = vec![100, 32, 57];
    println!("v before for loop -> {:?}", v);
    for i in &mut v {
        *i += 50;
    }
    println!("v after for loop -> {:?}", v);

    // Vector can store only one type. To store multiple types in them, we have
    // to use enum.
    #[derive(Debug)]
    enum SpreedsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreedsheetCell::Int(3),
        SpreedsheetCell::Text("blue".to_string()),
        SpreedsheetCell::Float(10.12),
    ];

    println!("row -> {:?}", row);
}
