/// macros 3 types:
/// - Custom #[derive] macros that specify code added with the derive attribute used on structs and enums
/// - Attribute-like macros that define custom attributes usable on any item
/// - Function-like macros that look like function calls but operate on the tokens specified as their argument
//https://doc.rust-lang.org/reference/macros-by-example.html
#[macro_export]
macro_rules! vec123 {
    //|------------ declare variable
    //|             zero or more
    //|             |
    //V             V
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
#[test]
fn code1() {
    let v: Vec<u32> = vec123![1, 2, 3];
    dbg!(v);
}

#[test]
fn code3() {}

#[test]
fn code4() {}

#[test]
fn code5() {}

#[test]
fn code6() {}
