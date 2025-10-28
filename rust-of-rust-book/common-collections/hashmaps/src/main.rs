use std::collections::HashMap;
fn main() {
    println!("Storing Keys with Associated Values in HashMaps!");

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0); // Accessing the score for the Blue team stored in the hashmap
    println!("{score}");

    for (key, value) in &scores { // using a for to loop through key-value pairs in a HashMap
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Purple");

    let mut map = HashMap::new();
    map.insert(field_name, field_value); // The HashMap, map takes ownership of field_name and field_value
    // println!("{field_name} {field_value}"); Invalid
    let mut play_maker = HashMap::new();
    
    play_maker.insert(10, String::from("Coutinho"));
    play_maker.insert(10, String::from("Flo")); // Flo has overwritten Coutinho
    println!("{:?}", play_maker);

    // Adding a Key and Value Only If a Key Isn’t Present
    play_maker.entry(9).or_insert(String::from("Haaland"));
    play_maker.entry(8).or_insert(String::from("Szobo"));

    println!("{:?}", play_maker);

    // Updating a Value Based on the Old Value
    let text = "hello world wonderful world";
    let mut mapping = HashMap::new();
    
    for word in text.split_whitespace() {
            let count = mapping.entry(word).or_insert(0);
            *count += 1;
    }
    println!("{:?}", mapping);
    println!("{:?}", mapping);
}
