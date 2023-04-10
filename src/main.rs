use renderer::render;
use scene::{build_scene, random_scene, two_perlin_spheres};

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
    // let (cam, world) = random_scene();
    let (cam, world) = two_perlin_spheres();
    render(&cam, &world);
}
