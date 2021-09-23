use anyhow::Context;
use cgmath::Matrix4;

use crate::domain::shader;
use crate::domain::shader::Uniform::Mat4;

/// 0000
/// ^^^^
/// ||||- forward
/// |||-- backward
/// ||--- left
/// |---- right
pub struct MoveBitMap(u16);

pub enum Way {
    Forward,
    Backward,
    Left,
    Right,
}

impl MoveBitMap {
    pub fn set(&self, value: Way) -> Self {
        match value {
            Way::Forward => Self(self.0 | 0b0001),
            Way::Backward => Self(self.0 | 0b0010),
            Way::Left => Self(self.0 | 0b0100),
            Way::Right => Self(self.0 | 0b1000),
        }
    }

    pub fn unset(&self, value: Way) -> Self {
        match value {
            Way::Forward => Self(self.0 & 0b1110),
            Way::Backward => Self(self.0 & 0b1101),
            Way::Left => Self(self.0 & 0b1011),
            Way::Right => Self(self.0 & 0b0111),
        }
    }

    pub fn has(&self, value: Way) -> bool {
        match value {
            Way::Forward => self.0 & 0b0001 > 0,
            Way::Backward => self.0 & 0b0010 > 0,
            Way::Left => self.0 & 0b0100 > 0,
            Way::Right => self.0 & 0b1000 > 0,
        }
    }
}

impl Default for MoveBitMap {
    fn default() -> Self {
        Self(0)
    }
}

pub fn set_transformations<B>(
    program: &mut B,
    model: Matrix4<f32>,
    view: Matrix4<f32>,
    projection: Matrix4<f32>,
) -> anyhow::Result<()>
where 
    B: shader::SetUniform,
{
    program
        .set_uniform("model", Mat4(&model.as_ref() as &[f32; 16]))
        .context("fail to set model matrix to vertex_textured_program")?;
    program
        .set_uniform("view", Mat4(&view.as_ref() as &[f32; 16]))
        .context("fail to set view matrix to vertex_textured_program")?;
    program
        .set_uniform("projection", Mat4(&projection.as_ref() as &[f32; 16]))
        .context("fail to set projection matrix to vertex_textured_program")?;
    Ok(())
}
