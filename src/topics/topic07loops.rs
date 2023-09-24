use std::cmp::Ordering;

fn playground() {

    loop {
        println!("inside");
        break;
    }

    /// labels
    'there: loop {
        break 'there;
        continue 'there;
    }

    while 1 > 2 {
        println!("yes...")
    }

    let mut x = 0;
    loop {
        println!("{x}");
        x += 1;
        match x.cmp(&5) {
            Ordering::Less => {}
            Ordering::Equal => {println!("Win!") ; break}
            Ordering::Greater => {}
        }
    }

}




