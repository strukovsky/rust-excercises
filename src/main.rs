mod algorithms;
mod structures;
mod patterns;

use patterns::observer::{MessageObserver, MessageSubscriber, Observer, Subscriber};
fn main() {
   let mut observer = MessageObserver::new();
   let first_subscriber = &mut MessageSubscriber::new(String::from("First subscriber received message")) as &mut dyn Subscriber<String>;
   observer.subscribe(first_subscriber);
   let second_subscriber = &mut MessageSubscriber::new(String::from("Second subscriber received message")) as &mut dyn Subscriber<String>;
   observer.subscribe(second_subscriber);
   observer.notify(String::from("Action"));
}
