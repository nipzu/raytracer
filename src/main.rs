mod color;
mod geometry;
mod renderer;
mod scene;
mod shader;

use std::collections::HashMap;

use image::io::Reader;

use color::Color;
use geometry::{Sphere, Triangle};
use renderer::Renderer;
use scene::{Camera, ImageSkyboxShader, Object, Scene};
use shader::{EmissionShader, MirrorShader};

fn main() {
    let renderer = Renderer {
        output_file: "rendered.png".into(),
        num_samples: 100,
        resolution_x: 1000,
        resolution_y: 1000,
    };

    let mut scene = Scene {
        objects: HashMap::new(),
        skybox_shader: Box::new(ImageSkyboxShader::new(
            Reader::open("target.png")
                .unwrap()
                .decode()
                .unwrap()
                .into_rgb(),
        )),
        camera: Camera {
            position: [-5.0, 0.0, 0.0].into(),
            forward: [1.0, 0.0, 0.0].into(),
            up: [0.0, 1.0, 0.0].into(),
            angle_x: std::f64::consts::FRAC_PI_3,
            angle_y: std::f64::consts::FRAC_PI_3,
        },
    };

    let s1 = Sphere::new([0.0, 0.0, 0.0].into(), 1.0);
    let s2 = Sphere::new([0.0, 2.0, 0.0].into(), 0.25);
    let t1 = Triangle::new([
        [2.0, 1.0, 0.0].into(),
        [3.1, -1.0, -1.0].into(),
        [3.0, 0.0, 0.8].into(),
    ]);
    let e1 = MirrorShader::new(Color::new(1.0, 1.0, 1.0));
    let e2 = EmissionShader::new(Color::new(0.5, 0.5, 0.5));
    scene.objects.insert(3, Object::new(&s1, &e1));
    scene.objects.insert(5, Object::new(&s2, &e1));
    scene.objects.insert(4, Object::new(&t1, &e2));

    renderer.render(&scene);
}
