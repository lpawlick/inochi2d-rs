use crate::glow;

pub struct ProgramBuilder<'a> {
    gl: &'a glow::Context,
    program: glow::NativeProgram,
}

pub struct Program<'a> {
    gl: &'a glow::Context,
    pub program: glow::NativeProgram,
}

impl<'a> Program<'a> {
    pub fn builder(gl: &'a glow::Context) -> Result<ProgramBuilder<'a>, String> {
        let program = gl.create_program().unwrap();
        Ok(ProgramBuilder { gl, program })
    }

    pub fn use_(&self) {
        let gl = self.gl;
        gl.use_program(Some(&self.program));
    }

    pub fn get_uniform_location(&self, location: &str) -> Option<glow::NativeUniformLocation> {
        let gl = self.gl;
        gl.get_uniform_location(&self.program, location)
    }
}

impl<'a> Drop for Program<'a> {
    fn drop(&mut self) {
        let gl = self.gl;
        gl.delete_program(Some(&self.program));
    }
}

impl<'a> ProgramBuilder<'a> {
    pub fn shader(self, foo: u32, data: &'static str) -> Result<ProgramBuilder<'a>, String> {
        let gl = self.gl;
        let shader = gl.create_shader(foo).unwrap();
        gl.shader_source(&shader, data);
        gl.compile_shader(&shader);
        if !gl.get_shader_compile_status(shader) {
            return Err(gl.get_shader_info_log(&shader).unwrap());
        }
        gl.attach_shader(&self.program, &shader);
        gl.delete_shader(Some(&shader));

        Ok(self)
    }

    pub fn link(self) -> Result<Program<'a>, String> {
        let gl = self.gl;
        gl.link_program(&self.program);
        if !gl.get_program_link_status(&self.program) {
            return Err(gl.get_program_info_log(&self.program).unwrap());
        }
        Ok(Program {
            gl,
            program: self.program,
        })
    }
}
