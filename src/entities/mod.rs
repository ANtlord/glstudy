mod vertex;

use crate::entities::vertex::VertexAttribPointer;

fn new_array_buffer(gl: &gl::Gl) -> gl::types::GLuint {
    let mut vbo: gl::types::GLuint = 0;
    unsafe { gl.GenBuffers(1, &mut vbo) };
    vbo
}

fn new_vertex_array(gl: &gl::Gl) -> gl::types::GLuint {
    let mut vao = 0;
    unsafe { gl.GenVertexArrays(1, &mut vao) };
    vao
}

pub struct Triangle {
    gl: gl::Gl,
    vbo: gl::types::GLuint,
    vao: gl::types::GLuint,
}

impl Triangle {
    pub fn new(gl: gl::Gl) -> Self {
        let vertices: Vec<vertex::Textured> = vec![
            [-0.5, -0.5, 0., 0., 0., 0., 0., 0.].into(),
            [0.0, 0.5, 0., 0., 0., 0., 0.5, 1.].into(),
            [0.5, -0.5, 0., 0., 0., 0., 1., 0.].into(),
        ];

        let (vao, vbo) = unsafe { build_data(&gl, &vertices, gl::STATIC_DRAW) };
        Self { vao, vbo, gl }
    }

    pub fn bind(&self) {
        unsafe { self.gl.BindVertexArray(self.vao) };
    }

    pub fn unbind(&self) {
        unsafe { self.gl.BindVertexArray(0) };
    }
}

pub struct VertLine {
    pub x: f32,
    gl: gl::Gl,
    vbo: gl::types::GLuint,
    vao: gl::types::GLuint,
}

unsafe fn build_data<T: vertex::VertexAttribPointer>(
    gl: &gl::Gl,
    data: &[T],
    draw: gl::types::GLenum,
) -> (u32, u32) {
    let vbo = new_array_buffer(&gl);
    let vao = new_vertex_array(&gl);
    gl.BindVertexArray(vao);
    gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
    buffer_data(gl, gl::ARRAY_BUFFER, data, draw);
    T::vertex_attrib_pointer(&gl);
    gl.BindBuffer(gl::ARRAY_BUFFER, 0);
    gl.BindVertexArray(0);
    (vao, vbo)
}

impl VertLine {
    pub fn new(gl: gl::Gl) -> Self {
        let vertices: Vec<vertex::Bald> = vec![[-1.0, 1.0, 0.0].into(), [-1.0, -1.0, 0.0].into()];

        unsafe {
            let (vao, vbo) = build_data(&gl, &vertices, gl::DYNAMIC_DRAW);
            VertLine {
                gl,
                vao,
                vbo,
                x: -1.,
            }
        }
    }

    pub fn update(&self) {
        let vertices: Vec<vertex::Bald> =
            vec![[self.x, 1.0, 0.0].into(), [self.x, -1.0, 0.0].into()];
        unsafe {
            self.gl.BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            buffer_data(&self.gl, gl::ARRAY_BUFFER, &vertices, gl::DYNAMIC_DRAW);
            self.gl.BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    pub fn bind(&self) {
        unsafe { self.gl.BindVertexArray(self.vao) };
    }

    pub fn unbind(&self) {
        unsafe { self.gl.BindVertexArray(0) };
    }
}

unsafe fn buffer_data<T>(
    gl: &gl::Gl,
    target: gl::types::GLenum,
    data: &[T],
    draw: gl::types::GLenum,
) {
    gl.BufferData(
        target,
        (data.len() * std::mem::size_of::<T>()) as _,
        data.as_ptr() as *const _,
        draw,
    );
}

pub struct Parallelogram {
    gl: gl::Gl,
    vbo: gl::types::GLuint,
    vao: gl::types::GLuint,
}

impl Parallelogram {
    pub fn new(gl: gl::Gl) -> Self {
        let data: Vec<vertex::Textured> = vec![
            [-0.5, -0.5, 0., 0., 0., 0., 0.0, 0.0].into(),
            [-0.5, 0.5, 0., 0., 0., 0., 0.0, 2.0].into(),
            [0.5, 0.5, 0., 0., 0., 0., 2.0, 2.0].into(),
            [0.5, -0.5, 0., 0., 0., 0., 2.0, 0.0].into(),
        ];

        let indices: Vec<u32> = vec![0, 1, 2, 0, 3, 2];

        let (vao, vbo) = //unsafe { build_data(&gl, &data, gl::STATIC_DRAW) };
            unsafe {
                let vbo = new_array_buffer(&gl);
                let vao = new_vertex_array(&gl);
                let ebo = new_vertex_array(&gl);
                gl.BindVertexArray(vao);

                gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
                buffer_data(&gl, gl::ARRAY_BUFFER, &data, gl::STATIC_DRAW);

                gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
                buffer_data(&gl, gl::ELEMENT_ARRAY_BUFFER, &indices, gl::STATIC_DRAW);

                vertex::Textured::vertex_attrib_pointer(&gl);
                gl.BindBuffer(gl::ARRAY_BUFFER, 0);
                gl.BindVertexArray(0);
                (vao, vbo)
            };

        Self { vao, vbo, gl }
    }

    pub fn bind(&self) {
        unsafe { self.gl.BindVertexArray(self.vao) };
    }

    pub fn unbind(&self) {
        unsafe { self.gl.BindVertexArray(0) };
    }
}
