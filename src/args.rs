use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Craft")]
#[command(author = "Aidan M. <aidan@muttleyville.org>")]
#[command(version)]
#[command(about = "A C++ package manager & build tool", long_about = None)]
pub struct Args {
	#[command(subcommand)]
	pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
	/// Creates a new C++ project
	New {
		/// Name of the new project
		name: Option<String>
	},

	/// Initialises a C++ project in the current directory
	Init {
		/// Name of the new project
		name: Option<String>
	}
}

