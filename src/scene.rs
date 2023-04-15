use crate::{
    bhv::Bvh,
    camera::Camera,
    geometry::{cube::Cube, rectangle::RectAA, sphere::Sphere},
    hit::{random_color_vector, random_double, Hittable, World},
    instance::{FlipFace, Rotate, Translate},
    material::{Dielectric, DiffuseLight, Lambertian, Metal},
    medium::Constant,
    texture::{Checker, Image, Noise, SolidColor},
};
use nalgebra::Vector3;

fn random_scene() -> (Camera, Box<dyn Hittable>, Box<dyn Hittable>, Vector3<f32>) {
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

    let mut lights = World::new();
    let lightmat = DiffuseLight::new(SolidColor::new(7.0, 7.0, 7.0));
    let light = Sphere::new(
        lookfrom + Vector3::new(0.0, 20.0, 0.0),
        2.5,
        lightmat.clone(),
    );
    lights.push(light);

    let mut world = World::new();

    let light = Sphere::new(Vector3::new(13.0, 20.0, 3.0), 2.5, lightmat.clone());
    world.push(light);

    let odd = SolidColor::new(0.2, 0.3, 0.1);
    let even = SolidColor::new(0.9, 0.9, 0.9);
    let checker = Checker::new(odd, even);
    let ground_material = Lambertian::new(checker);

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
                    world.push(Sphere::new(center, 0.2, mat));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo: Vector3<f32> = random_color_vector().scale(0.5).add_scalar(0.5);
                    let albedo = SolidColor::new(albedo.x, albedo.y, albedo.z);
                    let fuzz = random_double(0.0, 0.5);
                    let mat = Metal::new(albedo, fuzz);
                    world.push(Sphere::new(center, 0.2, mat));
                } else {
                    // glass
                    let mat = Dielectric::new(1.5);
                    world.push(Sphere::new(center, 0.2, mat));
                }
            }
        }
    }

    let bhv = Bvh::new(world.objects);

    let mut world = World::new();

    world.push(bhv);

    world.push(Sphere::new(
        Vector3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    let mat = Dielectric::new(1.5);
    world.push(Sphere::new(Vector3::new(0.0, 1.0, 0.0), 1.0, mat));

    let mat = Lambertian::new(SolidColor::new(0.4, 0.2, 0.1));
    world.push(Sphere::new(Vector3::new(-4.0, 1.0, 0.0), 1.0, mat));

    let mat = Metal::new(SolidColor::new(0.7, 0.6, 0.5), 0.0);
    world.push(Sphere::new(Vector3::new(4.0, 1.0, 0.0), 1.0, mat));

    // (cam, world)
    let background = Vector3::new(0.70, 0.80, 1.00);

    // (cam, world, background)

    (cam, Box::new(world), Box::new(lights), background)
}

fn three_spheres() -> (Camera, Box<dyn Hittable>, Box<dyn Hittable>, Vector3<f32>) {
    let aspect_ratio = 16.0 / 9.0;

    let lookat: Vector3<f32> = Vector3::new(0.0, 0.0, -1.0);
    let lookfrom: Vector3<f32> = Vector3::new(-2.0, 1.25, 1.0);

    let cam: Camera = Camera::new(
        lookfrom,
        lookat,
        Vector3::new(0.0, 1.0, 0.0),
        45.0,
        aspect_ratio,
        2.0 * 0.0,
        (lookfrom - lookat).magnitude(),
    );

    let mut lights = World::new();
    let lightmat = DiffuseLight::new(SolidColor::new(5.0, 5.0, 5.0));
    let light = Sphere::new(
        lookfrom + Vector3::new(0.0, 20.0, 0.0),
        2.5,
        lightmat.clone(),
    );
    lights.push(light);

    let mut world = World::new();

    let mat_ground = Lambertian::new(SolidColor::new(0.8, 0.8, 0.0));

    let mat_center = Lambertian::new(SolidColor::new(0.1, 0.2, 0.5));

    let mat_left = Dielectric::new(1.5);

    let mat_right = Metal::new(SolidColor::new(0.8, 0.6, 0.2), 0.0);

    world.push(Sphere::new(
        Vector3::new(0.0, -100.5, -1.0),
        100.0,
        mat_ground,
    ));

    world.push(Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5, mat_center));

    world.push(Sphere::new(
        Vector3::new(-1.0, 0.0, -1.0),
        0.5,
        mat_left.clone(),
    ));

    world.push(Sphere::new(Vector3::new(-1.0, 0.0, -1.0), -0.45, mat_left));

    world.push(Sphere::new(Vector3::new(1.0, 0.0, -1.0), 0.50, mat_right));

    // (cam, world)
    let background = Vector3::new(0.70, 0.80, 1.00);

    (cam, Box::new(world), Box::new(lights), background)
}

