use std::error::Error;

use cargo_gpu::{spirv_builder::SpirvMetadata, Install};

const SHADER_CRATE_NAME: &str = "simple_shader";
const SHADER_CRATE_PATH: &str = "./../simple_shader";

fn main() -> Result<(), Box<dyn Error>> {
    let backend_args = Install::from_shader_crate(SHADER_CRATE_PATH.into());
    let backend = backend_args.run()?;

    let builder = backend
        .to_spirv_builder(SHADER_CRATE_PATH, "spirv-unknown-vulkan1.2")
        .spirv_metadata(SpirvMetadata::Full);
    let compile_result = builder.build()?;

    let shader_file_path = compile_result.module.unwrap_single().display();
    println!("cargo::rustc-env={SHADER_CRATE_NAME}.spv={shader_file_path}");
    println!("cargo::rerun-if-changed={SHADER_CRATE_PATH}");

    Ok(())
}
