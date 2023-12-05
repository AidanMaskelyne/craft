// Arg stuff
use {
	craft::args::{Args, Commands},
	clap::Parser,
};
use log::error;

fn main() {
	if let Err(err) = try_main() {
		error!("{}", err);
		err.chain().skip(1).for_each(|cause| eprintln!("because: {}", cause));
		std::process::exit(1);
	}
}

fn try_main() -> anyhow::Result<()> {
	let args = Args::parse();
	craft::log::init(args.debug);

	match args.command {
		Commands::New { name } => {
			craft::commands::new(name)?;
		},
		Commands::Init { name } => {
			println!("{}", name.unwrap());
		},
	}

	return Ok(())
}
