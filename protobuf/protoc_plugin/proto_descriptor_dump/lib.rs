use prost_reflect::FileDescriptor;
use prost_types::compiler::code_generator_response::File;

/// An example protobuf compiler plugin which dumps message descriptor.
#[derive(Debug, Default, Clone)]
pub struct GenMessageDescriptorDump {}

impl protoc_plugin::GenFile for GenMessageDescriptorDump {
    fn gen_file(&self, target_proto: &str, fd: &FileDescriptor) -> Result<File, String> {
        let file_name = {
            let stem = target_proto
                .strip_suffix(".proto")
                .ok_or_else(|| format!("unexpected proto '{}'", target_proto))?;
            format!("{}.message_descriptor_dump", stem)
        };

        let mut buf = String::with_capacity(1 << 10);
        for msg_desc in fd.messages() {
            buf.push_str(&format!("{:#?}\n", msg_desc));
        }

        Ok(File { name: Some(file_name), content: Some(buf), ..Default::default() })
    }
}
