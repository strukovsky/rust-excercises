use std::string::String;

pub trait Subscriber<T: Clone> {
    fn handle(&self, action: T);
}

pub struct MessageSubscriber {
    message_placeholder: String,
}

impl MessageSubscriber {
    pub fn new(message_placeholder: String) -> Self {
        Self {
            message_placeholder,
        }
    }

}

impl Subscriber<String> for MessageSubscriber {
    fn handle(&self, action: String) {
        print!("{} ", self.message_placeholder);
        print!("{}", action);
        println!();
    }
}

pub trait Observer<'a, T: Clone> {
    fn notify(&self, action: T);
    fn subscribe(&mut self, subscriber: &'a mut dyn Subscriber<T>);
}

pub struct MessageObserver<'a> {
    subscribers: Vec<&'a mut dyn Subscriber<String>>,
}

impl <'a> MessageObserver<'a> {
    pub fn new() -> Self {
        Self {subscribers: vec![]}
    }
}

impl <'a> Observer<'a, String> for MessageObserver<'a> {

    fn notify(&self, action: String) {
        for subscriber in self.subscribers.iter() {
            subscriber.handle(action.clone());
        }
    }

    fn subscribe(&mut self, subscriber: &'a mut dyn Subscriber<String> ) {
        self.subscribers.push(subscriber);
    }
}
