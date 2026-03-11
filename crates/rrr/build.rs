fn main() -> std::io::Result<()> {
    rrrutil::write_mod("u8", 4)?;
    rrrutil::write_mod("u16", 15)?;
    rrrutil::write_mod("u32", 31)?;
    rrrutil::write_mod("u64", 63)?;
    Ok(())
}
