use crate::{camera::Camera, hit::World, material::Material, sphere::Sphere};
use nalgebra::Vector3;

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

    (cam, world.generate_bvh())
}
