use anyhow::Result;
use vulkanalia::prelude::v1_0::*;

use crate::AppData;
use crate::core;

pub unsafe fn create_descriptor_set_layout(
    device: &Device,
    data: &mut AppData,
) -> Result<()> {
    let ubo_binding = vk::DescriptorSetLayoutBinding::builder()
    .binding(0)
    .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
    .descriptor_count(1)
    .stage_flags(vk::ShaderStageFlags::VERTEX);

    Ok(())
}
