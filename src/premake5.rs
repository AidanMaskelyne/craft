// Module for generating premake5.lua file contents

use std::fs::{File, OpenOptions};
use std::path::PathBuf;
use std::fmt;
use anyhow::Context;

use log::debug;

pub struct PremakeConfig {
	project_name: String,
	language: String,
	include_dirs: Vec<PathBuf>
}

impl PremakeConfig {
	pub fn new() -> PremakeConfig {
		return PremakeConfig::default();
	}

	pub fn set_project_name(mut self, name: String) -> PremakeConfig {
		self.project_name = name;
		return self;
	}

	pub fn set_language(mut self, language: String) -> PremakeConfig {
		self.language = language;
		return self;
	}

	pub fn add_include_dir(mut self, dir: PathBuf) -> PremakeConfig {
		self.include_dirs.push(dir);
		return self;
	}

	pub fn write_to_file(&self, output_file: &PathBuf) -> anyhow::Result<()>{
		let current_premake_contents = File::open(output_file.clone());
		let premake_file = OpenOptions::new();

		Ok(())
	}
}

impl Default for PremakeConfig {
	fn default() -> Self {
		return Self {
			project_name: String::from("craft-project"),
			language: String::from("C++"),
			include_dirs: Vec::new(),
		}
	}
}

impl fmt::Display for PremakeConfig {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut include_dirs_display = String::new();

		for i in 0..self.include_dirs.len(){
			include_dirs_display.push_str(self.include_dirs[i].to_str().unwrap());
			if i != self.include_dirs.len() - 1 {
				include_dirs_display.push_str(", ");
			}
		}

		write!(
			f,
"Project name            {}
Language                {}
Include directories:    [ {} ]",
			self.project_name,
			self.language,
			include_dirs_display,
		)
	}
}
