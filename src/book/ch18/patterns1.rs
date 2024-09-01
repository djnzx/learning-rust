fn inc1(oa: Option<i32>) -> Option<i32> {
    match oa {
        Some(x) => Some(x + 1),
        None => None,
    }
}

fn inc2(oa: Option<i32>) -> Option<i32> {
    match oa {
        Some(x) => Some(x + 1),
        _ => None,
    }
}

#[test]
fn if_let() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age = "34".parse::<u8>();

    // just an one arm for pattern matching, ignoring else
    // --------------------
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

#[test]
fn while_let() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{top}");
    }
}

#[test]
fn for_let() {
    let v = vec!['a', 'b', 'c'];
    let indexed = v.iter().enumerate();

    for (index, value) in indexed {
        println!("{value} is at index {index}");
    }
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({x}, {y})");
}

#[test]
fn patterns() {
    let (x, y, z) = (1, 2., true);
    let (x, y, _) = (1, 2., true);
    let point = (3, 5);
}
