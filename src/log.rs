use std::io::Write;
use env_logger::{fmt::Color, Builder, WriteStyle};
use log::{LevelFilter, Level};

// Fatal errors should not be logged using `log::error!()`, but should be instead propogated
// out of the function with `return Err(anyhow!());`

pub fn init() {
	let mut builder = Builder::from_default_env();
	
	builder
		.format(|buf, record| {
			let mut style = buf.style();

			match record.level() {
				Level::Error => {
					style.set_color(Color::Red).set_bold(true);
				},
				Level::Warn => {
					style.set_color(Color::Yellow).set_bold(true);
				}
				_ => (),
			};

			writeln!(
				buf,
				"{}: {}",
				style.value(record.level()),
				record.args(),
			)
		})
		.filter(None, LevelFilter::Warn)
		.init();
}

pub fn init_debug() {
	let mut builder = Builder::from_default_env();

	builder
		.filter(None, LevelFilter::Debug)
		.write_style(WriteStyle::Always)
		.init();
}
