use rand::rngs::SmallRng;
use std::collections::HashMap;

use crate::color::Color;
use crate::renderer::IntersectionData;
use crate::renderer::Ray;
use crate::shader::Shader;
use image::RgbImage;
use nalgebra::{Point3, Vector3};

use crate::geometry::Geometry;

pub struct Scene<'s> {
    pub objects: HashMap<u64, Object<'s>>,
    pub skybox_shader: Box<dyn Shader>,
    pub camera: Camera,
}

impl<'s> Scene<'s> {
    pub fn get_closest_intersection(
        &self,
        ray: &Ray,
        ignore_id: u64,
    ) -> Option<(&'s Object, IntersectionData)> {
        let intersections = self.objects.iter().filter_map(|(id, obj)| {
            if *id != ignore_id {
                let intersection = obj.geometry.get_intersection(ray);
                if let Some((distance, normal)) = intersection {
                    return Some((
                        obj,
                        IntersectionData {
                            distance,
                            normal,
                            object_id: *id,
                            ray: *ray,
                        },
                    ));
                }
                return None;
            }
            None
        });

        intersections.min_by(|itc1, itc2| itc1.1.distance.partial_cmp(&itc2.1.distance).unwrap())
    }

    pub fn render_ray(&self, ray: &Ray, ignore_id: u64, rng: &mut SmallRng) -> Color {
        if let Some((object, intersection_data)) = self.get_closest_intersection(ray, ignore_id) {
            object.shader.apply(self, &intersection_data, rng)
        } else {
            self.skybox_shader.apply(
                self,
                &IntersectionData {
                    distance: f64::INFINITY,
                    normal: -ray.direction,
                    object_id: 0,
                    ray: *ray,
                },
                rng,
            )
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

pub struct Object<'s> {
    pub geometry: &'s dyn Geometry,
    pub shader: &'s dyn Shader,
}

impl<'s> Object<'s> {
    pub fn new(geometry: &'s dyn Geometry, shader: &'s dyn Shader) -> Object<'s> {
        Object { geometry, shader }
    }
}

pub struct ImageSkyboxShader {
    image: RgbImage,
}

impl ImageSkyboxShader {
    pub fn new(image: RgbImage) -> ImageSkyboxShader {
        ImageSkyboxShader { image }
    }
}

impl Shader for ImageSkyboxShader {
    fn apply(
        &self,
        _scene: &Scene,
        intersection_data: &IntersectionData,
        _rng: &mut SmallRng,
    ) -> Color {
        let ray = &intersection_data.ray;
        let yaw = if ray.direction.z.is_sign_positive() {
            1.0 - (-ray.direction.xz().normalize().x).acos() / (2.0 * std::f64::consts::PI)
        } else {
            (-ray.direction.xz().normalize().x).acos() / (2.0 * std::f64::consts::PI)
        };
        let pitch = (ray.direction.xz().norm().atan2(ray.direction.y)) / std::f64::consts::PI;

        let c = self.image.get_pixel(
            ((yaw * self.image.width() as f64) as u32).min(self.image.width() - 1),
            ((pitch * self.image.height() as f64) as u32).min(self.image.height() - 1),
        );
        Color::from_rbg(c[0], c[1], c[2])
    }
}
