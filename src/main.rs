extern crate cgmath;
extern crate image;
extern crate rand;

use cgmath::{dot, prelude::*, vec3, Vector3};

struct Ray {
    origin: Vector3<f64>,
    direction: Vector3<f64>,
}

struct Intersection {
    pos: Vector3<f64>,
    distance: f64,
    color: Vector3<f64>,
}

struct Sphere {
    center: Vector3<f64>,
    radius: f64,
    color: Vector3<f64>,
}

impl Sphere {
    fn get_intersection(&self, ray: &Ray) -> Option<Intersection> {
        // https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-sphere-intersection
        let l = self.center - ray.origin;
        let tca = dot(l, ray.direction);
        if tca < 0.0 {
            return None;
        }
        let d2 = l.magnitude2() - tca * tca;
        let radius2 = self.radius * self.radius;
        if d2 > radius2 {
            return None;
        }
        let thc = (radius2 - d2).sqrt();
        let mut t0 = tca - thc;
        let mut t1 = tca + thc;
        if t0 > t1 {
            std::mem::swap(&mut t0, &mut t1);
        }
        if t0 < 0.0 {
            t0 = t1;
            if t0 < 0.0 {
                return None;
            }
        }

        Some(Intersection {
            pos: ray.origin + t0 * ray.direction,
            distance: t0,
            color: self.color,
        })
    }
}

fn trace(spheres: &[Sphere], ray: &Ray) -> Vector3<f64> {
    let mut closest_hit = Intersection {
        pos: Vector3::zero(),
        distance: std::f64::MAX,
        color: Vector3::zero(),
    };
    for sphere in spheres {
        if let Some(hit) = sphere.get_intersection(&ray) {
            if hit.distance < closest_hit.distance {
                closest_hit = hit;
            }
        }
    }
    closest_hit.color
}

fn main() {
    let spheres = vec![
        Sphere {
            center: vec3(0.0, -10004.0, -20.0),
            radius: 10000.0,
            color: vec3(0.20, 0.20, 0.20),
        },
        Sphere {
            center: vec3(0.0, 0.0, -20.0),
            radius: 4.0,
            color: vec3(1.00, 0.32, 0.36),
        },
        Sphere {
            center: vec3(5.0, -1.0, -15.0),
            radius: 2.0,
            color: vec3(0.90, 0.76, 0.46),
        },
        Sphere {
            center: vec3(5.0, 0.0, -25.0),
            radius: 3.0,
            color: vec3(0.65, 0.77, 0.97),
        },
        Sphere {
            center: vec3(-5.5, 0.0, -15.0),
            radius: 3.0,
            color: vec3(0.90, 0.90, 0.90),
        },
    ];

    let nx = 640;
    let ny = 480;
    let ns = 1;

    let mut img = image::RgbImage::new(nx, ny);

    let inv_width = 1.0 / f64::from(nx);
    let inv_height = 1.0 / f64::from(ny);
    let fov = 30.0;
    let aspect_ratio = f64::from(nx) / f64::from(ny);
    let angle = (std::f64::consts::PI * 0.5 * fov / 180.0).tan();

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let mut pixel_color = Vector3::zero();
        for _ in 0..ns {
            let u = f64::from(x) + rand::random::<f64>();
            let v = f64::from(y) + rand::random::<f64>();
            let xx = (2.0 * ((u + 0.5) * inv_width) - 1.0) * angle * aspect_ratio;
            let yy = (1.0 - 2.0 * ((v + 0.5) * inv_height)) * angle;
            let ray = Ray {
                origin: Vector3::zero(),
                direction: vec3(xx, yy, -1.0).normalize(),
            };
            pixel_color += trace(&spheres, &ray);
        }
        pixel_color *= 255.9999;
        pixel_color /= f64::from(ns);
        *pixel = image::Rgb([
            pixel_color.x as u8,
            pixel_color.y as u8,
            pixel_color.z as u8,
        ]);
    }

    img.save("out.png").unwrap();
}
