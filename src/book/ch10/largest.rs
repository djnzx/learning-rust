fn largest_i32(xs: &[i32]) -> &i32 {
    let mut largest = &xs[0];

    for item in xs {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(xs: &[char]) -> &char {
    let mut largest = &xs[0];

    for item in xs {
        if item > largest {
            largest = item;
        }
    }

    largest
}

/// restriction, implementation required
/// A must have the PartialOrd implementation
fn largest<A: std::cmp::PartialOrd>(xs: &[A]) -> &A {
    let mut largest = &xs[0];

    for item in xs {
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[test]
fn main1() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {result}");
}
