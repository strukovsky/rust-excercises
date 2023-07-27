use std::ops::Deref;
pub struct ExampleBox<T>(T);

impl <T> ExampleBox<T> {
    pub fn new(x: T) -> Self {
        return Self(x);
    }
}

impl <T> Deref for ExampleBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        println!("Dereferencing");
        &self.0
    }
}
