use image::Rgb;
use nalgebra::Vector3;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::time::{Duration, SystemTime};

use crate::{
    camera::Camera,
    hit::{random_double, ray_color, World},
    material::Material,
    sphere::Sphere,
};

pub fn get_pixel_color(
    image_width: u32,
    image_height: u32,
    samples_per_pixel: u8,
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

pub fn render_image() {
    let max_depth = 50;
    let samples_per_pixel = 100;

    let aspect_ratio = 16.0 / 9.0;

    let lookat: Vector3<f32> = Vector3::new(0.0, 0.0, -1.0);
    let lookfrom: Vector3<f32> = Vector3::new(3.0, 3.0, 2.0);

    let mut cam: Camera = Camera::new(
        lookfrom,
        lookat,
        Vector3::new(0.0, 1.0, 0.0),
        30.0,
        aspect_ratio,
        2.0 * 0.0,
        (lookfrom - lookat).magnitude(),
    );

    let mut world = World::new();

    let mat_ground = Material::Lambertian {
        albedo: Vector3::new(0.8, 0.8, 0.0),
    };

    let mat_center = Material::Lambertian {
        albedo: Vector3::new(0.1, 0.2, 0.5),
    };

    let mat_left = Material::Dielectric {
        refraction_index: 1.5,
    };

    let mat_right = Material::Metal {
        albedo: Vector3::new(0.8, 0.6, 0.2),
        fuzz: 0.0,
    };

    if false {
        cam = Camera::new(
            Vector3::new(13.0, 2.0, 3.0),
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
            20.0,
            aspect_ratio,
            0.0,
            1.0,
        );

        let ground_material = Material::Lambertian {
            albedo: Vector3::new(0.5, 0.5, 0.5),
        };
        world.add(Box::new(Sphere::new(
            Vector3::new(0.0, -1000.0, 0.0),
            1000.0,
            ground_material,
        )));

        let material1 = Material::Dielectric {
            refraction_index: 1.5,
        };
        world.add(Box::new(Sphere::new(
            Vector3::new(0.0, 1.0, 0.0),
            1.0,
            material1,
        )));
        world.add(Box::new(Sphere::new(
            Vector3::new(0.0, 1.0, 0.0),
            -0.95,
            material1,
        )));

        let material2 = Material::Lambertian {
            albedo: Vector3::new(0.4, 0.2, 0.1),
        };
        world.add(Box::new(Sphere::new(
            Vector3::new(-4.0, 1.0, 0.0),
            1.0,
            material2,
        )));

        let material3 = Material::Metal {
            albedo: Vector3::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        };
        world.add(Box::new(Sphere::new(
            Vector3::new(4.0, 1.0, 0.0),
            1.0,
            material3,
        )));
    } else {
        // cam = Camera::new(
        //     Vector3::new(-2.0, 2.0, 1.0),
        //     Vector3::new(0.0, 0.0, -1.0),
        //     Vector3::new(0.0, 1.0, 0.0),
        //     90.0,
        //     aspect_ratio,
        //     0.0,
        //     1.0,
        // );

        world.add(Box::new(Sphere::new(
            Vector3::new(0.0, -100.5, -1.0),
            100.0,
            mat_ground,
        )));

        world.add(Box::new(Sphere::new(
            Vector3::new(0.0, 0.0, -1.0),
            0.5,
            mat_center,
        )));

        world.add(Box::new(Sphere::new(
            Vector3::new(-1.0, 0.0, -1.0),
            0.5,
            mat_left,
        )));

        world.add(Box::new(Sphere::new(
            Vector3::new(-1.0, 0.0, -1.0),
            -0.40,
            mat_left,
        )));

        world.add(Box::new(Sphere::new(
            Vector3::new(1.0, 0.0, -1.0),
            0.5,
            mat_right,
        )));
    }

    // generate output file
    let image_width = 1200 as u32;
    let image_height = (image_width as f32 / cam.aspect_ratio) as u32;

    let mut buffer: image::RgbImage = image::ImageBuffer::new(image_width, image_height);

    // This step wil convert the existing world to a BVH optimized immutable world
    let world = world.generate_bvh();
    // let world = world;

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
        "Render time: multi line {:?}",
        t_all.elapsed().unwrap().as_millis()
    );

    buffer.save("result.png").unwrap();

    println!("\rDONE!");
}
