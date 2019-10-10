// wengwengweng

use crate::Error;
use crate::Result;

use super::*;

#[derive(Clone, Debug)]
pub struct Response {
	body: Body,
	status: Status,
	headers: HeaderMap,
}

impl Response {

	pub fn from_raw(buf: &[u8]) -> Result<Self> {

		let mut headers = [httparse::EMPTY_HEADER; 128];
		let mut res = httparse::Response::new(&mut headers);

		let body_pos = match res.parse(&buf)? {
			httparse::Status::Complete(len) => len,
			httparse::Status::Partial => return Err(Error::Net("incomplete request message".into())),
		};

		let code = res.code
			.ok_or(Error::Net("failed to parse response status code".into()))?;

		let status = Status::from_code(code)?;

		let body = &buf[body_pos..];

		return Ok(Self {
			body: Body::from_raw(body),
			status: status,
			headers: HeaderMap::new(),
		});

	}

	pub fn set_body(&mut self, body: Body) {

		self.body = body;
		self.set_header(Header::ContentLength, &self.body.len().to_string());

	}

	pub fn redirect(url: &str) -> Self {

		let mut headers = HeaderMap::new();

		headers.set(Header::Location, url);

		return Self {
			body: Body::empty(),
			status: Status::SeeOther,
			headers: headers,
		}

	}

	pub fn new(status: Status, t: ContentType, body: impl AsRef<[u8]>) -> Self {

		let body = body.as_ref();
		let mut headers = HeaderMap::new();

		headers.set(Header::ContentLength, &body.len().to_string());
		headers.set(Header::ContentType, t.as_str());

		return Self {
			body: Body::from_raw(body),
			status: status,
			headers: headers,
		};

	}

	pub fn body(&self) -> &Body {
		return &self.body;
	}

	pub fn status(&self) -> Status {
		return self.status;
	}

	pub fn set_header(&mut self, key: Header, value: &str) {
		self.headers.set(key, value);
	}

	pub fn headers(&self) -> &HeaderMap {
		return &self.headers;
	}

	pub fn message(&self) -> Vec<u8> {

		let mut m = Vec::new();
		let status = self.status();

		m.extend_from_slice(&format!("HTTP/1.1 {} {}", status.code(), status.reason()).as_bytes());
		m.extend_from_slice("\r\n".as_bytes());
		m.extend_from_slice(&self.headers.to_string().as_bytes());
		m.extend_from_slice("\r\n".as_bytes());
		m.extend_from_slice(&self.body.as_bytes());

		return m;

	}

}

