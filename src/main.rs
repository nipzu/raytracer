mod renderer;
mod scene;

use renderer::Renderer;
use scene::{Geometry, Material, Object, Scene};

fn main() {
    let renderer = Renderer {
        output_file: "rendered.png".into(),
        num_samples: 1,

        resolution_x: 512,
        resolution_y: 512,
    };

    let mut scene = Scene::default();

    /*scene.objects.insert(
        0,
        Object {
            geometry: Geometry::Sphere {
                center: Vector3::new(-3.0, 1.0, 0.0),
                radius: 1.0,
            },
            material: Material {
                color: [0.5, 1.0, 1.0],
            },
        },
    );*/

    scene.objects.insert(
        1,
        Object {
            geometry: Geometry::Sphere {
                center: [-2.0, -2.0, 0.0].into(),
                radius: 1.0,
            },
            material: Material {
                color: [1.0, 0.5, 1.0],
            },
        },
    );

    scene.objects.insert(
        3,
        Object {
            geometry: Geometry::Sphere {
                center: [-3.0, 0.0, 2.0].into(),
                radius: 1.0,
            },
            material: Material {
                color: [1.0, 1.0, 1.0],
            },
        },
    );

    scene.objects.insert(
        4,
        Object {
            geometry: Geometry::Triangle {
                p1: [-2.0, 1.0, 0.0].into(),
                p2: [-3.0, 0.0, 0.8].into(),
                p3: [-3.1, -1.0, -1.0].into(),
            },
            material: Material {
                color: [0.5, 0.5, 0.5],
            },
        },
    );

    renderer.render(&scene);
}
