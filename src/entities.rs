const fn memsize<T>(count: usize) -> gl::types::GLint {
    (count * std::mem::size_of::<T>()) as _
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

impl From<[f32; 6]> for VertexChromatic {
    fn from(data: [f32; 6]) -> Self {
        Self {
            position: [data[0], data[1], data[2]],
            color: [data[3], data[4], data[5]],
        }
    }
}

impl VertexAttribPointer for VertexChromatic {
    fn vertex_attrib_pointer(gl: &gl::Gl) {
        // locations are defined in the corresponding vertex shader
        const IS_NORMALIZED: u8 = gl::FALSE;
        const POSITION_ATTRIBUTE_LOCATION: u32 = 0;
        const POSITION_ATTRIBUTE_SIZE: i32 = 3;
        const COLOR_ATTRIBUTE_LOCATION: u32 = 1;
        const COLOR_ATTRIBUTE_SIZE: i32 = 3;

        unsafe {
            gl.EnableVertexAttribArray(POSITION_ATTRIBUTE_LOCATION);
            gl.VertexAttribPointer(
                POSITION_ATTRIBUTE_LOCATION,
                POSITION_ATTRIBUTE_SIZE,
                gl::FLOAT,
                IS_NORMALIZED,
                memsize::<f32>((POSITION_ATTRIBUTE_SIZE + COLOR_ATTRIBUTE_SIZE) as _),
                // data for position of vertex is the FIRST 3 float numbers.
                std::ptr::null(),
            );

            gl.EnableVertexAttribArray(COLOR_ATTRIBUTE_LOCATION);
            gl.VertexAttribPointer(
                COLOR_ATTRIBUTE_LOCATION,
                COLOR_ATTRIBUTE_SIZE,
                gl::FLOAT,
                IS_NORMALIZED,
                memsize::<f32>((POSITION_ATTRIBUTE_SIZE + COLOR_ATTRIBUTE_SIZE) as _),
                // data for position of vertex is the SECOND 3 float numbers.
                memsize::<f32>(POSITION_ATTRIBUTE_SIZE as _) as *const _,
            );
        }
    }
}

pub struct Triangle {
    gl: gl::Gl,
    vbo: gl::types::GLuint,
    vao: gl::types::GLuint,
}

impl Triangle {
    pub fn new(gl: gl::Gl) -> Self {
        let vertices: Vec<VertexChromatic> = vec![
            [-0.5, -0.5, 0., 1., 0., 0.].into(),
            [0.0, 0.5, 0., 0., 1., 0.].into(),
            [0.5, -0.5, 0., 0., 0., 1.].into(),
        ];

        let (vao, vbo) =
            unsafe { build_data::<_, VertexChromatic>(&gl, &vertices, gl::STATIC_DRAW) };

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

#[repr(C)]
pub struct Vertex {
    inner: [f32; 3],
}

impl From<[f32; 3]> for Vertex {
    fn from(data: [f32; 3]) -> Self {
        Self { inner: data }
    }
}

impl VertexAttribPointer for Vertex {
    fn vertex_attrib_pointer(gl: &gl::Gl) {
        unsafe {
            const POSITION_ATTRIBUTE_SIZE: i32 = 3;
            const POSITION_ATTRIBUTE_LOCATION: gl::types::GLuint = 0;
            gl.EnableVertexAttribArray(POSITION_ATTRIBUTE_LOCATION);
            gl.VertexAttribPointer(
                POSITION_ATTRIBUTE_LOCATION,
                POSITION_ATTRIBUTE_SIZE,
                gl::FLOAT,
                gl::FALSE,
                memsize::<f32>(POSITION_ATTRIBUTE_SIZE as _),
                std::ptr::null(),
            );
        }
    }
}

pub struct VertLine {
    pub x: f32,
    gl: gl::Gl,
    vbo: gl::types::GLuint,
    vao: gl::types::GLuint,
}

trait VertexAttribPointer {
    fn vertex_attrib_pointer(_: &gl::Gl);
}

unsafe fn build_data<T, VertexDataInterpreter: VertexAttribPointer>(
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
        let vertices: Vec<Vertex> = vec![[-1.0, 1.0, 0.0].into(), [-1.0, -1.0, 0.0].into()];

        unsafe {
            let (vao, vbo) = build_data::<_, Vertex>(&gl, &vertices, gl::DYNAMIC_DRAW);
            VertLine { gl, vao, vbo, x: -1. }
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
