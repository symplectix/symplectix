use prost_reflect::FileDescriptor;
use prost_types::compiler::code_generator_response as codegen_response;

fn main() -> anyhow::Result<()> {
    protoc_plugin::run(MessageDescriptorDumpGenerator::default())
}

#[derive(Debug, Default, Clone)]
struct MessageDescriptorDumpGenerator {}

impl protoc_plugin::FileGenerator for MessageDescriptorDumpGenerator {
    fn gen_file(
        &self,
        target_proto: &str,
        file_desc: &FileDescriptor,
    ) -> Result<codegen_response::File, String> {
        let file_name = {
            let stem = target_proto
                .strip_suffix(".proto")
                .ok_or_else(|| format!("unexpected proto '{}'", target_proto))?;
            format!("{}.message_descriptor_dump", stem)
        };

        let mut buf = String::with_capacity(1 << 10);
        for msg_desc in file_desc.messages() {
            buf.push_str(&format!("{:#?}\n", msg_desc));
        }

        Ok(codegen_response::File {
            name: Some(file_name),
            content: Some(buf),
            ..Default::default()
        })
    }
}
