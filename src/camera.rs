use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
	origin: Vec3,
	horizontal: Vec3,
	vertical: Vec3,
	lower_left_corner: Vec3,
}

impl Camera {
	pub fn new(lookfrom: Vec3) -> Self {
		let lookat = Vec3::new(-0.2 , 0.0, -1.0);
		let vup = Vec3::new(0.0, 1.0, 0.0);


		let vfov = 1.1; // rad
		let h = ((vfov/2.0) as f32).tan();
		let aspect_ratio = 16.0/ 9.0;
		let viewport_height = 2.0 * h;
		let viewport_width = aspect_ratio * viewport_height;

		let w = (lookfrom - lookat).unit_vector();
		let u = vup.cross(w).unit_vector();
		let v = w.cross(u);

		let focal_length = 1.0;

		let origin = lookfrom;
		let horizontal = viewport_width * u;
		let vertical = viewport_height * v;
		let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - w;

		Camera {
			origin,
			horizontal,
			vertical,
			lower_left_corner,
		}
	}

	pub fn get_ray(&self, s: f32, t: f32) -> Ray {
		Ray::new(
			self.origin, 
			self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin
		)
	}
}