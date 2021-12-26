use criterion::{black_box, criterion_group, criterion_main, Criterion};
use raytracing_lib::constants::{ASPECT_RATIO, SCREEN_HEIGHT,SCREEN_WIDTH};

fn criterion_benchmark(c: &mut Criterion) {
    let p0 = cgmath::Vector3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let camera = raytracing_lib::raytracing::Camera::new(p0, 1.0, ASPECT_RATIO, cgmath::Deg(0.0));

    let scene = raytracing_lib::cornell_box::get_box();
    let scene = raytracing_lib::cornell_box::get_sphere();

    let mut screen: [u8; 3 * SCREEN_HEIGHT * SCREEN_WIDTH] = [0; 3 * SCREEN_HEIGHT* SCREEN_WIDTH];

    c.bench_function("fib 20", |b| b.iter(|| black_box(raytracing_lib::draw::draw_to_screen(&camera, &scene, &mut screen))));

    raytracing_lib::output::write_to_ppm_file(&screen);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);