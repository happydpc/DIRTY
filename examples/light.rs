// wengwengweng

use dirty::*;
use math::*;
use input::Key;
use gfx::Camera;

pub enum Light {
	Point {
		pos: Vec3,
		color: Color,
	},
	Directional {
		dir: Vec3,
	},
}

struct Game {
	model: gfx::Model,
	canvas: gfx::Canvas,
	canvas2: gfx::Canvas,
	blur_shader: gfx::Shader2D<BlurUniform>,
	vhs_shader: gfx::Shader2D<VHSUniform>,
	pix_shader: gfx::Shader2D<PixUniform>,
	filter_shader: gfx::Shader2D<FilterUniform>,
	shader: gfx::Shader3D<LightUniform>,
	cam: gfx::PerspectiveCam,
	move_speed: f32,
	eye_speed: f32,
	skybox: gfx::Skybox,
	size: f32,
}

#[derive(Clone)]
pub struct PixUniform {
	pub resolution: Vec2,
	pub size: f32,
}

impl gfx::Uniform for PixUniform {
	fn values(&self) -> gfx::UniformValues {
		return hmap![
			"u_resolution" => &self.resolution,
			"u_size" => &self.size,
		];
	}
}

#[derive(Clone)]
pub struct BlurUniform {
	pub dir: Vec2,
	pub resolution: Vec2,
}

impl gfx::Uniform for BlurUniform {
	fn values(&self) -> gfx::UniformValues {
		return hmap![
			"u_dir" => &self.dir,
			"u_resolution" => &self.resolution,
		];
	}
}

#[derive(Clone)]
pub struct VHSUniform {
	pub intensity: f32,
}

impl gfx::Uniform for VHSUniform {
	fn values(&self) -> gfx::UniformValues {
		return hmap![
			"u_intensity" => &self.intensity,
		];
	}
}

#[derive(Clone)]
pub struct FilterUniform {
	pub threshold: f32,
}

impl gfx::Uniform for FilterUniform {
	fn values(&self) -> gfx::UniformValues {
		return hmap![
			"u_threshold" => &self.threshold,
		];
	}
}

#[derive(Clone)]
struct LightUniform {
	pos: Vec3,
	color: Vec3,
	diffuse: f32,
	specular: f32,
	shininess: f32,
	view_pos: Vec3,
}

impl gfx::Uniform for LightUniform {
	fn values(&self) -> gfx::UniformValues {
		return hmap![
			"u_light.pos" => &self.pos,
			"u_light.color" => &self.color,
			"u_material.diffuse" => &self.diffuse,
			"u_material.specular" => &self.specular,
			"u_material.shininess" => &self.shininess,
			"u_view_pos" => &self.view_pos,
		];
	}
}

impl State for Game {

	fn init(ctx: &mut Ctx) -> Result<Self> {

		let model = gfx::Model::from_glb(
			ctx,
			include_bytes!("res/btfly.glb"),
		)?;

		let model = gfx::Model::from_obj(
			ctx,
			include_str!("res/truck.obj"),
			Some(include_str!("res/truck.mtl")),
			None,
		)?;

		let bbox = model.bbox();
		let size = (bbox.max - bbox.min).len();

// 		model.update(|data| {
// 			for m in data {
// 				for v in &mut m.vertices {
// 					v.color = rgba!(rand!(), rand!(), rand!(), 1);
// 				}
// 			}
// 		});

// 		let mut model = gfx::Model::from_gltf(ctx, "examples/res/Duck.gltf")?;
// 		let mut model = gfx::Model::from_glb(ctx, include_bytes!("res/duck.glb"))?;

		let skybox = gfx::Skybox::from_bytes(
			ctx,
			include_bytes!("res/forest_rt.png"),
			include_bytes!("res/forest_lf.png"),
			include_bytes!("res/forest_up.png"),
			include_bytes!("res/forest_dn.png"),
			include_bytes!("res/forest_bk.png"),
			include_bytes!("res/forest_ft.png"),
		)?;

		return Ok(Self {
			model: model,
			vhs_shader: gfx::Shader2D::from_frag(ctx, include_str!("res/vhs.frag"))?,
			pix_shader: gfx::Shader2D::from_frag(ctx, include_str!("res/pix.frag"))?,
			blur_shader: gfx::Shader2D::from_frag(ctx, include_str!("res/blur.frag"))?,
			filter_shader: gfx::Shader2D::from_frag(ctx, include_str!("res/filter.frag"))?,
			canvas: gfx::Canvas::new(ctx, 640, 480)?,
			canvas2: gfx::Canvas::new(ctx, 640, 480)?,
			cam: gfx::PerspectiveCam::new(60f32.to_radians(), ctx.width() as f32 / ctx.height() as f32, 0.1, 1024.0, vec3!(3, 3, 2), -0.92, -0.56),
			shader: gfx::Shader3D::from_frag(ctx, include_str!("res/light.frag"))?,
			size: size,
			move_speed: size * 2.0,
			eye_speed: 0.16,
			skybox: skybox,
		});

	}

