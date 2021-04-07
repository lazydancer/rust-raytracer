use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::material::Material;

pub struct HittableList {
	pub list: Vec<Box<dyn Hittable>>,
}

pub struct HitRecord {
	pub point: Vec3,
	pub normal: Vec3,
	pub t: f32,
	pub mat: Material,
	pub emit: Vec3,
}

pub trait Hittable {
	fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct Sphere {
	pub center: Vec3,
	pub radius: f32,
	pub material: Material,
	pub emit: Vec3,
}

impl Hittable for Sphere {
	fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
		let oc: Vec3 = r.orig - self.center;
		let a = r.dir.dot(r.dir);
		let b = oc.dot(r.dir);
		let c = oc.dot(oc) - self.radius*self.radius;
		let discriminant = b * b - a * c;
	 	if discriminant > 0.0 {
            let t1 = (-b - discriminant.sqrt()) / a;
            let t2 = (-b + discriminant.sqrt()) / a;
            if t1 < t_max && t1 > t_min {
                let point  = r.at(t1);
                let normal = (point  - self.center) / self.radius;
                Some(HitRecord { point , normal, t:t1, mat: self.material, emit: self.emit })
            } else if t2 < t_max && t2 > t_min {
                let point  = r.at(t2);
                let normal = (point  - self.center) / self.radius;
                Some(HitRecord { point , normal, t:t2, mat: self.material, emit: self.emit })
            } else {
                None
            }
        } else {
           None
        }
	}
}


pub struct XYRect {
	pub x0: f32,
	pub x1: f32,
	pub y0: f32,
	pub y1: f32,
	pub k: f32,
	pub material: Material,
	pub emit: Vec3,
}

impl Hittable for XYRect {
	fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
		let t = (self.k - r.orig.z) / (r.dir.z);
		if t < t_min || t > t_max {
			return None
		}

		let x = r.orig.x + t * r.dir.x;
		let y = r.orig.y + t * r.dir.y;
		if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
			return None
		}

		Some(HitRecord {
			point: r.at(t),
			normal: Vec3::new(0.0, 0.0, 1.0),
			t,
			mat: self.material,
			emit: self.emit,
		})
	}
}


pub struct XZRect {
	pub x0: f32,
	pub x1: f32,
	pub z0: f32,
	pub z1: f32,
	pub k: f32,
	pub material: Material,
	pub emit: Vec3,
}

impl Hittable for XZRect {
	fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
		let t = (self.k - r.orig.y) / (r.dir.y);
		if t < t_min || t > t_max {
			return None
		}

		let x = r.orig.x + t * r.dir.x;
		let z = r.orig.z + t * r.dir.z;
		if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
			return None
		}

		Some(HitRecord {
			point: r.at(t),
			normal: Vec3::new(0.0, 0.0, 1.0),
			t,
			mat: self.material,
			emit: self.emit,
		})
	}
}

pub struct YZRect {
	pub y0: f32,
	pub y1: f32,
	pub z0: f32,
	pub z1: f32,
	pub k: f32,
	pub material: Material,
	pub emit: Vec3,
}

impl Hittable for YZRect {
	fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
		let t = (self.k - r.orig.x) / (r.dir.x);
		if t < t_min || t > t_max {
			return None
		}

		let y = r.orig.y + t * r.dir.y;
		let z = r.orig.z + t * r.dir.z;
		if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
			return None
		}

		Some(HitRecord {
			point: r.at(t),
			normal: Vec3::new(0.0, 0.0, 1.0),
			t,
			mat: self.material,
			emit: self.emit,
		})
	}
}


impl Hittable for HittableList {
	fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
		let mut closest_so_far = t_max;
		let mut res = None;

		for obj in &self.list {
			if let Some(hit_record) = obj.hit(r, t_min, closest_so_far) {
				closest_so_far = hit_record.t;
				res = Some(hit_record)	
			}
		}
		res
	}
}