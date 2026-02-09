#![allow(missing_docs)]

macro_rules! generate_rrr_mod {
    ($data:ty, $size:expr, $class_size:expr) => {
        const SIZE: usize = $size;

        // It is a good idea to choose `size + 1` as a power of two,
        // so that the bits for `class` can be fully used (bitpacking).
        //  7: 0b000111
        // 15: 0b001111
        // 31: 0b011111
        // 63: 0b111111
        /// Minimum bits size to represents class value.
        pub const CLASS_SIZE: u8 = $class_size;

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
