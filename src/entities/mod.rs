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
            VertLine { gl, vao, vbo, x: -1. }
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

#[allow(unused)]
fn cube_indices() -> Vec<u32> {
    vec![
        0, 1, 2, 0, 3, 2, // front
        4, 5, 6, 4, 6, 7, // back
        1, 5, 6, 1, 6, 2, // top
        0, 4, 7, 0, 7, 3, // bottom
        0, 1, 5, 0, 5, 4, // left
        3, 2, 6, 3, 6, 7, // right
    ]
}

/// Data for to draw a cube poorly. Texture on its top and bottom is going to be applied in wrong
/// way.
#[allow(unused)]
#[rustfmt::skip]
pub fn textured_cube(gl: gl::Gl) -> Shape {
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

    let mut opts = ShapeOptions::new(data);
    opts.indices(cube_indices());
    opts.build(gl)
}

#[allow(unused)]
#[rustfmt::skip]
pub fn bald_cube(gl: gl::Gl) -> Shape {
    let data: Vec<vertex::Bald> = vec![
        // front
        [-0.5, -0.5, 0.5].into(),
        [-0.5, 0.5, 0.5].into(),
        [0.5, 0.5, 0.5].into(),
        [0.5, -0.5, 0.5].into(),

        // back
        [-0.5, -0.5, -0.5].into(),
        [-0.5, 0.5, -0.5].into(),
        [0.5, 0.5, -0.5].into(),
        [0.5, -0.5, -0.5].into(),
    ];

    let mut opts = ShapeOptions::new(data);
    opts.indices(cube_indices());
    opts.build(gl)
}

#[rustfmt::skip]
pub fn normalized_cube(gl: gl::Gl) -> Shape {
    let data: Vec<vertex::Textured> = vec![
        [-0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  0.0,  0.0].into(),
        [ 0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  0.0,  1.0].into(),
        [ 0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  1.0,  1.0].into(),
        [ 0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  1.0,  1.0].into(),
        [-0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  1.0,  0.0].into(),
        [-0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  0.0,  0.0].into(),

        [-0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  0.0,  0.0].into(),
        [ 0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  0.0,  1.0].into(),
        [ 0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  1.0,  1.0].into(),
        [ 0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  1.0,  1.0].into(),
        [-0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  1.0,  0.0].into(),
        [-0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  0.0,  0.0].into(),

        [-0.5,  0.5,  0.5, -1.0,  0.0,  0.0,  0.0,  0.0].into(),
        [-0.5,  0.5, -0.5, -1.0,  0.0,  0.0,  0.0,  1.0].into(),
        [-0.5, -0.5, -0.5, -1.0,  0.0,  0.0,  1.0,  1.0].into(),
        [-0.5, -0.5, -0.5, -1.0,  0.0,  0.0,  1.0,  1.0].into(),
        [-0.5, -0.5,  0.5, -1.0,  0.0,  0.0,  1.0,  0.0].into(),
        [-0.5,  0.5,  0.5, -1.0,  0.0,  0.0,  0.0,  0.0].into(),

        [ 0.5,  0.5,  0.5,  1.0,  0.0,  0.0,  0.0,  0.0].into(),
        [ 0.5,  0.5, -0.5,  1.0,  0.0,  0.0,  0.0,  1.0].into(),
        [ 0.5, -0.5, -0.5,  1.0,  0.0,  0.0,  1.0,  1.0].into(),
        [ 0.5, -0.5, -0.5,  1.0,  0.0,  0.0,  1.0,  1.0].into(),
        [ 0.5, -0.5,  0.5,  1.0,  0.0,  0.0,  1.0,  0.0].into(),
        [ 0.5,  0.5,  0.5,  1.0,  0.0,  0.0,  0.0,  0.0].into(),

        [-0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  0.0,  0.0].into(),
        [ 0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  0.0,  1.0].into(),
        [ 0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  1.0,  1.0].into(),
        [ 0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  1.0,  1.0].into(),
        [-0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  1.0,  0.0].into(),
        [-0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  0.0,  0.0].into(),

        [-0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  0.0,  0.0].into(),
        [ 0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  0.0,  1.0].into(),
        [ 0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  1.0,  1.0].into(),
        [ 0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  1.0,  1.0].into(),
        [-0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  1.0,  0.0].into(),
        [-0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  0.0,  0.0].into()
    ];

    let opts = ShapeOptions::new(data);
    opts.build(gl)
}

struct ShapeOptions<T> {
    vertices: Vec<T>,
    indices: Option<Vec<u32>>,
}

impl<T: vertex::VertexAttribPointer> ShapeOptions<T> {
    fn new(vertices: Vec<T>) -> Self {
        Self { vertices, indices: None }
    }

    fn indices(&mut self, vertices: Vec<u32>) -> &mut Self {
        self.indices = Some(vertices);
        self
    }

    fn build(self, gl: gl::Gl) -> Shape {
        let (vao, vbo) = match self.indices {
            Some(indices) => unsafe {
                load_render_data_indexed(&gl, &self.vertices, &indices, gl::STATIC_DRAW)
            },
            None => unsafe { load_render_data_raw(&gl, &self.vertices, gl::STATIC_DRAW) },
        };

        Shape { vao, vbo, gl }
    }
}

impl Shape {
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
