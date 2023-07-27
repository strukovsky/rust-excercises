use std::{ops::Deref, fmt::Display};
pub struct ExampleBox<T: Display>(T);

impl <T: Display> ExampleBox<T> {
    pub fn new(x: T) -> Self {
        return Self(x);
    }
}

impl <T: Display> Deref for ExampleBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        println!("Dereferencing");
        &self.0
    }
}

impl <T: Display> Drop for ExampleBox<T> {
    fn drop(&mut self) {
        let dropped = &self.0;
        println!("Dropping {}", dropped);
    }
}
