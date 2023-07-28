use std::cell::RefCell;

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl <'a, T> LimitTracker<'a, T>  where T: Messenger {
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker { messenger, value: 0, max }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max > 1.0 {
            self.messenger.send("You are over your quota");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("You are about to pass over your quota");
        }
    }
}

struct AccumulativeMessenger {
    pub messages: RefCell<Vec<String>>,
}

impl  AccumulativeMessenger {
    pub fn new() -> Self {
        Self {messages: RefCell::new(Vec::new())}
    }
}

impl  Messenger for AccumulativeMessenger {
    fn send(&self, msg: &str) {
        self.messages.borrow_mut().push(String::from(msg));
    }
}

pub fn ref_cell_demo() {
    let messenger = AccumulativeMessenger::new();
    let max = 100;
    let mut limit_tracker = LimitTracker::new(&messenger, max);
    limit_tracker.set_value(110);
    limit_tracker.set_value(120);
    let messages = messenger.messages.borrow();
    assert!(messages.len() == 2);
    let first_message = messages.get(0).unwrap();
    println!("{first_message:?}");
}
