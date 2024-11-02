use proto_descriptor_dump::GenMessageDescriptorDump;

fn main() -> anyhow::Result<()> {
    protoc_plugin::gen_code(GenMessageDescriptorDump::default())
}
