fn memsize<T>(count: usize) -> gl::types::GLint {
    (count * std::mem::size_of::<f32>()) as _
}

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

#[repr(C)]
pub struct VertexChromatic {
    position: [f32; 3],
    color: [f32; 3],
}

impl VertexChromatic {
    pub fn vertex_attrib_pointer(gl: &gl::Gl) {
        // locations are defined in the corresponding vertex shader
        let is_normalized = gl::FALSE;
        let position_attribute_location = 0;
        let position_attribute_size = 3;
        let color_attribute_location = 1;
        let color_attribute_size = 3;

        unsafe {
            gl.EnableVertexAttribArray(position_attribute_location);
            gl.VertexAttribPointer(
                position_attribute_location,
                position_attribute_size,
                gl::FLOAT,
                is_normalized,
                memsize::<f32>(position_attribute_size as _),
                std::ptr::null(),
            );

            gl.EnableVertexAttribArray(color_attribute_location);
            gl.VertexAttribPointer(
                color_attribute_location,
                color_attribute_size,
                gl::FLOAT,
                is_normalized,
                memsize::<f32>(color_attribute_size as _),
                memsize::<f32>(position_attribute_size as _) as *const _,
            );
        }
    }
}

#[repr(C)]
pub struct Vertex {
    inner: [f32; 3],
}

impl From<[f32; 3]> for Vertex {
    fn from(data: [f32; 3]) -> Self {
        Self { inner: data }
    }
}

impl Vertex {
    pub fn vertex_attrib_pointer(gl: &gl::Gl) {
        unsafe {
            let position_attribute_size = 3;
            let position_attribute_location = 3; // SHOULD BE ERROR!!! Change to 1 to fix.
            gl.EnableVertexAttribArray(position_attribute_location);
            gl.VertexAttribPointer(
                position_attribute_location,
                position_attribute_size,
                gl::FLOAT,
                gl::FALSE,
                memsize::<f32>(3),
                std::ptr::null(),
            );
        }
    }
}

pub struct VertLine {
    gl: gl::Gl,
    pub x: f32,
    vbo: gl::types::GLuint,
    vao: gl::types::GLuint,
}


impl VertLine {
    pub fn new(gl: gl::Gl) -> Self {
        let vertices: Vec<Vertex> = vec![[-1.0, 1.0, 0.0].into(), [-1.0, -1.0, 0.0].into()];

        unsafe {
            let vbo = new_array_buffer(&gl);
            let vao = new_vertex_array(&gl);
            gl.BindVertexArray(vao);
            gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
            buffer_dynamic_draw(&gl, &vertices);
            Vertex::vertex_attrib_pointer(&gl);
            gl.BindBuffer(gl::ARRAY_BUFFER, 0);
            gl.BindVertexArray(0);

            VertLine {
                gl,
                vao,
                vbo,
                x: -1.,
            }
        }
    }

    pub fn update(&self) {
        let vertices: Vec<Vertex> = vec![[self.x, 1.0, 0.0].into(), [self.x, -1.0, 0.0].into()];
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
