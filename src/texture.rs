use gl;
use std::ffi::c_void;

pub struct Texture {
    gl: gl::Gl,
    id: u32,
}

impl Texture {
    pub fn new(gl: gl::Gl, data: &[u8], size: (u32, u32)) -> Self {
        let mut id = 0;
        const MIPMAP_LEVEL: i32 = 0;
        unsafe {
            gl.GenTextures(1, &mut id);
            gl.BindTexture(gl::TEXTURE_2D, id);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as _);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as _);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as _);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as _);
            gl.TexImage2D(
                gl::TEXTURE_2D,
                MIPMAP_LEVEL,
                gl::RGB as _,
                size.0 as _,
                size.1 as _,
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
