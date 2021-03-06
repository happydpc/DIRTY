// wengwengweng

use super::*;

#[derive(Clone)]
pub struct Polygon {
	pub(super) pts: Vec<Vec2>,
	pub(super) fill: Option<Color>,
	pub(super) stroke: Option<Stroke>,
}

impl Polygon {
	pub fn from_pts(pts: &[Vec2]) -> Self {
		return Self {
			pts: pts.to_vec(),
			fill: Some(rgba!()),
			stroke: None,
		};
	}
	pub fn fill(mut self, c: Color) -> Self {
		self.fill = Some(c);
		return self;
	}
	pub fn no_fill(mut self) -> Self {
		self.fill = None;
		return self;
	}
	pub fn opacity(mut self, a: f32) -> Self {
		if let Some(fill) = &mut self.fill {
			fill.a = a;
		}
		if let Some(stroke) = &mut self.stroke {
			stroke.color.a = a;
		}
		return self;
	}
	pub fn stroke(mut self, c: Color) -> Self {
		self.stroke = Some(Stroke {
			width: 1.0,
			join: LineJoin::None,
			dash: None,
			color: c,
		});
		return self
	}
	pub fn line_join(mut self, j: LineJoin) -> Self {
		if let Some(stroke) = &mut self.stroke {
			stroke.join = j;
		}
		return self;
	}
	pub fn line_width(mut self, w: f32) -> Self {
		if let Some(stroke) = &mut self.stroke {
			stroke.width = w;
		}
		return self;
	}
}

pub fn polygon(pts: &[Vec2]) -> Polygon {
	return Polygon::from_pts(pts);
}

impl Drawable for Polygon {

	fn draw(&self, ctx: &mut Gfx) -> Result<()> {

		if self.pts.len() < 3 {
			return Ok(());
		}

		if let Some(color) = self.fill {

			let mut verts = Vec::with_capacity(self.pts.len());
			let mut indices = Vec::with_capacity((self.pts.len() - 2) * 3);

			for (i, p) in self.pts.iter().enumerate() {

				verts.push(Vertex {
					pos: ctx.transform * vec3!(p.x, p.y, 0.0),
					uv: vec2!(0),
					normal: vec3!(0, 0, 1),
					color,
				});

				// TODO: questionable triangulation
				if i >= 2 {
					indices.extend_from_slice(&[0, (i as u32 - 1), i as u32]);
				}

			}

			ctx.draw(&raw(&verts, &indices).transformed())?;

		}

		if let Some(stroke) = &self.stroke {

			// TODO: line join
			for i in 0..self.pts.len() {

				let p1 = self.pts[i];
				let p2 = self.pts[(i + 1) % self.pts.len()];

				use LineJoin::*;

				match stroke.join {
					None => {
						ctx.draw(&line(p1, p2).width(stroke.width).color(stroke.color))?;
					},
					Bevel => {
						// TODO
						ctx.draw(&line(p1, p2).width(stroke.width).color(stroke.color))?;
					},
					Miter => {
						// TODO
						ctx.draw(&line(p1, p2).width(stroke.width).color(stroke.color))?;
					},
					Round => {
						ctx.draw(&line(p1, p2).width(stroke.width).color(stroke.color).cap(LineCap::Round))?;
					},
				}

			}

		}

		return Ok(());

	}

}

