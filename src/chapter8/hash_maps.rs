use crate::util::chapter::Title;
use std::collections::HashMap;

pub fn play() {
    let title = Title {
        chapter: 8,
        name: "Storing Keys with Associated Values with Hash Maps".to_string(),
    };
    title.print();

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("scores -> {:?}", scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // Type annotation is needed because it's possible to collect into many
    // different data structures and Rust doesn't know which you want unless you
    // specify.
    let scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("scores is {:?}", scores);

    // You can write the same thing like this.
    let teams2 = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores2 = vec![10, 50];
    let scores2 = teams2
        .into_iter()
        .zip(initial_scores2.into_iter())
        .collect::<HashMap<_, _>>();
    println!("scores2 is {:?}", scores2);

    // Hash Maps and ownerships
    // For types that implement the Copy trait, like i32, the values are copied
    // into the hash map.
    // For owned values like String, the values will be moved and the hash map
    // will be the owner of those values.
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    // Accessing Values in a Hash Map
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("score is {:?}", score);

    // Iterate over Hash Map
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Replace value into Hash Map: Just insert a value for a key
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    // Updating a Value Based on the Old Value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
