use anyhow::Context;
use anyhow::anyhow;
use gl;
use glfw;
use glfw::{Action, Key};
use image;

mod entities;
mod render_gl;
mod buffer;
mod texture;

const WINDOW_WIDTH: i32 = 900;
const WINDOW_HEIGHT: i32 = 700;

fn build_shader_program(
    gl: &gl::Gl,
    vert: &str,
    frag: &str,
) -> anyhow::Result<render_gl::Program> {
    let vert_shader =
        render_gl::Shader::from_vert_source(gl.clone(), render_gl::Source::Filepath(vert))
            .with_context(|| format!("fail building shader {}", vert))?;
    let frag_shader =
        render_gl::Shader::from_frag_source(gl.clone(), render_gl::Source::Filepath(frag))
            .with_context(|| format!("fail building shader {}", frag))?;
    render_gl::Program::from_shaders(gl.clone(), &[vert_shader, frag_shader])
        .map_err(|e| anyhow!("fail building program: {}", e))
}

fn main() -> anyhow::Result<()> {
    // initialize a window and a context ***********************************************************
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let (mut window, events) = glfw
        .create_window(
            WINDOW_WIDTH as u32,
            WINDOW_HEIGHT as u32,
            "Draw line demo",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    let gl = gl::Gl::load_with(|s| window.get_proc_address(s) as *const _);
    glfw::Context::make_current(&mut window);
    window.set_cursor_pos_polling(true);
    // initialization ends *************************************************************************

    // load shader data ****************************************************************************
    let mut line = entities::VertLine::new(gl.clone());
    let triangle = entities::Triangle::new(gl.clone());
    unsafe {
        gl.Viewport(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT);
        gl.ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    // shader begins *******************************************************************************
    let vertex_chromatic_program = build_shader_program(
        &gl,
        "assets/shaders/vertex_chromatic.vert",
        "assets/shaders/vertex_chromatic.frag",
    ).context("fail building vertex chromatic program")?;

    let point_program = build_shader_program(
        &gl,
        "assets/shaders/point.vert",
        "assets/shaders/point.frag",
    ).context("fail building point program")?;

    let vertex_textured_program = build_shader_program(
        &gl,
        "assets/shaders/vertex_textured.vert",
        "assets/shaders/vertex_textured.frag",
    ).context("fail building vertex texture program")?;
    // shader ends ********************************************************************************

    // texture begins
    let wallimg = image::open("assets/textures/wall.jpg").context("fail loading")?.into_rgb8();
    let wall_texture = texture::Texture::new(gl.clone(), wallimg.as_raw(), wallimg.dimensions());
    // texture ends

    unsafe {
        let err = gl.GetError();
        if err != gl::NO_ERROR {
            panic!("opengl error: {}", err);
        }
    }

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                // catch mouse events **************************************************************
                glfw::WindowEvent::CursorPos(xpos, _ypos) => {
                    line.x = (xpos as f32 / WINDOW_WIDTH as f32) * 2. - 1.;
                    line.update();
                }
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                }
                _ => {}
            }
        }

        // drawing begins **************************************************************************
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT);
        }

        vertex_textured_program.set_used();
        unsafe {
            triangle.bind();
            gl.DrawArrays(gl::TRIANGLES, 0, 3);
            triangle.unbind();
        }

        point_program.set_used();
        unsafe {
            line.bind();
            gl.DrawArrays(gl::LINES, 0, 2);
            line.unbind();
        }

        unsafe {
            let err = gl.GetError();
            if err != gl::NO_ERROR {
                panic!("opengl error: {}", err);
            }
        }

        // drawing ends ****************************************************************************

        glfw::Context::swap_buffers(&mut window);
    }
    Ok(())
}
