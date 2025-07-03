#![no_std]

use spirv_std::{glam::UVec3, spirv};

#[spirv(compute(threads(64)))]
pub fn simple_shader(
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] data: &mut [u32],
) {
    data[id.x as usize] += 1;
}
