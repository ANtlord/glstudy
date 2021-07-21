mod vertex;

fn new_array_buffer(gl: &gl::Gl) -> gl::types::GLuint {
    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl.GenBuffers(1, &mut vbo);
    }
    vbo
}

fn new_vertex_array(gl: &gl::Gl) -> gl::types::GLuint {
    let mut vao = 0;
    unsafe {
        gl.GenVertexArrays(1, &mut vao);
    }
    vao
}

fn buffer_dynamic_draw<T>(gl: &gl::Gl, data: &[T]) {
    unsafe {
        gl.BufferData(
            gl::ARRAY_BUFFER,
            (data.len() * std::mem::size_of::<T>()) as gl::types::GLsizeiptr,
            data.as_ptr() as *const gl::types::GLvoid,
            gl::DYNAMIC_DRAW,
        );
    }
}

pub struct Triangle {
    gl: gl::Gl,
    vbo: gl::types::GLuint,
    vao: gl::types::GLuint,
}

impl Triangle {
    pub fn new(gl: gl::Gl) -> Self {
        let vertices: Vec<vertex::Chromatic> = vec![
            [-0.5, -0.5, 0., 1., 0., 0.].into(),
            [0.0, 0.5, 0., 0., 1., 0.].into(),
            [0.5, -0.5, 0., 0., 0., 1.].into(),
        ];

        let (vao, vbo) =
            unsafe { build_data::<_, vertex::Chromatic>(&gl, &vertices, gl::STATIC_DRAW) };

        Self { vao, vbo, gl }
    }

    pub fn bind(&self) {
        unsafe {
            self.gl.BindVertexArray(self.vao);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            self.gl.BindVertexArray(0);
        }
    }
}

pub struct VertLine {
    pub x: f32,
    gl: gl::Gl,
    vbo: gl::types::GLuint,
    vao: gl::types::GLuint,
}

unsafe fn build_data<T, VertexDataInterpreter: vertex::VertexAttribPointer>(
    gl: &gl::Gl,
    data: &[T],
    draw: gl::types::GLenum,
) -> (u32, u32) {
    let vbo = new_array_buffer(&gl);
    let vao = new_vertex_array(&gl);
    gl.BindVertexArray(vao);
    gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
    gl.BufferData(
        gl::ARRAY_BUFFER,
        (data.len() * std::mem::size_of::<T>()) as _,
        data.as_ptr() as *const _,
        draw,
    );
    VertexDataInterpreter::vertex_attrib_pointer(&gl);
    gl.BindBuffer(gl::ARRAY_BUFFER, 0);
    gl.BindVertexArray(0);
    (vao, vbo)
}

impl VertLine {
    pub fn new(gl: gl::Gl) -> Self {
        let vertices: Vec<vertex::Bald> = vec![[-1.0, 1.0, 0.0].into(), [-1.0, -1.0, 0.0].into()];

        unsafe {
            let (vao, vbo) = build_data::<_, vertex::Bald>(&gl, &vertices, gl::DYNAMIC_DRAW);
            VertLine { gl, vao, vbo, x: -1. }
        }
    }

    pub fn update(&self) {
        let vertices: Vec<vertex::Bald> = vec![[self.x, 1.0, 0.0].into(), [self.x, -1.0, 0.0].into()];
        unsafe {
            self.gl.BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            buffer_dynamic_draw(&self.gl, &vertices);
            self.gl.BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    pub fn bind(&self) {
        unsafe {
            self.gl.BindVertexArray(self.vao);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            self.gl.BindVertexArray(0);
        }
    }
}
