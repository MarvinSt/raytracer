use renderer::render;
use scene::select_scene;

mod bhv;
mod bounding_box;
mod camera;
mod hit;
mod material;
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
    let (cam, world, background) = select_scene(5);
    render(&cam, &world, &background);
}
