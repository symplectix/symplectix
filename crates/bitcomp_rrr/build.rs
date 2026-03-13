//! Build script for RRR.

fn main() -> std::io::Result<()> {
    bitcomp_rrrutil::write_mod("u8", 4)?;
    bitcomp_rrrutil::write_mod("u16", 15)?;
    bitcomp_rrrutil::write_mod("u32", 31)?;
    bitcomp_rrrutil::write_mod("u64", 63)?;
    Ok(())
}
