use crate::renderer::Ray;
use nalgebra::{Point3, Vector3};

pub trait Geometry {
    fn get_intersection(&self, ray: &Ray) -> Option<(f64, Vector3<f64>)>;
}

pub struct Triangle {
    vertices: [Point3<f64>; 3],
}

impl Triangle {
    pub fn new(vertices: [Point3<f64>; 3]) -> Triangle {
        Triangle { vertices }
    }
}

impl Geometry for Triangle {
    fn get_intersection(&self, ray: &Ray) -> Option<(f64, Vector3<f64>)> {
        let p = &self.vertices;
        let normal = ((p[1] - p[0]).cross(&(p[2] - p[0]))).normalize();
        let distance = -(normal.dot(&ray.origin.coords)) / normal.dot(&ray.direction);
        let r = ray.origin + distance * ray.direction;

        let c1 = (p[0] - p[1]).cross(&normal);
        let c2 = (p[1] - p[2]).cross(&normal);
        let c3 = (p[2] - p[0]).cross(&normal);

        // TWO SIDED OR NOT
        if normal.dot(&ray.direction) < 0.0
            && (r - p[0]).dot(&c1) >= 0.0
            && (r - p[1]).dot(&c2) >= 0.0
            && (r - p[2]).dot(&c3) >= 0.0
        {
            Some((distance, normal))
        } else {
            None
        }
    }
}

pub struct Sphere {
    center: Point3<f64>,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3<f64>, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Geometry for Sphere {
    fn get_intersection(&self, ray: &Ray) -> Option<(f64, Vector3<f64>)> {
        let b = ray.direction.dot(&(ray.origin - self.center));
        let c = (ray.origin - self.center).norm_squared() - self.radius.powi(2);
        let discr = b * b - c;

        if discr > 0.0 {
            let t1 = -b + discr.sqrt();
            let t2 = -b - discr.sqrt();

            if t1 > 0.0 && t2 > 0.0 {
                let distance = t1.min(t2);

                return Some((
                    distance,
                    (ray.origin + distance * ray.direction - self.center) / self.radius,
                ));
            }
        }
        None
    }
}
