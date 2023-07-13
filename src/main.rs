mod libs;

use anyhow::Context;
use clap::Parser;
use libs::structs::{
	Cli,
	Commands,
};
use std::{
	io::Write,
	os::unix::net::UnixStream,
};

fn main() -> anyhow::Result<()> {
	let cli = Cli::parse();
	let socket_path = "/tmp/strata_socket";

	let mut unix_stream = UnixStream::connect(socket_path).context("Could not create stream")?;

	match &cli.command {
		Commands::Launch(backend) => {
			match backend.backend.as_str() {
				"winit" => {
					unix_stream
						.write(b"launch winit")
						.context("Failed at writing onto the unix stream")?;
				}
				"udev" => {
					unix_stream
						.write(b"launch udev")
						.context("Failed at writing onto the unix stream")?;
				}
				&_ => {
					println!(
						"No backend provided or unknown backend. Please choose one of these: \
						 \"winit\" / \"udev\""
					);
				}
			}
		}
		Commands::Quit(_) => {
			println!("Quitting");
			std::process::Command::new("sh")
				.arg("-c")
				.arg("killall stratawm")
				.output()
				.expect("failed to execute process");
		}
	}

	Ok(())
}
