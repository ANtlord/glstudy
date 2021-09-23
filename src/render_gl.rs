use anyhow::Context;
use gl;

use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::Read;
use std::marker::PhantomData;
use std::ptr;

use crate::domain::shader;

pub struct Program {
    gl: gl::Gl,
    id: gl::types::GLuint,
}

impl Program {
    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    pub fn from_shaders(gl: gl::Gl, shaders: &[Shader]) -> anyhow::Result<Program, String> {
        let program_id = unsafe { gl.CreateProgram() };
        for shader in shaders {
            unsafe { gl.AttachShader(program_id, shader.id()) };
        }

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl.LinkProgram(program_id);
            gl.GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let len = unsafe {
                let mut len: gl::types::GLint = 0;
                gl.GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
                len
            };

            let error = create_whitespace_cstring_with_len(len as usize);
            unsafe {
                gl.GetProgramInfoLog(
                    program_id,
                    len,
                    ptr::null_mut(),
                    error.as_ptr() as *mut _,
                );
            }

            return Err(error.to_string_lossy().into_owned());
        }

        for shader in shaders {
            unsafe {
                gl.DetachShader(program_id, shader.id());
            }
        }

        Ok(Program { gl, id: program_id })
    }

    // pub fn attrib_location(&self, name: &str) -> anyhow::Result<gl::types::GLint> {
    //     unsafe {
    //         let name_c = CString::new(name)
    //             .with_context(|| format!("fail making C string from {}", name))?;
    //         let loc = self.gl.GetAttribLocation(self.id(), name_c.as_ptr());
    //         if self.gl.GetError() != gl::NO_ERROR {
    //             anyhow::bail!("fail getting attribute locations `{}`", name)
    //         } else {
    //             Ok(loc)
    //         }
    //     }
    // }

    // pub fn active_attrib_count(&self) -> anyhow::Result<gl::types::GLint> {
    //     unsafe {
    //         let mut count = 0;
    //         self.gl.GetProgramiv(self.id(), gl::ACTIVE_ATTRIBUTES, &mut count);
    //         if self.gl.GetError() != gl::NO_ERROR {
    //             anyhow::bail!("fail counting active attribute locations")
    //         } else {
    //             Ok(count)
    //         }
    //     }
    // }

    // pub fn id(&self) -> gl::types::GLuint {
    //     self.id
    // }

    fn assert_is_used(&self) -> anyhow::Result<()> {
        let prog = unsafe {
            let mut prog = 0;
            self.gl.GetIntegerv(gl::CURRENT_PROGRAM, &mut prog);
            prog as gl::types::GLuint
        };

        if self.id != prog {
            Err(anyhow::anyhow!("shader program is not active"))
        } else {
            Ok(())
        }
    }
}

pub struct ProgramCtl<'p, 'm> {
    program: &'p mut Program,
    master: PhantomData<&'m mut ProgramCtlMaster>,
}

impl<'p, 'm> shader::SetUniform for ProgramCtl<'p, 'm> {
    fn set_uniform<T: AsRef<str>>(&mut self, key: T, value: shader::Uniform) -> anyhow::Result<()> {
        self.program.set_uniform(key, value)
    }
}

/// Denies mutual activation of shaders in compile time.
pub struct ProgramCtlMaster;

impl ProgramCtlMaster {
    pub fn set_used<'me, 'prg>(&'me mut self, target: &'prg mut Program) -> ProgramCtl<'prg, 'me> {
        shader::SetUsed::set_used(target);
        ProgramCtl { program: target, master: PhantomData }
    }
}

impl shader::SetUsed for Program {
    fn set_used(&self) {
        unsafe {
            self.gl.UseProgram(self.id);
        }
    }
}

impl shader::SetUniform for Program {
    /// Don't use float64. Perhaps it's worth to consider T: Into<[f32; 3]> | Into<f32; 4>
    fn set_uniform<T: AsRef<str>>(&mut self, key: T, value: shader::Uniform) -> anyhow::Result<()> {
        self.assert_is_used()?;
        let key_c = CString::new(key.as_ref())
            .with_context(|| format!("fail building C-string from {}", key.as_ref()))?;
        unsafe {
            let loc = self.gl.GetUniformLocation(self.id, key_c.as_ptr());
            if loc == -1 {
                anyhow::bail!(
                    "location of uniform `{}` is not found in shader with id = {}",
                    key.as_ref(),
                    self.id
                )
            }

            use shader::Uniform;
            match value {
                Uniform::Mat4(x) => self.gl.UniformMatrix4fv(loc, 1, gl::FALSE, x.as_ptr() as _),
                Uniform::Vec3(x) => self.gl.Uniform3fv(loc, 1, x.as_ptr() as _),
                Uniform::Float32(x) => self.gl.Uniform1f(loc, x as _),
                Uniform::Int(x) => self.gl.Uniform1i(loc, x as _),
            }
        }
        Ok(())
    }
}

impl Clone for Program {
    fn clone(&self) -> Self {
        Self { gl: self.gl.clone(), id: self.id }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe { self.gl.DeleteProgram(self.id) };
    }
}

pub struct Shader {
    id: gl::types::GLuint,
    gl: gl::Gl,
}

pub enum Source<'a> {
    Filepath(&'a str),
}

impl Shader {
    pub fn from_source(
        gl: gl::Gl,
        source: Source,
        kind: gl::types::GLenum,
    ) -> anyhow::Result<Shader> {
        use Source::*;

        let id = match source {
            Filepath(filepath) => {
                let mut file = File::open(filepath)?;
                let mut buf = String::default();
                file.read_to_string(&mut buf)?;
                let source = CString::new(buf.as_str())?;
                shader_from_source(&gl, &source, kind)?
            }
        };
        Ok(Shader { gl, id })
    }

    pub fn from_vert_source(gl: gl::Gl, source: Source) -> anyhow::Result<Shader> {
        Shader::from_source(gl, source, gl::VERTEX_SHADER)
    }

    pub fn from_frag_source(gl: gl::Gl, source: Source) -> anyhow::Result<Shader> {
        Shader::from_source(gl, source, gl::FRAGMENT_SHADER)
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteShader(self.id);
        }
    }
}

fn shader_from_source(
    gl: &gl::Gl,
    source: &CStr,
    kind: gl::types::GLenum,
) -> anyhow::Result<gl::types::GLuint> {
    let id = unsafe { gl.CreateShader(kind) };
    unsafe {
        gl.ShaderSource(id, 1, &source.as_ptr(), ptr::null());
        gl.CompileShader(id);
    }

    let mut success: gl::types::GLint = 1;
    unsafe {
        gl.GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl.GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }

        let error = create_whitespace_cstring_with_len(len as usize);
        unsafe {
            gl.GetShaderInfoLog(
                id,
                len,
                ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar,
            );
        }

        let errtext = error.to_string_lossy().into_owned();
        return Err(anyhow::anyhow!("{}", errtext));
    }

    Ok(id)
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}
