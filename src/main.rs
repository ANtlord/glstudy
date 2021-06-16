use anyhow::Context;
use gl;
use glfw;
use glfw::{Action, Key};

mod render_gl;

const WINDOW_WIDTH: i32 = 900;
const WINDOW_HEIGHT: i32 = 700;

pub struct Vertex {
    inner: [f32; 3],
}

impl From<[f32; 3]> for Vertex {
    fn from(data: [f32; 3]) -> Self {
        Self { inner: data }
    }
}

impl Vertex {
    pub fn vertex_attrib_pointer() {
        unsafe {
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (3 * std::mem::size_of::<f32>()) as gl::types::GLint,
                std::ptr::null(),
            );
        }
    }
}

struct VertLine {
    x: f32,
    vbo: gl::types::GLuint,
    vao: gl::types::GLuint,
}

impl VertLine {
    fn update(&self) {
        let vertices: Vec<Vertex> = vec![
            [self.x, 1.0, 0.0].into(),
            [self.x, -1.0, 0.0].into(),
        ];

        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            buffer_dynamic_draw(&vertices);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

fn new_array_buffer() -> gl::types::GLuint {
    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
    }
    vbo
}

fn new_vertex_array() -> gl::types::GLuint {
    let mut vao = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
    }
    vao
}

fn buffer_dynamic_draw<T>(data: &[T]) {
    unsafe {
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (data.len() * std::mem::size_of::<T>()) as gl::types::GLsizeiptr,
            data.as_ptr() as *const gl::types::GLvoid,
            gl::DYNAMIC_DRAW,
        );
    }
}

fn make_line() -> VertLine {
    let vertices: Vec<Vertex> = vec![
        [-1.0, 1.0, 0.0].into(),
        [-1.0, -1.0, 0.0].into(),
    ];

    unsafe {
        let vbo = new_array_buffer();
        let vao = new_vertex_array();
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        buffer_dynamic_draw(&vertices);
        Vertex::vertex_attrib_pointer();
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);

        VertLine {
            vao,
            vbo,
            x: -1.,
        }
    }
}

fn main() -> anyhow::Result<()> {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw
        .create_window(
            WINDOW_WIDTH as u32,
            WINDOW_HEIGHT as u32,
            "Hello this is window",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    gl::load_with(|s| window.get_proc_address(s) as *const _);
    glfw::Context::make_current(&mut window);
    window.set_cursor_pos_polling(true);
    let mut line = make_line();
    unsafe {
        gl::Viewport(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    /***********************************************************************************************
     * shader begins
    ***********************************************************************************************/
    let vert_shader = render_gl::Shader::from_vert_source(
        render_gl::Source::Filepath("src/point.vert"),
    )
    .context("fail building point.vert")?;

    let frag_shader = render_gl::Shader::from_frag_source(
        render_gl::Source::Filepath("src/point.frag"),
    )
    .context("fail building src/point.frag")?;
    let program = render_gl::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();
    /***********************************************************************************************
     * shader ends
    ***********************************************************************************************/

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
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

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        program.set_used();
        unsafe {
            line.bind();
            gl::DrawArrays(gl::LINES, 0, 2);
            line.unbind();
        }

        glfw::Context::swap_buffers(&mut window);
    }
    Ok(())
}
