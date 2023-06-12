mod algorithms;
mod structures;

use structures::bytes_storage::BytesStorage;

fn main() {
    let mut bytes1 = &mut vec![192, 255, 0, 1];
    let mut bytes2 = vec![255, 255, 255, 255];
    let mut bytes3 = vec![0, 0, 0, 0];
    let mut a = BytesStorage::new(bytes1);
    let b = BytesStorage::new(&mut bytes2);
    let c = BytesStorage::new(&mut bytes3);
    println!("{}", a);
    a.or(&b);
    println!("{}", a);
    a.and(&c);
    println!("{}", a);
    let intermediate_value = &mut vec![255, 255, 255, 255];
    a.set(intermediate_value);
    println!("{}", a);
    a.xor(&b);
    println!("{}", a);
}
