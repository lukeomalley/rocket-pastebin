use rand::{self, Rng};
use rocket::request::FromParam;
use std::borrow::Cow;
use std::path::{Path, PathBuf};

#[derive(UriDisplayPath, Debug)]
pub struct PasteId<'a>(Cow<'a, str>);

impl PasteId<'_> {
	pub fn new(size: usize) -> PasteId<'static> {
		const BASE36: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

		let mut id = String::with_capacity(size);
		let mut rng = rand::thread_rng();

		for _ in 0..size {
			id.push(BASE36[rng.gen::<usize>() % 36] as char);
		}

		PasteId(Cow::Owned(id))
	}

	pub fn file_path(&self) -> PathBuf {
		let root = concat!(env!("CARGO_MANIFEST_DIR"), "/", "upload", "/");
		Path::new(root).join(self.0.as_ref())
	}
}

impl<'a> FromParam<'a> for PasteId<'a> {
	type Error = &'a str;
	fn from_param(param: &'a str) -> Result<Self, Self::Error> {
		param
			.chars()
			.all(|c| c.is_ascii_alphanumeric())
			.then(|| PasteId(param.into()))
			.ok_or(param)
	}
}
