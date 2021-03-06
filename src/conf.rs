// wengwengweng

//! Start Up Configs

use crate::*;
use math::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CanvasRoot {
	Body,
	Element(&'static str),
}

#[derive(Clone, Debug)]
pub struct Conf {
	pub width: i32,
	pub height: i32,
	pub title: String,
	pub resizable: bool,
	pub fullscreen: bool,
	pub always_on_top: bool,
	pub borderless: bool,
	pub transparent: bool,
	pub vsync: bool,
	pub cursor_hidden: bool,
	pub cursor_locked: bool,
	pub clear_color: Color,
	pub cull_face: bool,
	pub canvas_root: CanvasRoot,
	pub default_font: Option<gfx::BitmapFontData>,
}

impl Conf {

	pub fn basic(title: &str, width: i32, height: i32) -> Self {
		return Self {
			title: String::from(title),
			width,
			height,
			..Default::default()
		};
	}

}

impl Default for Conf {

	fn default() -> Self {
		return Self {
			width: 640,
			height: 480,
			title: String::new(),
			resizable: false,
			fullscreen: false,
			always_on_top: false,
			borderless: false,
			transparent: false,
			vsync: true,
			cursor_hidden: false,
			cursor_locked: false,
			clear_color: rgba!(0, 0, 0, 0),
			cull_face: false,
			canvas_root: CanvasRoot::Body,
			default_font: None,
		};
	}

}

#[derive(Default)]
pub struct Launcher {
	pub(crate) conf: Conf,
}

impl Launcher {

	pub fn conf(mut self, c: Conf) -> Self {
		self.conf = c;
		return self;
	}

	pub fn size(mut self, w: i32, h: i32) -> Self {
		self.conf.width = w;
		self.conf.height = h;
		return self;
	}

	pub fn title(mut self, t: &str) -> Self {
		self.conf.title = t.to_owned();
		return self;
	}

	pub fn resizable(mut self, b: bool) -> Self {
		self.conf.resizable = b;
		return self;
	}

	pub fn fullscreen(mut self, b: bool) -> Self {
		self.conf.fullscreen = b;
		return self;
	}

	pub fn vsync(mut self, b: bool) -> Self {
		self.conf.vsync = b;
		return self;
	}

	pub fn cursor_hidden(mut self, b: bool) -> Self {
		self.conf.cursor_hidden = b;
		return self;
	}

	pub fn cursor_locked(mut self, b: bool) -> Self {
		self.conf.cursor_locked = b;
		return self;
	}

	pub fn transparent(mut self, b: bool) -> Self {
		self.conf.transparent = b;
		return self;
	}

	pub fn always_on_top(mut self, b: bool) -> Self {
		self.conf.always_on_top = b;
		return self;
	}

	pub fn clear_color(mut self, c: Color) -> Self {
		self.conf.clear_color = c;
		return self;
	}

	pub fn cull_face(mut self, b: bool) -> Self {
		self.conf.cull_face = b;
		return self;
	}

	pub fn canvas_root(mut self, c: CanvasRoot) -> Self {
		self.conf.canvas_root = c;
		return self;
	}

	pub fn default_font(mut self, f: gfx::BitmapFontData) -> Self {
		self.conf.default_font = Some(f);
		return self;
	}

}

