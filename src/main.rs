mod algorithms;
mod structures;
mod patterns;

use patterns::fabric::{Cargo, Transport, TransportFactory};
fn main() {
   let candies_crate = Cargo::new(String::from("Candies crate"), 10);
   let taxi: Result<Box<dyn Transport>, patterns::fabric::BadTransportTypeError> = TransportFactory::create("Taxi", "Eugene");
   let bus = TransportFactory::create("Bus", "A228");
   let transports: Vec<Box<dyn Transport>> = vec![bus.unwrap(), taxi.unwrap()];
   for transport in transports.iter() {
        transport.deliver(&candies_crate);
   }
}
