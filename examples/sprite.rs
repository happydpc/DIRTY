// wengwengweng

use dirty::*;
use app::*;
use kit::*;
use sprite::*;
use input::Key;

struct Game {
	sprite: Sprite,
}

impl app::State for Game {

	fn init(ctx: &mut app::Ctx) -> Result<Self> {

		let mut sprite = Sprite::from_bytes(ctx, include_bytes!("res/car.png"))?;

		sprite.slice(4, 1);
		sprite.add_anim("run", 0, 3, true);
		sprite.play("run");

		return Ok(Self {
			sprite: sprite,
		});

	}

	fn event(&mut self, ctx: &mut app::Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match e {
			KeyPress(k) => {
				match *k {
					Key::Esc => ctx.quit(),
					Key::F => ctx.toggle_fullscreen(),
					_ => {},
				}
			},
			_ => {},
		}

		return Ok(());

	}

// 	fn imgui(&mut self, ui: &mut imgui::Ui) -> Result<()> {

// 		imgui::Window::new(imgui::im_str!("test"))
// 			.size([320.0, 240.0], imgui::Condition::FirstUseEver)
// 			.build(&ui, || {
// 				ui.text(imgui::im_str!("yo"));
// 			});

// 		return Ok(());
// 	}

	fn update(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		ctx.set_title(&format!("FPS: {} DCS: {}", ctx.fps(), ctx.draw_calls()));
		self.sprite.update(ctx.dt());

		return Ok(());

	}

	fn draw(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		ctx.draw(&self.sprite)?;

		return Ok(());

	}

}

fn main() {

	if let Err(err) = app::launcher()
		.resizable(true)
		.run::<Game>() {
		println!("{}", err);
	}

}