fn two_perlin_spheres(
    checker: bool,
) -> (Camera, Box<dyn Hittable>, Box<dyn Hittable>, Vector3<f32>) {
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

    let mut lights = World::new();
    let lightmat = DiffuseLight::new(SolidColor::new(7.0, 7.0, 7.0));
    let light = Sphere::new(
        lookfrom + Vector3::new(0.0, 20.0, 0.0),
        2.5,
        lightmat.clone(),
    );
    lights.push(light);

    let mut world = World::new();

    if checker {
        let mat = Lambertian::new(Checker::new(
            SolidColor::new(0.2, 0.3, 0.1),
            SolidColor::new(0.9, 0.9, 0.9),
        ));
        world.push(Sphere::new(
            Vector3::new(0.0, -1000.0, 0.0),
            1000.0,
            mat.clone(),
        ));
        world.push(Sphere::new(Vector3::new(0.0, 2.0, 0.0), 2.0, mat.clone()));
    } else {
        let mat = Lambertian::new(Noise::new(4.0));
        world.push(Sphere::new(
            Vector3::new(0.0, -1000.0, 0.0),
            1000.0,
            mat.clone(),
        ));
        world.push(Sphere::new(Vector3::new(0.0, 2.0, 0.0), 2.0, mat.clone()));
    };

    let background = Vector3::new(0.70, 0.80, 1.00);

    (cam, Box::new(world), Box::new(lights), background)
}

fn earth() -> (Camera, Box<dyn Hittable>, Box<dyn Hittable>, Vector3<f32>) {
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

    let mut lights = World::new();
    let lightmat = DiffuseLight::new(SolidColor::new(7.0, 7.0, 7.0));
    let light = Sphere::new(
        lookfrom + Vector3::new(0.0, 20.0, 0.0),
        2.5,
        lightmat.clone(),
    );
    lights.push(light);

    let mut world = World::new();

    let mat = Lambertian::new(Image::new("earthmap.jpg"));

    world.push(Sphere::new(Vector3::new(0.0, 0.0, 0.0), 2.0, mat.clone()));

    let background = Vector3::new(0.70, 0.80, 1.00);

    (cam, Box::new(world), Box::new(lights), background)
}

