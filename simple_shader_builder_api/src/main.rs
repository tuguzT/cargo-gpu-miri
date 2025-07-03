const PATH: &str = env!("simple_shader.spv");
const SHADER: &[u8] = include_bytes!(env!("simple_shader.spv"));

pub fn main() {
    println!("Reading SPIR-V shader from {PATH}");
    println!("Shader data length: {}", SHADER.len());
}
