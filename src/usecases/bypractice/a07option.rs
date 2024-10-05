use num::traits::NumOps;
use num::Num;
use std::ops::Add;

fn add1<A: Num + Add<Output = A>>(x: Option<A>) -> Option<A> {
    x.map(|x| x + A::one())
}

#[test]
fn test1() {
    let five = Some(5);
    let six = add1(five);
    let none = add1::<u8>(None);
}
