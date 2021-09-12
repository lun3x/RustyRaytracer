use crate::raytracing::*;
use crate::utils;
use crate::constants::*;

pub fn draw_to_screen(camera: &Camera, scene: &Scene, screen: &mut [u8]) {
    for (idx, pix) in screen.chunks_exact_mut(3).enumerate() {
        let u = idx as usize % SCREEN_WIDTH;
        let v = idx as usize / SCREEN_WIDTH;

        let x = u as f32 / (SCREEN_WIDTH - 1) as f32;
        let y = v as f32 / (SCREEN_HEIGHT - 1) as f32;

        let cam_ray = camera.create_camera_ray(x, y);
        let colour_float = trace(cam_ray, &scene, REFLECT_DEPTH);

        // Draw to screen buffer
        let colour_rgb = utils::correct_for_output(colour_float);
        pix.copy_from_slice(&colour_rgb);
    }
}
