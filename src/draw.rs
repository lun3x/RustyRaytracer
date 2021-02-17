use crate::visualiser::Visualiser;
use crate::{scene::Scene, visualiser::ColourFloat};
use crate::utils::*;
use cgmath::Vector3;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::{LogicalPosition, LogicalSize, PhysicalSize};
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit_input_helper::WinitInputHelper;

pub const SCREEN_HEIGHT: u32 = 400;
pub const SCREEN_WIDTH: u32 = 400;
pub const ANTIALIAS_SAMPLES: u32 = 1;

pub fn render_scene(mut visualiser: Visualiser, scene: Scene) -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let (window, p_width, p_height) = create_window("RustyRaytracer", &event_loop);
    let surface_texture = SurfaceTexture::new(p_width, p_height, &window);
    let mut pixels = Pixels::new(SCREEN_WIDTH, SCREEN_HEIGHT, surface_texture)?;

    event_loop.run(move |event, _, control_flow| {
        // The one and only event that winit_input_helper doesn't have for us...
        if let Event::RedrawRequested(_) = event {
            println!("REDRAW");
            draw(&mut visualiser, &scene, pixels.get_frame());
            if pixels
                .render()
                .map_err(|e| println!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                visualiser.save();
                *control_flow = ControlFlow::Exit;
                return;
            }
            let modified = {
                if input.key_pressed(VirtualKeyCode::Left) {
                    println!("Left");
                    visualiser.rotate(10.0);
                    true
                } else if input.key_pressed(VirtualKeyCode::Right) {
                    println!("Right");
                    visualiser.rotate(-10.0);
                    true
                } else if input.key_pressed(VirtualKeyCode::Up) {
                    println!("Forward");
                    visualiser.dolly(1.0);
                    true
                } else if input.key_pressed(VirtualKeyCode::Down) {
                    println!("Backward");
                    visualiser.dolly(-1.0);
                    true
                } else {
                    false
                }
            };
            if modified {
                window.request_redraw();
            }
        }
    });
}

fn draw(visualiser: &mut Visualiser, scene: &Scene, screen: &mut [u8]) {
    for (idx, pix) in screen.chunks_exact_mut(4).enumerate() {
        let x = idx as u32 % SCREEN_WIDTH;
        let y = idx as u32 / SCREEN_WIDTH;
        let mut colour_float = ColourFloat::new(0.0, 0.0, 0.0);
        for s in 0..ANTIALIAS_SAMPLES {
            let xx = x as f32 + rand_f32();
            let yy = y as f32 + rand_f32();
            let cam_ray = visualiser.create_camera_ray(xx, yy);
            colour_float += crate::rays::trace(cam_ray, &scene, 5);
        }
        colour_float /= ANTIALIAS_SAMPLES as f32;
        // Draw to screen buffer
        let colour_rgba = crate::objects::as_int4(colour_float);
        pix.copy_from_slice(&colour_rgba);
        // Save to render image
        let colour_rgb = crate::objects::as_int(colour_float);
        visualiser.put_pixel(x, y, colour_rgb)
    }
}

/// Create a window for the game.
///
/// Automatically scales the window to cover about 2/3 of the monitor height.
///
/// # Returns
///
/// Tuple of `(window, surface, width, height, hidpi_factor)`
/// `width` and `height` are in `PhysicalSize` units.
fn create_window(title: &str, event_loop: &EventLoop<()>) -> (winit::window::Window, u32, u32) {
    // Create a hidden window so we can estimate a good default window size
    let window = winit::window::WindowBuilder::new()
        .with_visible(false)
        .with_title(title)
        .build(&event_loop)
        .unwrap();
    let hidpi_factor = window.scale_factor();

    // Get dimensions
    let width = SCREEN_WIDTH as f64;
    let height = SCREEN_HEIGHT as f64;
    let (monitor_width, monitor_height) = {
        match window.current_monitor() {
            Some(monitor) => (
                monitor.size().width as f64 / hidpi_factor,
                monitor.size().height as f64 / hidpi_factor,
            ),
            None => (width, height),
        }
    };
    let scale = (monitor_height / height * 2.0 / 3.0).round();

    // Resize, center, and display the window
    let min_size: winit::dpi::LogicalSize<f64> =
        PhysicalSize::new(width, height).to_logical(hidpi_factor);
    let default_size = LogicalSize::new(width * scale, height * scale);
    let center = LogicalPosition::new(
        (monitor_width - width * scale) / 2.0,
        (monitor_height - height * scale) / 2.0,
    );
    window.set_inner_size(default_size);
    window.set_min_inner_size(Some(min_size));
    window.set_outer_position(center);
    window.set_visible(true);

    let size = default_size.to_physical::<f64>(hidpi_factor);

    (
        window,
        size.width.round() as u32,
        size.height.round() as u32,
    )
}
