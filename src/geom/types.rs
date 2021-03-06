// wengwengweng

use serde::Serialize;
use serde::Deserialize;

use crate::math::*;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Ray3 {
	pub origin: Vec3,
	pub dir: Vec3,
}

impl Ray3 {

	pub const fn new(origin: Vec3, dir: Vec3) -> Self {
		return Self {
			origin,
			dir,
		};
	}

	pub fn at(&self, d: f32) -> Vec3 {
		return self.origin + self.dir * d;
	}

}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Ray2 {
	pub origin: Vec2,
	pub dir: Vec2,
}

impl Ray2 {

	pub const fn new(origin: Vec2, dir: Vec2) -> Self {
		return Self {
			origin,
			dir,
		};
	}

	pub fn at(&self, d: f32) -> Vec2 {
		return self.origin + self.dir * d;
	}

}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Line2 {
	pub p1: Vec2,
	pub p2: Vec2,
}

impl Line2 {
	pub const fn new(p1: Vec2, p2: Vec2) -> Self {
		return Self {
			p1,
			p2,
		};
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Line3 {
	pub p1: Vec3,
	pub p2: Vec3,
}

impl Line3 {
	pub const fn new(p1: Vec3, p2: Vec3) -> Self {
		return Self {
			p1,
			p2,
		};
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Rect {
	pub p1: Vec2,
	pub p2: Vec2,
}

impl Rect {

	pub const fn new(p1: Vec2, p2: Vec2) -> Self {
		return Self {
			p1,
			p2,
		};
	}

	pub fn center(&self) -> Vec2 {
		return (self.p1 + self.p2) * 0.5;
	}

	pub fn width(&self) -> f32 {
		return (self.p2.x - self.p1.x).abs();
	}

	pub fn height(&self) -> f32 {
		return (self.p2.y - self.p1.y).abs();
	}

}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct BBox {
	pub min: Vec3,
	pub max: Vec3,
}

impl BBox {

	pub const fn new(min: Vec3, max: Vec3) -> Self {
		return Self {
			min,
			max,
		};
	}

	pub fn wrap(&self, mut pt: Vec3) -> Vec3 {

		if pt.x < self.min.x {
			pt.x = self.max.x;
		}

		if pt.x > self.max.x {
			pt.x = self.min.x;
		}

		if pt.y < self.min.y {
			pt.y = self.max.y;
		}

		if pt.y > self.max.y {
			pt.y = self.min.y;
		}

		if pt.z < self.min.z {
			pt.z = self.max.z;
		}

		if pt.z > self.max.z {
			pt.z = self.min.z;
		}

		return pt;

	}

	pub fn max(b1: Self, b2: Self) -> Self {

		let minx = f32::min(b1.min.x, b2.min.x);
		let miny = f32::min(b1.min.y, b2.min.y);
		let minz = f32::min(b1.min.z, b2.min.z);
		let maxx = f32::max(b1.max.x, b2.max.x);
		let maxy = f32::max(b1.max.y, b2.max.y);
		let maxz = f32::max(b1.max.z, b2.max.z);

		return Self {
			min: vec3!(minx, miny, minz),
			max: vec3!(maxx, maxy, maxz),
		};

	}

	pub fn min(b1: Self, b2: Self) -> Self {

		let minx = f32::max(b1.min.x, b2.min.x);
		let miny = f32::max(b1.min.y, b2.min.y);
		let minz = f32::max(b1.min.z, b2.min.z);
		let maxx = f32::min(b1.max.x, b2.max.x);
		let maxy = f32::min(b1.max.y, b2.max.y);
		let maxz = f32::min(b1.max.z, b2.max.z);

		return Self {
			min: vec3!(minx, miny, minz),
			max: vec3!(maxx, maxy, maxz),
		};

	}

	pub fn center(self) -> Vec3 {
		return (self.min + self.max) * 0.5;
	}

	pub fn transform(&self, t: Mat4) -> Self {

		let ax = self.min.x;
		let ay = self.min.y;
		let az = self.min.z;
		let bx = self.max.x;
		let by = self.max.y;
		let bz = self.max.z;

		let p1 = t * vec3!(ax, by, az);
		let p2 = t * vec3!(bx, by, az);
		let p3 = t * vec3!(bx, ay, az);
		let p4 = t * vec3!(ax, ay, az);
		let p5 = t * vec3!(ax, by, bz);
		let p6 = t * vec3!(bx, by, bz);
		let p7 = t * vec3!(bx, ay, bz);
		let p8 = t * vec3!(ax, ay, bz);

		return [p2, p3, p4, p5, p6, p7, p8].iter().fold(BBox::new(p1, p1), |bbox, p| {

			let minx = f32::min(bbox.min.x, p.x);
			let miny = f32::min(bbox.min.y, p.y);
			let minz = f32::min(bbox.min.z, p.z);
			let maxx = f32::max(bbox.max.x, p.x);
			let maxy = f32::max(bbox.max.y, p.y);
			let maxz = f32::max(bbox.max.z, p.z);

			return BBox {
				min: vec3!(minx, miny, minz),
				max: vec3!(maxx, maxy, maxz),
			};

		});

	}

}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Plane {
	pub normal: Vec3,
	pub dist: f32,
}

impl Plane {

	pub const fn new(normal: Vec3, dist: f32) -> Self {
		return Self {
			normal,
			dist,
		};
	}

	pub fn from_pts(p0: Vec3, p1: Vec3, p2: Vec3) -> Self {

		let normal = Vec3::cross(p1 - p0, p1 - p2).unit();
		let d = -Vec3::dot(p1, normal);

		return Self::new(normal, d);

	}

}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Circle {
	pub center: Vec2,
	pub radius: f32,
}

impl Circle {

	pub const fn new(center: Vec2, radius: f32) -> Self {
		return Self {
			center,
			radius,
		};
	}

	pub fn rect(&self) -> Rect {

		let min = self.center - vec2!(self.radius);
		let max = self.center + vec2!(self.radius);

		return Rect::new(min, max);

	}

}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sphere {
	pub center: Vec3,
	pub radius: f32,
}

impl Sphere {

	pub const fn new(center: Vec3, radius: f32) -> Self {
		return Self {
			center,
			radius,
		};
	}

	pub fn bbox(&self) -> BBox {

		let min = self.center - vec3!(self.radius);
		let max = self.center + vec3!(self.radius);

		return BBox::new(min, max);

	}

}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Axis {
	X,
	Y,
	Z,
}

impl Axis {
	pub fn as_vec3(&self) -> Vec3 {
		return match self {
			Axis::X => vec3!(1, 0, 0),
			Axis::Y => vec3!(0, 1, 0),
			Axis::Z => vec3!(0, 0, 1),
		};
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Dir {
	Right,
	Down,
	Left,
	Up,
}

impl Dir {
	pub fn as_vec2(&self) -> Vec2 {
		return match self {
			Dir::Right => vec2!(1, 0),
			Dir::Down => vec2!(0, 1),
			Dir::Left => vec2!(-1, 0),
			Dir::Up => vec2!(0, -1),
		};
	}
}

