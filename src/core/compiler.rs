use anyhow::{Context, Result};
use shaderc::{Compiler, ShaderKind, CompileOptions};
use log::*;

use crate::core;

pub fn compile_shader(relative_path: &str, kind: ShaderKind) -> Result<Vec<u32>> {
    let path = std::path::Path::new(core::config::PROJECT_ROOT).join(relative_path);
    let source = std::fs::read_to_string(&path)
        .with_context(|| format!("Failed to read file: {}", path.display()))?;

    let compiler = Compiler::new()
        .context("Failed to init Shader Compiler")?;

    let mut options = CompileOptions::new().unwrap();
    options.set_target_env(shaderc::TargetEnv::Vulkan, shaderc::EnvVersion::Vulkan1_0 as u32);

    let binary_result = compiler
        .compile_into_spirv(&source, 
            kind, 
            path.to_str().unwrap_or("shader_src"), 
            "main", 
            Some(&options)
            )
        .with_context(|| {
            error!("Syntax error, shader file: {}", path.display());
            format!("Shader compile error. ({})", path.display())
        })?;

    Ok(binary_result.as_binary().to_vec())
}
