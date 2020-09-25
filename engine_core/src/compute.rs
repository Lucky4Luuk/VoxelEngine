use gl::types::{
    GLint,
    GLchar
};

pub fn get_workgroup_count() -> (i32, i32, i32) {
    let mut result = (0,0,0);
    unsafe {
        gl::GetIntegeri_v(glow::MAX_COMPUTE_WORK_GROUP_COUNT, 0, &mut result.0);
        gl::GetIntegeri_v(glow::MAX_COMPUTE_WORK_GROUP_COUNT, 1, &mut result.1);
        gl::GetIntegeri_v(glow::MAX_COMPUTE_WORK_GROUP_COUNT, 2, &mut result.2);
    };
    result
}

pub fn get_workgroup_size() -> (i32, i32, i32) {
    let mut result = (0,0,0);
    unsafe {
        gl::GetIntegeri_v(glow::MAX_COMPUTE_WORK_GROUP_SIZE, 0, &mut result.0);
        gl::GetIntegeri_v(glow::MAX_COMPUTE_WORK_GROUP_SIZE, 1, &mut result.1);
        gl::GetIntegeri_v(glow::MAX_COMPUTE_WORK_GROUP_SIZE, 2, &mut result.2);
    };
    result
}

pub fn get_workgroup_invocations() -> i32 {
    let mut value = 0;
    unsafe {
        gl::GetIntegerv(glow::MAX_COMPUTE_WORK_GROUP_INVOCATIONS, &mut value);
    }
    value
}

pub fn get_compute_program(cs: &str) -> <glow::Context as glow::HasContext>::Program {
    unsafe {
        let shader = gl::CreateShader(glow::COMPUTE_SHADER);
        gl::ShaderSource(shader, 1, &(cs.as_ptr() as *const GLchar), &(cs.len() as GLint));
        gl::CompileShader(shader);

        let mut status = 0;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
        if status != 1 {
            let mut length = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut length);
            let error = if length > 0 {
                let mut log = String::with_capacity(length as usize);
                log.extend(std::iter::repeat('\0').take(length as usize));
                gl::GetShaderInfoLog(
                    shader,
                    length,
                    &mut length,
                    (&log[..]).as_ptr() as *mut GLchar,
                );
                log.truncate(length as usize);
                log
            } else {
                String::from("")
            };
            error!("{}", error);
            panic!("Failed to compile compute shader!");
        }

        let program = gl::CreateProgram();
        gl::AttachShader(program, shader);
        gl::LinkProgram(program);

        status = 0;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);
        if status != 1 {
            let mut length = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut length);
            let error = if length > 0 {
                let mut log = String::with_capacity(length as usize);
                log.extend(std::iter::repeat('\0').take(length as usize));
                gl::GetProgramInfoLog(
                    program,
                    length,
                    &mut length,
                    (&log[..]).as_ptr() as *mut GLchar,
                );
                log.truncate(length as usize);
                log
            } else {
                String::from("")
            };
            error!("{}", error);
            panic!("Failed to compile compute program!");
        }

        gl::DetachShader(program, shader);
        gl::DeleteShader(shader);

        program
    }
}
