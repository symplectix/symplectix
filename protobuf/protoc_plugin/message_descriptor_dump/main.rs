use prost_types::compiler::{code_generator_response, CodeGeneratorRequest, CodeGeneratorResponse};

fn main() -> anyhow::Result<()> {
    protoc_plugin::run(MessageDescriptorDumpGenerator::default())
}

#[derive(Debug, Default, Clone)]
struct MessageDescriptorDumpGenerator {}

impl protoc_plugin::CodeGenerator for MessageDescriptorDumpGenerator {
    fn gen_code(&self, req: CodeGeneratorRequest) -> CodeGeneratorResponse {
        let pool = protoc_plugin::create_descriptor_pool(&req);

        let file = req
            .file_to_generate
            .iter()
            .map(|file| {
                let file_name = {
                    let stem = file.strip_suffix(".proto").expect("no .proto suffix");
                    format!("{}.message_descriptor_dump", stem)
                };

                let file_desc = pool
                    .get_file_by_name(file.as_str())
                    .expect("failed to fetch a file descriptor from a pool");

                let mut buf = String::with_capacity(1 << 10);
                for msg_desc in file_desc.messages() {
                    buf.push_str(&format!("{:#?}\n", msg_desc));
                }

                code_generator_response::File {
                    name: Some(file_name),
                    content: Some(buf),
                    ..Default::default()
                }
            })
            .collect();

        CodeGeneratorResponse { file, ..Default::default() }
    }
}
