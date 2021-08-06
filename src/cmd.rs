use std::{
	error::Error,
	fs::File,
	io::{
		self,
		prelude::*,
	},
	net::TcpStream,
};

use clipboard::{
	ClipboardContext,
	ClipboardProvider,
};

use crate::app;

pub struct Cmd {
	file: Option<String>,
	clip: bool,
	remote: String,
}

impl Cmd {
	pub fn from_args() -> Self {
		let m = app::new().get_matches();

		let file = m.value_of("file").map(String::from);
		let remote = m.value_of("remote").unwrap().to_string();
		let clip = m.is_present("clip");
		Self { file, clip, remote }
	}

	pub fn run(&self) -> Result<(), Box<dyn Error>> {
		let url = match &self.file {
			None => self.send_stdin(),
			Some(f) => self.send_file(f),
		}?;

		if self.clip {
			let mut ctx: ClipboardContext = ClipboardProvider::new()?;
			ctx.set_contents(url).map(|_| {
				println!("copied the link to clipboard");
			})
		} else {
			println!("{}", &url);
			Ok(())
		}
	}
}

impl Cmd {
	fn send_file(&self, p: &str) -> Result<String, io::Error> {
		let mut file = File::open(p)?;
		let mut stream = TcpStream::connect(&self.remote)?;
		io::copy(&mut file, &mut stream)?;

		let mut buf = String::new();
		stream.read_to_string(&mut buf)?;

		Ok(buf)
	}

	fn send_stdin(&self) -> Result<String, io::Error> {
		let mut stdin = io::stdin();
		let mut stream = TcpStream::connect(&self.remote)?;
		io::copy(&mut stdin, &mut stream)?;

		let mut buf = String::new();
		stream.read_to_string(&mut buf)?;

		Ok(buf)
	}
}
