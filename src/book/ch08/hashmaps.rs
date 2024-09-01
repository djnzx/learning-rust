use std::collections::HashMap;

#[test]
fn test1() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let k = String::from("Blue");
    let score = scores.get(&k).copied().unwrap_or(0); // getOrElse

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
    scores.entry(String::from("Red")).or_insert(50);
}

#[test]
fn test2() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    let splitted = text.split_whitespace(); // Lazy ?

    /// counter
    for word in splitted {
        let entry = map.entry(word);
        let count = entry.or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
