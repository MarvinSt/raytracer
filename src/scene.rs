use crate::{
    camera::Camera,
    geometry::{cube::Cube, rectangle::RectAA, sphere::Sphere},
    hit::{random_color_vector, random_double, World},
    material::{Dielectric, DiffuseLight, Lambertian, Metal},
    texture::{Checker, Image, Noise, SolidColor},
};
use nalgebra::Vector3;

fn random_scene() -> (Camera, World, Vector3<f32>) {
    let aspect_ratio = 16.0 / 9.0;

    let lookfrom: Vector3<f32> = Vector3::new(13.0, 2.0, 3.0);
    let lookat: Vector3<f32> = Vector3::new(0.0, 0.0, 0.0);

    let cam: Camera = Camera::new(
        lookfrom,
        lookat,
        Vector3::new(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
        0.1,
        10.0, // (lookfrom - lookat).magnitude(),
    );

    let mut world = World::new();

    let odd = SolidColor::new(0.2, 0.3, 0.1);
    let even = SolidColor::new(0.9, 0.9, 0.9);
    let checker = Checker::new(odd, even);
    let ground_material = Lambertian::new(checker);

    world.add(Box::new(Sphere::new(
        Vector3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..=11 {
        for b in -11..=11 {
            let choose_mat = random_double(0.0, 1.0);

            let center = Vector3::new(
                a as f32 + random_double(0.0, 0.9),
                0.2,
                b as f32 + random_double(0.0, 0.9),
            );

            if (center - Vector3::new(4.0, 0.2, 0.0)).norm_squared() > 0.9 * 0.9 {
                if choose_mat < 0.8 {
                    // diffuse material
                    let ca: Vector3<f32> = random_color_vector();
                    let cb: Vector3<f32> = random_color_vector();
                    let albedo = SolidColor::new(ca[0] * cb[0], ca[1] * cb[1], ca[2] * cb[2]);
                    let mat = Lambertian::new(albedo);
                    world.add(Box::new(Sphere::new(center, 0.2, mat)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo: Vector3<f32> = random_color_vector().scale(0.5).add_scalar(0.5);
                    // let albedo = SolidColor::new(albedo.x, albedo.y, albedo.z);
                    let fuzz = random_double(0.0, 0.5);
                    let mat = Metal::new(albedo, fuzz);
                    world.add(Box::new(Sphere::new(center, 0.2, mat)));
                } else {
                    // glass
                    let mat = Dielectric::new(1.5);
                    world.add(Box::new(Sphere::new(center, 0.2, mat)));
                }
            }
        }
    }

    let mat = Dielectric::new(1.5);
    world.add(Box::new(Sphere::new(Vector3::new(0.0, 1.0, 0.0), 1.0, mat)));

    let mat = Lambertian::new(SolidColor::new(0.4, 0.2, 0.1));
    world.add(Box::new(Sphere::new(
        Vector3::new(-4.0, 1.0, 0.0),
        1.0,
        mat,
    )));

    let mat = Metal::new(Vector3::new(0.7, 0.6, 0.5), 0.0);
    world.add(Box::new(Sphere::new(Vector3::new(4.0, 1.0, 0.0), 1.0, mat)));

    // (cam, world)
    let background = Vector3::new(0.70, 0.80, 1.00);

    (cam, world.generate_bvh(), background)
}

fn three_spheres() -> (Camera, World, Vector3<f32>) {
    let aspect_ratio = 16.0 / 9.0;

    let lookat: Vector3<f32> = Vector3::new(0.0, 0.0, -1.0);
    let lookfrom: Vector3<f32> = Vector3::new(3.0, 3.0, 2.0);

    let cam: Camera = Camera::new(
        lookfrom,
        lookat,
        Vector3::new(0.0, 1.0, 0.0),
        30.0,
        aspect_ratio,
        2.0 * 0.0,
        (lookfrom - lookat).magnitude(),
    );

    let mut world = World::new();

    let mat_ground = Lambertian::new(SolidColor::new(0.8, 0.8, 0.0));

    let mat_center = Lambertian::new(SolidColor::new(0.1, 0.2, 0.5));

    let mat_left = Dielectric::new(1.5);

    let mat_right = Metal::new(Vector3::new(0.8, 0.6, 0.2), 0.0);

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
        mat_left.clone(),
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

    // (cam, world)
    let background = Vector3::new(0.70, 0.80, 1.00);

    (cam, world.generate_bvh(), background)
}

fn two_perlin_spheres() -> (Camera, World, Vector3<f32>) {
    let aspect_ratio = 16.0 / 9.0;

    let lookat: Vector3<f32> = Vector3::new(0.0, 0.0, 0.0);
    let lookfrom: Vector3<f32> = Vector3::new(13.0, 2.0, 3.0);

    let cam: Camera = Camera::new(
        lookfrom,
        lookat,
        Vector3::new(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
        0.0,
        (lookfrom - lookat).magnitude(),
    );

    let mut world = World::new();

    let mat = Lambertian::new(Noise::new(4.0));
    world.add(Box::new(Sphere::new(
        Vector3::new(0.0, -1000.0, 0.0),
        1000.0,
        mat.clone(),
    )));

    world.add(Box::new(Sphere::new(
        Vector3::new(0.0, 2.0, 0.0),
        2.0,
        mat.clone(),
    )));

    let background = Vector3::new(0.70, 0.80, 1.00);

    (cam, world.generate_bvh(), background)
}

fn earth() -> (Camera, World, Vector3<f32>) {
    let aspect_ratio = 16.0 / 9.0;

    let lookat: Vector3<f32> = Vector3::new(0.0, 0.0, 0.0);
    let lookfrom: Vector3<f32> = Vector3::new(13.0, 2.0, 3.0);

    let cam: Camera = Camera::new(
        lookfrom,
        lookat,
        Vector3::new(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
        0.0,
        (lookfrom - lookat).magnitude(),
    );

    let mut world = World::new();

    let mat = Lambertian::new(Image::new("earthmap.jpg"));

    world.add(Box::new(Sphere::new(
        Vector3::new(0.0, 0.0, 0.0),
        2.0,
        mat.clone(),
    )));

    let background = Vector3::new(0.70, 0.80, 1.00);

    (cam, world.generate_bvh(), background)
}

fn cornell_box() -> (Camera, World, Vector3<f32>) {
    let aspect_ratio = 1.0;

    let lookat: Vector3<f32> = Vector3::new(278.0, 278.0, 0.0);
    let lookfrom: Vector3<f32> = Vector3::new(278.0, 278.0, -800.0);

    let cam: Camera = Camera::new(
        lookfrom,
        lookat,
        Vector3::new(0.0, 1.0, 0.0),
        40.0,
        aspect_ratio,
        0.0,
        (lookfrom - lookat).magnitude(),
    );

    let mut world = World::new();

    let red = Lambertian::new(SolidColor::new(0.65, 0.05, 0.05));
    let white = Lambertian::new(SolidColor::new(0.73, 0.73, 0.73));
    let green = Lambertian::new(SolidColor::new(0.12, 0.45, 0.15));
    let light = DiffuseLight::new(SolidColor::new(15.0, 15.0, 15.0));

    world.add(Box::new(RectAA::yz(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        green.clone(),
    )));

    world.add(Box::new(RectAA::yz(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        red.clone(),
    )));

    world.add(Box::new(RectAA::xz(
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
        light.clone(),
    )));

    world.add(Box::new(RectAA::xz(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));

    world.add(Box::new(RectAA::xz(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));

    world.add(Box::new(RectAA::xy(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));

    world.add(Box::new(Cube::new(
        Vector3::new(130.0, 0.0, 65.0),
        Vector3::new(295.0, 165.0, 230.0),
        white.clone(),
    )));

    world.add(Box::new(Cube::new(
        Vector3::new(265.0, 0.0, 295.0),
        Vector3::new(430.0, 330.0, 460.0),
        white.clone(),
    )));

    let background = Vector3::new(0.0, 0.0, 0.0);

    (cam, world.generate_bvh(), background)
}

pub fn select_scene(i: usize) -> (Camera, World, Vector3<f32>) {
    match i {
        0 => three_spheres(),
        1 => random_scene(),
        2 => two_perlin_spheres(),
        3 => two_perlin_spheres(),
        4 => earth(),
        5 => cornell_box(),
        _ => three_spheres(),
    }
}
