// https://doc.rust-lang.org/book/ch17-03-oo-design-patterns.html

/// we can treat state as a trait
/// with many implementations
/// but all of them have similar methods
/// but with different implementations
/// actually, state machine
/// 1. it must work with subtypes => dyn State
/// 2. since we don't know about state in compile time => Box<dyn State>

pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost { content: String::new() }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    /// mutates
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    /// it doesn't mutate, it returns a new type
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost { content: self.content }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post { content: self.content }
    }
}
#[test]
fn code1() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    // there is no .content() on the Draft post
    //assert_eq!("", post.content());

    let post = post.request_review();
    // there is no .content() on the Pending post
    //assert_eq!("", post.content());

    let post = post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
