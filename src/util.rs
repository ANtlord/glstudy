pub mod gl {
    pub fn get_err(gl: &gl::Gl) -> anyhow::Result<()> {
        unsafe {
            match gl.GetError() {
                gl::NO_ERROR => Ok(()),
                err => anyhow::bail!("opengl error: {}", err),
            }
        }
    }
}
