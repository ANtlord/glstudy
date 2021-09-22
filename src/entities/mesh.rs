use anyhow;
use anyhow::Context;

use std::ptr;

use super::load_render_data_indexed;
use super::vertex::Textured;
use crate::domain;
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
