use std::iter::Enumerate;
use std::slice::Iter;

// https://doc.rust-lang.org/book/ch04-03-slices.html

#[test]
fn syntax() {
    let s = String::from("hello world");

    let hello1 = &s[0..5]; // not including the last
    let hello2 = &s[..5]; // zero can be omitted
    let world1 = &s[6..11];
    let world2 = &s[6..]; // if till end - second can be omitted
    let entire = &s[..]; // slices are always pointer
    dbg!(hello1);
    dbg!(hello2);
    dbg!(world1);
    dbg!(world2);
}

// slice doesn't have ownership

/// it returns index of space
fn first_space_idx(s: &String) -> usize {
    let bytes: &[u8] = s.as_bytes(); // ??? copy
    let it: Iter<u8> = bytes.iter();
    let en: Enumerate<Iter<u8>> = it.enumerate(); // withIndex

    for (i, &item) in en {
        if item == b' ' {
            // by the way, index is a bad idea, since we need a word, but string can be invalid
            return i;
        }
    }

    s.len()
}

#[test]
fn test1() {
    let s = String::from("qwe asd zxc");
    let x = first_space_idx(&s);
    dbg!(x);
}

// &String - allows to use String only
// &str - allows to use String AND str

/// first word
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    let it = bytes.iter();
    let en = it.enumerate();

    for (idx, &itm) in en {
        if itm == b' ' {
            let slice = &s[0..idx];
            return slice;
        }
    }

    &s[..]
}

fn first_word_better(s: &str) -> &str {
    let bytes = s.as_bytes();

    let it = bytes.iter();
    let en = it.enumerate();

    for (idx, &itm) in en {
        if itm == b' ' {
            let slice = &s[0..idx];
            return slice;
        }
    }

    &s[..]
}

#[test]
fn test2() {
    let mut s = String::from("qwe asd zxc");
    let x = first_word(&s);
    dbg!(x); // qwe
    s.clear();
    // no more x here since x is a part
    // dbg!(x); // qwe
}

#[test]
fn test3() {
    let x = first_word_better("qwe asd zxc");
    dbg!(x); // qwe
}

#[test]
fn array_slice() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}
