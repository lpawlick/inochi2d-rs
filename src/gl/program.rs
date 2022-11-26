use glow::HasContext;

pub struct ProgramBuilder<'a> {
    gl: &'a glow::Context,
    program: glow::NativeProgram,
    shaders: Vec<(u32, &'static str)>,
}

pub struct Program<'a> {
    gl: &'a glow::Context,
    pub program: glow::NativeProgram,
}

impl<'a> Program<'a> {
    pub fn builder(gl: &'a glow::Context) -> Result<ProgramBuilder<'a>, String> {
        let program = unsafe { gl.create_program()? };
        Ok(ProgramBuilder {
            gl,
            program,
            shaders: Vec::new(),
        })
    }

    pub fn use_(&self) {
        let gl = self.gl;
        unsafe { gl.use_program(Some(self.program)) };
    }

    pub fn get_uniform_location(&self, location: &str) -> Option<glow::NativeUniformLocation> {
        let gl = self.gl;
        unsafe { gl.get_uniform_location(self.program, location) }
    }
}

impl<'a> Drop for Program<'a> {
    fn drop(&mut self) {
        let gl = self.gl;
        unsafe { gl.delete_program(self.program) };
    }
}

impl<'a> ProgramBuilder<'a> {
    pub fn shader(self, foo: u32, data: &'static str) -> Result<ProgramBuilder<'a>, String> {
        let gl = self.gl;
        unsafe {
            let shader = gl.create_shader(foo)?;
            gl.shader_source(shader, data);
            gl.compile_shader(shader);
            if !gl.get_shader_compile_status(shader) {
                return Err(gl.get_shader_info_log(shader));
            }
            gl.attach_shader(self.program, shader);
            gl.delete_shader(shader);
        }

        Ok(self)
    }

    pub fn link(self) -> Result<Program<'a>, String> {
        let gl = self.gl;
        unsafe {
            gl.link_program(self.program);
            if !gl.get_program_link_status(self.program) {
                return Err(gl.get_program_info_log(self.program));
            }
        }
        Ok(Program {
            gl,
            program: self.program,
        })
    }
}