use std::io::Write;
use env_logger::{fmt::Color, Builder, WriteStyle};
use log::{LevelFilter, Level};

// Fatal errors should not be logged using `log::error!()`, but should be instead propogated
// out of the function with `return Err(anyhow!());`

pub fn init(verbose: bool) {
	let mut builder = Builder::from_default_env();

	if !verbose {
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
			.filter(None, LevelFilter::Warn);
	} else {
		builder.filter(None, LevelFilter::Debug);
	}

	builder.write_style(WriteStyle::Always).init();
}
