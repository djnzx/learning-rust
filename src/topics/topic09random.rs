use rand::prelude::ThreadRng;
use rand::Rng;
use std::ops::{Range, RangeInclusive};

fn playground() {
    /// excluding
    let r1: Range<i32> = 1..100;
    /// including
    let r1: RangeInclusive<i32> = 1..=100;
    /// generator
    let mut tr: ThreadRng = rand::thread_rng();
    ///
    let x = tr.gen_range(r1.clone());
    let y = tr.gen_range(r1);
}
