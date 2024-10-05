/// if/else
/// match
/// for/while/loop
/// continue/break

// 4h24min
#[test]
fn if_basic_syntax() {
    let n: i32 = 5;

    if n < 0 {
        println!("{} is negative", n);
    } else {
        println!("{} is non-negative", n);
    }
}

#[test]
fn if_stacked_syntax() {
    let n: i32 = 5;

    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }
}

#[test]
fn if_returns_syntax() {
    let n: i32 = 5;

    let s = if n < 0 {
        "negative"
    } else if n > 0 {
        "positive"
    } else {
        "zero"
    };
}

// TODO: boolean logic should be explainedx here
// OR, AND, NOT, XOR, EQU, IMPL

#[test]
fn for_basic_not_including_syntax() {
    for n in 0..5 {
        println!("{}", n);
    }
}

#[test]
fn for_basic_including_syntax() {
    for n in 0..=5 {
        println!("{}", n);
    }
}

/// FOR CONSUMES, for those who don't implement Copy

#[test]
fn while_basic_syntax() {
    let mut n = 1;
    while n < 20 {
        if n % 15 == 0 {
            println!("{}: fizzbuzz", n);
        } else if n % 3 == 0 {
            println!("{}: fizz", n);
        } else if n % 5 == 0 {
            println!("{}: buzz", n);
        } else {
            println!("{}", n);
        }
        n += 1;
    }
}

#[test]
fn break_syntax() {
    let mut n: i32 = 0;
    for i in 0..100 {
        n = i;
        if i == 66 {
            break;
        }
    }
    assert_eq!(66, n);
}

#[test]
fn continue_syntax() {
    let mut n: i32 = 0;
    for i in 0..100 {
        n = n + 1; // 100 times
        if i % 2 == 0 {
            continue;
        }

        n = n + 1; // 50 times
    }
    assert_eq!(150, n);
}

#[test]
fn loop_syntax() {
    let mut n = 0u32;
    loop {
        n += 1;
        println!("{}", n);
        if n == 10 {
            break;
        }
    }
}

#[test]
fn loop_syntax_with_labels() {
    let mut n = 0u32;

    'a: loop {
        n += 1;

        println!("b");

        break;
        continue 'a;
    }
}

#[test]
fn loop_returns() {
    let mut x = 0;

    let outcome = loop {
        //
        x += 1;
        //
        if x == 10 {
            break 33;
        }
        //
    };

    assert_eq!(33, outcome);
}

#[test]
fn nested_loops() {
    let mut x = 0;
    let mut y = 0;
    loop {
        loop {
            x += 1;
            y += 1;
            print!("{} ", x);
            if x == 5 {
                break;
            }
        }
        loop {
            x -= 1;
            y += 1;
            print!("{} ", x);
            if x == 0 {
                break;
            }
        }
        if y >= 20 {
            break;
        }
    }
}

#[test]
fn test4() {}
