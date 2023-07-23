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
		Commands::SwitchToWorkspace(workspace_args) => {
			let command = format!("workspace switch {}", workspace_args.id);
			unix_stream
				.write(command.as_bytes())
				.context("Failed at writing onto the unix stream")?;
		}
		Commands::MoveWindow(args) => {
			let command = format!("window move {}", args.id);
			unix_stream
				.write(command.as_bytes())
				.context("Failed at writing onto the unix stream")?;
		}
		Commands::MoveWindowAndFollow(args) => {
			let command = format!("window move_and_follow {}", args.id);
			unix_stream
				.write(command.as_bytes())
				.context("Failed at writing onto the unix stream")?;
		}
		Commands::CloseWindow(_) => {
			unix_stream
				.write("window close".as_bytes())
				.context("Failed at writing onto the unix stream")?;
		}
		Commands::Quit(_) => {
			unix_stream
				.write("quit".as_bytes())
				.context("Failed at writing onto the unix stream")?;
		}
	}

	Ok(())
}
