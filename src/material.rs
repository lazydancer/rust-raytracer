
use crate::ray::Ray;
use crate::vec3::Vec3;

use rand::Rng;


fn random_in_unit_sphere() -> Vec3 {
	let mut rng = rand::thread_rng();
	let mut p = Vec3::new(1.0, 1.0, 1.0);
    while p.length_squared() >= 1.0 {
        p = Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()) * 2.0 - Vec3::new(1.0, 1.0, 1.0);
    }
    p
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
	v - n * v.dot(n) * 2.0
}

#[derive(Copy, Clone, Debug)]
pub enum Material {
	Lambertian { attenuation: Vec3 },
	Metal { attenuation: Vec3, fuzziness: f32 },
}

impl Material {
	pub fn scatter(&self, r: &Ray, normal: Vec3, point: Vec3) -> (Ray, Vec3, bool) {
	 	let target = point + normal + random_in_unit_sphere();
		
		match self {
            Material::Lambertian { attenuation } => (Ray::new(point, target - point), *attenuation, true),
			Material::Metal { attenuation, fuzziness } => {
				let reflected = reflect(r.dir.unit_vector(), normal);
				let scattered = Ray::new(point, reflected + random_in_unit_sphere() * *fuzziness);
                let b = scattered.dir.dot(normal) > 0.0;
                (scattered, *attenuation, b)
			},
		}
	}

}