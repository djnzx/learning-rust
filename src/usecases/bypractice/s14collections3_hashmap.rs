use std::collections::HashMap;

#[test]
fn code1() {
    // SIMD lookup
    let mut m1 = HashMap::with_capacity(100);
    m1.insert(1, "Jim");

    // get unsafe
    let z = m1[&1];
    // get safe
    let x = m1.get(&1);
    // check
    let y = m1.contains_key(&1);
    // remove
    let removed = m1.remove(&1);

    for (k, v) in &m1 {}
}

#[test]
fn code2() {
    let teams: [(&str, i32); 3] = [("Chinese Team", 100), ("American Team", 10), ("France Team", 50)];
    let mut teams_map1: HashMap<&str, i32> = HashMap::new();
    for (team, score) in &teams {
        teams_map1.insert(team, *score);
    }
    let teams_map2 = teams_map1.clone();
    assert_eq!(teams_map1, teams_map2);

    let teams_map3 = HashMap::from(teams);
    assert_eq!(teams_map1, teams_map3);

    let teams_map4 = teams_map1
        // move here
        .into_iter()
        .collect::<HashMap<&str, i32>>();
    assert_eq!(teams_map3, teams_map4);
}

#[test]
fn or_insert_xxx() {
    // Type inference lets us omit an explicit type signature (which
    // would be HashMap<&str, u8> in this example).
    let mut player_stats = HashMap::new();
    // Insert a key only if it doesn't already exist
    let e = player_stats.entry("health");

    // insert value
    let r = e.or_insert(100);
    assert_eq!(player_stats["health"], 100);

    // insert value as a f(k)
    let e = player_stats
        .entry("health101")
        .or_insert_with_key(|k| k.len());
    assert_eq!(player_stats.get("health101"), Some(&9usize));

    // insert a value by function call
    let e = player_stats
        .entry("health102")
        .or_insert_with(|| 42usize);
    assert_eq!(player_stats.get("health102"), Some(&42usize));
}

#[derive(Hash, Debug, Eq, PartialEq)]
struct Viking {
    name: String,
    country: String,
}

impl Viking {
    fn new(name: &str, country: &str) -> Self {
        Viking {
            name: name.to_string(),
            country: country.to_string(),
        }
    }
}

#[test]
fn code4() {
    // key types must implement Hash
    // f32, f64 do not implement Hash, due to precision
    let mut xs = HashMap::new();
    xs.insert(Viking::new("Jim", "UA"), 33);
    xs.shrink_to_fit()
}

#[test]
fn ability_to_shrink() {
    let mut xs = HashMap::with_capacity(1000); // 1792
    xs.insert(Viking::new("Jim", "UA"), 33);
    println!("{}", xs.capacity());

    xs.shrink_to(500); // 896
    println!("{}", xs.capacity());

    xs.shrink_to_fit(); // 3
    println!("{}", xs.capacity());
}

/// ownership:
///
/// for those who implements `Copy` data is automatically copied
/// for owned, during insert, data moves to HashMap
/// so we need to do `.clone()` or use ref: `&`
#[test]
fn code6() {}
