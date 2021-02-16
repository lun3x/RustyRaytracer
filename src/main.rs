mod cornell_box;
mod draw;
mod intersect;
mod objects;
mod rays;
mod scene;
mod utils;
mod visualiser;

use crate::draw::*;
use crate::scene::Scene;
use crate::visualiser::*;
use std::time::{Duration, Instant};

use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::{LogicalPosition, LogicalSize, PhysicalSize};
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit_input_helper::WinitInputHelper;

fn main() {
    let start_load = Instant::now();

    let p0 = Point {
        x: 0.0,
        y: 0.0,
        z: 25.0,
    };
    let camera = Camera::new(p0, 1.0, cgmath::Deg(0.0));

    let mut visualiser = Visualiser::new(SCREEN_HEIGHT, SCREEN_WIDTH, camera);

    let scene = crate::cornell_box::get_scene2();

    let start_render = Instant::now();

    draw::render_scene(visualiser, scene);

    let render_time = start_render.elapsed().as_millis();
    let total_time = start_load.elapsed().as_millis();
    println!("Render time: {}ms", render_time);
    println!("Total time: {}ms", total_time);
}
