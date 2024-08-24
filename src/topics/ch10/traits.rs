use std::fmt::{format, Debug, Display};

/// any vector
fn max(xs: &Vec<i32>) -> &i32 {
    let mut max = &xs[0];

    for x in xs {
        if x > max {
            max = x;
        }
    }

    max
}

/// any slice
fn max_i32(xs: &[i32]) -> &i32 {
    let mut max = &xs[0];
    for x in xs {
        if x > max {
            max = x;
        }
    }
    max
}

fn max_char(xs: &[char]) -> &char {
    let mut max = &xs[0];
    for x in xs {
        if x > max {
            max = x;
        }
    }
    max
}

/// PartialOrd is a type class
fn max_gen<T: PartialOrd>(xs: &[T]) -> &T {
    let mut max = &xs[0];
    for x in xs {
        if x.gt(max) {
            max = x;
        }
    }
    max
}

fn playground1() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}

struct Tuple2<A, B> {
    a: A,
    b: B,
}

enum MyOption<A> {
    Some(A),
    None,
}

enum Either<E, A> {
    Right(A),
    Left(E),
}

struct Point<A> {
    x: A,
    y: A,
}

/// getter
impl<A> Point<A> {
    fn x(&self) -> &A {
        &self.x
    }
}

impl Point<f32> {
    /// static
    fn new(x: f32, y: f32) -> Point<f32> {
        Point { x, y }
    }

    /// extension methods here
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

/// just a Java interface or Scala trait
pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author()) // can have default impl
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// NewsArticle implements Summary
impl Summary for NewsArticle {
    // typeof(self) = NewsArticle
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }

    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

// Tweet implements Summary
impl Summary for Tweet {
    // type of (self) = Tweet
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

/// pull default impl
/// looks like a type class, behavior is already attached to the type
impl Summary for u8 {
    fn summarize_author(&self) -> String {
        String::from(format!("@u8:{self}"))
    }
}

#[test]
fn test1() {
    let x = 123u8;
    dbg!(x.summarize_author());
}

/// trait bounds (Scala)
pub fn notify_v2<A: Summary>(item: A) {
    println!("Breaking news! {}", item.summarize());
}

/// trait bounds (Rust)
pub fn notify_v1(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

/// multiple traits
pub fn notify_v3(item: &(impl Summary + Display)) {
    println!("Breaking news! {}", item.summarize());
}

/// multiple traits with bounds
pub fn notify_v4<A: Summary + Display>(item: A) {
    println!("Breaking news! {}", item.summarize());
}

fn function1<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    todo!()
}

/// better syntax ??? hm...
fn function2<T, U>(t: &T, u: &U) -> i32
where
    // haskell-ish
    T: Display + Clone,
    U: Clone + Debug,
{
    todo!()
}

/// return A implements Summary
fn returns_summarizable_v1() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
fn returns_summarizable_v2<A: Summary>() -> A {
    todo!()
}
/// returning type is NOT covariant, invariant !!!
fn returns_summarizable(switch: u8) -> impl Summary {
    match switch {
        1 => NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        },
        // 2 => Tweet {
        //     username: String::from("horse_ebooks"),
        //     content: String::from("of course, as you probably already know, people"),
        //     reply: false,
        //     retweet: false,
        // },
        // 3 => 123u8,
        _ => panic!(),
    }
}

/// TODO: how to print "impl Summary"
#[test]
fn test21() {
    let x = returns_summarizable(1);
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

/// for any T which has Display + PartialOrd
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// syntax to have `ToString` for any type has `Display`
// impl<T: Display> ToString for T {
//     fn to_string(&self) -> String {
//         todo!()
//     }
// }

fn f21() {
    let x = 3.to_string();
}

pub fn playground2() {
    let xs = vec![34, 50, 25, 100, 65];
    let largest = max(&xs);
    println!("The largest number is {}", largest);

    let t1 = Tuple2 { a: 1, b: "hello" };

    let t2: Tuple2<f64, Tuple2<i32, &str>> = Tuple2 { a: 3.14, b: t1 };

    let p1: Point<f32> = Point { x: 1.0, y: 2.0 };
    let x_val = p1.x;
    let x_ref = p1.x();
    let d1 = p1.distance_from_origin();

    let p2 = Point::new(1f32, 2f32);
    let l2 = p2.distance_from_origin();
}

pub fn playground() {
    let tweet: Tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("^^^: {}", 1u8.summarize());
    3.to_string();
}
