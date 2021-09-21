/// Project abstractions, relations, basic primitives.

// TODO: propably primitive should live in another place.

pub mod mesh {
    use super::texture;
    pub trait TextureBind {
        fn texture_bind<'a, B: 'a, I>(&mut self, _: I) -> anyhow::Result<()>
        where
            B: texture::Bind,
            I: Iterator<Item = &'a (B, texture::Kind)>;
    }

    pub trait Draw {
        fn draw(&self, _: &impl super::vdata::BindUnbind, _: &impl Mesh);
    }

    pub trait Mesh {
        fn vertex_count(&self) -> i32;
        fn index_count(&self) -> i32;
    }
}

pub mod texture {
    pub struct Unit(u32);

    impl Unit {
        pub fn zero() -> Self {
            Unit(0)
        }

        pub fn new(v: u32) -> anyhow::Result<Self> {
            if v < gl::TEXTURE31 {
                Ok(Unit(v))
            } else {
                anyhow::bail!("31 texture units are supported. {} is invalid", v)
            }
        }

        pub fn gl_value(&self) -> gl::types::GLenum {
            self.0 + gl::TEXTURE0
        }
    }

    pub trait Bind {
        fn bind(&self, _: Unit);
    }

    pub enum Kind {
        Diffuse,
        Specular,
    }
}

pub mod vdata {
    pub trait BindUnbind {
        fn bind(&self);
        fn unbind(&self);
    }
}
