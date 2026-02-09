use std::io::Write;
use std::path::Path;
use std::{
    env,
    fs,
    io,
};

fn gen_comb_table(size: usize) -> Vec<Vec<u128>> {
    let size = size + 1;
    let mut table = vec![vec![0u128; size]; size];
    for k in 0..size {
        table[k][k] = 1; // initialize diagonal
        table[0][k] = 0; // initialize first row
        table[k][0] = 1; // initialize first col
    }
    // number of ways to choose k items from n items without
    // repetition and without order.
    // i: n
    // j: k
    for i in 1..size {
        for j in 1..size {
            table[i][j] = table[i - 1][j - 1] + table[i - 1][j];
        }
    }
    table
}

fn write_comb_table<P: AsRef<Path>>(path: P, ty: &str, n: usize) -> io::Result<()> {
    let dir = env::var("OUT_DIR").unwrap();
    let mut file = fs::File::create(Path::new(&dir).join(path))?;
    writeln!(
        file,
        r#"#[cfg_attr(feature = "cargo-clippy", allow(unreadable_literal))]
pub static TABLE: {} = {:#?};
"#,
        ty,
        gen_comb_table(n)
    )
}

fn main() -> io::Result<()> {
    write_comb_table("table15.rs", "[[u16; 16]; 16]", 15)?;
    write_comb_table("table31.rs", "[[u32; 32]; 32]", 31)?;
    write_comb_table("table63.rs", "[[u64; 64]; 64]", 63)?;
    Ok(())
}
