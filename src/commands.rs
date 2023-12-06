use log::{debug, error};
use anyhow::{Context, anyhow};
use std::io::{stdin, stdout, Write};
use std::path::PathBuf;
use std::fs;

use crate::premake5::{self, PremakeConfig};

pub fn new(name: Option<String>) -> anyhow::Result<()> {
	let name = if let Some(name) = name {
		name
	} else {
		prompt_for_project_name()
	};

	let project_dir = PathBuf::from(name.clone());
	let mut project_craft_dir = project_dir.clone();
	project_craft_dir.push(".craft");

	if project_dir.exists() {
		return Err(anyhow!("Directory `{}` already exists", project_dir.display()));
	}

	fs::create_dir(project_dir).context("Failed to create project directory")?;
	fs::create_dir(project_craft_dir).context("Failed to create `.craft` directory in project folder")?;

	let premake5config = PremakeConfig::new()
		.set_project_name(name)
		.set_language("C++".to_string())
		.add_include_dir(PathBuf::from("include"))
		.add_include_dir(PathBuf::from("include2"));

	println!("{}", premake5config);

	return Ok(());
}

fn prompt_for_project_name() -> String {
	let mut input = String::new();
	print!("Enter the name of the project: ");
	stdout().flush().unwrap();
	stdin().read_line(&mut input).expect("Error reading from stdin");

	return input.trim().to_string();
}
