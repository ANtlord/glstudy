use anyhow::Context;
use cgmath::{Deg, Matrix4, Vector3};
use gl;
use glfw;
use glfw::{Action, Key};
use std::sync::mpsc::Receiver;

use std::time::{Duration, SystemTime};

mod camera;
mod entities;
mod render_gl;
mod shader_program_container;
mod texture;

use shader_program_container::ShaderProgramContainer;

const WINDOW_WIDTH: i32 = 900;
const WINDOW_HEIGHT: i32 = 700;
const WINDOW_ASPECT_RATIO: f32 = WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32;
const CAMERA_SENSETIVITY: f32 = 0.1;
const CAMERA_SPEED: f32 = 2.5;
const CUBE_VERTEX_COUNT: i32 = 36;

fn standard_camera() -> camera::Camera {
    camera::CameraOptions::new()
        .position([0.0f32, 0., 3.]) // just a little bit from the center of the world
        .front([0.0f32, 0., -1.])
        .up([0.0f32, 1., 0.])
        .yaw(-90.) // as we looking againg Z direction
        .aspect_ratio(WINDOW_ASPECT_RATIO)
        .zoom(45.)
        .fly(false)
        .build()
}

fn set_transformations(
    program: &mut render_gl::Program,
    model: Matrix4<f32>,
    view: Matrix4<f32>,
    projection: Matrix4<f32>,
) -> anyhow::Result<()> {
    use render_gl::Uniform::Mat4;
    program.set_used();
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

struct FrameRate {
    last: SystemTime,
    duration: Duration,
}

impl Default for FrameRate {
    fn default() -> Self {
        Self { last: SystemTime::now(), duration: Duration::default() }
    }
}

impl FrameRate {
    fn update(&mut self) -> anyhow::Result<()> {
        let current = SystemTime::now();
        self.duration =
            current.duration_since(self.last).context("fail getting duration between frames")?;
        self.last = current;
        Ok(())
    }
}

fn create_window(
    glfw_ctx: &glfw::Glfw,
) -> anyhow::Result<(glfw::Window, Receiver<(f64, glfw::WindowEvent)>)> {
    let (mut window, events) = glfw_ctx
        .create_window(
            WINDOW_WIDTH as u32,
            WINDOW_HEIGHT as u32,
            "Draw line demo",
            glfw::WindowMode::Windowed,
        )
        .context("Failed to create GLFW window within event listener.")?;
    window.set_cursor_pos_polling(true);
    window.set_key_polling(true);
    window.set_scroll_polling(true);
    if glfw_ctx.supports_raw_motion() {
        window.set_cursor_mode(glfw::CursorMode::Disabled);
        window.set_raw_mouse_motion(true);
    } else {
        println!("mouse raw motion is not supported");
    }

    glfw::Context::make_current(&mut window);
    Ok((window, events))
}

fn main() -> anyhow::Result<()> {
    // initialize a window and a context ***********************************************************
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).context("fail initiazing GLFW")?;
    let (mut window, events) = create_window(&glfw).context("fail creating windows")?;
    let gl = gl::Gl::load_with(|s| window.get_proc_address(s) as *const _);
    // initialization ends *************************************************************************

    // load shader data ****************************************************************************
    let cube = entities::bald_cube(gl.clone());
    unsafe {
        gl.Viewport(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT);
        gl.ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    let mut camera = standard_camera();
    let model = Matrix4::from_angle_y(Deg(0.0f32));
    // shader begins *******************************************************************************
    let shader_program_container = ShaderProgramContainer::new(gl.clone());
    let mut light_shader =
        shader_program_container.get_light_program().context("fail getting light shader")?;
    set_transformations(&mut light_shader, model, camera.view(), camera.projection())?;
    light_shader
        .set_uniform("lightColor", render_gl::Uniform::Vec3(&[1.0f32, 1., 1.]))
        .context("fail setting lightColor")?;
    light_shader
        .set_uniform("objectColor", render_gl::Uniform::Vec3(&[1.0f32, 0.5, 0.31]))
        .context("fail setting objectColor")?;

    unsafe {
        gl.Enable(gl::DEPTH_TEST);
        match gl.GetError() {
            gl::NO_ERROR => (),
            err => panic!("opengl error: {}", err),
        }
    }
    let mut lamp_shader =
        shader_program_container.get_lamp_program().context("fail getting lamp shader")?;
    set_transformations(&mut lamp_shader, model, camera.view(), camera.projection())?;
    // shader ends ********************************************************************************

    let cube_position_array = [[0.0f32, 0.0, 0.0], [2.0, 5.0, -15.0]];

    unsafe {
        gl.Enable(gl::DEPTH_TEST);
        match gl.GetError() {
            gl::NO_ERROR => (),
            err => panic!("opengl error: {}", err),
        }
    }

    let mut last_cursor_pos = None;
    let mut frame_rate = FrameRate::default();
    while !window.should_close() {
        frame_rate.update().context("fail updating frame rate")?;
        let frame_time_secs = frame_rate.duration.as_secs_f32();
        let camera_speed = CAMERA_SPEED * frame_time_secs;
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                // catch mouse events **************************************************************
                glfw::WindowEvent::CursorPos(xpos, ypos) => {
                    let (last_xpos, last_ypos) = last_cursor_pos.unwrap_or((xpos, ypos));
                    let (xoffset, yoffset) = (
                        (xpos - last_xpos) as f32 * CAMERA_SENSETIVITY,
                        (ypos - last_ypos) as f32 * CAMERA_SENSETIVITY,
                    );

                    camera.rotate(yoffset, xoffset);
                    last_cursor_pos = Some((xpos, ypos));
                }
                glfw::WindowEvent::Key(Key::W, _, Action::Repeat, _) => {
                    camera.go(camera::Way::Forward(camera_speed));
                }
                glfw::WindowEvent::Key(Key::A, _, Action::Repeat, _) => {
                    camera.go(camera::Way::Left(camera_speed));
                }
                glfw::WindowEvent::Key(Key::S, _, Action::Repeat, _) => {
                    camera.go(camera::Way::Backward(camera_speed));
                }
                glfw::WindowEvent::Key(Key::D, _, Action::Repeat, _) => {
                    camera.go(camera::Way::Right(camera_speed));
                }
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                }
                glfw::WindowEvent::Scroll(_, yoffset) => {
                    camera.shift_zoom(yoffset as _);
                }
                _ => {}
            }
        }

        // drawing begins **************************************************************************
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        unsafe {
            cube.bind();
            let pos = Matrix4::from_translation(cube_position_array[0].clone().into());
            set_transformations(&mut light_shader, pos, camera.view(), camera.projection())
                .context("fail transforming light_shader")?;
            gl.DrawElements(gl::TRIANGLES, CUBE_VERTEX_COUNT, gl::UNSIGNED_INT, 0 as *const _);

            let pos = Matrix4::from_translation(cube_position_array[1].clone().into());
            set_transformations(&mut lamp_shader, pos, camera.view(), camera.projection())
                .context("fail transforming light_shader")?;
            gl.DrawElements(gl::TRIANGLES, CUBE_VERTEX_COUNT, gl::UNSIGNED_INT, 0 as *const _);
            cube.unbind();
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
