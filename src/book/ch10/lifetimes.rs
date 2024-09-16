/// https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html
///
/// &i32        // a reference
/// &'a i32     // a reference with an explicit lifetime
/// &'a mut i32 // a mutable reference with an explicit lifetime
///
/// - only for references
/// - there are patterns - lifetimes can be inferred
///

/// prevent te issues
/// will not compile
#[test]
fn dangling1() {
    // let r;
    //
    // {
    //     let x = 5;
    //     // r = &x;
    // }
    //
    // println!("r: {r}");
}
#[test]
fn dangling_fixed() {
    let x = 5;
    let r = &x;
    println!("r: {r}");
}

/// since we don't know which is returned, we need to keep both
/// and mark them with the same lifetime
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    //     --      --          --          --
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[test]
fn test3() {
    let str1 = String::from("abcd");
    let str2 = "xyz";

    // reference syntax1
    let result = longest(&str1, str2);
    // reference syntax2
    let result = longest(str1.as_str(), str2);

    println!("1st was {}", str1);
    println!("2nd was {}", str2);
    println!("The longest string is {}", result);
}

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
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

/// without lifetime it will not compile
struct Excerpt<'a> {
    part: &'a str,
}

#[test]
fn test4() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let ex = Excerpt { part: first_sentence };
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
pub fn playground() {
    /// lifetime is FOREVER
    let s: &'static str = "I have a static lifetime.";
}
