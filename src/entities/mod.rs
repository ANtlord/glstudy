mod data;
mod vertex;

use data::{buffer_data, load_render_data_indexed, load_render_data_raw};

pub struct VertLine {
    pub x: f32,
    gl: gl::Gl,
    vbo: gl::types::GLuint,
    vao: gl::types::GLuint,
}

#[allow(unused)]
impl VertLine {
    pub fn new(gl: gl::Gl) -> Self {
        let vertices: Vec<vertex::Bald> = vec![[-1.0, 1.0, 0.0].into(), [-1.0, -1.0, 0.0].into()];

        unsafe {
            let (vao, vbo) = load_render_data_raw(&gl, &vertices, gl::DYNAMIC_DRAW);
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

pub struct Shape {
    gl: gl::Gl,
    vbo: gl::types::GLuint,
    vao: gl::types::GLuint,
}

impl Shape {
    #[rustfmt::skip]
    pub fn cube(gl: gl::Gl) -> Self {
        let data: Vec<vertex::Textured> = vec![
            // front
            [-0.5, -0.5, 0.5, 0., 0., 0., 0.0, 0.0].into(),
            [-0.5, 0.5, 0.5, 0., 0., 0., 0.0, 2.0].into(),
            [0.5, 0.5, 0.5, 0., 0., 0., 2.0, 2.0].into(),
            [0.5, -0.5, 0.5, 0., 0., 0., 2.0, 0.0].into(),

            // back
            [-0.5, -0.5, -0.5, 0., 0., 0., 2.0, 0.0].into(),
            [-0.5, 0.5, -0.5, 0., 0., 0., 2.0, 2.0].into(),
            [0.5, 0.5, -0.5, 0., 0., 0., 0.0, 2.0].into(),
            [0.5, -0.5, -0.5, 0., 0., 0., 0.0, 0.0].into(),
        ];

        let indices: Vec<u32> = vec![
            0, 1, 2, 0, 3, 2, // front
            4, 5, 6, 4, 6, 7, // back
            1, 5, 6, 1, 6, 2, // top
            0, 4, 7, 0, 7, 3, // bottom
            0, 1, 5, 0, 5, 4, // left
            3, 2, 6, 3, 6, 7, // right
        ];
        let (vao, vbo) = unsafe { load_render_data_indexed(&gl, &data, &indices, gl::STATIC_DRAW) };
        Self { vao, vbo, gl }
    }

    pub fn parallelogram(gl: gl::Gl) -> Self {
        let data: Vec<vertex::Textured> = vec![
            [-0.5, -0.5, 0., 0., 0., 0., 0.0, 0.0].into(),
            [-0.5, 0.5, 0., 0., 0., 0., 0.0, 2.0].into(),
            [0.5, 0.5, 0., 0., 0., 0., 2.0, 2.0].into(),
            [0.5, -0.5, 0., 0., 0., 0., 2.0, 0.0].into(),
        ];

        let indices: Vec<u32> = vec![0, 1, 2, 0, 3, 2];
        let (vao, vbo) = unsafe { load_render_data_indexed(&gl, &data, &indices, gl::STATIC_DRAW) };
        Self { vao, vbo, gl }
    }

    pub fn triangle(gl: gl::Gl) -> Self {
        let vertices: Vec<vertex::Textured> = vec![
            [-0.5, -0.5, 0., 0., 0., 0., 0., 0.].into(),
            [0.0, 0.5, 0., 0., 0., 0., 0.5, 1.].into(),
            [0.5, -0.5, 0., 0., 0., 0., 1., 0.].into(),
        ];

        let (vao, vbo) = unsafe { load_render_data_raw(&gl, &vertices, gl::STATIC_DRAW) };
        Self { vao, vbo, gl }
    }

    pub fn bind(&self) {
        unsafe { self.gl.BindVertexArray(self.vao) };
    }

    pub fn unbind(&self) {
        unsafe { self.gl.BindVertexArray(0) };
    }
}

impl Drop for Shape {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteBuffers(1, &mut self.vao);
            self.gl.DeleteBuffers(1, &mut self.vbo);
        }
    }
}
