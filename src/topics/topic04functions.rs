use std::iter::Enumerate;
use std::slice::Iter;

fn add(a: i8, b: i8) -> i8 {
    a + b
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

/// it returns index of space
fn first_word_idx(s: &String) -> usize {
    let bytes: &[u8] = s.as_bytes(); // ??? copy
    let it: Iter<u8> = bytes.iter();
    let en: Enumerate<Iter<u8>> = it.enumerate(); // withIndex

    for (i, &item) in en {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

/// first word
fn first_word_v2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
