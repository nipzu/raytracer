use std::path::PathBuf;

use crate::scene::{Geometry, Material, Object, Scene};

use nalgebra::{Point3, Vector3};

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
        let cam = &scene.camera;
        let right = cam.forward.cross(&cam.up).normalize();
        let hdx = f64::tan(cam.angle_x / 2.0) * right / self.resolution_x as f64;
        let hdy = -f64::tan(cam.angle_y / 2.0) * cam.up / self.resolution_y as f64;
        let tl = cam.forward - self.resolution_x as f64 * hdx - self.resolution_y as f64 * hdy;

        let mut rendered_image = vec![0.0; self.resolution_x * self.resolution_y * 3];
        for py in 0..self.resolution_y {
            for px in 0..self.resolution_x {
                let pixel_tl = tl + px as f64 * 2.0 * hdx + py as f64 * 2.0 * hdy;
                let pixel = self.render_pixel(scene, pixel_tl, 2.0 * hdx, 2.0 * hdy);
                let idx = 3 * (self.resolution_x * py + px);
                rendered_image[idx..idx + 3].copy_from_slice(&pixel);
            }
        }

        self.save_image(&rendered_image);
    }

    fn render_pixel(
        &self,
        scene: &Scene,
        pixel_tl: Vector3<f64>,
        dx: Vector3<f64>,
        dy: Vector3<f64>,
    ) -> [f64; 3] {
        let mut result = Vec::new();
        for (x, y) in [
            (0.125, 0.125), (0.125, 0.375), (0.125, 0.625), (0.125, 0.875),
            (0.375, 0.125), (0.375, 0.375), (0.375, 0.625), (0.375, 0.875),
            (0.625, 0.125), (0.625, 0.375), (0.625, 0.625), (0.625, 0.875),
            (0.875, 0.125), (0.875, 0.375), (0.875, 0.625), (0.875, 0.875)
            ].iter() {
            result.push(render_ray(
                scene,
                &Ray {
                    origin: scene.camera.position,
                    direction: (pixel_tl + *x * dx + *y * dy).normalize(),
                },
                0,
            ));
        }

        let tot = result.iter().fold([0.0; 3], |[xr, xg, xb], [yr, yg, yb]| {
            [xr + yr, xg + yg, xb + yb]
        });

        [tot[0] / 16.0, tot[1] / 16.0, tot[2] / 16.0]
    }

    fn save_image(&self, rendered_image: &[f64]) {
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

fn render_ray(scene: &Scene, ray: &Ray, ignore_id: u64) -> [f64; 3] {
    if let Some((obj, id, dist)) = get_closest_intersection(scene, ray, ignore_id) {
        match obj.material {
            Material::Diffuse { color } => {
                let reflected =
                    get_reflected_vector(obj, ray.origin + dist * ray.direction, ray.direction);
                let [rr, rg, rb] = render_ray(scene, &reflected, id);
                let [or, og, ob] = color;
                let ag = 0.7
                    + 0.3
                        * (1.0
                            - f64::cos(ray.direction.angle(&-get_intersection_normal(
                                obj,
                                ray.origin + dist * ray.direction,
                            ))))
                        .powf(5.0);
                [ag * rr * or, ag * rg * og, ag * rb * ob]
            }
            Material::Emission { color } => color,
        }
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
            let dist = get_intersection_dist(obj, ray);
            if let Some(dist) = dist {
                return Some((obj, *id, dist));
            }
        }
        None
    });

    intersections.min_by(|&(_, _, dist1), &(_, _, dist2)| dist1.partial_cmp(&dist2).unwrap())
}

fn get_intersection_dist(obj: &Object, ray: &Ray) -> Option<f64> {
    match obj.geometry {
        Geometry::Sphere { center, radius } => get_sphere_intersection(ray, center, radius),
        Geometry::Triangle { p } => get_triangle_intersection(ray, p),
    }
}

fn get_sphere_intersection(ray: &Ray, center: Point3<f64>, radius: f64) -> Option<f64> {
    let b = ray.direction.dot(&(ray.origin - center));
    let c = (ray.origin - center).norm_squared() - radius * radius;
    let discr = b * b - c;

    if discr > 0.0 {
        let t1 = -b + discr.sqrt();
        let t2 = -b - discr.sqrt();

        if t1 > 0.0 && t2 > 0.0 {
            return Some(t1.min(t2));
        }
    }
    None
}

fn get_triangle_intersection(ray: &Ray, p: [Point3<f64>; 3]) -> Option<f64> {
    /*let normal = get_intersection_normal(obj, Point3::origin());
    let dist = (normal.dot(&(p1 - ray.origin))) / normal.dot(&ray.direction);
    let s = ray.origin + dist * ray.direction - p1;
    let d1 = p2 - p1;
    let d2 = p3 - p1;

    let a = (s[0] * d1[1] - s[1] * d1[0]) / (d1[1] * d2[0] - d1[0] * d2[1]);
    let b = (s[0] * d2[1] - s[1] * d2[0]) / (d1[0] * d2[1] - d1[1] * d2[0]);

    if normal.dot(&ray.direction) < 0.0 && 0.0 <= a && 0.0 <= b && a + b <= 1.0 {
        Some(dist)
    } else {
        None
    }*/

    let normal = ((p[1] - p[0]).cross(&(p[2] - p[0]))).normalize();
    let dist = (normal.dot(&(p[0] - ray.origin))) / normal.dot(&ray.direction);
    let r = ray.origin + dist * ray.direction;

    let c1 = (p[0] - p[1]).cross(&normal);
    let c2 = (p[1] - p[2]).cross(&normal);
    let c3 = (p[2] - p[0]).cross(&normal);

    if normal.dot(&ray.direction) < 0.0
        && (r - p[0]).dot(&c1) >= 0.0
        && (r - p[1]).dot(&c2) >= 0.0
        && (r - p[2]).dot(&c3) >= 0.0
    {
        Some(dist)
    } else {
        None
    }
}

fn get_intersection_normal(obj: &Object, point: Point3<f64>) -> Vector3<f64> {
    match obj.geometry {
        Geometry::Sphere { center, radius } => (point - center) / radius,
        Geometry::Triangle { p } => ((p[1] - p[0]).cross(&(p[2] - p[0]))).normalize(),
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
