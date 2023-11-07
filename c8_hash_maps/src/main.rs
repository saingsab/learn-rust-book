use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Accessing 
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Blue {:?}", score);

    // Looping to access the score
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Updating a Hash Map
    scores.insert(String::from("Blue"), 25);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Blue {:?}", score);

    // Adding a Key and Value Only If a Key Isn’t Present
    /*
    Let’s say we want to check whether the key for the Yellow team has a value associated with it. 
    If it doesn’t, we want to insert the value 50, and the same for the Blue team.
    */
    let mut scores = HashMap::new();
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Yellow")).or_insert(50);

    println!("{:?}",  scores);

    // Updating a Value Based on the Old Value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
