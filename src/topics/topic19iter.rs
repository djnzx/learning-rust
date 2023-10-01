use std::iter::Map;
use std::slice::Iter;

pub fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for x in &v1 {
        print!("{} ", x)
    }
    println!();

    for x in v1_iter {
        print!("{} ", x)
    }
}

#[test]
fn iterator1() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator2() {
    let v1 = vec![1, 2, 3, 4, 5];

    let mut v1_iter = v1.iter();

    let mut cnt = 0;

    loop {
        match v1_iter.next() {
            Some(_) => {
                cnt += 1;
            }
            None => {
                break;
            }
        }
    }

    assert_eq!(cnt, v1.len());
}

#[test]
fn iterator3() {
    let v1 = vec![1, 2, 3, 4, 5];

    let mut v1_iter = v1.iter();
    let sum: i32 = v1_iter.sum();

    dbg!(sum);
    assert_eq!(sum, 15);
}

#[test]
fn iterator4() {
    let v1 = vec![1, 2, 3, 4, 5];
    let it2: Map<Iter<i32>, fn(&i32) -> i32> = v1.iter().map(|x| x + 1);
    let collected: Vec<i32> = it2.collect();
    // dbg!(collected);
    assert_eq!(collected, vec![2, 3, 4, 5, 6]);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker")
            },
            Shoe {
                size: 10,
                style: String::from("boot")
            },
        ]
    );
}
