use gl;

use anyhow::anyhow;
use anyhow::Context;

use crate::render_gl;

mod shader_paths {
    pub static VERTEX_TEXTURED_VERT: &str = "assets/shaders/vertex_textured.vert";
    pub static VERTEX_TEXTURED_FRAG: &str = "assets/shaders/vertex_textured.frag";
    pub static VERTEX_CHROMATIC_VERT: &str = "assets/shaders/vertex_chromatic.vert";
    pub static VERTEX_CHROMATIC_FRAG: &str = "assets/shaders/vertex_chromatic.frag";
    pub static POINT_VERT: &str = "assets/shaders/point.vert";
    pub static POINT_FRAG: &str = "assets/shaders/point.frag";
}

fn build_shader_program(gl: &gl::Gl, vert: &str, frag: &str) -> anyhow::Result<render_gl::Program> {
    let vert_shader =
        render_gl::Shader::from_vert_source(gl.clone(), render_gl::Source::Filepath(vert))
            .with_context(|| format!("fail building shader {}", vert))?;
    let frag_shader =
        render_gl::Shader::from_frag_source(gl.clone(), render_gl::Source::Filepath(frag))
            .with_context(|| format!("fail building shader {}", frag))?;
    render_gl::Program::from_shaders(gl.clone(), &[vert_shader, frag_shader])
        .map_err(|e| anyhow!("fail building program: {}", e))
}

pub struct ShaderProgramContainer {
    gl: gl::Gl,
}

fn make(
    gl: &gl::Gl,
    program: Option<render_gl::Program>,
    vert: &str,
    frag: &str,
) -> anyhow::Result<render_gl::Program> {
    match program {
        Some(x) => Ok(x),
        None => build_shader_program(&gl, vert, frag),
    }
}

impl ShaderProgramContainer {
    pub fn new(gl: gl::Gl) -> Self {
        Self { gl }
    }

    pub fn get_vertex_textured_program(&self) -> anyhow::Result<render_gl::Program> {
        make(
            &self.gl,
            None,
            shader_paths::VERTEX_TEXTURED_VERT,
            shader_paths::VERTEX_TEXTURED_FRAG,
        )
        .context("fail creating vertex textured program")
    }

    pub fn get_vertex_chromatic_program(&self) -> anyhow::Result<render_gl::Program> {
        make(
            &self.gl,
            None,
            shader_paths::VERTEX_CHROMATIC_VERT,
            shader_paths::VERTEX_CHROMATIC_FRAG,
        )
        .context("fail creating vertex chromatic program")
    }

    pub fn get_point_program(&self) -> anyhow::Result<render_gl::Program> {
        make(
            &self.gl,
            None,
            shader_paths::POINT_VERT,
            shader_paths::POINT_FRAG,
        )
        .context("fail creating point program")
    }
}
