use clap::{
	Args,
	Parser,
	Subcommand,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
	#[command(subcommand)]
	pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
	Launch(LaunchArgs),
	Quit(QuitArgs),
}

#[derive(Args)]
pub struct LaunchArgs {
	pub backend: String,
}

#[derive(Args)]
pub struct QuitArgs {}
