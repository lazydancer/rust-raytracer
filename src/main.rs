mod vec3;
mod ray;
mod hittable;
mod camera;
mod material;

use vec3::Vec3;
use ray::Ray;
use hittable::{ Sphere, Hittable, HittableList, XYRect, XZRect, YZRect };
use camera::Camera;
use material::Material;

use rand::Rng;
use std::fs;
use std::time::Instant;


fn ray_color(r: Ray, world: &HittableList, depth: i32) -> Vec3 {
	if depth <= 0 {
		return Vec3::zero()
	}

	match world.hit(&r, 0.001, std::f32::INFINITY) {
		None => {
			Vec3::zero()
		}
		Some(hit_record) => {
			if hit_record.emit != Vec3::zero() {
				return hit_record.emit
			}

			let (target, attenuation, bounce) = hit_record.mat.scatter(&r, hit_record.normal, hit_record.point);
			if bounce {
				attenuation * ray_color( target, world, depth - 1)
			} else {
				Vec3::zero()
			}
		}

	}
}

fn render(i: i32, cam_pos: Vec3) {

	// Image
	let image_width: i32 = 700;
	let image_height: i32 = (image_width as f32 / (16./9.)) as i32;
	let samples_per_pixel = i;
	let max_depth = 50;

	// Camera
	let camera = Camera::new(cam_pos);
	let mut rng = rand::thread_rng();

	// World
	let list: Vec<Box<dyn Hittable>> = vec![
		Box::new(
			Sphere{ 
				center: Vec3::new(0.0, 0.0, -1.0), 
				radius: 0.5, 
				material: Material::Lambertian { attenuation: Vec3::new(1.0,0.3,0.3) },
				emit: Vec3::zero(),
		}),
		Box::new(
			Sphere{ 
				center: Vec3::new(1.5, 0.0, -1.4), 
				radius: 0.5, 
				material: Material::Metal { attenuation: Vec3::new(0.3,1.0,0.3), fuzziness: 0.1 },
				emit: Vec3::zero(),
		}),	
		Box::new(
			Sphere{ 
				center: Vec3::new(0.0, -100.5, -1.0), 
				radius: 100.0, 
				material: Material::Lambertian { attenuation: Vec3::new(0.5,0.5,0.5) },
				emit: Vec3::zero(),
		}),
		Box::new(
			XZRect {
				x0: -1.0,
				x1: 1.0,
				z0: -1.0,
				z1: 1.0,
				k: 5.0,
				material: Material::Lambertian { attenuation: Vec3::new(0.5,0.5,0.5) },
				emit: Vec3::new(13.0,12.0,12.0),
		}),
		// Box 
		Box::new(
			XYRect {
				x0: -2.0,
				x1: -1.0,
				y0: -0.5,
				y1: 0.5,
				k: -1.5,
				material: Material::Metal { attenuation: Vec3::new(0.7,0.7,1.0), fuzziness: 0.8 },
				emit: Vec3::zero(),
		}),
		Box::new(
			XYRect {
				x0: -2.0,
				x1: -1.0,
				y0: -0.5,
				y1: 0.5,
				k: -0.5,
				material: Material::Metal { attenuation: Vec3::new(0.7,0.7,1.0), fuzziness: 0.8 },
				emit: Vec3::zero(),
		}),
		Box::new(
			YZRect {
				y0: -0.5,
				y1: 0.5,
				z0: -1.5,
				z1: -0.5,
				k: -1.0,
				material: Material::Metal { attenuation: Vec3::new(0.7,0.7,1.0), fuzziness: 0.8 },
				emit: Vec3::zero(),
		}),
		Box::new(
			YZRect {
				y0: -0.5,
				y1: 0.5,
				z0: -1.5,
				z1: -0.5,
				k: -2.0,
				material: Material::Metal { attenuation: Vec3::new(0.7,0.7,1.0), fuzziness: 0.8 },
				emit: Vec3::zero(),
		}),
		Box::new(
			XZRect {
				x0: -2.0,
				x1: -1.0,
				z0: -1.5,
				z1: -0.5,
				k: 0.5,
				material: Material::Metal { attenuation: Vec3::new(0.7,0.7,1.0), fuzziness: 0.8 },
				emit: Vec3::zero(),
		})
	];


	let world = HittableList{ list };


	let mut data = format!("P3\n{} {}\n255\n", image_width, image_height+1);

	let start = Instant::now();

	for j in (0..image_height-1).rev() {
		for i in 0..image_width {
			let mut pixel_color = Vec3::zero();

			for _s in 0..samples_per_pixel {
				let u = (i as f32  + rng.gen::<f32>())/ (image_width - 1) as f32;
				let v = (j as f32 + rng.gen::<f32>()) / (image_height - 1) as f32;
				let r = camera.get_ray(u, v);
				pixel_color = pixel_color + ray_color(r, &world, max_depth);
			}

			let ir = (255.999 * pixel_color.x / samples_per_pixel as f32).clamp(0.0, 256.0) as i32;
			let ig = (255.999 * pixel_color.y / samples_per_pixel as f32).clamp(0.0, 256.0) as i32;
			let ib = (255.999 * pixel_color.z / samples_per_pixel as f32).clamp(0.0, 256.0) as i32;
			let pixel = format!("{} {} {}\n", ir, ig, ib);

			data.push_str(&pixel);
		
		}
	}

	println!("Frame: {}, Millis: {}",i,  start.elapsed().as_millis());
	fs::write(format!("/home/james/Downloads/images/{:0>5}.ppm", i), data).expect("unable to write file");


}


fn main() {
	// let total = 1;
	// for frame in 0..total {
	// 	let x = -2.0*((frame as f32)/(total as f32)*2.0*PI).sin();
	// 	let z = 2.0*((frame as f32)/(total as f32)*2.0*PI).cos()-1.0;
	// 	render(frame, Vec3::new(x, 1.0, z));
	// }

	render(10000, Vec3::new(0.0, 1.0, 1.0));


}