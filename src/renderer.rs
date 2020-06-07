use std::path::PathBuf;

use crate::scene::{Geometry, Object, Scene};

use nalgebra::{Vector3, Point3};

pub struct Renderer {
    pub output_file: PathBuf,
    pub num_samples: usize,
    pub resolution_x: usize,
    pub resolution_y: usize,
}

#[derive(Debug)]
struct Ray {
    origin: Point3<f64>,
    direction: Vector3<f64>,
}

impl Renderer {
    pub fn render(&self, scene: &Scene) {
        let mut rendered_image = vec![0.0; self.resolution_x * self.resolution_y * 3];

        let cam = &scene.camera;

        let x = f64::tan(cam.angle_x / 2.0);
        let y = f64::tan(cam.angle_y / 2.0);

        let right = cam.forward.cross(&cam.up).normalize();

        let s = cam.position + cam.forward - x * right + y * cam.up;

        let dx = 2.0 * x / self.resolution_x as f64;
        let dy = 2.0 * y / self.resolution_y as f64;

        for py in 0..self.resolution_y {
            for px in 0..self.resolution_x {
                let ray = (s - (py as f64 + 0.5) * dy * cam.up + (px as f64 + 0.5) * dx * right
                    - cam.position)
                    .normalize();
                let pixel = render_ray(
                    scene,
                    &Ray {
                        origin: cam.position,
                        direction: ray,
                    },
                    0,
                );
                let idx = 3 * (self.resolution_x * py + px);
                rendered_image[idx..idx + 3].copy_from_slice(&pixel);
            }
        }

        let image_buffer = rendered_image
            .iter()
            .map(|x| (x * 255.0) as u8)
            .collect::<Vec<u8>>();

        image::save_buffer_with_format(
            &self.output_file,
            &image_buffer,
            self.resolution_x as u32,
            self.resolution_y as u32,
            image::ColorType::Rgb8,
            image::ImageFormat::Png,
        )
        .unwrap();
    }
}

/*fn render_pixel(scene: &Scene, ray: Vector3<f64>) -> [f64; 3] {
    if let Some((_, dist)) = get_closest_intersection(
        scene,
        &Ray {
            origin: scene.camera.position,
            direction: ray,
        },
    ) {
        [0.8 - dist, 0.0, dist + 0.2]
    } else {
        scene.sky_color
    }
}*/

fn render_ray(scene: &Scene, ray: &Ray, ignore_id: u64) -> [f64; 3] {
    if let Some((obj, id, dist)) = get_closest_intersection(scene, ray, ignore_id) {
        let reflected = get_reflected_vector(obj, ray.origin + dist * ray.direction, ray.direction);
        let [rr, rg, rb] = render_ray(scene, &reflected, id);
        let [or, og, ob] = obj.material.color;
        let ag = 0.7
            + 0.3
                * (1.0
                    - f64::cos(ray.direction.angle(&-get_intersection_normal(
                        obj,
                        ray.origin + dist * ray.direction,
                    ))))
                .powf(5.0);
        [ag * rr * or, ag * rg * og, ag * rb * ob]
    } else {
        scene.sky_color
    }
}

fn get_closest_intersection<'a>(
    scene: &'a Scene,
    ray: &Ray,
    ignore_id: u64,
) -> Option<(&'a Object, u64, f64)> {
    let intersections = scene.objects.iter().filter_map(|(id, obj)| {
        if *id != ignore_id {
            Some((id, obj, get_intersection_dist(obj, ray)))
        } else {
            None
        }
    });

    let mut closest: Option<(&Object, u64, f64)> = None;

    for (id, obj, option_dist) in intersections {
        if let Some(dist) = option_dist {
            if closest.is_none() || dist < closest.unwrap().2 {
                closest = Some((obj, *id, dist));
            }
        }
    }

    closest
}

fn get_intersection_dist(obj: &Object, ray: &Ray) -> Option<f64> {
    match obj.geometry {
        Geometry::Sphere { center, radius } => {
            let a = ray.direction.norm_squared();
            let b = 2.0
                * ray
                    .direction
                    .component_mul(&(ray.origin - center))
                    .dot(&Vector3::new(1.0, 1.0, 1.0));
            let c = (ray.origin - center).norm_squared() - radius * radius;

            let discr = b * b - 4.0 * a * c;

            if discr > 0.0 {
                let t1 = (-b + discr.sqrt()) / (2.0 * a);
                let t2 = (-b - discr.sqrt()) / (2.0 * a);

                if t1 < 0.0 && t2 < 0.0 {
                    None
                } else if t1 > 0.0 && t2 > 0.0 {
                    Some(t1.min(t2))
                } else {
                    None
                    //panic!("camera inside sphere!");
                }
            } else {
                None
            }
        }
        Geometry::Triangle { p1, p2, p3 } => {
            let normal = get_intersection_normal(obj, Point3::origin());
            let dist = (normal.dot(&(p1 - ray.origin))) / normal.dot(&ray.direction);
            let s = ray.origin + dist * ray.direction - p1;
            let d1 = p2 - p1;
            let d2 = p3 - p1;

            let a = (s[0] * d1[1] - s[1] * d1[0]) / (d1[1] * d2[0] - d1[0] * d2[1]);
            let b = (s[0] * d2[1] - s[1] * d2[0]) / (d1[0] * d2[1] - d1[1] * d2[0]);

            if normal.dot(&ray.direction) < 0.0 && 0.0 <= a && 0.0 <= b && a+b <= 1.0 {
                Some(dist)
            } else {
                None
            }
        }
    }
}

fn get_intersection_normal(obj: &Object, point: Point3<f64>) -> Vector3<f64> {
    match obj.geometry {
        Geometry::Sphere { center, radius } => (point - center) / radius,
        Geometry::Triangle { p1, p2, p3 } => ((p2 - p1).cross(&(p3 - p1))).normalize(),
    }
}

fn get_reflected_vector(obj: &Object, intersection: Point3<f64>, ray_dir: Vector3<f64>) -> Ray {
    let n = get_intersection_normal(obj, intersection);
    let w = n.dot(&ray_dir) / n.norm_squared() * n;
    Ray {
        origin: intersection,
        direction: (ray_dir - 2.0 * w).normalize(),
    }
}
