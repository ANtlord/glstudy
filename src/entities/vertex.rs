pub trait VertexAttribPointer {
    fn vertex_attrib_pointer(_: &gl::Gl);
}

const fn memsize<T>(count: usize) -> gl::types::GLint {
    (count * std::mem::size_of::<T>()) as _
}

#[repr(C)]
pub struct Bald {
    inner: [f32; 3],
}

impl From<[f32; 3]> for Bald {
    fn from(data: [f32; 3]) -> Self {
        Self { inner: data }
    }
}

impl VertexAttribPointer for Bald {
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

#[repr(C)]
pub struct Chromatic {
    position: [f32; 3],
    color: [f32; 3],
}

impl From<[f32; 6]> for Chromatic {
    fn from(data: [f32; 6]) -> Self {
        Self { position: [data[0], data[1], data[2]], color: [data[3], data[4], data[5]] }
    }
}

impl VertexAttribPointer for Chromatic {
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

#[repr(C)]
pub struct Textured {
    position: [f32; 3],
    color: [f32; 3],
    texture_position: [f32; 2],
}

impl From<[f32; 8]> for Textured {
    fn from(data: [f32; 8]) -> Self {
        Self {
            position: [data[0], data[1], data[2]],
            color: [data[3], data[4], data[5]],
            texture_position: [data[6], data[7]],
        }
    }
}

impl VertexAttribPointer for Textured {
    fn vertex_attrib_pointer(gl: &gl::Gl) {
        // locations are defined in the corresponding vertex shader
        const IS_NORMALIZED: u8 = gl::FALSE;
        const POSITION_ATTRIBUTE_LOCATION: u32 = 0;
        const POSITION_ATTRIBUTE_SIZE: i32 = 3;
        const COLOR_ATTRIBUTE_LOCATION: u32 = 1;
        const COLOR_ATTRIBUTE_SIZE: i32 = 3;
        const TEXTURE_ATTRIBUTE_LOCATION: u32 = 2;
        const TEXTURE_ATTRIBUTE_SIZE: i32 = 2;
        const BLOCK_SIZE: usize =
            (POSITION_ATTRIBUTE_SIZE + COLOR_ATTRIBUTE_SIZE + TEXTURE_ATTRIBUTE_SIZE) as _;

        unsafe {
            gl.EnableVertexAttribArray(POSITION_ATTRIBUTE_LOCATION);
            gl.VertexAttribPointer(
                POSITION_ATTRIBUTE_LOCATION,
                POSITION_ATTRIBUTE_SIZE,
                gl::FLOAT,
                IS_NORMALIZED,
                memsize::<f32>(BLOCK_SIZE),
                // data for position of vertex is the FIRST 3 float numbers.
                std::ptr::null(),
            );

            gl.EnableVertexAttribArray(COLOR_ATTRIBUTE_LOCATION);
            gl.VertexAttribPointer(
                COLOR_ATTRIBUTE_LOCATION,
                COLOR_ATTRIBUTE_SIZE,
                gl::FLOAT,
                IS_NORMALIZED,
                memsize::<f32>(BLOCK_SIZE),
                // data for position of vertex is the SECOND 3 float numbers.
                memsize::<f32>(POSITION_ATTRIBUTE_SIZE as _) as *const _,
            );

            gl.EnableVertexAttribArray(TEXTURE_ATTRIBUTE_LOCATION);
            gl.VertexAttribPointer(
                TEXTURE_ATTRIBUTE_LOCATION,
                TEXTURE_ATTRIBUTE_SIZE,
                gl::FLOAT,
                IS_NORMALIZED,
                memsize::<f32>(BLOCK_SIZE),
                // data for position of vertex is the THIRD 2 float numbers.
                memsize::<f32>((POSITION_ATTRIBUTE_SIZE + COLOR_ATTRIBUTE_SIZE) as _) as *const _,
            );
        }
    }
}

pub type Normalized = Chromatic;
