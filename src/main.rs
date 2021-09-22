use anyhow::Context;
use cgmath::{Deg, InnerSpace, Matrix4, Vector3, Vector4};
use gl;
use glfw;
use glfw::{Action, Key};

use std::path::Path;
use std::ptr;

mod camera;
mod domain;
mod entities;
mod frame_rate;
mod init;
mod movement;
mod render_gl;
mod shader_program_container;
mod texture;
mod util;

use frame_rate::FrameRate;
use movement::{set_transformations, MoveBitMap, Way};
use shader_program_container::{ShaderProgramBuilder, ShaderProgramContainer};
use util::gl as glutil;

const WINDOW_WIDTH: i32 = 900;
const WINDOW_HEIGHT: i32 = 700;
const WINDOW_ASPECT_RATIO: f32 = WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32;
const CAMERA_SENSETIVITY: f32 = 0.1;
const CAMERA_SPEED: f32 = 5.;
const CUBE_VERTEX_COUNT: i32 = 36;

fn move_camera(camera: &mut camera::Camera, speed: f32, directions: &MoveBitMap) {
    if directions.has(Way::Forward) {
        camera.go(camera::Way::Forward(speed));
    } else if directions.has(Way::Backward) {
        camera.go(camera::Way::Backward(speed));
    } else if directions.has(Way::Left) {
        camera.go(camera::Way::Left(speed));
    } else if directions.has(Way::Right) {
        camera.go(camera::Way::Right(speed));
    }
}

/// Rotates around Z (rolls)
struct Roller {
    position: [f32; 3],
    rot_y: f32,
    rot_z: f32,
}

impl Roller {
    fn new(position: [f32; 3], rot_y: f32) -> Self {
        Self { position, rot_y, rot_z: 0.0f32 }
    }

    fn add_rot_z(&mut self, delta: f32) {
        self.rot_z += delta;
    }

    fn model_matrix(&self) -> Matrix4<f32> {
        let pos = Matrix4::from_translation(self.position.into());
        let mut rot = Matrix4::from_angle_z(Deg(self.rot_z));
        rot = Matrix4::from_angle_y(Deg(self.rot_y)) * rot;
        rot * pos
    }
}

