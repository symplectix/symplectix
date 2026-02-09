fn main() -> std::io::Result<()> {
    rrrbuild::write_statics("[[u8; 5]; 5]", 4, 3)
}
