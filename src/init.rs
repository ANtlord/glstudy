use std::sync::mpsc::Receiver;

use anyhow::Context;

use crate::camera;

pub fn standard_camera(aspect: f32) -> camera::Camera {
    camera::CameraOptions::new()
        .position([0.0f32, 0., 3.]) // just a little bit from the center of the world
        .front([0.0f32, 0., -1.])
        .up([0.0f32, 1., 0.])
        .yaw(-90.) // as we looking againg Z direction
        .aspect_ratio(aspect)
        .zoom(45.)
        .fly(false)
        .build()
}

pub fn window(
    glfw_ctx: &glfw::Glfw,
    width: u32,
    height: u32,
) -> anyhow::Result<(glfw::Window, Receiver<(f64, glfw::WindowEvent)>)> {
    let (mut window, events) = glfw_ctx
        .create_window(width, height, "Draw line demo", glfw::WindowMode::Windowed)
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
