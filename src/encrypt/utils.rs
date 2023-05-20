use core::fmt;

use byteorder::{BigEndian, ReadBytesExt};
use rand::{distributions::Alphanumeric, Rng};

#[derive(Debug, Clone)]
pub struct ValuePassedMismatch; 


impl fmt::Display for ValuePassedMismatch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value passed does not match the encryped value")
    }
}


pub fn get_salt() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect()
}

pub fn string_to_binary(value: &String) -> Vec<u8> {
    let mut binary_rep: Vec<u8> = vec![];
    for char in value.as_bytes() {
        let bit_string = format!("{:b}", char);
        for bit in bit_string.chars() {
            binary_rep.push(bit as u8 - 0x30);
        }
    }
    binary_rep
}

pub fn binary_rep_to_u32(mut value: &[u8]) -> u32 {
    value.read_u32::<BigEndian>().unwrap()
}

pub fn u32_to_binary(x: u32) -> [u8; 32] {
    let mut bin_rep = [0; 32];
    for i in 0..32 {
        bin_rep[i] = u8::try_from(x >> i & 1).unwrap();
    }
    bin_rep
}

pub fn is_multiple_of_512(num: usize) -> bool {
    num % 512 == 0
}
