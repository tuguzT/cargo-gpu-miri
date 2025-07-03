use std::error::Error;

use rspirv::{
    binary::{parse_bytes, Disassemble},
    dr::Loader,
};

const PATH: &str = env!("simple_shader.spv");
const SHADER: &[u8] = include_bytes!(env!("simple_shader.spv"));

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("Reading SPIR-V shader from {PATH}");

    let mut loader = Loader::new();
    parse_bytes(SHADER, &mut loader)?;

    let module = loader.module();
    println!("Shader disassembly:\n{}", module.disassemble());

    Ok(())
}
