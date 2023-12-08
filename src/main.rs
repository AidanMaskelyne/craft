// Arg stuff
use {
	craft::args::{Args, Commands},
	clap::Parser,
};
use log::{error, debug};

fn main() {
	if let Err(err) = try_main() {
		error!("{}", err);
		err.chain().skip(1).for_each(|cause| eprintln!("	because: {}", cause));
		std::process::exit(1);
	}
}

fn try_main() -> anyhow::Result<()> {
	let args = Args::parse();

	if args.debug {
		craft::log::init_debug();
	} else {
		#[cfg(debug_assertions)]
		craft::log::init_debug();

		#[cfg(not(debug_assertions))]
		craft::log::init();
	}

	debug!("Debug logging enabled");

	match args.command {
		Commands::New { name } => {
			craft::commands::new(name)?;
		},
		Commands::Init { name: _ } => {
		},
	}

	return Ok(());
}