fn new_texture<T: AsRef<Path>>(gl: &gl::Gl, filepath: &T) -> anyhow::Result<texture::Texture> {
    let image = image::open(filepath).context("fail loading image")?.into_rgb8();
    let tex = texture::Texture::new(gl.clone(), image.as_raw(), image.dimensions());
    Ok(tex)
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
    unsafe {
        gl.Viewport(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT);
        gl.ClearColor(0.0, 0.0, 0.0, 1.0);
    }

    let mut camera = init::standard_camera(WINDOW_ASPECT_RATIO);
    // shader begins *******************************************************************************
    let shader_program_builder = ShaderProgramBuilder::new(gl.clone());
    let mut shader_container = ShaderProgramContainer::new(&shader_program_builder, &camera)
        .context("fail creating shader container")?;

    let wall_texture =
        new_texture(&gl, &"assets/textures/wall.jpg").context("fail loading wall.jpg")?;
    let cube_diffuse_texture = new_texture(&gl, &"assets/textures/container2.png")
        .context("fail loading container2.png")?;
    let cube_specular_texture =
        new_texture(&gl, &"assets/textures/lighting_maps_specular_color.png")
            .context("fail loading container2_specular.png")?;

    // shader ends ********************************************************************************
    // some meshes ******************************************
    let cube = entities::normalized_cube(gl.clone(), cube_diffuse_texture, cube_specular_texture)
        .context("fail building cube")?;
    let ground =
        entities::parallelogram(gl.clone(), wall_texture).context("fail building ground")?;
    // some meshes ******************************************

    let second_lamp_pos = [2.0, 5.0, -15.0];
    let cube_position_array = [
        [0.0, 0.0, 0.0],
        [-1.5, -2.2, -2.5],
        [-3.8, -2.0, -12.3],
        [2.4, -0.4, -3.5],
        [-1.7, 3.0, -7.5],
        [1.3, -2.0, -2.5],
        [1.5, 2.0, -2.5],
        [1.5, 0.2, -1.5],
        [-1.3, 1.0, -1.5],
    ];

    unsafe {
        gl.Enable(gl::DEPTH_TEST);
        glutil::get_err(&gl).context("fail enabling delth test")?;
    }

    let mut light_roller = Roller::new([0.0f32, 3., 10.], 45.);
    let mut last_cursor_pos = None;
    let mut frame_rate = FrameRate::default();
    let mut camera_move = MoveBitMap::default();

    let cube_draw = entities::mesh::DrawArrays { gl: gl.clone(), mode: gl::TRIANGLES };
    let ground_draw = entities::mesh::DrawElements { gl: gl.clone(), mode: gl::TRIANGLES };

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

        light_roller.add_rot_z(frame_time_secs * 20.);
        let light_model_view = light_roller.model_matrix();
        use domain::shader::{
            SetUniform, SetUsed,
            Uniform::{Mat4, Vec3},
        };

        shader_container.light_shader.set_used();
        // (0, 0, 0, 1) - it's just the center of the space. After the multiplication it
        // has the same position as the lamp has.
        let pos = light_model_view * Vector4::new(0.0f32, 0., 0., 1.);
        shader_container
            .light_shader
            .set_uniforms([
                ("light.position", Vec3(&[pos.x, pos.y, pos.z])),
                ("viewPosition", Vec3(&camera.position())),
                ("view", Mat4(&camera.view().as_ref() as &[f32; 16])),
                ("projection", Mat4(&camera.projection().as_ref() as &[f32; 16])),
            ])
            .context("fail changing uniforms for light_shader")?;

        for (i, cube_pos) in cube_position_array.iter().enumerate() {
            let pos = Matrix4::from_translation(cube_pos.clone().into());
            let rot = Matrix4::from_axis_angle(
                Vector3::new(0.5, 0.5, 0.5f32).normalize(),
                Deg(20.0f32 * i as f32),
            );

            let model = pos * rot;
            shader_container
                .light_shader
                .set_uniform("model", Mat4(&model.as_ref() as &[f32; 16]))
                .context("fail to set model matrix to vertex_textured_program")?;

            let mut texbind = texture::Binder {
                gl: gl.clone(),
                shader_program: &mut shader_container.light_shader,
            };

            // TODO: here one shader is bound and unbound repeatedly. Find out how long it is.
            cube.draw(&mut texbind, &cube_draw).with_context(|| {
                format!("fail drawing cube with shader id = {}", shader_container.light_shader.id())
            })?;
        }

        {
            shader_container.lamp_shader.set_used();
            set_transformations(
                &mut shader_container.lamp_shader,
                light_model_view,
                camera.view(),
                camera.projection(),
            )
            .context("fail transforming light_shader")?;
            cube.draw(&mut domain::mesh::NoTexture, &cube_draw)
                .context("fail drawing cube lamp 1")?;
        }

        {
            let lamp_shader_other_model = Matrix4::from_translation(second_lamp_pos.into());
            shader_container.lamp_shader_other.set_used();
            set_transformations(
                &mut shader_container.lamp_shader_other,
                lamp_shader_other_model,
                camera.view(),
                camera.projection(),
            )
            .context("fail transforming light_shader_other")?;
            cube.draw(&mut domain::mesh::NoTexture, &cube_draw)
                .context("fail drawing cube lamp 2")?;
        }

        {
            shader_container.texture_shader.set_used();
            shader_container
                .texture_shader
                .set_uniforms([
                    ("projection", Mat4(camera.projection().as_ref() as &[f32; 16])),
                    ("view", Mat4(camera.view().as_ref() as &[f32; 16])),
                ])
                .context("fail changing unifroms of texture_shader")?;

            let mut texbind = texture::Binder {
                gl: gl.clone(),
                shader_program: &mut shader_container.texture_shader,
            };

            ground.draw(&mut texbind, &ground_draw).context("fail drawing ground")?;
        }

        glutil::get_err(&gl).context("drawing frame error")?;
        // drawing ends ****************************************************************************
        glfw::Context::swap_buffers(&mut window);
    }
    Ok(())
}
