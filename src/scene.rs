use std::collections::HashMap;

use nalgebra::{Point3, Vector3};

pub struct Scene {
    pub objects: HashMap<u64, Object>,
    pub sky_color: [f64; 3],
    pub camera: Camera,
}

impl Default for Scene {
    fn default() -> Self {
        Scene {
            objects: HashMap::new(),
            sky_color: [1.0; 3],
            camera: Camera::default(),
        }
    }
}

pub struct Camera {
    pub position: Point3<f64>,
    pub forward: Vector3<f64>,
    pub up: Vector3<f64>,
    pub angle_x: f64,
    pub angle_y: f64,
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            position: [0.0; 3].into(),
            forward: [-1.0, 0.0, 0.0].into(),
            up: [0.0, 1.0, 0.0].into(),
            angle_x: std::f64::consts::FRAC_PI_2,
            angle_y: std::f64::consts::FRAC_PI_2,
        }
    }
}

pub struct Object {
    pub geometry: Geometry,
    pub material: Material,
}

pub struct Material {
    pub color: [f64; 3],
}

pub enum Geometry {
    Sphere {
        center: Point3<f64>,
        radius: f64,
    },
    Triangle {
        p1: Point3<f64>,
        p2: Point3<f64>,
        p3: Point3<f64>,
    },
}
