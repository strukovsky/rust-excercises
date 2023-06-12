use std::{fmt::Display, vec};

pub struct BytesStorage<'a> {
    bytes: &'a mut Vec<u8>,
    length: usize,
}

impl<'a> BytesStorage<'a> {
    pub fn new(bytes: &'a mut Vec<u8>) -> Self {
        let length = bytes.len();
        return Self { bytes, length };
    }

    pub fn set(&mut self, bytes: &'a mut Vec<u8>) {
        let length = bytes.len();
        self.bytes = bytes;
        self.length = length;
    }

    pub fn get(&self) -> &'_ Vec<u8> {
        return self.bytes;
    }

    pub fn len(&self) -> usize {
        return self.length;
    }

    pub fn or(&mut self, other: &Self) {
        let other_length = other.length;
        assert!(other_length == self.length);
        let result = &mut self.bytes;
        for (index, byte) in other.bytes.iter().enumerate() {
            result[index] |= byte;
        }
    }

    pub fn and(&mut self, other: &Self) {
        let other_length = other.length;
        assert!(other_length == self.length);
        let result = &mut self.bytes;
        for (index, byte) in other.bytes.iter().enumerate() {
            result[index] &= byte;
        }
    }

    pub fn xor(&mut self, other: &Self) {
        let other_length = other.length;
        assert!(other_length == self.length);
        let result = &mut self.bytes;
        for (index, byte) in other.bytes.iter().enumerate() {
            result[index] ^= byte;
        }
    }
}

impl<'a> Display for BytesStorage<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string: String = String::new();
        self.bytes.iter().for_each(|byte: &u8| {
            if *byte < 17 {
                string.push_str("0");
            }
            string.push_str(format!("{:x}", *byte).as_str());
        });
        write!(f, "{}", string.as_str())
    }
}
