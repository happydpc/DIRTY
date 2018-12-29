// wengwengweng

use core::ops::Mul;

macro_rules! make_vec {

	// ugly hack
	($name:ident($short_name:ident) -> ($($member:ident),*): $type:ty, $count:expr) => {
		make_vec!($name($short_name) -> ($($member),*): $type, $count, $);
	};

	($name:ident($short_name:ident) -> ($($member:ident),*): $type:ty, $count:expr, $dol:tt) => {

		#[derive(Debug, Copy, Clone, Default)]
		pub struct $name {
			$(pub $member: $type),*
		}

		impl $name {
			pub fn new($($member: $type,)*) -> Self {
				return Self {
					$($member: $member,)*
				};
			}
			pub fn all(x: $type) -> Self {
				return Self {
					$($member: x,)*
				}
			}
			pub fn as_arr(&self) -> [$type; $count] {
				return [$(self.$member),*];
			}
		}

		pub fn $short_name($($member: $type,)*) -> $name {
			return $name {
				$($member: $member,)*
			};
		}

		#[macro_export]
		macro_rules! $short_name {
			() => {
				$name::default();
			};
			($v:expr) => {
				$name::all($v as $type);
			};
			($dol($v:expr),*) => {
				$name::new($dol($v as $type),*)
			}
		}

	}

}

make_vec!(Vector2(vec2) -> (x, y): f32, 2);
make_vec!(Vector3(vec3) -> (x, y, z): f32, 3);
make_vec!(Vector4(vec4) -> (x, y, z, w): f32, 4);
make_vec!(Color(color) -> (r, g, b, a): f32, 4);
make_vec!(Rect(rect) -> (x, y, w, h): f32, 4);

#[derive(Debug, Copy, Clone)]
pub struct Matrix4 {
	m: [[f32; 4]; 4],
}

impl Matrix4 {

	pub fn identity() -> Self {

		return Self {
			m: [
				[1.0, 0.0, 0.0, 0.0],
				[0.0, 1.0, 0.0, 0.0],
				[0.0, 0.0, 1.0, 0.0],
				[0.0, 0.0, 0.0, 1.0],
			]
		};

	}

