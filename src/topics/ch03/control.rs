#[test]
pub fn sandbox1() {
    let mut counter = 0;

    // loop can return value
    let result = loop {
        counter += 1;

        if counter == 10 {
            // break can return value
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

#[test]
pub fn labels() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

#[test]
fn while1() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("finally {number}!!!");
}

#[test]
fn iteration_indexed() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
}

#[test]
fn iteration_foreach() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

#[test]
fn reverse() {
    // takes no time, just reverse iterator
    let xs = (1..5).rev();
    for x in xs {
        dbg!(x);
    }
}
