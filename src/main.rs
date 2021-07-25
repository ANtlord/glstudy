use anyhow::anyhow;
use anyhow::Context;
use cgmath::{Deg, Matrix4, SquareMatrix, Vector3};
use gl;
use glfw;
use glfw::{Action, Key};
use image;

mod buffer;
mod entities;
mod render_gl;
mod shader_program_container;
mod texture;

use shader_program_container::ShaderProgramContainer;

const WINDOW_WIDTH: i32 = 900;
const WINDOW_HEIGHT: i32 = 700;

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
    let mut line = entities::VertLine::new(gl.clone());
    let cube = entities::Cube::new(gl.clone());
    unsafe {
        gl.Viewport(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT);
        gl.ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    let shader_program_container = ShaderProgramContainer::new(gl.clone());

    // shader begins *******************************************************************************
    let point_program = shader_program_container
        .get_point_program()
        .context("fail getting point program")?;
    let mut vertex_textured_program = shader_program_container
        .get_vertex_textured_program()
        .context("fail getting textured shader program")?;

    vertex_textured_program.set_used();
    // let mut mat: Matrix4<f32> = cgmath::Matrix4::identity();
    // mat = mat * Matrix4::from_translation(Vector3::new(0.5, 0., 0.));
    let model: Matrix4<f32> = Matrix4::from_angle_y(Deg(0.));
    let view: Matrix4<f32> = Matrix4::from_translation([0., 0., -3.].into());
    let mut aspect_ratio = WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32;
    let projection = cgmath::perspective(Deg(45.0f32), aspect_ratio, 0.1, 100.);
    vertex_textured_program
        .set_uniform("model", &model.as_ref() as &[f32; 16])
        .context("fail to set model matrix to vertex_textured_program")?;
    vertex_textured_program
        .set_uniform("view", &view.as_ref() as &[f32; 16])
        .context("fail to set view matrix to vertex_textured_program")?;
    vertex_textured_program
        .set_uniform("projection", &projection.as_ref() as &[f32; 16])
        .context("fail to set projection matrix to vertex_textured_program")?;
    // shader ends ********************************************************************************

    // texture begins
    let wallimg = image::open("assets/textures/wall.jpg")
        .context("fail loading")?
        .into_rgb8();
    let wall_texture = texture::Texture::new(gl.clone(), wallimg.as_raw(), wallimg.dimensions());
    // texture ends

    unsafe {
        gl.Enable(gl::DEPTH_TEST);
        let err = gl.GetError();
        if err != gl::NO_ERROR {
            panic!("opengl error: {}", err);
        }
    }

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

    let mut xaxis = 0.;
    let mut yaxis = 0.;
    let mut is_around_x = false;
    let mut angle = 45.0f32;
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
                glfw::WindowEvent::Key(Key::X, _, Action::Press, _) => {
                    is_around_x = true;
                }
                glfw::WindowEvent::Key(Key::X, _, Action::Release, _) => {
                    is_around_x = false;
                }
                glfw::WindowEvent::Scroll(_, yoffset) => {
                    // if is_around_x {
                    //     xaxis += 5. * yoffset as f32;
                    // } else {
                    //     yaxis += 5. * yoffset as f32;
                    // };
                    // aspect_ratio += yoffset as f32 * 0.01f32;
                    angle += yoffset as f32 * 0.1f32;
                    let projection = cgmath::perspective(Deg(angle), aspect_ratio, 0.1, 100.);
                    vertex_textured_program.set_used();
                    vertex_textured_program
                        .set_uniform("projection", &projection.as_ref() as &[f32; 16])?;
                }
                _ => {}
            }
        }

        // drawing begins **************************************************************************
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        vertex_textured_program.set_used();
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
