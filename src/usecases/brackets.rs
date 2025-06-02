use std::collections::{HashMap, HashSet};

fn is_valid(s: String) -> bool {
    let open = HashSet::<char>::from_iter("{[(<".chars());
    let pairs = HashMap::from([('>', '<'), (']', '['), (')', '('), ('}', '{')]);

    let mut stack = Vec::new();

    let matches = |op: char, cl: char| match pairs.get(&cl) {
        Some(&op2) => op == op2,
        _ => false,
    };

    for c in s.chars() {
        match c {
            c if open.contains(&c) => stack.push(c),
            c if pairs.contains_key(&c) => match stack.pop() {
                Some(x) if matches(x, c) => {}
                _ => return false,
            },
            _ => continue,
        }
    }

    stack.is_empty()
}

// {} () [] <>
#[test]
fn is_valid_test2() {
    let test_data = [
        //input expected
        ("{}<>", true),
        ("{}{[]}", true),
        ("{{}}", true),
        ("{{()[]<>}{[({{()}[]})]}}", true),
        ("{[}]", false), // TODO
        ("{{}", false),
        ("{}}", false),
        ("}{", false),
        ("{", false),
        ("}", false),
    ];

    for (s, r) in test_data {
        println!("Testing {} {}", s, r);

        assert_eq!(is_valid(s.to_string()), r);
    }
}
