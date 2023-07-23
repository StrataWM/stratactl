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
	SwitchToWorkspace(WorkspaceArgs),
	MoveWindow(WorkspaceArgs),
	MoveWindowAndFollow(WorkspaceArgs),
	CloseWindow(NoArgs),
	Quit(NoArgs),
}

#[derive(Args)]
pub struct WorkspaceArgs {
	pub id: u8,
}

#[derive(Args)]
pub struct NoArgs {}
