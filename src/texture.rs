/// There are should be 16 textures at a time. The number depends on version of OpenGL. The flow of
/// loading a texture is:
/// 1. glGenTextures(1, texture_id);
/// 2. glActiveTexture(GL_TEXTURE0); // GL_TEXTURE0 .. GL_TEXTURE15
/// 3. glBindTexture(GL_TEXTURE_2D, texture_id);
/// 4. Customizations of the texture (set parameters of wraping, uploading data (image), mipmaps)
/// 5. glUniform1i(glGetUniformLocation(shader_id, "textue_field_name"), 0);
///
/// Note: the shader which is designed for the texture must have field `uniform Sampler2D
/// textue_field_name;`
use gl;
use std::ffi::c_void;

#[allow(unused)]
pub struct Texture {
    gl: gl::Gl,
    id: u32,
}

#[allow(unused)]
impl Texture {
    pub fn new(gl: gl::Gl, data: &[u8], (width, height): (u32, u32)) -> Self {
        let mut id = 0;
        const MIPMAP_LEVEL: i32 = 0;
        unsafe {
            gl.GenTextures(1, &mut id);
            gl.BindTexture(gl::TEXTURE_2D, id);
            // Here you are able to handle what to do if texture is short for a polygon. It can
            // repeat or not, or repeat with mirroring etc. Like background: left top repeat;
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as _);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as _);
            // This is about using proper scale of the texture. It's called mipmaps which are
            // generated a little bit later.
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as _);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as _);
            gl.TexImage2D(
                gl::TEXTURE_2D,
                MIPMAP_LEVEL,
                gl::RGB as _,
                width as _,
                height as _,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const c_void,
            );
            gl.GenerateMipmap(gl::TEXTURE_2D);
        }

        Self { id, gl }
    }

    fn bind(&self) {
        unsafe { self.gl.BindTexture(gl::TEXTURE_2D, self.id) }
    }
}
