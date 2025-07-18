use std::{env, error::Error};

use cargo_gpu::{spirv_builder::SpirvMetadata, Install};

const SHADER_CRATE_NAME: &str = "simple_shader";
const SHADER_CRATE_PATH: &str = "./../simple_shader";

fn main() -> Result<(), Box<dyn Error>> {
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_LOG", "trace");
    env_logger::init();

    env::vars().for_each(|(key, value)| log::debug!("{key}={value}"));

    let random_int = unsafe { unsafe_abstraction::random_i32() };
    println!("cargo::rustc-env=RANDOM_INT={random_int}");

    log::debug!("Installing backend for shader crate {SHADER_CRATE_NAME}...");
    let backend_args = Install::from_shader_crate(SHADER_CRATE_PATH.into());
    let backend = backend_args.run()?;
    log::debug!("Backend installed successfully:\n{backend:#?}");

    log::debug!("Building SPIR-V for shader crate {SHADER_CRATE_NAME}...");
    let builder = backend
        .to_spirv_builder(SHADER_CRATE_PATH, "spirv-unknown-vulkan1.2")
        .spirv_metadata(SpirvMetadata::Full);
    let compile_result = builder.build()?;
    log::debug!("Build result:\n{compile_result:#?}");

    let shader_file_path = compile_result.module.unwrap_single().display();
    println!("cargo::rustc-env={SHADER_CRATE_NAME}.spv={shader_file_path}");
    println!("cargo::rerun-if-changed={SHADER_CRATE_PATH}");

    Ok(())
}
