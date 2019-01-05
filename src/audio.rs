// wengwengweng

//! Handles sounds

use std::io::Cursor;
use std::collections::HashMap;

use rodio::Source;
use rodio::Decoder;
use rodio::Sink;
use rodio::buffer::SamplesBuffer;

use crate::*;

// context
ctx!(AUDIO: AudioCtx);

struct AudioCtx {
	device: rodio::Device,
}

pub fn init() {

	if !app::enabled() {
		app::error("can't init audio without app");
	}

	if let Some(device) = rodio::default_output_device() {
		ctx_init(AudioCtx {
			device: device,
		});
	} else {
		app::error("cannot find audio device")
	}

}

pub fn enabled() -> bool {
	return ctx_is_ok();
}

pub fn sound(sound: &Sound) {

	let audio = ctx_get();

	rodio::play_raw(&audio.device, sound.to_buffer());

}

pub fn play(music: &Music) {

	let data = music.data.clone();

	if let Ok(source) = rodio::Decoder::new(data) {
		music.sink.append(source);
	} else {
		app::error("failed to decode audio");
	}

}

pub fn pause(music: &Music) {
	music.sink.pause();
}

pub struct Music {

	sink: Sink,
	data: Cursor<Vec<u8>>,

}

pub struct Sound {

	channels: u16,
	samples_rate: u32,
	samples: Vec<f32>,

}

impl Music {

	pub fn from_bytes(data: &[u8]) -> Self {

		return Self {
			data: Cursor::new(data.to_owned()),
			sink: Sink::new(&ctx_get().device),
		};

	}

	pub fn from_file(fname: &str) -> Self {
		return Self::from_bytes(&fs::read_bytes(fname));
	}

}

impl Sound {

	pub fn from_bytes(data: &[u8]) -> Self {

		let cursor = Cursor::new(data.to_owned());
		let source = Decoder::new(cursor).unwrap();

		return Self {

			channels: source.channels(),
			samples_rate: source.sample_rate(),
			samples: source.convert_samples().collect::<Vec<f32>>(),

		};

	}

	pub fn from_file(fname: &str) -> Self {
		return Self::from_bytes(&fs::read_bytes(fname));
	}

	fn to_buffer(&self) -> SamplesBuffer<f32> {
		return SamplesBuffer::new(self.channels, self.samples_rate, self.samples.clone());
	}

}

