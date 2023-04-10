use crate::{
    camera::Camera,
    hit::{random_color_vector, random_double, World},
    material::{Dielectric, Lambertian, Metal},
    sphere::Sphere,
    texture::{Checker, SolidColor},
};
use nalgebra::Vector3;

pub fn random_scene() -> (Camera, World) {
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

    //  {
    //     albedo: checker, // Texture::solid_color(0.5, 0.5, 0.5),
    // };

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
    (cam, world.generate_bvh())
}

pub fn build_scene() -> (Camera, World) {
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
    (cam, world.generate_bvh())
}
