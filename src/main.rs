use anyhow::Context;
use cgmath::{Deg, Matrix4};
use gl;
use glfw;
use glfw::{Action, Key};
use image;

use std::time::SystemTime;

mod buffer;
mod camera;
mod entities;
mod render_gl;
mod shader_program_container;
mod texture;

use shader_program_container::ShaderProgramContainer;

const WINDOW_WIDTH: i32 = 900;
const WINDOW_HEIGHT: i32 = 700;
const WINDOW_ASPECT_RATIO: f32 = WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32;

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

fn setup_coordinate_system(
    program: &mut render_gl::Program,
    camera: &camera::Camera,
) -> anyhow::Result<()> {
    program.set_used();
    let model = Matrix4::from_angle_y(Deg(0.0f32));
    program
        .set_uniform("model", &model.as_ref() as &[f32; 16])
        .context("fail to set model matrix to vertex_textured_program")?;
    program
        .set_uniform("view", &camera.view().as_ref() as &[f32; 16])
        .context("fail to set view matrix to vertex_textured_program")?;
    program
        .set_uniform("projection", &camera.projection().as_ref() as &[f32; 16])
        .context("fail to set projection matrix to vertex_textured_program")?;
    Ok(())
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
    window.set_scroll_polling(true);
    let gl = gl::Gl::load_with(|s| window.get_proc_address(s) as *const _);
    glfw::Context::make_current(&mut window);
    window.set_cursor_pos_polling(true);
    // initialization ends *************************************************************************

    // load shader data ****************************************************************************
    let cube = entities::Shape::cube(gl.clone());
    unsafe {
        gl.Viewport(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT);
        gl.ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    let shader_program_container = ShaderProgramContainer::new(gl.clone());
    // shader begins *******************************************************************************
    let mut vertex_textured_program = shader_program_container
        .get_vertex_textured_program()
        .context("fail getting textured shader program")?;

    let mut camera = standard_camera();
    setup_coordinate_system(&mut vertex_textured_program, &camera)
        .context("fail setup coordinate system")?;
    // shader ends ********************************************************************************

    // texture begins
    let wallimg = image::open("assets/textures/wall.jpg")
        .context("fail loading")?
        .into_rgb8();
    let _wall_texture = texture::Texture::new(gl.clone(), wallimg.as_raw(), wallimg.dimensions());
    // texture ends

    let cube_position_array = [
        [0.0, 0.0, 0.0],
        [2.0, 5.0, -15.0],
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
        let err = gl.GetError();
        if err != gl::NO_ERROR {
            panic!("opengl error: {}", err);
        }
    }

    if glfw.supports_raw_motion() {
        window.set_cursor_mode(glfw::CursorMode::Disabled);
        window.set_raw_mouse_motion(true);
    } else {
        println!("mouse raw motion is not supported");
    }

    let mut first_mouse_move = true;
    let mut last_frame_time = SystemTime::now();
    let mut last_cursor_pos = (0., 0.);
    const CAMERA_SENSETIVITY: f32 = 0.1;
    while !window.should_close() {
        // count last frame duration.
        let current_frame_time = SystemTime::now();
        let frame_time = current_frame_time
            .duration_since(last_frame_time)
            .context("fail getting duration between frames")?;
        let frame_time_secs = frame_time.as_secs_f32();
        last_frame_time = current_frame_time;
        let camera_speed = 2.5f32 * frame_time_secs;

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                // catch mouse events **************************************************************
                glfw::WindowEvent::CursorPos(xpos, ypos) => {
                    if first_mouse_move {
                        last_cursor_pos.0 = xpos;
                        last_cursor_pos.1 = ypos;
                        first_mouse_move = false;
                    }

                    let xoffset = xpos - last_cursor_pos.0;
                    let yoffset = ypos - last_cursor_pos.1;
                    camera.rotate(
                        yoffset as f32 * CAMERA_SENSETIVITY,
                        xoffset as f32 * CAMERA_SENSETIVITY,
                    );

                    last_cursor_pos.0 = xpos;
                    last_cursor_pos.1 = ypos;
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

        vertex_textured_program.set_used();
        vertex_textured_program
            .set_uniform("view", &camera.view().as_ref() as &[f32; 16])
            .context("fail to set view matrix to vertex_textured_program")?;

        vertex_textured_program
            .set_uniform("projection", &camera.projection().as_ref() as &[f32; 16])
            .context("fail to set projection matrix to vertex_textured_program")?;

        unsafe {
            cube.bind();
            for (i, pos) in cube_position_array.iter().enumerate() {
                let pos = Matrix4::from_translation(pos.clone().into());
                let rot = Matrix4::from_axis_angle([1., 0.3, 0.5].into(), Deg(20. * i as f32));
                let model = pos * rot;
                vertex_textured_program
                    .set_uniform("model", &model.as_ref() as &[f32; 16])
                    .with_context(|| format!("fail changine `model` uniform of cube {}", i))?;
                gl.DrawElements(gl::TRIANGLES, 36, gl::UNSIGNED_INT, 0 as *const _);
            }

            cube.unbind();
        }

        // vertex_textured_program.set_used();
        // unsafe {
        //     triangle.bind();
        //     gl.DrawArrays(gl::TRIANGLES, 0, 3);
        //     triangle.unbind();
        // }

        // point_program.set_used();
        // unsafe {
        //     line.bind();
        //     gl.DrawArrays(gl::LINES, 0, 2);
        //     line.unbind();
        // }

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
