mod camera;
mod color;
mod common;
mod cube;
mod cylinder;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod vec3;
mod flat_plane;
// use std::io;
use std::rc::Rc;

use camera::Camera;
use color::Color;
use cylinder::Cylinder;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use material::{Lambertian, Opaque};
use std::io::{self, Write};
use ray::Ray;
use sphere::Sphere;
use vec3::{Point3, Vec3};
use flat_plane::Plane;

pub struct Light {
    pub position: Point3,
    pub intensity: Color,
}

impl Light {
    pub fn new(position: Point3, intensity: Color) -> Light {
        Light { position, intensity }
    }
}

fn ray_color(r: &Ray, world: &dyn Hittable, light: &Light, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    let mut rec = HitRecord::new();
    if world.hit(r, 0.001, common::INFINITY, &mut rec) {
        let target = rec.p + rec.normal + vec3::random_in_unit_sphere();
        let shadow_ray = Ray::new(rec.p, light.position - rec.p);
        let mut shadow_rec = HitRecord::new();
        
        if world.hit(&shadow_ray, 0.001, (light.position - rec.p).length(), &mut shadow_rec) {
            // Point is in shadow
            return Color::new(0.0, 0.0, 0.0);
        }
        
        let light_dir = vec3::unit_vector(light.position - rec.p);
        let light_intensity = vec3::dot(light_dir, rec.normal).max(0.0);
        
        let attenuation = light.intensity * light_intensity;
        
        return attenuation * ray_color(&Ray::new(rec.p, target - rec.p), world, light, depth - 1);
    }

    let unit_direction = vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 720;
    const MAX_DEPTH: i32 = 50;

    // World
    let mut world = HittableList::new();
    
    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_cube = Rc::new(Opaque::new(Color::new(0.4, 0.2, 0.1),0.1));
    let material_cylinder = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.0, 0.4, 0.8)));
    
    world.add(Box::new(Plane::new(
        Point3::new(0.0, -0.5, 0.0), // Point sur le plan
        Vec3::new(0.0, 1.0, 0.0),    // Normale du plan (vers le haut)
        material_ground,
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.5, 1.0),
        1.0,
        material_center,
    )));

    //Ajoutez un cube à la scène
    world.add(Box::new(cube::Cube::new(
        Point3::new(3.0, -0.5, -2.0),
        Point3::new(1.0, 1.5, 0.0),
        Some(material_cube),
    )));

    
    world.add(Cylinder::new(
        Vec3::new(5.5, -1.0, 0.5), 
        1.0, 
        2.5, 
        material_cylinder.clone(),
    ));

    // Light
    let light = Light::new(Point3::new(1.0, 10.0, 10.0), Color::new(1.0, 1.0, 1.0));

    // Camera
    let lookfrom = Point3::new(1.0, 15.0, 10.0);
    let lookat = Point3::new(1.0, 0.0, -1.0);
    let vup = Point3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (lookfrom - lookat).length();
    let aperture = 0.0;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        30.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    // Render
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        // Calcul du pourcentage de progression
        let progress = (IMAGE_HEIGHT - j) as f32 / IMAGE_HEIGHT as f32 * 100.0;
        
        // Affichage de la barre de progression
        print_progress_bar(progress);

        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + common::random_double()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + common::random_double()) / (IMAGE_HEIGHT - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, &light, MAX_DEPTH);
            }
            color::write_color(&mut io::stdout(), pixel_color, SAMPLES_PER_PIXEL);
        }
    }
    eprint!("\nDone.\n");
}


fn print_progress_bar(progress: f32) {
    let bar_width = 50;
    let completed_width = (bar_width as f32 * progress / 100.0) as usize;

    eprint!("\r[");
    for i in 0..bar_width {
        if i < completed_width {
            eprint!("#");
        } else {
            eprint!(" ");
        }
    }
    eprint!("] {:.1}%", progress);
    io::stderr().flush().unwrap();
}
