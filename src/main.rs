use anyhow::Context;
use cgmath::{Deg, Matrix4, One, Vector4};
use gl;
use glfw;
use glfw::{Action, Key};

use std::ptr;
use std::time::{Duration, SystemTime};

mod camera;
mod entities;
mod init;
mod movement;
mod render_gl;
mod shader_program_container;
mod texture;

use movement::{set_transformations, MoveBitMap, Way};
use shader_program_container::ShaderProgramContainer;

const WINDOW_WIDTH: i32 = 900;
const WINDOW_HEIGHT: i32 = 700;
const WINDOW_ASPECT_RATIO: f32 = WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32;
const CAMERA_SENSETIVITY: f32 = 0.1;
const CAMERA_SPEED: f32 = 5.;
const CUBE_VERTEX_COUNT: i32 = 36;

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

fn move_camera(camera: &mut camera::Camera, speed: f32, directions: &MoveBitMap) {
    if directions.has(Way::Forward) {
        camera.go(camera::Way::Forward(speed));
    }

    if directions.has(Way::Backward) {
        camera.go(camera::Way::Backward(speed));
    }

    if directions.has(Way::Left) {
        camera.go(camera::Way::Left(speed));
    }

    if directions.has(Way::Right) {
        camera.go(camera::Way::Right(speed));
    }
}

fn set_light_shader_uniforms(light_shader: &mut render_gl::Program) -> anyhow::Result<()> {
    let light_shader_uniforms = [
        ("light.ambient", render_gl::Uniform::Vec3(&[0.2, 0.2, 0.2])),
        ("light.diffuse", render_gl::Uniform::Vec3(&[0.5, 0.5, 0.5])),
        ("light.specular", render_gl::Uniform::Vec3(&[1.0f32, 1., 1.])),
        ("material.ambient", render_gl::Uniform::Vec3(&[1.0, 0.5, 0.31])),
        ("material.diffuse", render_gl::Uniform::Vec3(&[1.0, 0.5, 0.31])),
        ("material.specular", render_gl::Uniform::Vec3(&[0.5, 0.5, 0.5])),
        ("material.shininess", render_gl::Uniform::Float32(32.)),
    ];
    light_shader.set_uniforms(light_shader_uniforms).context("fail setting initial uniforms")
}

fn glerr(gl: &gl::Gl) -> anyhow::Result<()> {
    unsafe {
        match gl.GetError() {
            gl::NO_ERROR => Ok(()),
            err => anyhow::bail!("opengl error: {}", err),
        }
    }
}

fn ground_model_transformations() -> Matrix4<f32> {
    Matrix4::from_translation([0.0f32, -1.0, 0.].into())
        * Matrix4::from_nonuniform_scale(20.0f32, 0., 20.)
        * Matrix4::from_angle_x(Deg(90.0f32))
}

fn main() -> anyhow::Result<()> {
    // initialize a window and a context ***********************************************************
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).context("fail initiazing GLFW")?;
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
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
    let mut light_shader = {
        let mut shader_program =
            shader_program_container.get_light_program().context("fail getting light shader")?;
        set_transformations(&mut shader_program, model, camera.view(), camera.projection())?;
        set_light_shader_uniforms(&mut shader_program)?;
        shader_program
    };

    let mut lamp_shader =
        shader_program_container.get_lamp_program().context("fail getting lamp shader")?;
    let mut lamp_shader_other =
        shader_program_container.get_lamp_program().context("fail getting lamp shader")?;
    set_transformations(&mut lamp_shader_other, model, camera.view(), camera.projection())?;
    let mut texture_shader = shader_program_container
        .get_vertex_textured_program()
        .context("fail getting textured shader")?;
    set_transformations(
        &mut texture_shader,
        ground_model_transformations(),
        camera.view(),
        camera.projection(),
    )?;

    let wallimg = image::open("assets/textures/wall.jpg").context("fail loading")?.into_rgb8();
    let _wall_texture = texture::Texture::new(gl.clone(), wallimg.as_raw(), wallimg.dimensions());
    // shader ends ********************************************************************************

    let cube_position_array = [[0.0f32, 0.0, 0.0], [2.0, 5.0, -15.0]];
    unsafe {
        gl.Enable(gl::DEPTH_TEST);
        glerr(&gl).context("fail enabling delth test")?;
    }

    let light_position = [0.0f32, 3., 10.];
    let light_rot_y = 45.0f32;
    let mut light_rot_z = 0.0f32;
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

        move_camera(&mut camera, camera_speed, &camera_move);
        // drawing begins **************************************************************************
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        let light_model_view = {
            let pos = Matrix4::from_translation(light_position.into());
            light_rot_z += frame_time_secs * 20.;
            let mut rot = Matrix4::from_angle_z(Deg(light_rot_z));
            rot = Matrix4::from_angle_y(Deg(light_rot_y)) * rot;
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
                // (0, 0, 0, 1) - it's just the center of the space. After the multiplication it
                // has the same position as the lamp has.
                let pos = light_model_view * Vector4::new(0.0f32, 0., 0., 1.);
                light_shader
                    .set_uniform("light.position", Vec3(&[pos.x, pos.y, pos.z]))
                    .context("fail setting light.position for light_shader")?;

                let view_position = camera.position();
                light_shader
                    .set_uniform("viewPosition", Vec3(&view_position))
                    .context("fail setting viewPosition for light_shader")?;

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

            {
                let lamp_shader_other_model =
                    Matrix4::from_translation(cube_position_array[1].into());
                lamp_shader_other.set_used();
                set_transformations(
                    &mut lamp_shader_other,
                    lamp_shader_other_model,
                    camera.view(),
                    camera.projection(),
                )
                .context("fail transforming light_shader_other")?;
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
            gl.DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
            ground.unbind();
        }

        glerr(&gl).context("drawing frame error")?;
        // drawing ends ****************************************************************************
        glfw::Context::swap_buffers(&mut window);
    }
    Ok(())
}
