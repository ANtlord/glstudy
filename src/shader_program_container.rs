use gl;

use anyhow::anyhow;
use anyhow::Context;

use crate::camera::Camera;
use crate::movement::set_transformations;
use crate::render_gl;
use cgmath::{Deg, Matrix4, One, Rad};

#[allow(unused)]
mod shader_paths {
    pub static VERTEX_TEXTURED_VERT: &str = "assets/shaders/vertex_textured.vert";
    pub static VERTEX_TEXTURED_FRAG: &str = "assets/shaders/vertex_textured.frag";
    pub static VERTEX_CHROMATIC_VERT: &str = "assets/shaders/vertex_chromatic.vert";
    pub static VERTEX_CHROMATIC_FRAG: &str = "assets/shaders/vertex_chromatic.frag";
    pub static POINT3D_VERT: &str = "assets/shaders/point3d.vert";
    pub static POINT_VERT: &str = "assets/shaders/point.vert";
    pub static POINT_FRAG: &str = "assets/shaders/point.frag";
    pub static LIGHT_VERT: &str = "assets/shaders/light.vert";
    pub static LIGHT_FRAG: &str = "assets/shaders/light.frag";
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

pub struct ShaderProgramBuilder {
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

#[allow(unused)]
impl ShaderProgramBuilder {
    pub fn new(gl: gl::Gl) -> Self {
        Self { gl }
    }

    pub fn get_vertex_textured_program(&self) -> anyhow::Result<render_gl::Program> {
        make(&self.gl, None, shader_paths::VERTEX_TEXTURED_VERT, shader_paths::VERTEX_TEXTURED_FRAG)
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
        make(&self.gl, None, shader_paths::POINT_VERT, shader_paths::POINT_FRAG)
            .context("fail creating point program")
    }

    pub fn get_light_program(&self) -> anyhow::Result<render_gl::Program> {
        make(&self.gl, None, shader_paths::LIGHT_VERT, shader_paths::LIGHT_FRAG)
            .context("fail creating light program")
    }

    pub fn get_lamp_program(&self) -> anyhow::Result<render_gl::Program> {
        make(&self.gl, None, shader_paths::POINT3D_VERT, shader_paths::POINT_FRAG)
            .context("fail creating lamp program")
    }
}

fn set_light_shader_uniforms(light_shader: &mut render_gl::Program) -> anyhow::Result<()> {
    use std::env;
    use std::str::FromStr;
    let value = env::var("VALUE").unwrap_or("0.0".to_owned());
    let value = value.parse::<f32>().unwrap_or(0.);
    let pos = [-2., 0., value];

    let light_shader_uniforms = [
        ("light.ambient", render_gl::Uniform::Vec3(&[0.2, 0.2, 0.2])),
        ("light.diffuse", render_gl::Uniform::Vec3(&[0.5, 0.5, 0.5])),
        ("light.specular", render_gl::Uniform::Vec3(&[1.0f32, 1., 1.])),
        ("material.diffuseMap", render_gl::Uniform::Int(0)), // GL_TEXTURE0
        ("material.specularMap", render_gl::Uniform::Int(1)), // GL_TEXTURE1
        ("material.emissionMap", render_gl::Uniform::Int(2)), // GL_TEXTURE2
        // ("material.specular", render_gl::Uniform::Vec3(&[0.5, 0.5, 0.5])),
        ("material.shininess", render_gl::Uniform::Float32(32.)),

        ("spotLight.position", render_gl::Uniform::Vec3(&pos)),
        ("spotLight.direction", render_gl::Uniform::Vec3(&[1., 0.0, 0.0])),
        ("spotLight.cutoff", render_gl::Uniform::Float32(Rad::from(Deg(12.0f32)).0.cos())),
        ("spotLight.outerCutoff", render_gl::Uniform::Float32(Rad::from(Deg(15.0f32)).0.cos())),
        ("spotLight.ambient", render_gl::Uniform::Vec3(&[0.0, 0.0, 0.0])),
        ("spotLight.diffuse", render_gl::Uniform::Vec3(&[1., 1., 1.,])),
        ("spotLight.specular", render_gl::Uniform::Vec3(&[1., 1., 1.,])),
    ];
    light_shader.set_uniforms(light_shader_uniforms).context("fail setting initial uniforms")
}

fn ground_model_transformations() -> Matrix4<f32> {
    Matrix4::from_translation([0.0f32, -1.0, 0.].into())
        * Matrix4::from_nonuniform_scale(20.0f32, 0., 20.)
        * Matrix4::from_angle_x(Deg(90.0f32))
}

pub struct ShaderProgramContainer {
    pub light_shader: render_gl::Program,
    pub lamp_shader: render_gl::Program,
    pub lamp_shader_other: render_gl::Program,
    pub texture_shader: render_gl::Program,
}

impl ShaderProgramContainer {
    pub fn new(builder: &ShaderProgramBuilder, camera: &Camera) -> anyhow::Result<Self> {
        let model = Matrix4::one();
        let light_shader = {
            let mut shader_program =
                builder.get_light_program().context("fail getting light shader")?;
            set_transformations(&mut shader_program, model, camera.view(), camera.projection())?;
            set_light_shader_uniforms(&mut shader_program)?;
            shader_program
        };

        let lamp_shader = builder.get_lamp_program().context("fail getting lamp shader")?;
        let mut lamp_shader_other =
            builder.get_lamp_program().context("fail getting other lamp shader")?;
        set_transformations(&mut lamp_shader_other, model, camera.view(), camera.projection())?;
        let mut texture_shader =
            builder.get_vertex_textured_program().context("fail getting textured shader")?;
        set_transformations(
            &mut texture_shader,
            ground_model_transformations(),
            camera.view(),
            camera.projection(),
        )?;
        Ok(Self { light_shader, lamp_shader, lamp_shader_other, texture_shader })
    }
}
