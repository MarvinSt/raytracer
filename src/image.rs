use nalgebra::Vector3;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
// use rayon::prelude::*;
use std::{
    fs::File,
    io::Write,
    time::{Duration, SystemTime},
};

use crate::{
    camera::Camera,
    hit::{random_double, ray_color, World},
    material::Material,
    sphere::Sphere,
};

#[inline]
pub fn write_color(f: &mut File, color: &Vector3<f32>) {
    writeln!(
        f,
        "{} {} {}",
        (color.x.sqrt() * 255.0) as u8,
        (color.y.sqrt() * 255.0) as u8,
        (color.z.sqrt() * 255.0) as u8
    )
    .unwrap();
}

#[inline]
pub fn get_pixel_color(
    image_width: u16,
    image_height: u16,
    samples_per_pixel: u8,
    cam: &Camera,
    world: &World,
    max_depth: u8,
    i: u16,
    j: u16,
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
    let samples_per_pixel = 100;
    let max_depth = 50;

    let aspect_ratio = 16.0 / 9.0;
    let cam: Camera = Camera::new(
        Vector3::new(-2.0, 2.0, 1.0),
        Vector3::new(0.0, 0.0, -1.0),
        Vector3::new(0.0, 1.0, 0.0),
        90.0,
        aspect_ratio,
        0.0,
        1.0,
    );

    let lookfrom: Vector3<f32> = Vector3::new(3.0, 3.0, 2.0);
    let lookat: Vector3<f32> = Vector3::new(0.0, 0.0, -1.0);

    let cam: Camera = Camera::new(
        lookfrom,
        lookat,
        Vector3::new(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
        2.0 * 0.0,
        (lookfrom - lookat).magnitude(),
    );

    // let cam: Camera = Camera::new(
    //     Vector3::new(13.0, 2.0, 3.0),
    //     Vector3::new(0.0, 0.0, 0.0),
    //     Vector3::new(0.0, 1.0, 0.0),
    //     20.0,
    //     aspect_ratio,
    //     0.0,
    //     1.0,
    // );

    let image_width = 1200 as u16;
    let image_height = (image_width as f32 / cam.aspect_ratio) as u16;

    let mut world = World::new();
    // world.add(Box::new(Sphere::new(Vector3::new(1.0, -2.0, -5.0), 1.0)));
    // world.add(Box::new(Sphere::new(Vector3::new(-1.0, -2.0, -5.0), 0.75)));
    // world.add(Box::new(Sphere::new(Vector3::new(1.0, 1.0, -5.0), 1.75)));
    // world.add(Box::new(Sphere::new(Vector3::new(-1.0, 1.0, -5.0), 0.75)));

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
    // world.add(Box::new(Sphere::new(
    //     Vector3::new(-1.0, 0.0, -1.0),
    //     -0.45,
    //     mat_left,
    // )));

    world.add(Box::new(Sphere::new(
        Vector3::new(1.0, 0.0, -1.0),
        0.5,
        mat_right,
    )));

    if false {
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
    }

    // generate output file
    let mut f = File::create("result.ppm").unwrap();

    writeln!(&mut f, "P3").unwrap();
    writeln!(&mut f, "{} {}", image_width, image_height).unwrap();
    writeln!(&mut f, "255").unwrap();

    let world_fix = world;

    let t_all = SystemTime::now();

    for j in (0..image_height).rev() {
        // println!("Progress: {:?} lines remaining", j);

        let t_line = SystemTime::now();

        let pixels: Vec<(Vector3<f32>, Duration)> = (0..image_width)
            .into_par_iter()
            .map(|i| {
                let t_pixel = SystemTime::now();
                let mut color: Vector3<f32> = Vector3::new(0.0, 0.0, 0.0);

                for _ in 0..samples_per_pixel {
                    let u = (i as f32 + random_double(0.0, 1.0)) / (image_width - 1) as f32;
                    let v = (j as f32 + random_double(0.0, 1.0)) / (image_height - 1) as f32;
                    let r = &cam.ray(u, v);
                    color += ray_color(&r, &world_fix, max_depth);
                }

                (color / samples_per_pixel as f32, t_pixel.elapsed().unwrap())
            })
            .collect();

        let mut pix_time = 0;

        let t_line = t_line.elapsed().unwrap().as_micros();

        for (pix, duration) in pixels {
            write!(
                f,
                "{} {} {}\n",
                (pix.x.sqrt() * 255.0) as u8,
                (pix.y.sqrt() * 255.0) as u8,
                (pix.z.sqrt() * 255.0) as u8
            )
            .expect("Not written");

            pix_time += duration.as_micros();
        }

        println!(
            "Remaining {j} - Render time: line {:?} | pix {pix_time} | ratio {:?}",
            t_line,
            pix_time as f32 / t_line as f32
        );
    }

    println!(
        "Render time: multi line {:?}",
        t_all.elapsed().unwrap().as_millis()
    );

    println!("\rDONE!");
}
