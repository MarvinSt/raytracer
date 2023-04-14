use renderer::render;
use scene::select_scene;
mod bhv;
mod bounding_box;
mod camera;
mod hit;
mod instance;
mod material;
mod medium;
mod noise;
mod ray;
mod renderer;
mod scene;
mod texture;

mod geometry {
    pub mod cube;
    pub mod rectangle;
    pub mod sphere;
}

fn main() {
    // let samples_per_pixel = 1000;
    // for i in 0..=7 {
    //     let (cam, world, background) = select_scene(i);
    //     let path = format!("./tests/result_{}.png", i);
    //     render(&cam, &world, &background, &path, samples_per_pixel);
    // }

    let samples_per_pixel = 1000;
    let i = 1;
    let (cam, world, background) = select_scene(i);
    let path = format!("./tests/result_{}.png", i);
    render(&cam, &world, &background, &path, samples_per_pixel);
}
