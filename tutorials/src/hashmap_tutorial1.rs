use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 12);
    let capacity = scores.capacity();
    println!("capacity of hashmap {}", capacity);
    let find_key = String::from("Red");
    let result_score = scores.get(&find_key).copied().unwrap_or_default();
    println!("score of team {find_key} is {result_score}");

    //looping through hashmap
    println!("getting HashMap key, values");
    for (key, val) in &scores {
        println!("value of key {key} is {val}");
    }

    //trying to access invalid hashmap owned string
    let field_name = String::from("Favorite color");
    let field_value = String::from("Yellow");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // println!("trying to access invalid values {field_name}");
    map.entry("Favorite color".to_string())
        .or_insert("Red".to_string());
    println!("scores > {:?}", scores);

    let mut for_map = HashMap::new();
    let text = "am I learning rust programming?. Yes I am";
    for word in text.split_whitespace() {
        let count = for_map.entry(word).or_insert(0);
        *count += 1
    }
    println!("formap data: {:?}", for_map);
}
