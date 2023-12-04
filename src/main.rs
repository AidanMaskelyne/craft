// Arg stuff
use {
	craft::args::{Args, Commands},
	clap::Parser,
};

fn main() -> anyhow::Result<()> {
	let args = Args::parse();

	match args.command {
		Commands::New { name } => {
			craft::commands::new(name);
		},
		Commands::Init { name } => {
			println!("{}", name.unwrap());
		},
	}

	return Ok(())
}
