///
/// &i32        // a reference
/// &'a i32     // a reference with an explicit lifetime
/// &'a mut i32 // a mutable reference with an explicit lifetime
///
/// - only for references
/// - there are patterns - lifetimes can be inferred
///
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
    where
        T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
trait ImportantExcerpt<'a> {
    fn part(&self) -> &str {
        "s"
    }
}

impl<'a> dyn ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

impl<'a> dyn ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part()
    }
}
/// to prevent dangling references
pub fn playground() {
    let r;

    {
        let x = 5;
        r = &x;
    }
    // `x` does not live long enough
    // `x` is disappeared, so `r` - also
    // println!("r: {}", r);


    let r;
    let x = 5;
    r = &x;
    println!("r: {}", r);

    let string1 = String::from("abcd");
    let str1 = string1.as_str();
    let str2 = "xyz";

    let result = longest(str1, str2);
    println!("The longest string is {}", result);
    // println!("1st was {}", string1);
    println!("1st was {}", str1);
    println!("2nd was {}", str2);

    /// lifetime is FOREVER
    let s: &'static str = "I have a static lifetime.";
}
