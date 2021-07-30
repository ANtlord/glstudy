/// There are 3 main entities:
/// - Vertex Buffer Object (VBO)
/// - Vertex Array Object (VAO)
/// - Element Buffer Object (EBO)
///
/// Vertex Buffer Object (VBO) is purposed to store some data for a shader.
///
/// Vertex Array Object (VAO) defines the way how to pass the data from VBO to a shader.
///
/// Element Buffer Object (EBO) is an array of indexes to pieces of the data from VBO.
use crate::entities::vertex;
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

/// load_render_data_raw creates vertex buffer object and vertex array object.
/// - vertex buffer object holds passed `data` for `draw`;
/// - vertex array object is used to setup way the data is read for render. For example we can use
/// (0 - 2) values for the first parameter of a shader, (3 - 5) for the second parameter;
pub unsafe fn load_render_data_raw<T>(gl: &gl::Gl, data: &[T], draw: GLenum) -> (u32, u32)
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

pub unsafe fn load_render_data_indexed<T>(
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

pub unsafe fn buffer_data<T>(gl: &gl::Gl, target: GLenum, data: &[T], draw: GLenum) {
    let size = data.len() * std::mem::size_of::<T>();
    gl.BufferData(target, size as _, data.as_ptr() as *const _, draw);
}
