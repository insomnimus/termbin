use std::process::exit;

use termbin::cmd::Cmd;

fn main() {
	if let Err(e) = Cmd::from_args().run() {
		eprintln!("error: {}", e);
		exit(2);
	}
}
