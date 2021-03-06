// wengwengweng

//! Data Saving & Loading

use serde::ser::Serialize;
use serde::de::Deserialize;
use std::path::PathBuf;
use crate::*;

pub fn to_json<D: Serialize>(data: &D) -> Result<String> {
	return serde_json::to_string(data)
		.map_err(|_| format!("failed to encode json"));
}

pub fn from_json<D: for<'a> Deserialize<'a>>(json: &str) -> Result<D> {
	return serde_json::from_str(&json)
		.map_err(|_| format!("failed to decode json"));
}

pub fn to_bin<D: Serialize>(data: &D) -> Result<Vec<u8>> {
	return bincode::serialize(data)
		.map_err(|_| format!("failed to encode bin"));
}

pub fn from_bin<D: for<'a> Deserialize<'a>>(bin: &[u8]) -> Result<D> {
	return bincode::deserialize(&bin)
		.map_err(|_| format!("failed to decode bin"));
}

pub fn path(proj: &'static str) -> Result<PathBuf> {
	return Ok(dirs_next::data_dir()
		.ok_or_else(|| format!("failed to get data dir"))?
		.join(proj));
}

#[cfg(not(web))]
pub fn save<D: Serialize>(
	proj: &'static str,
	entry: &'static str,
	data: &D
) -> Result<()> {

	let data_dir = path(proj)?;

	if !data_dir.exists() {
		std::fs::create_dir_all(&data_dir)
			.map_err(|_| format!("failed to create dir {}", data_dir.display()))?;
	}

	let data_file = data_dir.join(&format!("{}.json", entry));
	let content = to_json(data)?;

	std::fs::write(&data_file, content)
		.map_err(|_| format!("failed to write file {}", data_file.display()))?;

	return Ok(());

}

#[cfg(not(web))]
pub fn load<D: for<'a> Deserialize<'a>>(
	proj: &'static str,
	entry: &'static str,
) -> Result<D> {

	let data_dir = path(proj)?;
	let data_file = data_dir.join(&format!("{}.json", entry));
	let content = fs::read_str(data_file)?;

	return from_json(&content);

}

#[cfg(web)]
pub fn save<D: Serialize>(
	_: &'static str,
	entry: &'static str,
	data: &D
) -> Result<()> {

	let window = web_sys::window()
		.ok_or_else(|| format!("failed to get window"))?;

	let storage = window
		.local_storage()
		.map_err(|_| format!("failed to get local storage"))?
		.ok_or_else(|| format!("failed to get local storage"))?;

	let content = to_json(data)?;

	storage
		.set_item(entry, &content)
		.map_err(|_| format!("failed to set entry {}", entry))?;

	return Ok(());

}

#[cfg(web)]
pub fn load<D: for<'a> Deserialize<'a>>(
	_: &'static str,
	entry: &'static str,
) -> Result<D> {

	let window = web_sys::window()
		.ok_or_else(|| format!("failed to get window"))?;

	let storage = window
		.local_storage()
		.map_err(|_| format!("failed to get local storage"))?
		.ok_or_else(|| format!("failed to get local storage"))?;

	let content = storage
		.get_item(entry)
		.map_err(|_| format!("failed to get entry {}", entry))?
		.ok_or_else(|| format!("failed to get entry {}", entry))?;

	return from_json(&content);

}

