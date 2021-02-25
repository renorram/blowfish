use crate::blowfish_data::{P_ARRAY, S_ARRAY};
use std::convert::TryInto;
use std::str;

mod blowfish_data;

const N: usize = 16;

struct BlowFish {
    p: [u32; 18],
    s: [[u32; 256]; 4],
}

impl BlowFish {
    fn f(&self, value: u32) -> u32 {
        let mut x_value = value.clone();

        let d: u8 = x_value as u8;
        x_value >>= 8;

        let c: u8 = x_value as u8;
        x_value >>= 8;

        let b: u8 = x_value as u8;
        x_value >>= 8;

        let a: u8 = x_value as u8;

        // let h: u32 = self.s[0][a as usize] + self.s[1][b as usize];
        let h = self.s[0][a as usize].wrapping_add(self.s[1][b as usize]);

        // return (h ^ self.s[2][c as usize]) + self.s[3][d as usize];
        (h ^ self.s[2][c as usize]).wrapping_add(self.s[3][d as usize])
    }

    fn new(key: String) -> BlowFish {
        let mut bl = BlowFish {
            p: P_ARRAY.clone(),
            s: S_ARRAY.clone(),
        };

        let len = key.len();
        let key_chars: Vec<u32> = key.chars().map(|c| c as u32).collect();

        for i in 0..18 {
            bl.p[i] = P_ARRAY[i] ^ key_chars[i % len];
        }

        bl
    }

    fn encrypt(&self, text: String) -> String {
        let (text_input, _) = text.split_at(8);
        let (left_str, right_str) = text_input.split_at(4);
        let mut right: u32 = u32::from_ne_bytes(right_str.as_bytes().try_into().unwrap());
        let mut left: u32 = u32::from_ne_bytes(left_str.as_bytes().try_into().unwrap());
        let mut aux: u32;

        for i in 0..N {
            left ^= self.p[i];
            right ^= self.f(left);
            aux = left;
            left = right;
            right = aux;
        }

        // undo last swap
        aux = left;
        left = right;
        right = aux;

        right ^= self.p[16];
        left ^= self.p[17];

        let left_bytes: Vec<u8> = left.to_ne_bytes().to_vec();
        let right_bytes: Vec<u8> = right.to_ne_bytes().to_vec();
        let b = [left_bytes, right_bytes].concat();

        str::from_utf8(&b).unwrap().to_string()
    }
}

fn main() {
    let blow_fish = BlowFish::new(String::from("Test Key"));
    println!("Encription result: {}", blow_fish.encrypt(String::from("The encryption consists")));
}

// #[cfg(test)]
// mod test {
//
//     #[test]
//     fn test_f_function() {
//
//     }
// }
