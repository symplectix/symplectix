//! Build time utilities for rrr.

use std::io::Write;
use std::path::Path;
use std::{
    env,
    fs,
    io,
};

fn comb_table(size: usize) -> Vec<Vec<u64>> {
    let size = size + 1;
    let mut table = vec![vec![0; size]; size];

    #[allow(clippy::needless_range_loop)]
    for i in 0..size {
        table[i][i] = 1; // initialize diagonal
        table[i][0] = 1; // initialize first col
    }

    // number of ways to choose k items from n items without
    // repetition and without order.
    for n in 1..size {
        for k in 1..size {
            table[n][k] = table[n - 1][k - 1] + table[n - 1][k];
        }
    }
    table
}

/// Writes static values definitions e.g., precomputed comb table to OUT_DIR.
///
/// It is a good idea to choose `b + 1` as a power of two,
/// so that the bits for `class` can be fully used (bitpacking).
///  7: 0b_0000_0111
/// 15: 0b_0000_1111
/// 31: 0b_0001_1111
/// 63: 0b_0011_1111
pub fn write_statics(table_type: &str, b: usize, _class_size: u8) -> io::Result<()> {
    let class_size = u8::BITS - (b as u8).leading_zeros();
    let dir = env::var("OUT_DIR").unwrap();
    let mut file = fs::File::create(Path::new(&dir).join(format!("table{b}.rs")))?;
    writeln!(
        file,
        r#"
// Size of the rrr block in bits.
const SIZE: usize = {b};

/// Minimum bits size to represents class value.
pub const CLASS_SIZE: u8 = {class_size};

#[allow(clippy::unreadable_literal)]
static TABLE: {table_type} = {table:#?};
"#,
        // size = b,
        // table_type = table_type,
        table = comb_table(b)
    )
}

/// Creates a rrr module.
#[macro_export]
macro_rules! rrr_mod {
    ($data:ty, $size:expr, $class_size:expr) => {
        /// Minimum bits size to represents class value.
        // pub const CLASS_SIZE: u8 = $class_size;

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
