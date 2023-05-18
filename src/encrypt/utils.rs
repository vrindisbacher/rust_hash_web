use rand::{distributions::Alphanumeric, Rng};

pub fn get_salt() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect()
}

pub fn string_to_binary(value: String) -> String {
    let mut binary_rep: String = "".to_owned();
    for char in value.as_bytes() {
        binary_rep += &format!("{:b}", char);
    }
    binary_rep
}

pub fn binary_rep_to_u32(value: &String) -> u32 {
    u32::from_str_radix(value, 2).unwrap()
}

pub fn u32_to_binary(x: u32) -> String {
    let mut str_rep = "".to_owned();
    for v in (0..32).rev().map(|n| (x >> n) & 1) {
        str_rep += &v.to_string();
    }
    str_rep
}

pub fn is_multiple_of_512(num: usize) -> bool {
    num % 512 == 0
}
