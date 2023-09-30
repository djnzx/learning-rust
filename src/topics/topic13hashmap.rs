pub fn playground() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let k = String::from("Blue");
    let score = scores
        .get(&k)
        .copied()
        .unwrap_or(0); // getOrElse

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
    scores.entry(String::from("Red"))
        .or_insert(50);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    /// counter
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
