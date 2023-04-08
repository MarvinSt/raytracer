use crate::image::render_image;

mod bhv;
mod bounding_box;
mod camera;
mod hit;
mod image;
mod material;
mod ray;
mod sphere;

fn main() {
    render_image();
}
