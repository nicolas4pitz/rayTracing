use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::color::Color;
use crate::vecry::{cross, random_in_unit_disk, unit_vector, Point3, Vec3};
use crate::rtweekend::{degrees_to_radians, random_double, INFINITYCONST};
use indicatif::{ProgressBar, ProgressStyle};
use std::sync::Arc;
use std::io::{self, Write};
use std::{fs::File, ops::Mul};
use crate::color::write_color;

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub vfov: f64,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,
    pub defocus_angle: f64,
    pub focus_dist: f64,
    pixel_sample_scale: f64,
    image_height: u32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: u32, samples_per_pixel: u32, max_depth: u32, vfov: f64, lookfrom: Point3, lookat: Point3, vup: Vec3, defocus_angle: f64, focus_dist: f64) -> Self {
        let mut camera = Self {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            vfov,
            lookfrom,
            lookat,
            vup,
            defocus_angle,
            focus_dist,
            pixel_sample_scale: 0.0,
            image_height: 0,
            center: Point3::new(0.0, 0.0, 0.0),
            pixel00_loc: Point3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3::new(0.0, 0.0, 0.0),
            u: Vec3::new(0.0, 0.0, 0.0),
            v: Vec3::new(0.0, 0.0, 0.0),
            w: Vec3::new(0.0, 0.0, 0.0),
            defocus_disk_u: Vec3::new(0.0, 0.0, 0.0),
            defocus_disk_v: Vec3::new(0.0, 0.0, 0.0),
        };
        camera.initialize();
        camera
    }

    pub fn render(&self, world: &dyn Hittable) -> io::Result<()> {
        let mut file = File::create("image.ppm")?;
        writeln!(file, "P3")?;
        writeln!(file, "{} {}", self.image_width, self.image_height)?;
        writeln!(file, "255")?;

        let progressbar = ProgressBar::new(self.image_height as u64);
        progressbar.set_style(ProgressStyle::default_bar().template("[{elapsed}] [{wide_bar:.green}] {percent}% {msg}").unwrap());

        for j in 0..self.image_height {
            progressbar.set_message(format!("Scanlines remaining: {}", self.image_height - j));
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    pixel_color += self.ray_color(&ray, self.max_depth, world);
                }
                write_color(&mut file, &pixel_color.mul(self.pixel_sample_scale));
            }
            progressbar.inc(1);
        }

        progressbar.finish_with_message("Done.");
        Ok(())
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as u32;
        self.image_height = if self.image_height < 1 { 1 } else { self.image_height };

        self.pixel_sample_scale = 1.0 / self.samples_per_pixel as f64;

        self.center = self.lookfrom;

        let theta = degrees_to_radians(self.vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        self.w = unit_vector(&(self.lookfrom - self.lookat));
        self.u = unit_vector(&cross(&self.vup, &self.w));
        self.v = cross(&self.w, &self.u);

        let viewport_u = self.u * viewport_width;
        let viewport_v = -self.v * viewport_height;

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left = self.center - (self.w * self.focus_dist) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;

        let defocus_radius = self.focus_dist * (degrees_to_radians(self.defocus_angle / 2.0)).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc + (self.pixel_delta_u * (i as f64 + offset.x())) + (self.pixel_delta_v * (j as f64 + offset.y()));

        let ray_origin = if self.defocus_angle <= 0.0 { self.center } else { self.defocus_disk_sample() };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square(&self) -> Vec3 {
        Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }

    fn sample_disk(&self, radius: f64) -> Vec3 {
      random_in_unit_disk() * radius 
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = random_in_unit_disk();
        self.center + (self.defocus_disk_u * p.x()) + (self.defocus_disk_v * p.y())
    }

    fn ray_color(&self, r: &Ray, depth: u32, world: &dyn Hittable) -> Color {
        if depth == 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let mut rec = HitRecord::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0), 0.0, false);

        if world.hit(r, Interval::new(0.001, INFINITYCONST), &mut rec) {
            let mut scattered = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
            let mut attenuation: Vec3 = Color::new(0.0, 0.0, 0.0);
            if rec.mat.as_ref().unwrap().scatter(r, &rec, &mut attenuation, &mut scattered) {
                return self.ray_color(&scattered, depth - 1, world);
            }
            return Color::new(0.0, 0.0, 0.0);
        }

        let unit_direction = unit_vector(r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
    }
}