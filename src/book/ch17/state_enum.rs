#[derive(Clone, PartialEq, Debug)]
enum State {
    Draft,
    PendingReview,
    Approved,
}

impl State {
    fn new() -> State {
        State::Draft
    }

    fn request_review(&self) -> State {
        match self {
            State::Draft => State::PendingReview,
            s => s.clone(),
        }
    }

    fn approve(&self) -> State {
        match self {
            State::PendingReview => State::Approved,
            s => s.clone(),
        }
    }
}

pub struct Post {
    state: State,
    content: String,
}

impl Post {
    /// static constructor
    pub fn new() -> Post {
        Post {
            state: State::new(),
            content: String::new(),
        }
    }

    /// accessor
    pub fn state(&self) -> &State {
        &self.state
    }

    /// accessor based on the state value
    pub fn content(&self) -> Option<&str> {
        match self.state {
            State::Approved => Some(self.content.as_str()),
            _ => None,
        }
    }

    /// mutator
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    /// change state, but indirectly
    pub fn request_review(&mut self) {
        self.state = self.state.request_review();
    }

    /// change state, but indirectly
    pub fn approve(&mut self) {
        self.state = self.state.approve();
    }
}

#[test]
fn test1() {
    let msg = "I ate a salad for lunch today";
    let mut post = Post::new();
    assert_eq!(None, post.content());
    assert_eq!(&State::Draft, post.state());

    post.add_text(msg.as_ref());
    assert_eq!(None, post.content()); // we hide text until it's approved
    assert_eq!(&State::Draft, post.state());

    post.request_review();
    assert_eq!(None, post.content()); // we hide text until it's approved
    assert_eq!(&State::PendingReview, post.state());

    post.approve();
    assert_eq!(Some(msg), post.content());
    assert_eq!(&State::Approved, post.state());
}
