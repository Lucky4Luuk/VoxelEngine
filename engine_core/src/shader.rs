use std::ffi::{CString, CStr};
use luminance::shader::program::Program;

use crate::rasterizer::{
    ShaderInterface,
    VertexSemantics,
};

use std::collections::HashMap;

pub struct ShaderSource {
    pub vertex_shader: String,
    pub geometry_shader: Option<String>,
    pub tesselation_shader: Option<String>,
    pub fragment_shader: String,
}

pub struct Shader {
    pub program: Program<VertexSemantics, (), ShaderInterface>,
}

impl Shader {
    pub fn from_source(source: ShaderSource) -> Self {
        let program: Program<VertexSemantics, (), ShaderInterface> = match Program::from_strings(None, &source.vertex_shader, None, &source.fragment_shader) {
            Ok(program) => program.ignore_warnings(),
            Err(err) => {
                error!("{}", err);
                panic!("Failed to compile shaders!");
            }
        };

        Self {
            program: program,
        }
    }

    pub fn program(&self) -> &Program<VertexSemantics, (), ShaderInterface> {
        &self.program
    }
}

pub struct RawShader {
    pub program: u32,
}

impl RawShader {
    pub fn from_compute(source: &str) -> Self {
        unsafe {
            let c_source_tmp = CString::new(source).unwrap();
            let c_source = c_source_tmp.as_c_str();

            let shader = gl::CreateShader(gl::COMPUTE_SHADER);
            debug!("yep");
            gl::ShaderSource(shader, 1, &c_source.as_ptr(), std::ptr::null());
            debug!("nope");
            gl::CompileShader(shader);

            let mut is_compiled = 0;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut is_compiled);
            if is_compiled == 0 {
                let mut len = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
                let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
                buffer.extend([b' '].iter().cycle().take(len as usize));
                let error: CString = CString::from_vec_unchecked(buffer);
                gl::GetShaderInfoLog(shader, len, std::ptr::null_mut(), error.as_ptr() as *mut gl::types::GLchar);
                error!("{}", error.to_string_lossy().into_owned());
                panic!("shader broken uwu");
            }

            unimplemented!();
            // Self {
            //     program: 0
            // }
        }
    }
}
