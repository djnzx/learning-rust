use std::cell::RefCell;
use std::rc::Rc;

// Box<A>     - invariants are enforced at compile time
// RefCell<T> - invariants are enforced at runtime.
// the most famous example is the Halting Problem, which is beyond the scope of this book but is an interesting topic to research.
// Similar to Rc<T>, RefCell<T> is only for use in single-threaded scenarios and will give you a compile-time error if you try using it in a multithreaded context.
// Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
// Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only immutable borrows checked at compile time; RefCell<T> allows immutable or mutable borrows checked at runtime.
// Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.
#[test]
fn code1() {
    let x = 5;
    // will not compile
    // let y = &mut x;
}

/// behavior contract
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        let msg = match percentage_of_max {
            x if x >= 1.0 => Some("Error: You are over your quota!"),
            x if x >= 0.9 => Some("Urgent warning: You've used up over 90% of your quota!"),
            x if x >= 0.75 => Some("Warning: You've used up over 75% of your quota!"),
            _ => None,
        };

        // actually we can't check that functionality
        msg.iter().for_each(|msg| self.messenger.send(msg));

        // if percentage_of_max >= 1.0 {
        //     self.messenger.send("Error: You are over your quota!");
        // } else if percentage_of_max >= 0.9 {
        //     self.messenger
        //         .send("Urgent warning: You've used up over 90% of your quota!");
        // } else if percentage_of_max >= 0.75 {
        //     self.messenger
        //         .send("Warning: You've used up over 75% of your quota!");
        // }
    }
}

#[cfg(test)]
mod test_design {
    use super::*;
    use std::cell::RefCell;

    /// structure to collect messages sent
    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    /// constructor with empty vector
    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // will not work
            // self.sent_messages.push(String::from(message));

            // explicit borrow works well
            self.sent_messages.borrow_mut().push(String::from(message));

            // will panic, since we try to mutate twice
            // let mut one_borrow = self.sent_messages.borrow_mut();
            // let mut two_borrow = self.sent_messages.borrow_mut(); // panic here
            //
            // one_borrow.push(String::from(message));
            // two_borrow.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
        assert_eq!(
            mock_messenger.sent_messages.borrow()[0], //.iter().next().unwrap(),
            "Warning: You've used up over 75% of your quota!"
        );
    }
}
