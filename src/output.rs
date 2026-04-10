/// Output formatters: table (human-readable), JSON, and CSV.
use crate::scanner::LanguageScore;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OutputFormat {
	Table,
	Json,
	Csv,
}

impl OutputFormat {
	pub fn from_str(s: &str) -> Option<Self> {
		match s.to_lowercase().as_str() {
			"table" => Some(Self::Table),
			"json" => Some(Self::Json),
			"csv" => Some(Self::Csv),
			_ => None,
		}
	}
}

pub fn render(format: OutputFormat, file_path: &str, scores: &[LanguageScore]) {
	match format {
		OutputFormat::Table => render_table(file_path, scores),
		OutputFormat::Json => render_json(file_path, scores),
		OutputFormat::Csv => render_csv(scores),
	}
}

fn bcp47_display(bcp47: &Option<String>) -> &str {
	match bcp47 {
		Some(s) => s.as_str(),
		None => "-",
	}
}

fn render_table(file_path: &str, scores: &[LanguageScore]) {
	let total_words = scores.first().map_or(0, |s| s.total_words);

	println!(
		"╔══════════════════════════════════════════════════════════════════════════════════════════════════════╗"
	);
	println!(
		"║                                 Language Detection Results                                           ║"
	);
	println!(
		"╠══════════════════════════════════════════════════════════════════════════════════════════════════════╣"
	);
	println!("║  File: {:<84}          ║", truncate(file_path, 84));
	println!("║  Total words parsed: {:<70}          ║", total_words);
	println!(
		"╠══════════════════════════════════════════════════════════════════════════════════════════════════════╣"
	);

	if let Some(w) = scores.first() {
		println!(
			"║                                                                                                      ║"
		);
		println!("║  ✓ DETECTED LANGUAGE: {:<70}         ║", w.english_name);
		println!("║    ISO 639-1: {:<78}         ║", w.iso_639_1);
		println!("║    ISO 639-2: {:<78}         ║", w.iso_639_2);
		println!("║    BCP 47:    {:<78}         ║", bcp47_display(&w.bcp47));
		println!(
			"║    Confidence: {:<77}         ║",
			format!(
				"{:.2}%  ({} / {} words matched)",
				w.confidence, w.matched_words, w.total_words
			)
		);
		println!(
			"║                                                                                                      ║"
		);
	}

	println!(
		"╠══════════════════════════════════════════════════════════════════════════════════════════════════════╣"
	);
	println!(
		"║  All language scores (top 10):                                                                       ║"
	);
	println!(
		"╠══════════════════════════════════════════════════════════════════════════════════════════════════════╣"
	);
	println!(
		"║  {:<3} {:<28} {:<7} {:<7} {:<14} {:<12} {:<12}           ║",
		"#", "Language", "639-1", "639-2", "BCP 47", "Matches", "Score"
	);
	println!(
		"║  {:-<3} {:-<28} {:-<7} {:-<7} {:-<14} {:-<12} {:-<12}           ║",
		"", "", "", "", "", "", ""
	);

	for (i, s) in scores.iter().take(10).enumerate() {
		println!(
			"║  {:<3} {:<28} {:<7} {:<7} {:<14} {:<12} {:<12}           ║",
			i + 1,
			s.english_name,
			s.iso_639_1,
			s.iso_639_2,
			bcp47_display(&s.bcp47),
			s.matched_words,
			format!("{:.2}%", s.confidence),
		);
	}

	println!(
		"╚══════════════════════════════════════════════════════════════════════════════════════════════════════╝"
	);
}

fn truncate(s: &str, max: usize) -> String {
	if s.len() <= max {
		s.to_string()
	} else {
		format!("…{}", &s[s.len() - max + 1..])
	}
}

fn render_json(file_path: &str, scores: &[LanguageScore]) {
	let detected = scores.first();

	println!("{{");
	println!("  \"file\": \"{}\",", json_escape(file_path));
	println!(
		"  \"total_words\": {},",
		detected.map_or(0, |s| s.total_words)
	);

	if let Some(w) = detected {
		println!("  \"detected\": {{");
		println!("    \"language\": \"{}\",", json_escape(&w.english_name));
		println!("    \"iso_639_1\": \"{}\",", json_escape(&w.iso_639_1));
		println!("    \"iso_639_2\": \"{}\",", json_escape(&w.iso_639_2));
		println!(
			"    \"bcp47\": {},",
			match &w.bcp47 {
				Some(s) => format!("\"{}\"", json_escape(s)),
				None => "null".to_string(),
			}
		);
		println!("    \"matched_words\": {},", w.matched_words);
		println!("    \"confidence\": {:.4}", w.confidence / 100.0);
		println!("  }},");
	} else {
		println!("  \"detected\": null,");
	}

	println!("  \"scores\": [");
	for (i, s) in scores.iter().enumerate() {
		let comma = if i + 1 < scores.len() { "," } else { "" };
		println!("    {{");
		println!("      \"rank\": {},", i + 1);
		println!("      \"language\": \"{}\",", json_escape(&s.english_name));
		println!("      \"iso_639_1\": \"{}\",", json_escape(&s.iso_639_1));
		println!("      \"iso_639_2\": \"{}\",", json_escape(&s.iso_639_2));
		println!(
			"      \"bcp47\": {},",
			match &s.bcp47 {
				Some(tag) => format!("\"{}\"", json_escape(tag)),
				None => "null".to_string(),
			}
		);
		println!("      \"matched_words\": {},", s.matched_words);
		println!("      \"total_words\": {},", s.total_words);
		println!("      \"confidence\": {:.4}", s.confidence / 100.0);
		println!("    }}{}", comma);
	}
	println!("  ]");
	println!("}}");
}

fn json_escape(s: &str) -> String {
	let mut out = String::with_capacity(s.len());
	for c in s.chars() {
		match c {
			'"' => out.push_str("\\\""),
			'\\' => out.push_str("\\\\"),
			'\n' => out.push_str("\\n"),
			'\r' => out.push_str("\\r"),
			'\t' => out.push_str("\\t"),
			c if (c as u32) < 0x20 => {
				out.push_str(&format!("\\u{:04x}", c as u32));
			}
			_ => out.push(c),
		}
	}
	out
}

fn render_csv(scores: &[LanguageScore]) {
	println!("rank,language,iso_639_1,iso_639_2,bcp47,matched_words,total_words,confidence");
	for (i, s) in scores.iter().enumerate() {
		println!(
			"{},{},{},{},{},{},{},{:.4}",
			i + 1,
			csv_escape(&s.english_name),
			csv_escape(&s.iso_639_1),
			csv_escape(&s.iso_639_2),
			csv_escape(bcp47_display(&s.bcp47)),
			s.matched_words,
			s.total_words,
			s.confidence / 100.0,
		);
	}
}

fn csv_escape(s: &str) -> String {
	if s.contains(',') || s.contains('"') || s.contains('\n') {
		format!("\"{}\"", s.replace('"', "\"\""))
	} else {
		s.to_string()
	}
}
