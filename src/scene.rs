use std::collections::HashMap;

use nalgebra::{Point3, Vector3};

use image::RgbImage;

pub struct Scene {
    pub objects: HashMap<u64, Object>,
    pub skybox: Skybox,
    pub camera: Camera,
}

pub struct Camera {
    pub position: Point3<f64>,
    pub forward: Vector3<f64>,
    pub up: Vector3<f64>,
    pub angle_x: f64,
    pub angle_y: f64,
}

pub struct Object {
    pub geometry: Geometry,
    pub material: Material,
}
pub enum Material {
    Diffuse { color: [f64; 3] },
    Emission { color: [f64; 3] },
}

pub enum Geometry {
    Sphere { center: Point3<f64>, radius: f64 },
    Triangle { p: [Point3<f64>; 3] },
}

pub enum Skybox {
    Static { color: [f64; 3] },
    Image { image: RgbImage },
}
