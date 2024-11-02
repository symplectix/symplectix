use prost_types::compiler::{code_generator_response, CodeGeneratorRequest, CodeGeneratorResponse};

fn main() -> anyhow::Result<()> {
    protoc_plugin::run(EmptyFileGenerator::default())
}

#[derive(Default, Clone)]
struct EmptyFileGenerator {}

impl protoc_plugin::CodeGenerator for EmptyFileGenerator {
    fn gen_code(&self, req: CodeGeneratorRequest) -> CodeGeneratorResponse {
        let mut resp = CodeGeneratorResponse::default();

        resp.file.extend(req.file_to_generate.iter().map(|f| {
            let stem = f.strip_suffix(".proto").expect("no .proto suffix");
            code_generator_response::File {
                name: Some(format!("{}.empty", stem)),
                content: None,
                ..Default::default()
            }
        }));

        resp
    }
}
