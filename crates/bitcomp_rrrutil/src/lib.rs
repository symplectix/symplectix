//! Provides helper utilities for rrr implementations.

use std::io::Write;
use std::path::Path;
use std::{
    env,
    fs,
    io,
};

fn comb_table(size: usize) -> Vec<Vec<u64>> {
    let mut table = vec![vec![0; size]; size];

    #[allow(clippy::needless_range_loop)]
    for i in 0..size {
        table[i][i] = 1; // initialize diagonal
        table[i][0] = 1; // initialize first col
    }

    for n in 1..size {
        for k in 1..size {
            table[n][k] = table[n - 1][k - 1] + table[n - 1][k];
        }
    }
    table
}

/// Build time utilities for rrr, which writes static values
/// and functions to OUT_DIR.
///
/// It is a good idea to choose `b + 1` as a power of two,
/// so that the bits for `class` can be fully used (bitpacking).
///  7: 0b_0000_0111
/// 15: 0b_0000_1111
/// 31: 0b_0001_1111
/// 63: 0b_0011_1111
pub fn write_mod(item_type: &str, b: usize) -> io::Result<()> {
    env::var_os("OUT_DIR").ok_or(io::Error::other("OUT_DIR not found")).and_then(|out_dir| {
        let table = comb_table(b + 1);
        let table_len = table.len();
        let class_size = u8::BITS - (b as u8).leading_zeros();
        let mut file = fs::File::create(Path::new(&out_dir).join(format!("rrr{b}.rs")))?;
        writeln!(
            file,
            r#"
// Size of the rrr block in bits.
const SIZE: usize = {b};

/// Minimum bits size to represents class value.
pub const CLASS_SIZE: u8 = {class_size};

/// Encodes data into a pair of `class` and `offset`.
pub fn encode(data: {item_type}) -> (u8, {item_type}) {{
    bitcomp_rrrutil::encode!(data)
}}

/// Decodes data from a pair of `class` and `offset`.
pub fn decode(class: u8, offset: {item_type}) -> {item_type} {{
    bitcomp_rrrutil::decode!(class, offset)
}}

#[allow(clippy::unreadable_literal)]
static COMB: [[{item_type}; {table_len}]; {table_len}] = {table:?};
"#,
        )
    })
}

// NOTE: COMB[n][k] is the number of ways to choose k items
// from n items without repetition and without order.

/// Encode using a static table.
#[macro_export]
macro_rules! encode {
    ($data:expr) => {{
        let data = $data & ((1 << SIZE) - 1);

        let class = data.count_ones();
        let offset = {
            let mut b = SIZE;
            let mut c = class as usize;
            let mut o = 0;

            while b > 0 && c > 0 {
                b -= 1;
                if data & (1 << b) != 0 {
                    o += COMB[b][c];
                    c -= 1;
                }
            }
            o
        };
        (class as u8, offset)
    }};
}

/// Decode using a static table.
#[macro_export]
macro_rules! decode {
    ($class:expr, $offset:expr) => {{
        let mut data = 0;
        let mut b = SIZE;
        let mut c = $class as usize;
        let mut o = $offset;

        while b > 0 && c > 0 {
            b -= 1;
            if o >= COMB[b][c] {
                data |= 1 << b;
                o -= COMB[b][c];
                c -= 1;
            }
        }
        data
    }};
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::{
        env,
        fs,
    };

    use serde::{
        Deserialize,
        Serialize,
    };

    use super::comb_table;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct Comb {
        table: Vec<Vec<u64>>,
    }

    // Tests that comb_table correctly constructs the binomial coefficient table.
    // Verifies that the generated table is equivalent to another table produced by
    // `comb.py`.
    fn eq_to_comb_py(n: usize) {
        let testdata = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/testdata"));
        let path = testdata.join(format!("comb_table_{n}.json"));
        let json_file =
            fs::OpenOptions::new().read(true).open(path).expect("failed to open a json file");
        let buf = std::io::BufReader::new(json_file);
        let comb: Comb = serde_json::from_reader(buf).expect("failed to deserialize a table");
        assert_eq!(comb.table, comb_table(n));
    }

    #[test]
    fn comb_table_16() {
        eq_to_comb_py(16);
    }

    #[test]
    fn comb_table_32() {
        eq_to_comb_py(32);
    }
}
