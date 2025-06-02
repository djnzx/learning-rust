fn pack(s: String) -> Vec<(char, usize)> {
    s.chars()
        .fold(Vec::new(), |mut outcome, c| match outcome.last() {
            Some((last, cnt)) if c == *last => {
                outcome
                    .last_mut()
                    .unwrap()
                    .1 = cnt + 1;
                outcome
            }
            _ => {
                outcome.push((c, 1));
                outcome
            }
        })
}

#[test]
fn pack_test() {
    vec![
        ("", vec![]),
        ("a", vec![('a', 1)]),
        ("ab", vec![('a', 1), ('b', 1)]),
        ("abc", vec![('a', 1), ('b', 1), ('c', 1)]),
        ("abbc", vec![('a', 1), ('b', 2), ('c', 1)]),
        ("abbca", vec![('a', 1), ('b', 2), ('c', 1), ('a', 1)]),
    ]
    .iter()
    .for_each(|(unpacked, packed)| {
        let real = pack(unpacked.to_string());
        println!("unpacked: {:?} packed real: {:?} packed expected: {:?}", unpacked, real, packed);
        assert_eq!(real, *packed);
    })
}
