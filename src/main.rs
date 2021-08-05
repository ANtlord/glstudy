use anyhow::Context;
use cgmath::{Deg, Matrix4, One, Vector4};
use gl;
use glfw;
use glfw::{Action, Key};

use std::time::{Duration, SystemTime, UNIX_EPOCH};

mod camera;
mod entities;
mod init;
mod movement;
mod render_gl;
mod shader_program_container;
mod texture;

use movement::{MoveBitMap, Way};
use shader_program_container::ShaderProgramContainer;

const WINDOW_WIDTH: i32 = 900;
const WINDOW_HEIGHT: i32 = 700;
const WINDOW_ASPECT_RATIO: f32 = WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32;
const CAMERA_SENSETIVITY: f32 = 0.1;
const CAMERA_SPEED: f32 = 20.;
const CUBE_VERTEX_COUNT: i32 = 36;

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

fn main() -> anyhow::Result<()> {
    // initialize a window and a context ***********************************************************
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).context("fail initiazing GLFW")?;
    let (mut window, events) = init::window(&glfw, WINDOW_WIDTH as _, WINDOW_HEIGHT as _)
        .context("fail creating windows")?;
    let gl = gl::Gl::load_with(|s| window.get_proc_address(s) as *const _);
    // initialization ends *************************************************************************

    // load shader data ****************************************************************************
    let cube = entities::normalized_cube(gl.clone());
    let ground = entities::Shape::parallelogram(gl.clone());
    unsafe {
        gl.Viewport(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT);
        gl.ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    let mut camera = init::standard_camera(WINDOW_ASPECT_RATIO);
    let model = Matrix4::one();
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
    light_shader
        .set_uniform("lightPosition", render_gl::Uniform::Vec3(&[2.0f32, 3., 1.]))
        .context("fail setting lightPosition for light_shader")?;

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

    let mut texture_shader = shader_program_container
        .get_vertex_textured_program()
        .context("fail getting textured shader")?;
    let ground_model = Matrix4::from_translation([0.0f32, -1.0, 0.].into())
        * Matrix4::from_nonuniform_scale(20.0f32, 0., 20.)
        * Matrix4::from_angle_x(Deg(90.0f32));
    set_transformations(&mut texture_shader, ground_model, camera.view(), camera.projection())?;
    let _wallimg = image::open("assets/textures/wall.jpg").context("fail loading")?.into_rgb8();
    let wallimg = image::open("assets/textures/wall.jpg").context("fail loading")?.into_rgb8();
    let _wall_texture = texture::Texture::new(gl.clone(), wallimg.as_raw(), wallimg.dimensions());
    // shader ends ********************************************************************************

    let cube_position_array = [[0.0f32, 0.0, 0.0], [2.0, 5.0, -15.0]];

    unsafe {
        gl.Enable(gl::DEPTH_TEST);
        match gl.GetError() {
            gl::NO_ERROR => (),
            err => panic!("opengl error: {}", err),
        }
    }

    let position = [0.0f32, 3., 10.];
    let rot_y = 45.0f32;
    let mut rot_z = 0.0f32;
    let mut last_cursor_pos = None;
    let mut frame_rate = FrameRate::default();
    let mut camera_move = MoveBitMap::default();
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

                    camera.rotate(-yoffset, xoffset);
                    last_cursor_pos = Some((xpos, ypos));
                }

                glfw::WindowEvent::Key(Key::W, _, Action::Press, _) => {
                    camera_move = camera_move.set(Way::Forward);
                }
                glfw::WindowEvent::Key(Key::A, _, Action::Press, _) => {
                    camera_move = camera_move.set(Way::Left);
                }
                glfw::WindowEvent::Key(Key::S, _, Action::Press, _) => {
                    camera_move = camera_move.set(Way::Backward);
                }
                glfw::WindowEvent::Key(Key::D, _, Action::Press, _) => {
                    camera_move = camera_move.set(Way::Right);
                }

                glfw::WindowEvent::Key(Key::W, _, Action::Release, _) => {
                    camera_move = camera_move.unset(Way::Forward);
                }
                glfw::WindowEvent::Key(Key::A, _, Action::Release, _) => {
                    camera_move = camera_move.unset(Way::Left);
                }
                glfw::WindowEvent::Key(Key::S, _, Action::Release, _) => {
                    camera_move = camera_move.unset(Way::Backward);
                }
                glfw::WindowEvent::Key(Key::D, _, Action::Release, _) => {
                    camera_move = camera_move.unset(Way::Right);
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

        if camera_move.is(Way::Forward) {
            camera.go(camera::Way::Forward(camera_speed));
        }

        if camera_move.is(Way::Backward) {
            camera.go(camera::Way::Backward(camera_speed));
        }

        if camera_move.is(Way::Left) {
            camera.go(camera::Way::Left(camera_speed));
        }

        if camera_move.is(Way::Right) {
            camera.go(camera::Way::Right(camera_speed));
        }

        // drawing begins **************************************************************************
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        let light_model_view = {
            let pos = Matrix4::from_translation(position.into());
            rot_z += frame_time_secs * 20.;
            let mut rot = Matrix4::from_angle_z(Deg(rot_z));
            rot = Matrix4::from_angle_y(Deg(rot_y)) * rot;
            rot * pos
        };

        unsafe {
            use render_gl::Uniform::{Mat4, Vec3};

            // cube is an instance of entities::Shape which data is buffer in the graphics system.
            // Its building determines way to draw it (glDrawArrays or glDrawElements).
            //
            // Question #1: Is its responsibility to provide way of drawing?
            cube.bind();
            {
                light_shader.set_used();
                light_x += frame_time_secs;

                let pos = Vector4::new(0.0f32, 0., 0., 1.);
                let pos = light_model_view * pos;
                light_shader
                    .set_uniform("lightPosition", Vec3(&[pos.x, pos.y, pos.z]))
                    .context("fail setting lightPosition for light_shader")?;

                let pos = Matrix4::from_translation(cube_position_array[0].into());
                set_transformations(&mut light_shader, pos, camera.view(), camera.projection())
                    .context("fail transforming light_shader")?;
                gl.DrawArrays(gl::TRIANGLES, 0, CUBE_VERTEX_COUNT);
            }

            {
                lamp_shader.set_used();
                set_transformations(
                    &mut lamp_shader,
                    light_model_view,
                    camera.view(),
                    camera.projection(),
                )
                .context("fail transforming light_shader")?;
                gl.DrawArrays(gl::TRIANGLES, 0, CUBE_VERTEX_COUNT);
            }
            cube.unbind();

            ground.bind();
            texture_shader.set_used();
            texture_shader
                .set_uniform("view", Mat4(camera.view().as_ref() as &[f32; 16]))
                .context("fail setting view matrix for texture_shader")?;
            texture_shader
                .set_uniform("projection", Mat4(camera.projection().as_ref() as &[f32; 16]))
                .context("fail setting projection matrix for texture_shader")?;
            gl.DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as *const _);
            ground.unbind();
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