fn simple_light() -> (Camera, Box<dyn Hittable>, Box<dyn Hittable>, Vector3<f32>) {
    let aspect_ratio = 16.0 / 9.0;

    let lookat: Vector3<f32> = Vector3::new(0.0, 2.0, 0.0);
    let lookfrom: Vector3<f32> = Vector3::new(26.0, 3.0, 6.0);

    let cam: Camera = Camera::new(
        lookfrom,
        lookat,
        Vector3::new(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
        0.0,
        10.0, //(lookfrom - lookat).magnitude(),
    );

    let mut world = World::new();

    // let boundary = Sphere::new(Vector3::new(0.0, 0.0, 0.0), 50.0, Dielectric::new(1.5));
    // world.push(Constant::new(
    //     boundary,
    //     0.01,
    //     SolidColor::new(1.0, 0.0, 0.0),
    // ));

    let pretext = Noise::new(4.0);
    let mat = Lambertian::new(pretext);

    world.push(Sphere::new(
        Vector3::new(0.0, -1000.0, 0.0),
        1000.0,
        mat.clone(),
    ));

    world.push(Sphere::new(Vector3::new(0.0, 2.0, 0.0), 2.0, mat.clone()));

    let difflight = DiffuseLight::new(SolidColor::new(7.0, 7.0, 7.0));
    let light = RectAA::xy(3.0, 5.0, 1.0, 3.0, -2.0, difflight.clone());
    world.push(light);

    let light = RectAA::xy(3.0, 5.0, 1.0, 3.0, -2.0, difflight.clone());
    let mut lights = World::new();
    lights.push(light);

    let background = Vector3::new(0.0, 0.0, 0.0);

    (cam, Box::new(world), Box::new(lights), background)
}

pub fn cornell_box_smoke(
    smoke: bool,
) -> (Camera, Box<dyn Hittable>, Box<dyn Hittable>, Vector3<f32>) {
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
        10.0,
        // (lookfrom - lookat).magnitude(),
    );

    let mut world = World::new();

    let red = Lambertian::new(SolidColor::new(0.65, 0.05, 0.05));
    let white = Lambertian::new(SolidColor::new(0.73, 0.73, 0.73));
    let green = Lambertian::new(SolidColor::new(0.12, 0.45, 0.15));

    world.push(RectAA::yz(0.0, 555.0, 0.0, 555.0, 555.0, green.clone()));

    world.push(RectAA::yz(0.0, 555.0, 0.0, 555.0, 0.0, red.clone()));

    let mut lights = World::new();

    if smoke {
        let light = DiffuseLight::new(SolidColor::new(7.0, 7.0, 7.0));

        let light1 = RectAA::xz(113.0, 443.0, 127.0, 432.0, 554.0, light.clone());
        lights.push(light1);

        let light1 = RectAA::xz(113.0, 443.0, 127.0, 432.0, 554.0, light.clone());
        world.push(FlipFace::new(light1));
    } else {
        let light = DiffuseLight::new(SolidColor::new(15.0, 15.0, 15.0));

        let light1 = RectAA::xz(213.0, 343.0, 227.0, 332.0, 554.0, light.clone());
        lights.push(light1);

        let light1 = RectAA::xz(213.0, 343.0, 227.0, 332.0, 554.0, light.clone());
        world.push(FlipFace::new(light1));
    }

    world.push(RectAA::xz(0.0, 555.0, 0.0, 555.0, 0.0, white.clone()));

    world.push(RectAA::xz(0.0, 555.0, 0.0, 555.0, 555.0, white.clone()));

    world.push(RectAA::xy(0.0, 555.0, 0.0, 555.0, 555.0, white.clone()));

    let aluminum = Metal::new(SolidColor::new(0.8, 0.85, 0.88), 0.0);

    let cube = Cube::new(
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(165.0, 330.0, 165.0),
        aluminum.clone(),
    );

    let cube = Rotate::new(cube, 15.0);
    let cube = Translate::new(cube, Vector3::new(265.0, 0.0, 295.0));
    if smoke {
        let cube = Constant::new(cube, 0.01, SolidColor::new(0.0, 0.0, 0.0));
        world.push(cube);
    } else {
        world.push(cube);
    }

    let cube = Cube::new(
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(165.0, 165.0, 165.0),
        white.clone(),
    );
    let cube = Rotate::new(cube, -18.0);
    let cube = Translate::new(cube, Vector3::new(130.0, 0.0, 65.0));
    if smoke {
        let cube = Constant::new(cube, 0.01, SolidColor::new(1.0, 1.0, 1.0));
        world.push(cube);
    } else {
        let glass = Dielectric::new(1.5);

        world.push(Sphere::new(
            Vector3::new(190.0, 90.0, 190.0),
            90.0,
            glass.clone(),
        ));
    }

    let background = Vector3::new(0.0, 0.0, 0.0);

    (cam, Box::new(world), Box::new(lights), background)
}

