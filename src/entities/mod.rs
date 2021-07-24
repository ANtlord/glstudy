mod vertex;

use crate::entities::vertex::VertexAttribPointer;
use gl::types::GLenum;

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

pub struct VertLine {
    pub x: f32,
    gl: gl::Gl,
    vbo: gl::types::GLuint,
    vao: gl::types::GLuint,
}

/// load_render_data_raw creates vertex buffer object and vertex array object.
/// - vertex buffer object holds passed `data` for `draw`;
/// - vertex array object is used to setup way the data is read for render. For example we can use
/// (0 - 2) values for the first parameter of a shader, (3 - 5) for the second parameter;
unsafe fn load_render_data_raw<T>(gl: &gl::Gl, data: &[T], draw: GLenum) -> (u32, u32)
where
    T: vertex::VertexAttribPointer,
{
    let vertex_buffer_object = new_array_buffer(&gl);
    let vertex_array_object = new_vertex_array(&gl);
    gl.BindVertexArray(vertex_array_object);
    gl.BindBuffer(gl::ARRAY_BUFFER, vertex_buffer_object);
    buffer_data(gl, gl::ARRAY_BUFFER, data, draw);
    T::vertex_attrib_pointer(&gl);
    gl.BindBuffer(gl::ARRAY_BUFFER, 0);
    gl.BindVertexArray(0);
    (vertex_array_object, vertex_buffer_object)
}

unsafe fn load_render_data_indexed<T>(
    gl: &gl::Gl,
    data: &[T],
    indices: &[u32],
    draw: GLenum,
) -> (u32, u32)
where
    T: vertex::VertexAttribPointer,
{
    let vertex_buffer_object = new_array_buffer(&gl);
    let vertex_array_object = new_vertex_array(&gl);
    // don't unload because it's stored within vertex_array_object.
    let element_buffer_object = new_vertex_array(&gl);
    gl.BindVertexArray(vertex_array_object);

    gl.BindBuffer(gl::ARRAY_BUFFER, vertex_buffer_object);
    buffer_data(&gl, gl::ARRAY_BUFFER, &data, draw);

    gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, element_buffer_object);
    buffer_data(&gl, gl::ELEMENT_ARRAY_BUFFER, &indices, draw);

    T::vertex_attrib_pointer(&gl);
    gl.BindBuffer(gl::ARRAY_BUFFER, 0);
    gl.BindVertexArray(0);
    (vertex_array_object, vertex_buffer_object)
}

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

unsafe fn buffer_data<T>(gl: &gl::Gl, target: GLenum, data: &[T], draw: GLenum) {
    let size = data.len() * std::mem::size_of::<T>();
    gl.BufferData(target, size as _, data.as_ptr() as *const _, draw);
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
        let (vao, vbo) = unsafe { load_render_data_indexed(&gl, &data, &indices, gl::STATIC_DRAW) };
        Self { vao, vbo, gl }
    }

    pub fn bind(&self) {
        unsafe { self.gl.BindVertexArray(self.vao) };
    }

    pub fn unbind(&self) {
        unsafe { self.gl.BindVertexArray(0) };
    }
}

pub struct Cube {
    gl: gl::Gl,
    vbo: gl::types::GLuint,
    vao: gl::types::GLuint,
}

impl Cube {
    #[rustfmt::skip]
    pub fn new(gl: gl::Gl) -> Self {
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

    pub fn bind(&self) {
        unsafe { self.gl.BindVertexArray(self.vao) };
    }

    pub fn unbind(&self) {
        unsafe { self.gl.BindVertexArray(0) };
    }
}
