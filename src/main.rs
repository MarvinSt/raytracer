use renderer::render;
use scene::build_scene;

mod bhv;
mod bounding_box;
mod camera;
mod hit;
mod material;
mod ray;
mod renderer;
mod scene;
mod sphere;

fn main() {
    let (cam, world) = build_scene();
    render(&cam, &world);
}
