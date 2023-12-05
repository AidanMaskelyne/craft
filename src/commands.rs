use log::error;
use anyhow::{Context, anyhow};
use std::io::{stdin, stdout, Write};
use std::path::PathBuf;
use std::fs;

pub fn new(name: Option<String>) -> anyhow::Result<()> {
	let name = if let Some(name) = name {
		name
	} else {
		prompt_for_project_name()
	};

	let current_dir = std::env::current_dir().context("Failed getting current directory")?;
	let current_dir = current_dir.canonicalize().context("Failed canonicalizing current directory")?;

	let mut project_dir = current_dir.clone();
	project_dir.push(name.clone());

	if project_dir.exists() {
		return Err(anyhow!("Directory `{}` already exists", project_dir.display()));
	}

	println!("Creating project {} in {}", name, project_dir.display());

	return Ok(());
}

fn prompt_for_project_name() -> String {
	let mut input = String::new();
	print!("Enter the name of the project: ");
	stdout().flush().unwrap();
	stdin().read_line(&mut input).expect("Error reading from stdin");

	return input.trim().to_string();
}
