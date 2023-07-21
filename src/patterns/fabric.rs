use std::string::String;

pub struct Cargo {
    name: String,
    weight_kg: u32,
}

impl Cargo {
    pub fn new(name: String, weight_kg: u32) -> Self {
        Self { name, weight_kg }
    }
}

pub trait Transport {
    fn deliver(&self, cargo: &Cargo);
}

pub struct Taxi {
    driver: String,
}

impl Transport for Taxi {
    fn deliver(&self, cargo: &Cargo) {
        println!(
            "Taxi driver {} delivered cargo {} of {} kg",
            self.driver, cargo.name, cargo.weight_kg
        );
    }
}

pub struct Bus {
    serial: String,
}

impl Transport for Bus {
    fn deliver(&self, cargo: &Cargo) {
        println!(
            "Bus {} delivered cargo {} of {} kg",
            self.serial, cargo.name, cargo.weight_kg
        );
    }
}

pub struct TransportFactory {}

impl TransportFactory {
    pub fn create(
        transport_type: &str,
        identifier: &str,
    ) -> Result<Box<dyn Transport>, BadTransportTypeError> {
        match transport_type {
            "Taxi" => Ok(Box::new(Taxi { driver: String::from(identifier) })),
            "Bus" => Ok(Box::new(Bus { serial: String::from(identifier) })),
            _ => Err(BadTransportTypeError {}),
        }
    }
}

#[derive(Debug)]
pub struct BadTransportTypeError {}
