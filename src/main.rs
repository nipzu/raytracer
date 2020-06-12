mod renderer;
mod scene;

use std::collections::HashMap;

use image::io::Reader;

use renderer::Renderer;
use scene::{Camera, Geometry, Material, Object, Scene, Skybox};

fn main() {
    let renderer = Renderer {
        output_file: "rendered.png".into(),
        num_samples: 100,
        resolution_x: 1000,
        resolution_y: 1000,
    };

    let mut scene = Scene {
        objects: HashMap::new(),
        skybox: Skybox::Image {
            image: Reader::open("target.png")
                .unwrap()
                .decode()
                .unwrap()
                .into_rgb(),
        },
        camera: Camera {
            position: [6.0, 0.0, 0.0].into(),
            forward: [-1.0, 0.0, 0.0].into(),
            up: [0.0, 1.0, 0.0].into(),
            angle_x: std::f64::consts::FRAC_PI_4,
            angle_y: std::f64::consts::FRAC_PI_4,
        },
    };

    /*scene.objects.insert(
        1,
        Object {
            geometry: Geometry::Sphere {
                center: [-2.0, -2.0, 0.0].into(),
                radius: 1.0,
            },
            material: Material::Emission {
                color: [1.0, 0.5, 1.0],
            },
        },
    );*/

    scene.objects.insert(
        3,
        Object {
            geometry: Geometry::Sphere {
                center: [-3.0, 0.0, 2.0].into(),
                radius: 1.0,
            },
            material: Material::Diffuse {
                color: [1.0, 1.0, 1.0],
            },
        },
    );

    scene.objects.insert(
        5,
        Object {
            geometry: Geometry::Sphere {
                center: [-2.5, -1.0, 1.0].into(),
                radius: 0.25,
            },
            material: Material::Diffuse {
                color: [1.0, 1.0, 1.0],
            },
        },
    );

    scene.objects.insert(
        4,
        Object {
            geometry: Geometry::Triangle {
                p: [
                    [-2.0, 1.0, 0.0].into(),
                    [-3.0, 0.0, 0.8].into(),
                    [-3.1, -1.0, -1.0].into(),
                ],
            },
            material: Material::Diffuse {
                color: [0.5, 0.5, 0.5],
            },
        },
    );

    /*scene.objects.insert(
        6,
        Object {
            geometry: Geometry::Triangle {
                p: [
                    [-200.0, -2.0, -100.0].into(),
                    [-200.0, -2.0, 100.0].into(),
                    [100.0, -2.0, 0.0].into(),
                ],
            },
            material: Material::Diffuse {
                color: [0.8, 0.5, 0.5],
            },
        },
    );*/

    renderer.render(&scene);
}
