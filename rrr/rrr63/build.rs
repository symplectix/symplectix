fn main() -> std::io::Result<()> {
    rrrbuild::write_statics("[[u64; 64]; 64]", 63, 6)
}
