use crate::{camera::Camera, hit::Hittable};
use image::Rgb;
use nalgebra::Vector3;
use rand::Rng;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::time::{Duration, SystemTime};

pub fn get_pixel_color(
    image_width: u32,
    image_height: u32,
    samples_per_pixel: u16,
    cam: &Camera,
    world: &Box<dyn Hittable>,
    lights: &Box<dyn Hittable>,
    background: &Vector3<f32>,
    max_depth: u8,
    i: u32,
    j: u32,
) -> Vector3<f32> {
    let mut rng = rand::thread_rng();
    let mut color: Vector3<f32> = Vector3::zeros();

    // launch parallel iterator
    let mut pixel_cnt = 0;
    for _ in 0..samples_per_pixel {
        // need a new rng for each thread
        let u = (i as f32 + rng.gen::<f32>()) / (image_width - 1) as f32;
        let v = (j as f32 + rng.gen::<f32>()) / (image_height - 1) as f32;

        // accumulate the color for each sample
        let new_color = &cam.ray(u, v).color(background, &world, &lights, max_depth);

        // cull pixel colors with nan's components (invalid calculations otherwise show up as black pixels)
        if !(new_color.x != new_color.x || new_color.y != new_color.y || new_color.z != new_color.z)
        {
            color += new_color;
            pixel_cnt += 1;
        }
    }

    // return the color normalized per sample
    color / pixel_cnt.max(1) as f32
}

pub fn render(
    cam: &Camera,
    world: &Box<dyn Hittable>,
    lights: &Box<dyn Hittable>,
    background: &Vector3<f32>,
    path: &str,
    samples_per_pixel: u16,
) {
    let max_depth = 50;

    // generate output buffer
    let image_width = 800 as u32;
    let image_height = (image_width as f32 / cam.aspect_ratio) as u32;
    let mut buffer: image::RgbImage = image::ImageBuffer::new(image_width, image_height);

    let total_time = SystemTime::now();
    let mut line_time_avg = 0.0;
    for j in (0..image_height).rev() {
        let line_time = SystemTime::now();

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
                    &lights,
                    &background,
                    max_depth,
                    i,
                    j,
                );

                (color, t_pixel.elapsed().unwrap())
            })
            .collect();

        let line_time = line_time.elapsed().unwrap().as_micros() as f32 * 1.0e-3;

        // low pass filter the average line time
        if line_time_avg > 0.0 {
            line_time_avg = line_time_avg + 0.1 * (line_time - line_time_avg);
        } else {
            line_time_avg = line_time;
        }

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

        let pix_time = pix_time as f32 * 1.0e-3;

        let rem_rows = j;

        let eta = line_time_avg * rem_rows as f32 * 1.0e-3;

        println!(
            "# {}\t| Line {:>10.3} [ms]\t| Pixel {:>10.3} [ms]\t| Ratio {:>6.3} \t| ETA: {:>9.2} [s] \t| ELA: {:>9.2} [s]",
            j,
            line_time,
            pix_time,
            pix_time / line_time,
            eta,
            total_time.elapsed().unwrap().as_millis() as f32 * 1.0e-3
        );
    }

    println!(
        "Total render time: {:?} [s]",
        total_time.elapsed().unwrap().as_millis() as f32 * 1.0e-3
    );

    buffer.save(path).unwrap();

    println!("\rDone!");
}