	fn event(&mut self, ctx: &mut Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match e {

			Resize(w, h) => {

				self.canvas.resize(ctx, *w, *h)?;
				self.canvas2.resize(ctx, *w, *h)?;

			},

			KeyPress(k) => {
				let mods = ctx.key_mods();
				match *k {
					Key::Esc => {
						ctx.toggle_cursor_hidden();
						ctx.toggle_cursor_locked()?;
					},
					Key::F => ctx.toggle_fullscreen(),
					Key::Q if mods.meta => ctx.quit(),
					_ => {},
				}
			},

			MouseMove(delta) => {

				if ctx.is_cursor_hidden() {

					let mut rx = self.cam.yaw();
					let mut ry = self.cam.pitch();
					let dead = 48.0f32.to_radians();

					rx += delta.x * self.eye_speed * ctx.dt();
					ry -= delta.y * self.eye_speed * ctx.dt();

					if ry > dead {
						ry = dead;
					}

					if ry < -dead {
						ry = -dead;
					}

					self.cam.set_angle(rx, ry);

				}

			},

			_ => {},

		}

		return Ok(());

	}

	fn update(&mut self, ctx: &mut Ctx) -> Result<()> {

		if ctx.key_down(Key::W) {
			self.cam.set_pos(self.cam.pos() + self.cam.front() * ctx.dt() * self.move_speed);
		}

		if ctx.key_down(Key::S) {
			self.cam.set_pos(self.cam.pos() - self.cam.front() * ctx.dt() * self.move_speed);
		}

		if ctx.key_down(Key::A) {
			self.cam.set_pos(self.cam.pos() - self.cam.front().cross(vec3!(0, 1, 0)).unit() * ctx.dt() * self.move_speed);
		}

		if ctx.key_down(Key::D) {
			self.cam.set_pos(self.cam.pos() + self.cam.front().cross(vec3!(0, 1, 0)).unit() * ctx.dt() * self.move_speed);
		}

		ctx.draw_on(&self.canvas, |ctx| {

			ctx.clear();

			let light_pos = vec3!(
				ctx.time().sin() * self.size / 2.0,
				self.size / 2.0,
				ctx.time().cos() * self.size / 2.0,
			);

			ctx.use_cam(&self.cam, |ctx| {

// 				ctx.draw(&shapes::skybox(&self.skybox))?;
// 				ctx.draw_t(&gfx::t().t3(light_pos).s3(vec3!(self.size / 24.0)), &shapes::cube())?;

				ctx.draw_3d_with(&self.shader, &LightUniform {
					pos: light_pos,
					color: vec3!(1, 1, 1),
					diffuse: 0.4,
					specular: 0.4,
					shininess: 16.0,
					view_pos: self.cam.pos(),
				}, |ctx| {
					ctx.draw(&shapes::model(&self.model))?;
					return Ok(());
				})?;

// 				ctx.draw(&shapes::rect(vec2!(-100), vec2!(100)))?;
// 				ctx.draw(&shapes::cube())?;

				return Ok(());

			})?;

			return Ok(());

		})?;

		ctx.draw_on(&self.canvas2, |ctx| {
			ctx.clear();
			ctx.draw_2d_with(&self.pix_shader, &PixUniform {
				resolution: vec2!(ctx.width(), ctx.height()),
				size: 3.0,
			}, |ctx| {
				ctx.draw(&shapes::canvas(&self.canvas))?;
				return Ok(());
			})?;
			return Ok(());
		})?;

		ctx.set_title(&format!("{}", ctx.fps()));

		return Ok(());

	}

	fn draw(&mut self, ctx: &mut Ctx) -> Result<()> {

// 		ctx.draw_2d_with(&self.vhs_shader, &VHSUniform {
// 			intensity: 16.0,
// 		}, |ctx| {
			ctx.draw(&shapes::canvas(&self.canvas2))?;
// 			return Ok(());
// 		})?;

		return Ok(());

	}

}

fn main() {

	if let Err(err) = launcher()
// 		.cursor_relative(true)
		.cursor_hidden(true)
		.cursor_locked(true)
		.resizable(true)
		.run::<Game>() {
		println!("{}", err);
	}

}
