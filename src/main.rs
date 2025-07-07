#![deny(clippy::complexity, clippy::suspicious, clippy::correctness, clippy::perf, clippy::nursery)] 
#![allow(clippy::style, clippy::restriction, clippy::match_bool, clippy::too_many_lines, clippy::single_match_else, clippy::ignored_unit_patterns, clippy::module_name_repetitions, clippy::needless_for_each, clippy::derive_partial_eq_without_eq, clippy::missing_const_for_fn, clippy::cognitive_complexity, clippy::option_if_let_else, clippy::option_map_unit_fn, clippy::type_complexity)]

use std::path::PathBuf;

use clap::Parser;

mod host;
mod generate;

#[derive(Parser)]
#[command(version, about)]
struct Args {
	#[clap(subcommand)]
	command: Command,
}

#[derive(clap::Subcommand)]
enum Command {
	Host {
		dir:  PathBuf,
		#[arg(short, long, default_value = "127.0.0.1:8080")]
		addr: String,
		#[arg(short, long)]
		theme: String,
	},
	Generate {
		source: PathBuf,
		output: PathBuf,
		syntax: PathBuf,
	},
}

fn main() {
	match Args::parse().command {
		Command::Host { dir, addr, theme } => host::host(dir, &addr, theme),
		Command::Generate { source, output, syntax } => generate::generate(&source, &output, syntax),
	}
}