fn final_scene() -> (Camera, Box<dyn Hittable>, Box<dyn Hittable>, Vector3<f32>) {
    let aspect_ratio = 1.0;

    let lookat: Vector3<f32> = Vector3::new(278.0, 278.0, 0.0);
    let lookfrom: Vector3<f32> = Vector3::new(478.0, 278.0, -600.0);

    let cam: Camera = Camera::new(
        lookfrom,
        lookat,
        Vector3::new(0.0, 1.0, 0.0),
        40.0,
        aspect_ratio,
        0.0,
        10.0, // (lookfrom - lookat).magnitude(),
    );

    let mut boxes1 = World::new();

    let ground = Lambertian::new(SolidColor::new(0.48, 0.83, 0.53));

    const BOXES_PER_SIDE: usize = 20;
    for i in 0..BOXES_PER_SIDE {
        for j in 0..BOXES_PER_SIDE {
            let w = 100.0;
            let x0 = -1000.0 + i as f32 * w;
            let z0 = -1000.0 + j as f32 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_double(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.push(Cube::new(
                Vector3::new(x0, y0, z0),
                Vector3::new(x1, y1, z1),
                ground.clone(),
            ));
        }
    }

    let mut world = World::new();

    let boxes1 = Bvh::new(boxes1.objects);

    world.push(boxes1);

    let light = DiffuseLight::new(SolidColor::new(7.0, 7.0, 7.0));
    world.push(FlipFace::new(RectAA::xz(
        123.0,
        423.0,
        147.0,
        412.0,
        554.0,
        light.clone(),
    )));

    let mut lights = World::new();
    lights.push(RectAA::xz(123.0, 423.0, 147.0, 412.0, 554.0, light.clone()));

    let center1: Vector3<f32> = Vector3::new(400.0, 400.0, 200.0);
    // let center2 = center1 + Vector3::new(30.0, 0.0, 0.0);
    let moving_sphere_material = Lambertian::new(SolidColor::new(0.7, 0.3, 0.1));

    // Should be a moving sphere, but never implemented this
    world.push(Sphere::new(center1, 50.0, moving_sphere_material));
    world.push(Sphere::new(
        Vector3::new(260.0, 150.0, 45.0),
        50.0,
        Dielectric::new(1.5),
    ));

    world.push(Sphere::new(
        Vector3::new(0.0, 150.0, 145.0),
        50.0,
        Metal::new(SolidColor::new(0.8, 0.8, 0.9), 1.0),
    ));

    let boundary = Sphere::new(
        Vector3::new(360.0, 150.0, 145.0),
        70.0,
        Dielectric::new(1.5),
    );
    world.push(boundary.clone());
    world.push(Constant::new(boundary, 0.2, SolidColor::new(0.2, 0.4, 0.9)));

    let boundary = Sphere::new(Vector3::new(0.0, 0.0, 0.0), 5000.0, Dielectric::new(1.5));
    world.push(Constant::new(
        boundary,
        0.0001,
        SolidColor::new(1.0, 1.0, 1.0),
    ));

    let emat = Lambertian::new(Image::new("earthmap.jpg"));
    world.push(Sphere::new(Vector3::new(400.0, 200.0, 400.0), 100.0, emat));

    let pertext = Noise::new(0.1);
    world.push(Sphere::new(
        Vector3::new(220.0, 280.0, 300.0),
        80.0,
        Lambertian::new(pertext),
    ));

    let mut boxes2 = World::new();

    let white = Lambertian::new(SolidColor::new(0.73, 0.73, 0.73));

    for _ in 0..1000 {
        boxes2.push(Sphere::new(
            Vector3::new(
                random_double(0.0, 1.0),
                random_double(0.0, 1.0),
                random_double(0.0, 1.0),
            )
            .scale(165.0),
            10.0,
            white.clone(),
        ));
    }

    let boxes2 = Bvh::new(boxes2.objects);

    let boxes2 = Translate::new(
        Rotate::new(boxes2, 15.0),
        Vector3::new(-100.0, 270.0, 395.0),
    );

    world.push(boxes2);

    let background = Vector3::new(0.0, 0.0, 0.0);

    (cam, Box::new(world), Box::new(lights), background)
}

pub fn select_scene(i: usize) -> (Camera, Box<dyn Hittable>, Box<dyn Hittable>, Vector3<f32>) {
    match i {
        0 => three_spheres(),
        1 => random_scene(),
        2 => two_perlin_spheres(true),
        3 => two_perlin_spheres(false),
        4 => earth(),
        5 => simple_light(),
        6 => cornell_box_smoke(false),
        7 => cornell_box_smoke(true),
        8 => final_scene(),
        _ => simple_light(), // _ => three_spheres(),
    }
}
