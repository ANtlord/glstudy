use gl;
use std::marker::PhantomData;
use gl::types::GLenum;

pub type Array<const DRAW: GLenum> = Buffer<BufferTypeArray, DRAW>;
pub type ElementArray<const DRAW: GLenum> = Buffer<BufferTypeElementArray, DRAW>;

pub trait BufferType {
    const BUFFER_TYPE: gl::types::GLuint;
}

pub struct BufferTypeArray;
impl BufferType for BufferTypeArray {
    const BUFFER_TYPE: gl::types::GLuint = gl::ARRAY_BUFFER;
}

pub struct BufferTypeElementArray;
impl BufferType for BufferTypeElementArray {
    const BUFFER_TYPE: gl::types::GLuint = gl::ELEMENT_ARRAY_BUFFER;
}

pub struct Buffer<B, const DRAW: GLenum>
where
    B: BufferType,
{
    gl: gl::Gl,
    vbo: gl::types::GLuint,
    _marker: PhantomData<B>,
}

impl<B, const DRAW: GLenum> Buffer<B, DRAW>
where
    B: BufferType,
{
    pub fn new(gl: &gl::Gl) -> Self {
        let mut vbo: gl::types::GLuint = 0;
        unsafe {
            gl.GenBuffers(1, &mut vbo);
        }

        Buffer {
            gl: gl.clone(),
            vbo,
            _marker: PhantomData,
        }
    }

    pub fn bind(&self) {
        unsafe {
            self.gl.BindBuffer(B::BUFFER_TYPE, self.vbo);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            self.gl.BindBuffer(B::BUFFER_TYPE, 0);
        }
    }

    pub fn subdata<T>(&self, data: &[T]) {
        unsafe {
            self.gl.BufferSubData(
                B::BUFFER_TYPE,                                                   // target
                0,
                (data.len() * std::mem::size_of::<T>()) as gl::types::GLsizeiptr, // size of data in bytes
                data.as_ptr() as *const gl::types::GLvoid,                        // pointer to data
            );
        }
    }

    pub fn static_draw_data<T>(&self, data: &[T]) {
        unsafe {
            self.gl.BufferData(
                B::BUFFER_TYPE,                                                   // target
                (data.len() * std::mem::size_of::<T>()) as gl::types::GLsizeiptr, // size of data in bytes
                data.as_ptr() as *const gl::types::GLvoid,                        // pointer to data
                DRAW,                                                 // usage
            );
        }
    }

    pub fn write<T>(&self, data: &[T]) {
        self.bind();
        self.static_draw_data(data);
        self.unbind();
    }
}

impl<B, const DRAW: GLenum> Drop for Buffer<B, DRAW>
where
    B: BufferType,
{
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteBuffers(1, &mut self.vbo);
        }
    }
}

pub struct VertexArray {
    gl: gl::Gl,
    vao: gl::types::GLuint,
}

impl VertexArray {
    pub fn new(gl: &gl::Gl) -> Self {
        let mut vao = 0;
        unsafe {
            gl.GenVertexArrays(1, &mut vao);
        }

        Self {
            gl: gl.clone(),
            vao,
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

    pub fn bind_with<const DRAW: GLenum>(&self, ebo: &ElementArray<DRAW>) {
        self.bind();
        ebo.bind();
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe { self.gl.DeleteBuffers(1, &mut self.vao) }
    }
}
