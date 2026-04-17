mod cleaner;
mod languages;
mod output;
mod scanner;

use output::OutputFormat;
use std::env;
use std::fs;

const VERSION: &str = env!("CARGO_PKG_VERSION");

struct CliArgs {
	file_path: String,
	format: OutputFormat,
	debug_lang: Option<String>,
	dump_text: bool,
}

fn print_usage(program: &str) {
	eprintln!("Language Detector");
	eprintln!();
	eprintln!("USAGE:");
	eprintln!("    {} [OPTIONS] <FILE>", program);
	eprintln!();
	eprintln!("ARGS:");
	eprintln!("    <FILE>    Path to a file (.srt, .ass, .ssa, .sub, .txt, ...)");
	eprintln!();
	eprintln!("OPTIONS:");
	eprintln!("    -f, --format <FORMAT>    Output format: table (default), json, csv");
	eprintln!("    -d, --debug <LANG>       Debug mode: show detailed match info for a language");
	eprintln!("                             Accepts language name, ISO 639-1/2 code, or BCP 47 tag");
	eprintln!("                             Examples: spa, es-419, \"Spanish\", por, pt-BR");
	eprintln!("        --dump-text          Print the cleaned text used for word matching and exit");
	eprintln!("                             Useful for verifying SRT/SSA/ASS dialogue extraction");
	eprintln!("    -V, --version            Print version information");
	eprintln!("    -h, --help               Show this help message");
	eprintln!();
	eprintln!("EXAMPLES:");
	eprintln!("    {} movie.srt", program);
	eprintln!("    {} -f json movie.srt", program);
	eprintln!("    {} --format csv movie.srt", program);
	eprintln!("    {} -f json movie.srt > result.json", program);
	eprintln!("    {} --dump-text movie.ass > extracted.txt", program);
}

fn parse_args() -> CliArgs {
	let args: Vec<String> = env::args().collect();
	let program = args[0].clone();

	let mut format = OutputFormat::Table;
	let mut file_path: Option<String> = None;
	let mut debug_lang: Option<String> = None;
	let mut dump_text = false;

	let mut i = 1;
	while i < args.len() {
		match args[i].as_str() {
			"-h" | "--help" => {
				print_usage(&program);
				std::process::exit(0);
			}
			"-f" | "--format" => {
				i += 1;
				if i >= args.len() {
					eprintln!("Error: --format requires a value (table, json, csv)");
					std::process::exit(1);
				}
				match OutputFormat::from_str(&args[i]) {
					Some(f) => format = f,
					None => {
						eprintln!(
							"Error: Unknown format '{}'. Choose from: table, json, csv",
							args[i]
						);
						std::process::exit(1);
					}
				}
			}
			"-d" | "--debug" => {
				i += 1;
				if i >= args.len() {
					eprintln!("Error: --debug requires a language name, ISO code, or BCP 47 tag");
					std::process::exit(1);
				}
				debug_lang = Some(args[i].clone());
			}
			"--dump-text" => {
				dump_text = true;
			}
			"-V" | "--version" => {
				println!("Language Detector v{}", VERSION);
				std::process::exit(0);
			}
			other => {
				if other.starts_with('-') {
					eprintln!("Error: Unknown option '{}'", other);
					eprintln!("Run '{} --help' for usage information.", program);
					std::process::exit(1);
				}
				if file_path.is_some() {
					eprintln!("Error: Multiple file paths provided. Only one file at a time is supported.");
					std::process::exit(1);
				}
				file_path = Some(other.to_string());
			}
		}
		i += 1;
	}

	let file_path = match file_path {
		Some(p) => p,
		None => {
			eprintln!("Error: No file provided.\n");
			print_usage(&program);
			std::process::exit(1);
		}
	};

	CliArgs {
		file_path,
		format,
		debug_lang,
		dump_text,
	}
}

fn main() {
	let cli = parse_args();

	// Read file
	let raw_text = match fs::read_to_string(&cli.file_path) {
		Ok(text) => text,
		Err(e) => {
			eprintln!("Error reading file '{}': {}", cli.file_path, e);
			std::process::exit(1);
		}
	};

	// Clean subtitle markup
	let cleaned = cleaner::clean_subtitle_text(&raw_text);

	// --dump-text: print the cleaned text that will be fed to the tokenizer
	if cli.dump_text {
		print!("{}", cleaned);
		// Ensure trailing newline so terminal prompts look right
		if !cleaned.ends_with('\n') {
			println!();
		}
		return;
	}

	let words = cleaner::tokenize(&cleaned);

	if words.is_empty() {
		eprintln!("No words found in the file.");
		std::process::exit(1);
	}

	let catalogue = languages::build_catalogue();

	// Debug mode: show detailed match info and exit
	if let Some(ref filter) = cli.debug_lang {
		let reports = scanner::debug_languages(&catalogue, &words, filter);
		output::render_debug(&reports);
		return;
	}

	// Normal mode: scan all languages and render
	let scores = scanner::scan(catalogue, words);
	output::render(cli.format, &cli.file_path, &scores);
}
