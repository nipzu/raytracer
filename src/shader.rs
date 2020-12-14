use rand::rngs::SmallRng;

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
        let ag = 0.7 + 0.3 * (1.0 - ray.direction.dot(&-n)).powf(5.0);
        ag * &(&r * &self.color)
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
