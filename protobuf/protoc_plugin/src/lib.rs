use std::io;
use std::io::{Read, Write};

use anyhow::Context;
use prost::Message;
use prost_types::compiler::{CodeGeneratorRequest, CodeGeneratorResponse};

pub trait CodeGenerator {
    fn gen_code(&self, req: CodeGeneratorRequest) -> CodeGeneratorResponse;
}

pub fn run<T: CodeGenerator>(generator: T) -> anyhow::Result<()> {
    let mut buf = Vec::with_capacity(1 << 10);
    let n = io::stdin().read_to_end(&mut buf).context("failed to read proto message from stdin")?;
    let req = CodeGeneratorRequest::decode(&buf[..n])?;

    let resp = generator.gen_code(req);

    buf.clear();
    resp.encode(&mut buf).context("failed to encode a response message")?;

    Ok(io::stdout().write_all(&buf)?)
}
