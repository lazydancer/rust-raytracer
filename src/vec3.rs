use std::ops::{Add, Sub, Mul, Div, Neg};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3 {
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

impl Vec3 {
	pub fn new(x: f32, y: f32, z: f32) -> Self {
		Self { x, y, z }
	}

	pub fn zero() -> Self {
		Self { x: 0.0, y: 0.0, z: 0.0 }
	}

	pub fn dot(self, other: Self) -> f32 {
		self.x * other.x + self.y * other.y + self.z * other.z
	}

	pub fn cross(self, other: Self) -> Self {
		Self { x: self.y * other.z - self.z * other.y,
			   y: self.z * other.x - self.x * other.z,
			   z: self.x * other.y - self.y * other.x}
	}

	pub fn length_squared(self) -> f32 {
		self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)
	}

	pub fn length(self) -> f32 {
		self.length_squared().sqrt()
	}

	pub fn unit_vector(self) -> Self {
		self / self.length()
	}


}

impl Add for Vec3 {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		Self {x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
	}
}

impl Sub for Vec3 {
	type Output = Self;

	fn sub(self, other: Self) -> Self {
		Self {x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
	}
}

impl Mul for Vec3 {
	type Output = Self;

	fn mul(self, other: Self) -> Self {
		Self {x: self.x * other.x, y: self.y * other.y, z: self.z * other.z}
	}
}

impl Div for Vec3 {
	type Output = Self;

	fn div(self, other: Self) -> Self {
		Self {x: self.x / other.x, y: self.y / other.y, z: self.z / other.z}
	}
}

impl Mul<f32> for Vec3 {
	type Output = Self;

	fn mul(self, other: f32) -> Self {
		Self {x: self.x * other, y: self.y * other, z: self.z * other}
	}
}
// Reverse of above
impl Mul<Vec3> for f32 {
	type Output = Vec3;

	fn mul(self, other: Vec3) -> Vec3 {
		Vec3 {x: self * other.x, y: self * other.y, z: self * other.z}
	}
}

impl Div<f32> for Vec3 {
	type Output = Self;

	fn div(self, other: f32) -> Self {
		Self {x: self.x / other, y: self.y / other, z: self.z / other}
	}
}

impl Neg for Vec3 {
	type Output = Self;

	fn neg(self) -> Self {
		Vec3 { x: -self.x, y: -self.y, z: -self.z}
	}
}