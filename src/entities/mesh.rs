use anyhow;
use anyhow::Context;

use std::ptr;

use super::load_render_data_indexed;
use super::vertex::Textured;
use crate::domain;
use crate::render_gl::Program;
use crate::render_gl::Uniform;
use crate::texture;
use crate::util::gl as glutil;

struct VertexArrayObject<'a> {
    id: gl::types::GLuint,
    gl: &'a gl::Gl,
}

impl<'a> domain::vdata::BindUnbind for VertexArrayObject<'a> {
    fn bind(&self) {
        unsafe { self.gl.BindVertexArray(self.id) };
    }

    fn unbind(&self) {
        unsafe { self.gl.BindVertexArray(0) };
    }
}

type Texture = (texture::Texture, domain::texture::Kind);

pub struct Mesh {
    vertices: Vec<Textured>,
    textures: Vec<Texture>,
    indices: Vec<u32>,
    vbo: gl::types::GLuint,
    vao: gl::types::GLuint,
    ebo: gl::types::GLuint,
    gl: gl::Gl,
}

pub struct NoTexture;

impl domain::mesh::TextureBind for NoTexture {
    fn texture_bind<'a, B: 'a, I>(&mut self, _: I) -> anyhow::Result<()>
    where
        B: domain::texture::Bind,
        I: Iterator<Item = &'a (B, domain::texture::Kind)> {
            Ok(())
    }
}

impl Mesh {
    /// shader_texture_bind - binds textures to the shader that its responsible for.
    /// draw_data - defines how to draw the mesh data.
    pub fn draw<TB, D>(&self, shader_texture_bind: &mut TB, drawer: &D) -> anyhow::Result<()>
    where
        TB: domain::mesh::TextureBind,
        D: domain::mesh::Draw,
    {
        shader_texture_bind
            .texture_bind(self.textures.iter())
            .context("fail binding texture to shader")?;
        let vao = VertexArrayObject { id: self.vao, gl: &self.gl };
        drawer.draw(&vao, self);
        glutil::get_err(&self.gl).context("fail drawing elements of mesh")
    }
}

impl domain::mesh::Mesh for Mesh {
    fn vertex_count(&self) -> i32 {
        self.vertices.len() as _
    }

    fn index_count(&self) -> i32 {
        self.indices.len() as _
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteBuffers(3, [self.vao, self.vbo, self.ebo].as_ptr());
            let textures_ids: Vec<_> = self.textures.iter().map(|(x, _)| x.id()).collect();
            self.gl.DeleteTextures(textures_ids.len() as _, textures_ids.as_ptr());
        }
    }
}

pub struct Builder {
    pub gl: gl::Gl,
}

impl Builder {
    pub fn build(
        &self,
        vertices: Vec<Textured>,
        textures: Vec<Texture>,
        indices: Vec<u32>,
    ) -> anyhow::Result<Mesh> {
        let (vao, vbo, ebo) =
            unsafe { load_render_data_indexed(&self.gl, &vertices, &indices, gl::STATIC_DRAW) };
        glutil::get_err(&self.gl).context("fail binding data")?;
        Ok(Mesh { vertices, textures, indices, vao, vbo, ebo, gl: self.gl.clone() })
    }
}

pub struct TextureBind<'a> {
    pub gl: gl::Gl,
    pub shader_program: &'a mut Program,
}

impl<'z> domain::mesh::TextureBind for TextureBind<'z> {
    /// binds textures from `texture_iter` sequentialy to `shader_program`
    ///
    /// The `shader_program` must have 
    /// uniform Material material;
    ///
    /// where
    /// Material
    /// struct Material {
    ///     sampler2D diffuseMap0;
    ///     sampler2D specularMap0;
    ///     ...
    ///     sampler2D diffuseMapN;
    ///     sampler2D specularMapN;
    /// };
    fn texture_bind<'a, B: 'a, I>(&mut self, texture_iter: I) -> anyhow::Result<()>
    where
        B: domain::texture::Bind,
        I: Iterator<Item = &'a (B, domain::texture::Kind)>,
    {
        use domain::texture::Unit;
        let mut diffuse_count = 0;
        let mut specular_count = 0;
        texture_iter
            .enumerate()
            .map(|(texture_index, (texture, kind))| {
                texture.bind(Unit::new(texture_index as _).context("fail making texture unit")?);
                let name = match kind {
                    domain::texture::Kind::Diffuse => {
                        diffuse_count += 1;
                        format!("material.diffuseMap{}", diffuse_count - 1)
                    }

                    domain::texture::Kind::Specular => {
                        specular_count += 1;
                        format!("material.specularMap{}", specular_count - 1)
                    }
                };

                self.shader_program
                    .set_uniform(&name, Uniform::Int(texture_index as _))
                    .with_context(|| format!("fail setting uniform {}", name))?;

                Ok(())
            })
            .collect::<anyhow::Result<()>>()
            .context("fail processing textures")
    }
}

pub struct DrawElements {
    pub gl: gl::Gl,
    pub mode: gl::types::GLenum,
}

impl domain::mesh::Draw for DrawElements {
    fn draw(&self, vao: &impl domain::vdata::BindUnbind, mesh: &impl domain::mesh::Mesh) {
        vao.bind();
        unsafe {
            self.gl.DrawElements(self.mode, mesh.index_count(), gl::UNSIGNED_INT, ptr::null());
        }
        vao.unbind();
    }
}

pub struct DrawArrays {
    pub gl: gl::Gl,
    pub mode: gl::types::GLenum,
}

impl domain::mesh::Draw for DrawArrays {
    fn draw(&self, vao: &impl domain::vdata::BindUnbind, mesh: &impl domain::mesh::Mesh) {
        vao.bind();
        unsafe {
            self.gl.DrawArrays(self.mode, 0, mesh.vertex_count());
        }
        vao.unbind();
    }
}