	pub fn ortho(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {

		let mut m = Matrix4::identity();

		m.m[0][0] = 2.0 / (right - left);
		m.m[1][1] = 2.0 / (top - bottom);
		m.m[2][2] = 2.0 / (near - far);

		m.m[3][0] = (left + right) / (left - right);
		m.m[3][1] = (bottom + top) / (bottom - top);
		m.m[3][2] = (far + near) / (near - far);

		return m;

	}

	pub fn translate(self, x: f32, y: f32) -> Self {

		let mut m = Matrix4::identity();

		m.m[3][0] = x;
		m.m[3][1] = y;

		return self * m;

	}

	pub fn scale(self, sx: f32, sy: f32) -> Self {

		let mut m = Matrix4::identity();

		m.m[0][0] = sx;
		m.m[1][1] = sy;

		return self * m;

	}

	pub fn rotate(self, rot: f32) -> Self {

		let mut m = Matrix4::identity();

		let c = rot.cos();
		let s = rot.sin();
		let cv = 1.0 - c;

		let axis = vec3!(0, 0, 1);

		m.m[0][0] = (axis.x * axis.x * cv) + c;
		m.m[0][1] = (axis.x * axis.y * cv) + (axis.z * s);
		m.m[0][2] = (axis.x * axis.z * cv) - (axis.y * s);

		m.m[1][0] = (axis.y * axis.x * cv) - (axis.z * s);
		m.m[1][1] = (axis.y * axis.y * cv) + c;
		m.m[1][2] = (axis.y * axis.z * cv) + (axis.x * s);

		m.m[2][0] = (axis.z * axis.x * cv) + (axis.y * s);
		m.m[2][1] = (axis.z * axis.y * cv) - (axis.x * s);
		m.m[2][2] = (axis.z * axis.z * cv) + c;

		return self * m;

	}

	pub fn inverse(&self) -> Self {

		let mut nm = Matrix4::identity();

		let f00 = self.m[2][2] * self.m[3][3] - self.m[3][2] * self.m[2][3];
		let f01 = self.m[2][1] * self.m[3][3] - self.m[3][1] * self.m[2][3];
		let f02 = self.m[2][1] * self.m[3][2] - self.m[3][1] * self.m[2][2];
		let f03 = self.m[2][0] * self.m[3][3] - self.m[3][0] * self.m[2][3];
		let f04 = self.m[2][0] * self.m[3][2] - self.m[3][0] * self.m[2][2];
		let f05 = self.m[2][0] * self.m[3][1] - self.m[3][0] * self.m[2][1];
		let f06 = self.m[1][2] * self.m[3][3] - self.m[3][2] * self.m[1][3];
		let f07 = self.m[1][1] * self.m[3][3] - self.m[3][1] * self.m[1][3];
		let f08 = self.m[1][1] * self.m[3][2] - self.m[3][1] * self.m[1][2];
		let f09 = self.m[1][0] * self.m[3][3] - self.m[3][0] * self.m[1][3];
		let f10 = self.m[1][0] * self.m[3][2] - self.m[3][0] * self.m[1][2];
		let f11 = self.m[1][1] * self.m[3][3] - self.m[3][1] * self.m[1][3];
		let f12 = self.m[1][0] * self.m[3][1] - self.m[3][0] * self.m[1][1];
		let f13 = self.m[1][2] * self.m[2][3] - self.m[2][2] * self.m[1][3];
		let f14 = self.m[1][1] * self.m[2][3] - self.m[2][1] * self.m[1][3];
		let f15 = self.m[1][1] * self.m[2][2] - self.m[2][1] * self.m[1][2];
		let f16 = self.m[1][0] * self.m[2][3] - self.m[2][0] * self.m[1][3];
		let f17 = self.m[1][0] * self.m[2][2] - self.m[2][0] * self.m[1][2];
		let f18 = self.m[1][0] * self.m[2][1] - self.m[2][0] * self.m[1][1];

		nm.m[0][0] = (self.m[1][1] * f00 - self.m[1][2] * f01 + self.m[1][3] * f02);
		nm.m[1][0] = -(self.m[1][0] * f00 - self.m[1][2] * f03 + self.m[1][3] * f04);
		nm.m[2][0] = (self.m[1][0] * f01 - self.m[1][1] * f03 + self.m[1][3] * f05);
		nm.m[3][0] = -(self.m[1][0] * f02 - self.m[1][1] * f04 + self.m[1][2] * f05);

		nm.m[0][1] = -(self.m[0][1] * f00 - self.m[0][2] * f01 + self.m[0][3] * f02);
		nm.m[1][1] = (self.m[0][0] * f00 - self.m[0][2] * f03 + self.m[0][3] * f04);
		nm.m[2][1] = -(self.m[0][0] * f01 - self.m[0][1] * f03 + self.m[0][3] * f05);
		nm.m[3][1] = (self.m[0][0] * f02 - self.m[0][1] * f04 + self.m[0][2] * f05);

		nm.m[0][2] = (self.m[0][1] * f06 - self.m[0][2] * f07 + self.m[0][3] * f08);
		nm.m[1][2] = -(self.m[0][0] * f06 - self.m[0][2] * f09 + self.m[0][3] * f10);
		nm.m[2][2] = (self.m[0][0] * f11 - self.m[0][1] * f09 + self.m[0][3] * f12);
		nm.m[3][2] = -(self.m[0][0] * f08 - self.m[0][1] * f10 + self.m[0][2] * f12);

		nm.m[0][3] = -(self.m[0][1] * f13 - self.m[0][2] * f14 + self.m[0][3] * f15);
		nm.m[1][3] = (self.m[0][0] * f13 - self.m[0][2] * f16 + self.m[0][3] * f17);
		nm.m[2][3] = -(self.m[0][0] * f14 - self.m[0][1] * f16 + self.m[0][3] * f18);
		nm.m[3][3] = (self.m[0][0] * f15 - self.m[0][1] * f17 + self.m[0][2] * f18);

		let det =
			self.m[0][0] * nm.m[0][0] +
			self.m[0][1] * nm.m[1][0] +
			self.m[0][2] * nm.m[2][0] +
			self.m[0][3] * nm.m[3][0];

		for i in 0..4 {
			for j in 0..4 {
				nm.m[i][j] *= (1.0 / det);
			}
		}

		return nm;

	}

	pub fn forward(&self, pt: Vector2) -> Vector2 {

		let mut npt = vec2!();

		npt.x = pt.x * self.m[0][0] + pt.y * self.m[1][0] + self.m[3][0];
		npt.y = pt.x * self.m[0][1] + pt.y * self.m[1][1] + self.m[3][1];

		return npt;

	}

	pub fn as_arr(&self) -> [[f32; 4]; 4] {
		return self.m;
	}

}

impl Mul for Matrix4 {

	type Output = Self;

	fn mul(self, other: Self) -> Self {

		let mut nm = Matrix4::identity();

		for i in 0..4 {
			for j in 0..4 {
				nm.m[i][j] =
					self.m[0][j] * other.m[i][0] +
					self.m[1][j] * other.m[i][1] +
					self.m[2][j] * other.m[i][2] +
					self.m[3][j] * other.m[i][3];
			}
		};

		return nm;

	}

}

