use image::Rgb;
use nalgebra::Vector3;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::time::{Duration, SystemTime};

use crate::{
    camera::Camera,
    hit::{random_double, ray_color, World},
};

pub fn get_pixel_color(
    image_width: u32,
    image_height: u32,
    samples_per_pixel: u16,
    cam: &Camera,
    world: &World,
    max_depth: u8,
    i: u32,
    j: u32,
) -> Vector3<f32> {
    let mut color: Vector3<f32> = Vector3::default();
    // launch parallel iterator
    for _ in 0..samples_per_pixel {
        // need a new rng for each thread
        let u = (i as f32 + random_double(0.0, 1.0)) / (image_width - 1) as f32;
        let v = (j as f32 + random_double(0.0, 1.0)) / (image_height - 1) as f32;

        // Return the colour (note lack of semicolon)
        color += ray_color(&cam.ray(u, v), &world, max_depth);
    }

    color / samples_per_pixel as f32
}

pub fn render(cam: &Camera, world: &World) {
    let max_depth = 50;
    let samples_per_pixel = 50;

    // generate output buffer
    let image_width = 1200 as u32;
    let image_height = (image_width as f32 / cam.aspect_ratio) as u32;
    let mut buffer: image::RgbImage = image::ImageBuffer::new(image_width, image_height);

    let t_all = SystemTime::now();
    for j in (0..image_height).rev() {
        let t_line = SystemTime::now();

        let pixels: Vec<(Vector3<f32>, Duration)> = (0..image_width)
            .into_par_iter()
            .map(|i| {
                let t_pixel = SystemTime::now();
                let color: Vector3<f32> = get_pixel_color(
                    image_width,
                    image_height,
                    samples_per_pixel,
                    &cam,
                    &world,
                    max_depth,
                    i,
                    j,
                );

                (color, t_pixel.elapsed().unwrap())
            })
            .collect();

        let t_line = t_line.elapsed().unwrap().as_micros();

        let mut pix_time = 0;
        for x in 0..image_width {
            let (pix, duration) = pixels[x as usize];
            buffer.put_pixel(
                x,
                image_height - 1 - j,
                Rgb([
                    (pix.x.sqrt() * 255.0) as u8,
                    (pix.y.sqrt() * 255.0) as u8,
                    (pix.z.sqrt() * 255.0) as u8,
                ]),
            );
            pix_time += duration.as_micros();
        }

        println!(
            "Line No.# {} \t| Line Time {:?} [us] \t| Pix Time {:?} [us] \t| Ratio {:?}",
            j,
            t_line,
            pix_time,
            pix_time as f32 / t_line as f32
        );
    }

    println!(
        "Total render time: {:?} [ms]",
        t_all.elapsed().unwrap().as_millis()
    );

    buffer.save("result.png").unwrap();

    println!("\rDone!");
}
