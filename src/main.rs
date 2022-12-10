use rand::Rng;
use rtweekend::{
    material::{Dielectric, Lambertian, Metal},
    texture::{Checker, SolidColor},
    Color, Point3, RayTracer, Sphere, Vec3, World,
};
use std::{error::Error, fs, io::BufWriter};

#[allow(dead_code)]
mod scene {
    const SAMPLES_PER_PIXEL: u64 = 100;
    const SKY: Color = Color::new(0.7, 0.8, 1.0);

    use std::sync::Arc;

    use super::*;
    use rtweekend::{
        camera::CameraBuilder,
        material::DiffuseLight,
        object::XYRectangle,
        texture::{Image, Noise},
    };

    pub struct Scene {
        pub world: World,
        pub background: Color,
        pub camera_builder: CameraBuilder,
        pub samples_per_pixel: u64,
    }

    impl Default for Scene {
        fn default() -> Self {
            Self {
                world: Default::default(),
                background: SKY,
                camera_builder: Default::default(),
                samples_per_pixel: SAMPLES_PER_PIXEL,
            }
        }
    }

    pub fn random_scene() -> Scene {
        let mut rng = rand::thread_rng();
        let mut world = World::new();

        let ground_mat = Arc::new(Lambertian::new(Checker::new_solids(
            Color::new(0.2, 0.3, 0.1),
            Color::new(0.9, 0.9, 0.9),
        )));
        let ground_sphere = Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_mat);

        world.add(ground_sphere);

        for a in -11..=11 {
            for b in -11..=11 {
                let choose_mat: f64 = rng.gen();
                let center = Point3::new(
                    (a as f64) + rng.gen_range(0.0..0.9),
                    0.2,
                    (b as f64) + rng.gen_range(0.0..0.9),
                );
                let new_center = center + Vec3::new(0.0, rng.gen_range(0.0..0.5), 0.0);

                if choose_mat < 0.8 {
                    // Diffuse
                    let albedo = SolidColor::new(Color::random(0.0..1.0) * Color::random(0.0..1.0));
                    let sphere_mat = Arc::new(Lambertian::new(albedo));
                    let sphere =
                        Sphere::new(center, 0.2, sphere_mat).into_moving(0.0, 1.0, new_center);

                    world.add(sphere);
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = Color::random(0.4..1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    let sphere_mat = Arc::new(Metal::new(albedo, fuzz));
                    let sphere = Sphere::new(center, 0.2, sphere_mat);

                    world.add(sphere);
                } else {
                    // Glass
                    let sphere_mat = Arc::new(Dielectric::new(1.5));
                    let sphere = Sphere::new(center, 0.2, sphere_mat);

                    world.add(sphere);
                }
            }
        }

        let mat1 = Arc::new(Dielectric::new(1.5));
        let mat2 = Arc::new(Lambertian::new(SolidColor::new_rgb(0.4, 0.2, 0.1)));
        let mat3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));

        let sphere1 = Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat1);
        let sphere2 = Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, mat2);
        let sphere3 = Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, mat3);

        world.add(sphere1);
        world.add(sphere2);
        world.add(sphere3);

        Scene {
            world,
            camera_builder: CameraBuilder::default()
                .look_from(13.0, 2.0, 3.0)
                .look_at(0.0, 0.0, 0.0)
                .vertical_field_of_view(20.0)
                .aperture(0.1),
            ..Default::default()
        }
    }

    pub fn two_spheres() -> Scene {
        let mut world = World::new();

        let checker = Checker::new_solids(Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));
        let material = Arc::new(Lambertian::new(checker));
        world.add(Sphere::new(
            Point3::new(0.0, -10.0, 0.0),
            10.0,
            material.clone(),
        ));
        world.add(Sphere::new(Point3::new(0.0, 10.0, 0.0), 10.0, material));

        Scene {
            world,
            camera_builder: CameraBuilder::default()
                .look_from(13.0, 2.0, 3.0)
                .look_at(0.0, 0.0, 0.0)
                .vertical_field_of_view(20.0),
            ..Default::default()
        }
    }

    pub fn two_perlin_spheres() -> Scene {
        let mut world = World::new();

        let perlin = Noise::new(4.0);
        let material = Arc::new(Lambertian::new(perlin));
        world.add(Sphere::new(
            Point3::new(0.0, -1000.0, 0.0),
            1000.0,
            material.clone(),
        ));
        world.add(Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, material));

        Scene {
            world,
            camera_builder: CameraBuilder::default()
                .look_from(13.0, 2.0, 3.0)
                .look_at(0.0, 0.0, 0.0)
                .vertical_field_of_view(20.0),
            ..Default::default()
        }
    }

    pub fn earth() -> Scene {
        let earth_texture = Image::open("img/earthmap.jpg").unwrap();
        let earth_surface = Arc::new(Lambertian::new(earth_texture));
        let globe = Sphere::new(Point3::zeros(), 2.0, earth_surface);

        let world = World::from_vec(vec![Box::new(globe)]);

        Scene {
            world,
            camera_builder: CameraBuilder::new()
                .look_from(13.0, 2.0, 3.0)
                .look_at(0.0, 0.0, 0.0)
                .vertical_field_of_view(20.0),
            ..Default::default()
        }
    }

    pub fn simple_light() -> Scene {
        let mut world = World::new();

        let perlin = Arc::new(Lambertian::new(Noise::new(4.0)));
        world.add(Sphere::new(
            Point3::new(0.0, -1000.0, 0.0),
            1000.0,
            perlin.clone(),
        ));
        world.add(Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, perlin));

        // light is brighter than `(1.0, 1.0, 1.0)` to bright enough to light up the scene
        let diffuse_light = Arc::new(DiffuseLight::solid(Color::new(4.0, 4.0, 4.0)));
        world.add(XYRectangle::new(
            (3.0, 1.0),
            (5.0, 3.0),
            -2.0,
            diffuse_light,
        ));

        Scene {
            world,
            background: Color::BLACK,
            camera_builder: CameraBuilder::new()
                .look_from(26.0, 3.0, 6.0)
                .look_at(0.0, 2.0, 0.0)
                .vertical_field_of_view(20.0),
            samples_per_pixel: 400,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Image
    // Use 16:9 aspect ratio
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_HEIGHT: u64 = 400 / ASPECT_RATIO as u64;
    const MAX_DEPTH: i64 = 50;

    // World
    let scene = scene::simple_light();
    let world = scene.world;
    let background = scene.background;
    let samples_per_pixel = scene.samples_per_pixel;

    // Camera (-1 to 1, -1 to 1, -1 to 0)
    let camera = scene
        .camera_builder
        .view_up(0.0, 1.0, 0.0)
        .aspect_ratio(ASPECT_RATIO)
        .focus_distance(10.0)
        .build();
    println!("{}", camera);

    let mut file = BufWriter::new(fs::File::create("image.ppm")?);

    let tracer = RayTracer {
        world,
        camera,
        background,
        image_height: IMAGE_HEIGHT,
        samples_per_pixel,
        max_depth: MAX_DEPTH,
    };
    tracer.trace(&mut file)?;

    Ok(())
}
