#![allow(dead_code)]

extern crate image;
extern crate rand;

mod executor;
mod vector;

use vector::{V3, V1, ToVector};
use std::thread;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
struct Ray {
    origin: V3,
    direction: V3
}

impl Ray {
    fn at(&self, t: V1) -> V3 {
        self.origin + t.xxx() * self.direction
    }
}

#[derive(Debug, Clone, Copy)]
struct Material {
    diffuse: V3,
    ambient: V3,
    specular: V3,
    shininess: V1
}

#[derive(Debug, Clone, Copy)]
struct Intersection {
    point: V3,
    normal: V3,
    incidence: V3,
    distance: V1,
    material: Material
}

#[derive(Debug, Clone, Copy)]
enum Shape {
    Sphere {
        center: V3,
        radius: V1,
        material: Material
    },
    Plane {
        point: V3,
        normal: V3,
        material: Material
    }
}

impl Shape {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        match *self {
            Shape::Sphere { center, radius, material } => {
                let l = center - ray.origin;
                let tca = l.dot(ray.direction);

                if *tca < 0.0 {
                    return None;
                }

                let d2 = l.mag2() - tca.mag2();
                let thc2 = radius.mag2() - d2;

                if *thc2 < 0.0 {
                    return None;
                }

                let thc = thc2.fmap(f32::sqrt);
                let t0 = tca - thc;
                let t1 = tca + thc;

                let t = if t0 < t1 && *t0 > 0.0 {
                    t0
                } else if *t1 > 0.0 {
                    t1
                } else {
                    return None;
                };

                let point = ray.at(t);
                let normal = (point - center).norm();
                let material = material.clone();
                let distance = t;
                let incidence = ray.direction;

                Some(Intersection { point, normal, distance, material, incidence })
            },
            Shape::Plane { point, normal, material } => {
                let d = ray.direction.dot(normal);

                if *d == 0.0 {
                    return None;
                }

                let t = (point - ray.origin).dot(normal) / d;
                
                if *t < 0.0 {
                    return None;
                }

                let point = ray.at(t);
                let normal = if *d < 0.0 {
                    normal
                } else {
                    -normal
                };
                let material = material.clone();
                let distance = t;
                let incidence = ray.direction;

                Some(Intersection { point, normal, distance, material, incidence })
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Camera {
    origin: V3,
    x: V3,
    y: V3,
    z: V3,
    fov: f32,
    pixels: (usize, usize)
}

struct Rays {
    pixels: (usize, usize),
    x: V3,
    y: V3,
    z: V3,
    origin: V3,
    tanfov_2: f32,
    aspect_ratio: f32,
    cur_x: usize,
    cur_y: usize,
    start_x: usize,
    start_y: usize,
    n: usize,
    spp: usize
}

impl Iterator for Rays {
    type Item = (usize, usize, Vec<Ray>);

    fn next(&mut self) -> Option<(usize, usize, Vec<Ray>)> {
        let x = self.cur_x + self.start_x;
        let y = self.cur_y + self.start_y;

        if self.cur_y == self.pixels.1 {
            return None;
        }

        if self.cur_x == self.pixels.0 - 1 {
            self.cur_x = 0;
            self.cur_y += 1;
        } else {
            self.cur_x += 1;
        }

        let mut samples = Vec::new();
        for _ in 0..self.spp {
            let pxndc = (x as f32 + rand::random::<f32>() * 0.5 + 0.25) / ((self.pixels.0 * self.n) as f32);
            let pyndc = (y as f32 + rand::random::<f32>() * 0.5 + 0.25) / ((self.pixels.1 * self.n) as f32);

            let px = (2.0 * pxndc - 1.0) / self.aspect_ratio * self.tanfov_2;
            let py = (1.0 - 2.0 * pyndc) * self.tanfov_2;
            let to = px.v().xxx() * self.x + py.v().xxx() * self.y + self.z;
            samples.push(Ray {
                origin: self.origin,
                direction: (to - self.origin).norm()
            });
        }

        Some((x, y, samples))
    }
}

impl Camera {
    fn new(origin: V3, z: V3, y: V3, fov: f32, pixels: (usize, usize)) -> Camera {
        Camera {
            origin, z, y, pixels, fov,
            x: z.cross(-y)
        }
    }

    fn rays(&self, n: usize, spp: usize) -> Vec<Rays> {
        let aspect_ratio = self.pixels.1 as f32 / self.pixels.0 as f32;
        let tanfov_2 = (self.fov/2.0).tan();
        let mut out = Vec::new();
        for i in 0..n {
            for j in 0..n {
                println!("{} {} {} {}", i * (self.pixels.0/n), j * (self.pixels.1/n), (self.pixels.0/n), (self.pixels.1/n));
                out.push(Rays {
                    pixels: (self.pixels.0/n, self.pixels.1/n),
                    x: self.x,
                    y: self.y,
                    z: self.z,
                    origin: self.origin,
                    tanfov_2,
                    aspect_ratio,
                    cur_x: 0,
                    cur_y: 0,
                    start_x: i * (self.pixels.0/n),
                    start_y: j * (self.pixels.1/n),
                    n, spp
                });
            }
        }
        out
    }
}

#[derive(Debug, Clone)]
enum Light{
    Distant { direction: V3, color: V3 },
    Point { position: V3, color: V3 }
}

impl Light {
    fn direction(&self, to: V3) -> V3 {
        match self {
            Light::Distant { direction, .. } => *direction,
            Light::Point { position, .. } => (to - *position).norm()
        }
    }

    fn color(&self) -> V3 {
        match self {
            Light::Distant { color, .. } => *color,
            Light::Point { color, .. } => *color
        }
    }

    fn distance(&self, point: V3) -> V1 {
        match self {
            Light::Distant { .. } => V1(1.0/0.0),
            Light::Point { position, .. } => (point - *position).mag()
        }
    }
}

#[derive(Clone)]
struct RayTracer {
    scene: Vec<Shape>,
    lights: Vec<Light>,
    camera: Camera,
    background_color: V3,
    ambient_intensity: V1,
    max_depth: usize,
    bounce_rays: usize,
    samples_per_pixel: usize
}

impl RayTracer {
    fn trace(&self, ray: &Ray) -> Option<Intersection> {
        let mut best = None;

        for shape in &self.scene {
            if let Some(intersection) = shape.intersect(ray) {
                if let Some(Intersection { distance, .. }) = best {
                    if intersection.distance < distance {
                        best = Some(intersection);
                    }
                } else {
                    best = Some(intersection)
                }
            }
        }

        best
    }

    fn diffuse_at(&self, intersection: &Intersection, depth: usize) -> V3 {
        let mut l_d = 0.0.v().xxx();
        
        for light in &self.lights {
            let ray = Ray {
                origin: intersection.point + 0.01.v().xxx() * intersection.normal,
                direction: -light.direction(intersection.point)
            };

            let occluded = if let Some(intersection) = self.trace(&ray) { 
                intersection.distance < light.distance(intersection.point)
            } else {
                false
            };

            if !occluded {
                l_d += light.color() * intersection.normal.dot(-light.direction(intersection.point)).max(0.0).v().xxx();
            }
        }

        let mut l_i = 0.0.v().xxx();
        if depth < self.max_depth {
            for _ in 0..self.bounce_rays {
                let u1 = rand::random::<f32>();
                let r = u1.sqrt();
                let theta = 2.0 * 3.14159265 * rand::random::<f32>();
                let x = r * theta.cos();
                let y = r * theta.sin();
                let z = (1.0 - u1).max(0.0).sqrt();
                let normal = intersection.normal;
                let tangent = if normal.y().abs() >= normal.x().abs() {
                    (normal.xzy() * V3(0.0, -1.0, 1.0)).norm()
                } else {
                    (normal.zyx() * V3(1.0, 0.0, -1.0)).norm()
                };
                let bitangent = normal.cross(tangent);
                let direction = x.v().xxx() * tangent + y.v().xxx() * normal + z.v().xxx() * bitangent;
                
                let ray = Ray {
                    origin: intersection.point + 0.01.v().xxx() * intersection.normal, direction
                };

                l_i += self.color(&ray, depth + 1);
            }

            l_i /= ((self.bounce_rays as f32) / (2.0 * 3.14159265)).v().xxx();
        }

        intersection.material.diffuse * (l_d + l_i) / 3.14159265.v().xxx()
    }

    fn ambient_at(&self, intersection: &Intersection) -> V3 {
        intersection.material.ambient * self.ambient_intensity.xxx()
    }

    fn specular_at(&self, intersection: &Intersection, depth: usize) -> V3 {
        if depth >= self.max_depth {
            return self.background_color;
        } else {
            let u1: f32 = rand::random();
            let u2: f32 = rand::random();
            let theta = (1.0 - u1).powf(1.0 / (1.0 + intersection.material.shininess.val())).acos();
            let phi = 2.0 * 3.14159265 * u2;
            let x = theta.sin() * phi.cos();
            let z = theta.sin() * phi.sin();
            let y = theta.cos();
            let reflection = intersection.incidence - (2.0.v() * intersection.incidence.dot(intersection.normal)).xxx() * intersection.normal;
            let normal = reflection;
            let tangent = if normal.y().abs() >= normal.x().abs() {
                (normal.xzy() * V3(0.0, -1.0, 1.0)).norm()
            } else {
                (normal.zyx() * V3(1.0, 0.0, -1.0)).norm()
            };
            let bitangent = normal.cross(tangent);
            let direction = x.v().xxx() * tangent + y.v().xxx() * normal + z.v().xxx() * bitangent;

            let ray = Ray {
                origin: intersection.point + 0.01.v().xxx() * intersection.normal, direction
            };

            return intersection.material.specular * self.color(&ray, depth + 1);
        }
    }

    fn color(&self, ray: &Ray, depth: usize) -> V3 {
        if let Some(intersection) = self.trace(ray) {
            let diffuse = self.diffuse_at(&intersection, depth);
            let ambient = self.ambient_at(&intersection);
            let specular = self.specular_at(&intersection, depth);

            diffuse + ambient + specular
        } else {
            self.background_color
        }
    }

    fn render(&self) -> Arc<Mutex<image::RgbImage>> {
        let buffer = Arc::new(Mutex::new(image::ImageBuffer::new(self.camera.pixels.0 as u32, self.camera.pixels.1 as u32)));
        let count = Arc::new(Mutex::new(0));

        let mut handles = Vec::new();
        for rays in self.camera.rays(4, self.samples_per_pixel) {
            let new_self = self.clone();
            let new_buffer = buffer.clone();
            let new_count = count.clone();
            handles.push(thread::spawn(move || {
                for (x, y, samples) in rays {
                    let mut color = 0.0.v().xxx();
                    for ray in samples {
                        color += new_self.color(&ray, 0);
                    }
                    color /= (new_self.samples_per_pixel as f32).v().xxx();

                    {
                        let mut cnt = new_count.lock().unwrap();
                        *cnt += 1;
                        print!("\r{}", *cnt);
                        new_buffer.lock().unwrap().put_pixel(x as u32, y as u32, image::Rgb([
                            (color.x().val().max(0.0).min(1.0) * 255.0) as u8,
                            (color.y().val().max(0.0).min(1.0) * 255.0) as u8,
                            (color.z().val().max(0.0).min(1.0) * 255.0) as u8
                        ]));
                    }

                }
            }));
        }

        handles.drain(..).for_each(|h| h.join().unwrap());
        buffer
    }
}

fn main() {
    let tracer = RayTracer {
        scene: vec![
            Shape::Sphere {
                center: V3(0.0, -5.0, 20.0),
                radius: V1(5.0),
                material: Material {
                    diffuse: V3(0.05, 0.05, 0.1),
                    ambient: V3(0.0, 0.0, 0.0),
                    specular: V3(0.7, 0.7, 0.9),
                    shininess: V1(100.0)
                }
            },
            Shape::Plane {
                point: V3(0.0, -10.0, 0.0),
                normal: V3(0.0, -1.0, 0.0),
                material: Material {
                    diffuse: V3(0.6, 0.6, 0.6),
                    ambient: V3(0.6, 0.6, 0.6),
                    specular: V3(0.0, 0.0, 0.0),
                    shininess: V1(0.0)
                }
            },
            Shape::Plane {
                point: V3(0.0, 10.0, 0.0),
                normal: V3(0.0, 1.0, 0.0),
                material: Material {
                    diffuse: V3(0.6, 0.6, 0.6),
                    ambient: V3(0.6, 0.6, 0.6),
                    specular: V3(0.0, 0.0, 0.0),
                    shininess: V1(0.0)
                }
            },
            Shape::Plane {
                point: V3(-10.0, 0.0, 0.0),
                normal: V3(1.0, 0.0, 0.0),
                material: Material {
                    diffuse: V3(0.6, 0.6, 0.6),
                    ambient: V3(0.6, 0.6, 0.6),
                    specular: V3(0.0, 0.0, 0.0),
                    shininess: V1(0.0)
                }
            },
            Shape::Plane {
                point: V3(10.0, 0.0, 0.0),
                normal: V3(-1.0, 0.0, 0.0),
                material: Material {
                    diffuse: V3(0.6, 0.6, 0.6),
                    ambient: V3(0.6, 0.6, 0.6),
                    specular: V3(0.0, 0.0, 0.0),
                    shininess: V1(0.0)
                }
            },
            Shape::Plane {
                point: V3(0.0, 0.0, 50.0),
                normal: V3(0.0, 0.0, -1.0),
                material: Material {
                    diffuse: V3(0.6, 0.6, 0.6),
                    ambient: V3(0.6, 0.6, 0.6),
                    specular: V3(0.0, 0.0, 0.0),
                    shininess: V1(0.0)
                }
            }
        ],
        lights: vec![
            Light::Point {
                position: V3(4.0, 9.0, 7.0),
                color: V3(0.8, 0.8, 0.8)
            },
            Light::Point {
                position: V3(-4.0, 5.0, 25.0),
                color: V3(0.8, 0.8, 0.8)
            }
        ],
        camera: Camera::new(
            V3(0.0, 0.0, 0.0),
            V3(0.0, 0.0, 1.0),
            V3(0.0, 1.0, 0.0),
            3.141592/2.0,
            (1440, 900)
        ),
        background_color: V3(0.0, 0.0, 0.0),
        ambient_intensity: V1(0.0),
        max_depth: 3,
        bounce_rays: 1,
        samples_per_pixel: 1
    };

    let img = tracer.render();
    img.lock().unwrap().save("out.png").unwrap();
}