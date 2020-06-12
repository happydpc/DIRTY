// wengwengweng

use std::sync::Mutex;
use std::sync::Arc;
use std::time::Duration;
use std::collections::VecDeque;

use super::*;

/// Chainable Audio Effect
pub trait Effect {
	fn process(&mut self, _: Frame) -> Frame;
	fn leftover(&mut self) -> Option<Frame> {
		return None;
	}
}

#[derive(Clone, Debug)]
pub struct Volume {
	left: f32,
	right: f32,
}

impl Volume {
	pub fn new(v: f32) -> Self {
		return Self::panned(v, v);
	}
	pub fn panned(l: f32, r: f32) -> Self {
		return Self {
			left: l.max(0.0),
			right: r.max(0.0),
		};
	}
}

impl Default for Volume {
	fn default() -> Self {
		return Self::new(1.0);
	}
}

impl Effect for Volume {
	fn process(&mut self, f: Frame) -> Frame {
		return Frame::new(
			f.left * self.left,
			f.right * self.right,
		);
	}
}

#[derive(Clone, Debug)]
pub struct Distortion {
	crunch: f32,
}

impl Distortion {
	pub fn new(c: f32) -> Self {
		return Self {
			crunch: c.max(0.0).min(1.0),
		};
	}
}

impl Default for Distortion {
	fn default() -> Self {
		return Self::new(0.0);
	}
}

impl Effect for Distortion {

	fn process(&mut self, f: Frame) -> Frame {

        let c = 1.0 - self.crunch;
		let l_sign = f.left.signum();
		let r_sign = f.right.signum();
		let l = (f.left * l_sign).powf(c).min(1.0) * l_sign;
		let r = (f.right * r_sign).powf(c).min(1.0) * r_sign;

		return Frame::new(l, r);

	}

}

#[derive(Clone, Debug)]
pub struct Delay {
	buffer: VecDeque<Frame>,
	len: usize,
	cycles: usize,
	decay: f32,
}

impl Delay {

	pub fn new(duration: Duration, cycles: usize, decay: f32) -> Self {

		let len = (duration.as_secs_f32() * SPEC.sample_rate as f32) as usize;
		let mut buffer = VecDeque::with_capacity(len * cycles);

		for _ in 0..len * cycles {
			buffer.push_back(Frame::new(0.0, 0.0));
		}

		return Self {
			buffer: buffer,
			len: len,
			cycles: cycles,
			decay: decay,
		};

	}

}

impl Default for Delay {
	fn default() -> Self {
		return Self::new(Duration::from_secs_f32(0.0), 0, 0.0);
	}
}

impl Effect for Delay {

	fn process(&mut self, f: Frame) -> Frame {

		if self.len == 0 || self.cycles == 0 || self.decay == 0.0 {
			return f;
		}

		let of = (0..self.cycles).fold(f, |frame_acc, i| {
			return frame_acc + self.buffer
				.get(i * self.len)
				.map(|sample| *sample * self.decay.powf((self.cycles - i) as f32))
				.unwrap_or_default()
				;
		});

		self.buffer.push_back(f);
		self.buffer.pop_front();

		return of;

	}

	fn leftover(&mut self) -> Option<Frame> {

		if self.buffer.is_empty() {
			return None;
		}

		let of = (0..self.cycles).fold(Frame::default(), |frame_acc, i| {
			return frame_acc + self.buffer
				.get(i * self.len)
				.map(|sample| *sample * self.decay.powf((self.cycles - i) as f32))
				.unwrap_or_default()
				;
		});

		self.buffer.pop_front();

		return Some(of);

	}

}

// TODO
#[derive(Clone, Debug)]
pub struct Reverb {
}

impl Reverb {
	pub fn new(d: f32) -> Self {
		return Self {
		};
	}
}

impl Default for Reverb {
	fn default() -> Self {
		return Self::new(0.0);
	}
}

impl Effect for Reverb {
	fn process(&mut self, f: Frame) -> Frame {
		return f;
	}
}

#[derive(Clone)]
pub(super) struct BasicEffectChain {
	volume: Arc<Mutex<Volume>>,
	delay: Arc<Mutex<Delay>>,
	distortion: Arc<Mutex<Distortion>>,
	reverb: Arc<Mutex<Reverb>>,
}

impl BasicEffectChain {

	pub fn new() -> Self {
		return Self {
			volume: Arc::new(Mutex::new(Volume::default())),
			delay: Arc::new(Mutex::new(Delay::default())),
			distortion: Arc::new(Mutex::new(Distortion::default())),
			reverb: Arc::new(Mutex::new(Reverb::default())),
		};
	}

	pub fn chain(&self) -> Vec<Arc<Mutex<dyn Effect + Send>>> {
		return vec![
			self.distortion.clone(),
			self.delay.clone(),
			self.reverb.clone(),
			self.volume.clone(),
		];
	}

	pub fn set_volume(&self, v: Volume) {
		if let Ok(mut volume) = self.volume.lock() {
			*volume = v;
		}
	}

	pub fn set_distortion(&self, d: Distortion) {
		if let Ok(mut distortion) = self.distortion.lock() {
			*distortion = d;
		}
	}

	pub fn set_reverb(&self, r: Reverb) {
		if let Ok(mut reverb) = self.reverb.lock() {
			*reverb = r;
		}
	}

	pub fn set_delay(&self, d: Delay) {
		if let Ok(mut delay) = self.delay.lock() {
			*delay = d;
		}
	}

}

