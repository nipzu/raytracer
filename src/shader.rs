use rand::rngs::SmallRng;
use rand::Rng;

use nalgebra::Rotation3;

use crate::color::Color;
use crate::renderer::{IntersectionData, Ray};
use crate::scene::Scene;

pub trait Shader {
    fn apply(
        &self,
        scene: &Scene,
        intersection_data: &IntersectionData,
        rng: &mut SmallRng,
    ) -> Color;
}

pub struct DiffuseShader {
    color: Color,
}

impl DiffuseShader {
    pub fn new(color: Color) -> DiffuseShader {
        DiffuseShader { color }
    }
}

impl Shader for DiffuseShader {
    fn apply(
        &self,
        scene: &Scene,
        intersection_data: &IntersectionData,
        rng: &mut SmallRng,
    ) -> Color {
        let pitch = rng.gen::<f64>() * std::f64::consts::FRAC_PI_2;
        let yaw = rng.gen::<f64>() * 2.0 * std::f64::consts::PI;

        let plane = intersection_data
            .ray
            .direction
            .cross(&intersection_data.normal)
            .normalize();
        let mut reflected_dir = intersection_data.normal.clone();
        let pitch_rotation = Rotation3::new(plane * pitch);
        let yaw_rotation = Rotation3::new(intersection_data.normal * yaw);

        reflected_dir = pitch_rotation * reflected_dir;
        reflected_dir = yaw_rotation * reflected_dir;

        let reflected = Ray {
            direction: reflected_dir,
            origin: intersection_data.ray.origin
                + intersection_data.distance * intersection_data.ray.direction,
        };

        let r = scene.render_ray(&reflected, intersection_data.object_id, rng);
        let ag = 0.7
            + 0.3
                * (1.0
                    - intersection_data
                        .ray
                        .direction
                        .dot(&-intersection_data.normal))
                .powf(5.0);
        ag * r * self.color
    }
}

pub struct MirrorShader {
    color: Color,
}

impl MirrorShader {
    pub fn new(color: Color) -> MirrorShader {
        MirrorShader { color }
    }
}

impl Shader for MirrorShader {
    fn apply(
        &self,
        scene: &Scene,
        intersection_data: &IntersectionData,
        rng: &mut SmallRng,
    ) -> Color {
        let n = intersection_data.normal;
        let ray = &intersection_data.ray;
        let w = n.dot(&ray.direction) / n.norm_squared() * n;
        let reflected = Ray {
            origin: ray.origin + intersection_data.distance * ray.direction,
            direction: (ray.direction - 2.0 * w).normalize().normalize(),
        };
        let r = scene.render_ray(&reflected, intersection_data.object_id, rng);
        r * self.color
    }
}

pub struct EmissionShader {
    color: Color,
}

impl EmissionShader {
    pub fn new(color: Color) -> EmissionShader {
        EmissionShader { color }
    }
}

impl Shader for EmissionShader {
    fn apply(
        &self,
        _scene: &Scene,
        _intersection_data: &IntersectionData,
        _rng: &mut SmallRng,
    ) -> Color {
        self.color
    }
}
