use flexi_logger::Logger;
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
    const IMAGE_WIDTH: u64 = 400;
    const ASPECT_RATIO: f64 = 16.0 / 9.0;

    use std::sync::Arc;

    use super::*;
    use rtweekend::{
        camera::CameraBuilder,
        hit::{rotation::Rotate, translation::Translate, ConstantMedium, BVH},
        material::DiffuseLight,
        object::{rectangle::AxisAlignedRectangle, sphere::MovingSphere, Block},
        texture::{Image, Noise},
        Hit,
    };

    pub struct Scene {
        pub world: World,
        pub background: Color,
        pub camera_builder: CameraBuilder,
        pub samples_per_pixel: u64,
        pub image_width: u64,
        pub aspect_ratio: f64,
    }

    impl Default for Scene {
        fn default() -> Self {
            Self {
                world: Default::default(),
                background: SKY,
                camera_builder: Default::default(),
                samples_per_pixel: SAMPLES_PER_PIXEL,
                image_width: IMAGE_WIDTH,
                aspect_ratio: ASPECT_RATIO,
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
                        Sphere::new(center, 0.2, sphere_mat).into_moving(0.0..1.0, new_center);

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
        let earth_texture = Image::open("texture/earthmap.jpg").unwrap();
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
        let sphere = Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, perlin.clone());
        world.add(sphere);
        world.add(Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, perlin));

        // light is brighter than `(1.0, 1.0, 1.0)` to bright enough to light up the scene
        let diffuse_light = Arc::new(DiffuseLight::new_solid(Color::new(4.0, 4.0, 4.0)));
        world.add(AxisAlignedRectangle::new_xy(
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
            ..Default::default()
        }
    }

    pub fn cornell_box() -> Scene {
        const RED: Color = Color::new(0.65, 0.05, 0.05);
        const WHITE: Color = Color::new(0.73, 0.73, 0.73);
        const GREEN: Color = Color::new(0.12, 0.45, 0.15);
        const LIGHT: Color = Color::new(15.0, 15.0, 15.0);

        let red = Arc::new(Lambertian::new_solid(RED));
        let white = Arc::new(Lambertian::new_solid(WHITE));
        let green = Arc::new(Lambertian::new_solid(GREEN));
        let light = Arc::new(DiffuseLight::new_solid(LIGHT));

        let block_front = Block::new(
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(165.0, 330.0, 165.0),
            white.clone(),
        );
        let block_front = Rotate::new_y(block_front, 15.0);
        let block_front = Translate::new(block_front, Vec3::new(265.0, 0.0, 295.0));

        let block_back = Block::new(
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(165.0, 165.0, 165.0),
            white.clone(),
        );
        let block_back = Rotate::new_y(block_back, -18.0);
        let block_back = Translate::new(block_back, Vec3::new(130.0, 0.0, 65.0));

        let objects: Vec<Box<dyn Hit>> = vec![
            Box::new(AxisAlignedRectangle::new_yz(
                (0.0, 0.0),
                (555.0, 555.0),
                555.0,
                green,
            )),
            Box::new(AxisAlignedRectangle::new_yz(
                (0.0, 0.0),
                (555.0, 555.0),
                0.0,
                red,
            )),
            Box::new(AxisAlignedRectangle::new_xz(
                (213.0, 227.0),
                (343.0, 332.0),
                554.0,
                light,
            )),
            Box::new(AxisAlignedRectangle::new_xz(
                (0.0, 0.0),
                (555.0, 555.0),
                0.0,
                white.clone(),
            )),
            Box::new(AxisAlignedRectangle::new_xz(
                (0.0, 0.0),
                (555.0, 555.0),
                555.0,
                white.clone(),
            )),
            Box::new(AxisAlignedRectangle::new_xy(
                (0.0, 0.0),
                (555.0, 555.0),
                555.0,
                white,
            )),
            Box::new(block_front),
            Box::new(block_back),
        ];

        Scene {
            world: World::from_vec(objects),
            background: Color::BLACK,
            camera_builder: CameraBuilder::new()
                .look_from(278.0, 278.0, -800.0)
                .look_at(278.0, 278.0, 0.0)
                .vertical_field_of_view(40.0),
            aspect_ratio: 1.0,
            image_width: 600,
            samples_per_pixel: 200,
        }
    }

    pub fn cornell_smoke() -> Scene {
        const RED: Color = Color::new(0.65, 0.05, 0.05);
        const WHITE: Color = Color::new(0.73, 0.73, 0.73);
        const GREEN: Color = Color::new(0.12, 0.45, 0.15);
        const LIGHT: Color = Color::new(7.0, 7.0, 7.0);

        let red = Arc::new(Lambertian::new_solid(RED));
        let white = Arc::new(Lambertian::new_solid(WHITE));
        let green = Arc::new(Lambertian::new_solid(GREEN));
        let light = Arc::new(DiffuseLight::new_solid(LIGHT));

        let block_front = Block::new(
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(165.0, 330.0, 165.0),
            red.clone(),
        );
        let block_front = Rotate::new_y(block_front, 15.0);
        let block_front = Translate::new(block_front, Vec3::new(265.0, 0.0, 295.0));
        let block_front = ConstantMedium::new_solid(block_front, Color::BLACK, 0.01);

        let block_back = Block::new(
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(165.0, 165.0, 165.0),
            white.clone(),
        );
        let block_back = Rotate::new_y(block_back, -18.0);
        let block_back = Translate::new(block_back, Vec3::new(130.0, 0.0, 65.0));
        let block_back = ConstantMedium::new_solid(block_back, Color::WHITE, 0.01);

        let objects: Vec<Box<dyn Hit>> = vec![
            Box::new(AxisAlignedRectangle::new_yz(
                (0.0, 0.0),
                (555.0, 555.0),
                555.0,
                green,
            )),
            Box::new(AxisAlignedRectangle::new_yz(
                (0.0, 0.0),
                (555.0, 555.0),
                0.0,
                red,
            )),
            Box::new(AxisAlignedRectangle::new_xz(
                (113.0, 127.0),
                (443.0, 432.0),
                554.0,
                light,
            )),
            Box::new(AxisAlignedRectangle::new_xz(
                (0.0, 0.0),
                (555.0, 555.0),
                0.0,
                white.clone(),
            )),
            Box::new(AxisAlignedRectangle::new_xz(
                (0.0, 0.0),
                (555.0, 555.0),
                555.0,
                white.clone(),
            )),
            Box::new(AxisAlignedRectangle::new_xy(
                (0.0, 0.0),
                (555.0, 555.0),
                555.0,
                white,
            )),
            Box::new(block_front),
            Box::new(block_back),
        ];

        Scene {
            world: World::from_vec(objects),
            background: Color::BLACK,
            camera_builder: CameraBuilder::new()
                .look_from(278.0, 278.0, -800.0)
                .look_at(278.0, 278.0, 0.0)
                .vertical_field_of_view(40.0),
            aspect_ratio: 1.0,
            image_width: 600,
            samples_per_pixel: 200,
        }
    }

    pub fn dielectric_scene() -> Scene {
        let ground = Arc::new(Lambertian::new_solid(Color::new(0.8, 0.8, 0.0)));
        let center = Arc::new(Lambertian::new_solid(Color::new(0.1, 0.2, 0.5)));
        let left = Arc::new(Dielectric::new(1.5));
        let right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

        let mut world = World::new();
        world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, ground));
        world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, center));
        world.add(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, left.clone()));
        world.add(Sphere::new(Point3::new(-1.0, 0.0, -1.0), -0.4, left));
        world.add(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, right));

        Scene {
            world,
            samples_per_pixel: 100,
            camera_builder: CameraBuilder::new()
                .look_from(0.0, 0.0, 0.0)
                .look_at(0.0, 0.0, -1.0),
            ..Default::default()
        }
    }

    pub fn final_scene() -> Scene {
        let ground = Arc::new(Lambertian::new_solid(Color::new(0.48, 0.83, 0.53)));
        let box_width = 100.0;
        let time_range = 0.0..1.0;
        let bottom_blocks = (0..20)
            .flat_map(|i| {
                let ground = ground.clone();
                let mut rng = rand::thread_rng();
                (0..20).map(move |j| {
                    let (i, j) = (i as f64, j as f64);
                    let min_point =
                        Point3::new(-1000.0 + i * box_width, 0.0, -1000.0 + j * box_width);
                    let max_point =
                        min_point + Vec3::new(box_width, rng.gen_range(1.0..101.0), box_width);
                    Block::new(min_point, max_point, ground.clone())
                })
            })
            .map(|b| -> Box<dyn Hit> { Box::new(b) })
            .collect();
        let bottom_blocks = BVH::new(bottom_blocks, time_range.clone());

        let light_material = Arc::new(DiffuseLight::new_solid(Color::new(7.0, 7.0, 7.0)));
        let light =
            AxisAlignedRectangle::new_xz((123.0, 147.0), (423.0, 412.0), 554.0, light_material);

        let center_from = Point3::new(400.0, 400.0, 200.0);
        let center_to = center_from + Vec3::new(30.0, 0.0, 0.0);
        let moving_sphere_material = Arc::new(Lambertian::new_solid(Color::new(0.7, 0.3, 0.1)));
        let moving_sphere = MovingSphere::new(
            time_range.clone(),
            center_from,
            center_to,
            50.0,
            moving_sphere_material,
        );

        let glass_material = Arc::new(Dielectric::new(1.5));
        let glass_sphere = Sphere::new(
            Point3::new(260.0, 150.0, 45.0),
            50.0,
            glass_material.clone(),
        );

        let metal_sphere = Sphere::new(
            Point3::new(0.0, 150.0, 145.0),
            50.0,
            Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0)),
        );

        let blue_sphere_boundary = Sphere::new(
            Point3::new(360.0, 150.0, 145.0),
            70.0,
            glass_material.clone(),
        );
        let blue_sphere =
            ConstantMedium::new_solid(blue_sphere_boundary.clone(), Color::new(0.2, 0.4, 0.9), 0.2);

        let white_sphere_boundary = Sphere::new(Point3::zeros(), 5000.0, glass_material);
        let white_sphere = ConstantMedium::new_solid(white_sphere_boundary, Color::WHITE, 0.0001);

        let earth_texture = Image::open("texture/earthmap.jpg").unwrap();
        let earth = Sphere::new(
            Point3::new(400.0, 200.0, 400.0),
            100.0,
            Arc::new(Lambertian::new(earth_texture)),
        );

        let perlin_texture = Noise::new(0.1);
        let perlin_sphere = Sphere::new(
            Point3::new(220.0, 280.0, 300.0),
            80.0,
            Arc::new(Lambertian::new(perlin_texture)),
        );

        let white = Arc::new(Lambertian::new_solid(Color::constant(0.73)));
        let sphere_blocks = (0..1000)
            .map(|_| Sphere::new(Point3::random(0.0..165.0), 10.0, white.clone()))
            .map(|b| -> Box<dyn Hit> { Box::new(b) })
            .collect();
        let sphere_blocks = Translate::new(
            Rotate::new_y(BVH::new(sphere_blocks, time_range), 15.0),
            Vec3::new(-100.0, 270.0, 395.0),
        );

        let world = World::from_vec(vec![
            Box::new(bottom_blocks),
            Box::new(light),
            Box::new(moving_sphere),
            Box::new(glass_sphere),
            Box::new(metal_sphere),
            Box::new(blue_sphere),
            Box::new(blue_sphere_boundary),
            Box::new(white_sphere),
            Box::new(earth),
            Box::new(perlin_sphere),
            Box::new(sphere_blocks),
        ]);

        Scene {
            world,
            background: Color::BLACK,
            aspect_ratio: 1.0,
            image_width: 800,
            samples_per_pixel: 10000,
            camera_builder: CameraBuilder::new()
                .look_from(478.0, 278.0, -600.0)
                .look_at(278.0, 278.0, 0.0)
                .vertical_field_of_view(40.0),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    Logger::try_with_env()?.start()?;

    // Image
    const MAX_DEPTH: i64 = 50;

    // World
    let scene = scene::final_scene();
    let aspect_ratio = scene.aspect_ratio;
    let image_height = (scene.image_width as f64 / aspect_ratio) as u64;

    // Camera (-1 to 1, -1 to 1, -1 to 0)
    let camera = scene
        .camera_builder
        .view_up(0.0, 1.0, 0.0)
        .aspect_ratio(aspect_ratio)
        .focus_distance(10.0)
        .build();
    println!("{}", camera);

    let mut file = BufWriter::new(fs::File::create("image.ppm")?);

    let tracer = RayTracer {
        world: scene.world,
        camera,
        background: scene.background,
        image_height,
        samples_per_pixel: scene.samples_per_pixel,
        max_depth: MAX_DEPTH,
    };
    tracer.trace(&mut file)?;

    Ok(())
}
