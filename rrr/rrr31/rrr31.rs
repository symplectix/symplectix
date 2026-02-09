#![allow(missing_docs)]

macro_rules! generate_rrr_mod {
    ($data:ty, $size:expr, $class_size:expr) => {
        pub fn encode(mut data: $data) -> (u32, $data) {
            data &= (1 << SIZE) - 1;

            let class = data.count_ones();
            let offset = {
                let mut c = class as usize;
                let mut o = 0;
                let mut j = 1;

                while 0 < c && c <= SIZE - j {
                    if data & (1 << (SIZE - j)) != 0 {
                        o += TABLE[SIZE - j][c];
                        c -= 1;
                    }
                    j += 1;
                }
                o
            };
            (class, offset)
        }

        pub fn decode(class: u32, offset: $data) -> $data {
            let mut data = 0;
            let mut c = class as usize;
            let mut o = offset;
            let mut j = 1usize;

            while c > 0 {
                if o >= TABLE[SIZE - j][c] {
                    data |= 1 << (SIZE - j);
                    o -= TABLE[SIZE - j][c];
                    c -= 1;
                }
                j += 1;
            }
            data
        }
    };
}

mod imp {
    include!(concat!(env!("OUT_DIR"), "/table31.rs"));
    generate_rrr_mod!(u32, 31usize, 5);
}

pub use imp::{
    CLASS_SIZE,
    decode,
    encode,
};
