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

    pub struct NoTexture;

    impl TextureBind for NoTexture {
        fn texture_bind<'a, B: 'a, I>(&mut self, _: I) -> anyhow::Result<()>
        where
            B: texture::Bind,
            I: Iterator<Item = &'a (B, texture::Kind)> {
                Ok(())
        }
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

pub mod shader {
    // TODO: use static arrays of proper sizes instead of slices.
    #[derive(Clone, Copy)]
    pub enum Uniform<'a> {
        Mat4(&'a [f32]), // 16
        Vec3(&'a [f32]), // 3
        Float32(f32),
        Int(i32),
    }

    pub trait SetUniform {
        fn set_uniform<T: AsRef<str>>(&mut self, key: T, value: Uniform) -> anyhow::Result<()>;
    }

    pub trait SetUsed {
        fn set_used(&self);
    }

    pub fn set_uniforms<'a, K, S>(shader: &mut impl SetUniform, args: S) -> anyhow::Result<()>
    where
        K: AsRef<str>,
        S: AsRef<[(K, Uniform<'a>)]>,
    {
        args.as_ref().iter().map(|(k, v)| shader.set_uniform(k, *v)).collect()
    }

}
