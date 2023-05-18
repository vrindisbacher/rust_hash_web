use std::str::Chars;

use itertools::{self, Chunk, Itertools};

use utils::{binary_rep_to_u32, get_salt, is_multiple_of_512, string_to_binary, u32_to_binary};
mod utils;

fn pad(mut value: String) -> String {
    value.push_str("1");
    let original_size: usize = value.len();

    // start by padding until we have a multiple of 512 (less 64 bits)
    while !(is_multiple_of_512(value.len() + 64)) {
        value.push_str("0");
    }

    // now add the last 64 bits by using big endian rep of value size
    let big_endian_rep_of_orig_size = format!("{original_size:b}");
    let num_to_pad = 64 - big_endian_rep_of_orig_size.len();

    for _ in 0..num_to_pad {
        value.push_str("0");
    }
    value.push_str(&big_endian_rep_of_orig_size);

    value
}

fn array_of_32_bit_words_from_chunk(chunk: Chunk<Chars>) -> Vec<String> {
    // todo - make this not on the heap since it is known size
    let mut array = vec![];
    let mut curr_word = "".to_owned();
    for char in chunk {
        curr_word += &char.to_string();
        if curr_word.len() == 32 {
            array.push(curr_word);
            curr_word = "".to_owned();
        }
    }
    // add 48 more 32 bit words
    let mut s = String::with_capacity(32);
    for _ in [..48 * 32] {
        s += "0";
        if s.len() == 32 {
            array.push(s.to_owned());
            s.clear();
        }
    }
    array
}

fn hash(value: String) -> String {
    let mut h0: u32 = 0x6a09e667;
    let mut h1: u32 = 0xbb67ae85;
    let mut h2: u32 = 0x3c6ef372;
    let mut h3: u32 = 0xa54ff53a;
    let mut h4: u32 = 0x510e527f;
    let mut h5: u32 = 0x9b05688c;
    let mut h6: u32 = 0x1f83d9ab;
    let mut h7: u32 = 0x5be0cd19;

    let k: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];

    for chunk in &value.chars().chunks(512) {
        let mut a = h0;
        let mut b = h1;
        let mut c = h2;
        let mut d = h3;
        let mut e = h4;
        let mut f = h5;
        let mut g = h6;
        let mut h = h7;

        let mut array_of_words = array_of_32_bit_words_from_chunk(chunk);

        for i in 16..64 {
            let temp1 = binary_rep_to_u32(&array_of_words[i - 15]);
            let s0 = (temp1.rotate_right(7)) ^ (temp1.rotate_right(18)) ^ (temp1 >> 3);
            let temp2 = binary_rep_to_u32(&array_of_words[i - 2]);
            let s1 = (temp2.rotate_right(17)) ^ (temp2.rotate_right(19)) ^ (temp2 >> 10);
            let temp3 = binary_rep_to_u32(&array_of_words[i - 16]);
            let temp4 = binary_rep_to_u32(&array_of_words[i - 7]);
            array_of_words.push(u32_to_binary(
                temp3.wrapping_add(s0).wrapping_add(temp4).wrapping_add(s1),
            ));
        }

        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ (!e & g);
            let temp1 = h
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(k[i])
                .wrapping_add(binary_rep_to_u32(&array_of_words[i]));
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);

            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }
        h0 = h0.wrapping_add(a);
        h1 = h1.wrapping_add(b);
        h2 = h2.wrapping_add(c);
        h3 = h3.wrapping_add(d);
        h4 = h4.wrapping_add(e);
        h5 = h5.wrapping_add(f);
        h6 = h6.wrapping_add(g);
        h7 = h7.wrapping_add(h);
    }

    format!(
        "{:08X?}{:08X?}{:08X?}{:08X?}{:08X?}{:08X?}{:08X?}{:08X?}",
        h0, h1, h2, h3, h4, h5, h6, h7
    )
    .to_ascii_lowercase()
}

pub struct HashResult {
    pub salt: String, 
    pub hash: String,
}

pub fn sha_256(value: &String) -> HashResult {
    let mut salt = get_salt();
    salt.push_str(value);
    let owned_salt = salt.to_owned();
    let binary_rep = string_to_binary(salt);
    let padded_binary_rep = pad(binary_rep);
    HashResult {
        salt : owned_salt, 
        hash: hash(padded_binary_rep)
    }
}
