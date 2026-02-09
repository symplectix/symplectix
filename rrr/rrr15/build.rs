fn main() -> std::io::Result<()> {
    rrrbuild::write_statics("[[u16; 16]; 16]", 15, 4)
}
