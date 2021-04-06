use std::path::PathBuf;
use std::io::{Write, stdout};

use crate::color::Color;
use crate::scene::Scene;

use nalgebra::{Point3, Vector3};

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

pub struct Renderer {
    pub output_file: PathBuf,
    pub num_samples: usize,
    pub resolution_x: usize,
    pub resolution_y: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point3<f64>,
    pub direction: Vector3<f64>,
}

pub struct IntersectionData {
    pub distance: f64,
    pub normal: Vector3<f64>,
    pub object_id: u64,
    pub ray: Ray,
}

impl Renderer {
    pub fn render(&self, scene: &Scene) {
        let cam = &scene.camera;
        let right = cam.forward.cross(&cam.up).normalize();
        let hdx = f64::tan(cam.angle_x / 2.0) * right / self.resolution_x as f64;
        let hdy = -f64::tan(cam.angle_y / 2.0) * cam.up / self.resolution_y as f64;
        let tl = cam.forward - self.resolution_x as f64 * hdx - self.resolution_y as f64 * hdy;

        let mut rng = SmallRng::from_entropy();

        let mut rendered_image = vec![Color::black(); self.resolution_x * self.resolution_y];
        for py in 0..self.resolution_y {
            for px in 0..self.resolution_x {
                if (py * self.resolution_x + px) % 1000 == 0 {
                    print!(
                        "\rRendering {:.1} %",
                        100.0 * (py * self.resolution_x + px) as f64
                            / (self.resolution_x * self.resolution_y) as f64
                    );
                    stdout().flush().unwrap();
                }

                let pixel_tl = tl + px as f64 * 2.0 * hdx + py as f64 * 2.0 * hdy;
                let pixel = self.render_pixel(scene, pixel_tl, 2.0 * hdx, 2.0 * hdy, &mut rng);
                let idx = self.resolution_x * py + px;
                rendered_image[idx] = pixel;
            }
        }
        // TODO clear stdout
        println!("\rRendered 100.0 %");
        self.save_image(&rendered_image);
    }

    fn render_pixel(
        &self,
        scene: &Scene,
        pixel_tl: Vector3<f64>,
        dx: Vector3<f64>,
        dy: Vector3<f64>,
        rng: &mut SmallRng,
    ) -> Color {
        let mut result = Vec::new();
        for _ in 0..self.num_samples {
            let x: f64 = rng.gen();
            let y: f64 = rng.gen();
            result.push(scene.render_ray(
                &Ray {
                    origin: scene.camera.position,
                    direction: (pixel_tl + x * dx + y * dy).normalize(),
                },
                0,
                rng,
            ));
        }

        let tot = result.into_iter().fold(Color::black(), |cx, cy| cx + cy);

        1.0 / self.num_samples as f64 * tot
    }

    fn save_image(&self, rendered_image: &[Color]) {
        let image_buffer = rendered_image
            .into_iter()
            .map(|x| x.to_rgb().to_vec())
            .flatten()
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

        println!("Saved image to {}", self.output_file.to_str().unwrap());
    }
}
