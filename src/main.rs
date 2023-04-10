use renderer::render;
use scene::{build_scene, random_scene};

mod bhv;
mod bounding_box;
mod camera;
mod hit;
mod material;
mod ray;
mod renderer;
mod scene;
mod sphere;
mod texture;

fn main() {
    // let (cam, world) = build_scene();
    let (cam, world) = random_scene();
    render(&cam, &world);
}
