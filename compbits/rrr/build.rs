use std::io::Write;
use std::path::Path;
use std::{
    env,
    fs,
    io,
};

fn gen_comb_table(size: usize) -> Vec<Vec<u64>> {
    let size = size + 1;
    let mut table = vec![vec![0; size]; size];
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

fn write_comb_table<P: AsRef<Path>>(path: P, ty: &str, n: usize) -> io::Result<()> {
    let dir = env::var("OUT_DIR").unwrap();
    let mut file = fs::File::create(Path::new(&dir).join(path))?;
    writeln!(
        file,
        r#"#[allow(clippy::unreadable_literal)]
pub static TABLE: {} = {:#?};
"#,
        ty,
        gen_comb_table(n)
    )
}

fn main() -> io::Result<()> {
    write_comb_table("table4.rs", "[[u8; 5]; 5]", 4)?;
    write_comb_table("table15.rs", "[[u16; 16]; 16]", 15)?;
    write_comb_table("table31.rs", "[[u32; 32]; 32]", 31)?;
    write_comb_table("table63.rs", "[[u64; 64]; 64]", 63)?;
    Ok(())
}
