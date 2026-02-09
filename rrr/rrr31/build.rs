fn main() -> std::io::Result<()> {
    rrrbuild::write_statics("[[u32; 32]; 32]", 31, 5)
}
