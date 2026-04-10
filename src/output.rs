/// Output formatters: table (human-readable), JSON, and CSV.
use crate::scanner::{self, LanguageScore};
use tabled::{
	Table, Tabled,
	settings::{Alignment, Modify, Style, Width, object::Rows},
};

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

#[derive(Tabled)]
#[tabled(rename_all = "PascalCase")]
struct InfoRow {
	#[tabled(rename = "File")]
	file: String,
	#[tabled(rename = "Total words parsed")]
	total_words: String,
}

#[derive(Tabled)]
struct DetectedRow {
	#[tabled(rename = "Detected language")]
	language: String,
	#[tabled(rename = "ISO 639-1")]
	iso1: String,
	#[tabled(rename = "ISO 639-2")]
	iso2: String,
	#[tabled(rename = "BCP 47")]
	bcp47: String,
	#[tabled(rename = "Confidence")]
	confidence: String,
	#[tabled(rename = "Weighted score")]
	weighted: String,
}

#[derive(Tabled)]
struct ScoreRow {
	#[tabled(rename = "#")]
	rank: usize,
	#[tabled(rename = "Language")]
	language: String,
	#[tabled(rename = "639-1")]
	iso1: String,
	#[tabled(rename = "639-2")]
	iso2: String,
	#[tabled(rename = "BCP 47")]
	bcp47: String,
	#[tabled(rename = "Matches")]
	matches: usize,
	#[tabled(rename = "Confidence")]
	confidence: String,
	#[tabled(rename = "Weighted")]
	weighted: String,
}

#[derive(Tabled)]
struct DebugMatchRow {
	#[tabled(rename = "Word")]
	word: String,
	#[tabled(rename = "Count")]
	count: usize,
	#[tabled(rename = "Weight")]
	weight: String,
	#[tabled(rename = "Subtotal")]
	subtotal: String,
	#[tabled(rename = "Match")]
	match_type: String,
}

#[derive(Tabled)]
struct DebugSummaryRow {
	#[tabled(rename = "Metric")]
	metric: String,
	#[tabled(rename = "Value")]
	value: String,
}

fn render_table(file_path: &str, scores: &[LanguageScore]) {
	let total_words = scores.first().map_or(0, |s| s.total_words);

	// File info table
	let info_rows = vec![InfoRow {
		file: truncate(file_path, 60),
		total_words: total_words.to_string(),
	}];
	let mut info_table = Table::new(info_rows);
	info_table
		.with(Style::modern())
		.with(Modify::new(Rows::first()).with(Alignment::center()))
		.with(Width::wrap(100));

	println!("{}", info_table);

	// Detected language table (if any)
	if let Some(top) = scores.first() {
		let detected_row = DetectedRow {
			language: top.english_name.clone(),
			iso1: top.iso_639_1.clone(),
			iso2: top.iso_639_2.clone(),
			bcp47: bcp47_display(&top.bcp47).to_string(),
			confidence: format!(
				"{:.2}% ({} / {})",
				top.confidence, top.matched_words, top.total_words
			),
			weighted: format!("{:.2}", top.weighted_score),
		};
		let mut detected_table = Table::new(vec![detected_row]);
		detected_table
			.with(Style::modern())
			.with(Modify::new(Rows::first()).with(Alignment::center()))
			.with(Width::wrap(100));

		println!("\n{}", detected_table);
	}

	// Top 10 scores table
	let score_rows: Vec<ScoreRow> = scores
		.iter()
		.take(10)
		.enumerate()
		.map(|(i, s)| ScoreRow {
			rank: i + 1,
			language: s.english_name.clone(),
			iso1: s.iso_639_1.clone(),
			iso2: s.iso_639_2.clone(),
			bcp47: bcp47_display(&s.bcp47).to_string(),
			matches: s.matched_words,
			confidence: format!("{:.2}%", s.confidence),
			weighted: format!("{:.2}", s.weighted_score),
		})
		.collect();

	let mut scores_table = Table::new(score_rows);
	scores_table
		.with(Style::modern())
		.with(Modify::new(Rows::first()).with(Alignment::center()))
		.with(Width::wrap(100));

	println!("\n{}", scores_table);
}

pub fn render_debug(reports: &[scanner::DebugLanguageReport]) {
	for report in reports {
		// Summary table
		let mut summary_rows = vec![
			DebugSummaryRow {
				metric: "Language".to_string(),
				value: report.language_name.clone(),
			},
			DebugSummaryRow {
				metric: "ISO 639-2".to_string(),
				value: report.iso_639_2.clone(),
			},
			DebugSummaryRow {
				metric: "BCP 47".to_string(),
				value: report.bcp47.clone().unwrap_or_else(|| "-".to_string()),
			},
			DebugSummaryRow {
				metric: "Total tokens".to_string(),
				value: report.total_tokens.to_string(),
			},
			DebugSummaryRow {
				metric: "Pass 1 matches".to_string(),
				value: report.pass1.total_matches.to_string(),
			},
			DebugSummaryRow {
				metric: "Pass 1 confidence".to_string(),
				value: format!("{:.2}%", report.confidence),
			},
		];

		if let Some(ref p2) = report.pass2 {
			summary_rows.push(DebugSummaryRow {
				metric: "Pass 2 weighted score".to_string(),
				value: format!("{:.2}", p2.total_score),
			});
		}

		let mut summary_table = Table::new(summary_rows);
		summary_table.with(Style::modern());

		println!("{}", summary_table);

		// Pass 1 details table
		println!(
			"\nPass 1 — common_words ({} entries)",
			report.pass1.lexicon_size
		);
		let pass1_rows: Vec<DebugMatchRow> = report
			.pass1
			.matches
			.iter()
			.map(|m| DebugMatchRow {
				word: truncate_word(&m.word, 30),
				count: m.count,
				weight: format!("{:.1}", m.weight),
				subtotal: format!("{:.1}", m.subtotal),
				match_type: m.match_type.clone(),
			})
			.collect();

		let mut pass1_table = Table::new(pass1_rows);
		pass1_table
			.with(Style::modern())
			.with(Modify::new(Rows::first()).with(Alignment::center()))
			.with(Width::wrap(120));

		println!("{}", pass1_table);
		println!(
			"Pass 1 TOTAL matches: {}, score: {:.1}\n",
			report.pass1.total_matches, report.pass1.total_score
		);

		// Pass 2 details table (if present)
		if let Some(ref p2) = report.pass2 {
			println!("Pass 2 — weighted_words ({} entries)", p2.lexicon_size);
			let pass2_rows: Vec<DebugMatchRow> = p2
				.matches
				.iter()
				.map(|m| DebugMatchRow {
					word: truncate_word(&m.word, 30),
					count: m.count,
					weight: format!("{:.1}", m.weight),
					subtotal: format!("{:.1}", m.subtotal),
					match_type: m.match_type.clone(),
				})
				.collect();

			let mut pass2_table = Table::new(pass2_rows);
			pass2_table
				.with(Style::modern())
				.with(Modify::new(Rows::first()).with(Alignment::center()))
				.with(Width::wrap(120));

			println!("{}", pass2_table);
			println!(
				"Pass 2 TOTAL matches: {}, score: {:.1}\n",
				p2.total_matches, p2.total_score
			);
		}
	}
}

fn truncate_word(s: &str, max: usize) -> String {
	if s.chars().count() <= max {
		s.to_string()
	} else {
		let truncated: String = s.chars().take(max - 1).collect();
		format!("{}…", truncated)
	}
}

fn bcp47_display(bcp47: &Option<String>) -> &str {
	match bcp47 {
		Some(s) => s.as_str(),
		None => "-",
	}
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
		println!("    \"confidence\": {:.4},", w.confidence / 100.0);
		println!("    \"weighted_score\": {:.4}", w.weighted_score);
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
		println!("      \"confidence\": {:.4},", s.confidence / 100.0);
		println!("      \"weighted_score\": {:.4}", s.weighted_score);
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
	println!(
		"rank,language,iso_639_1,iso_639_2,bcp47,matched_words,total_words,confidence,weighted_score"
	);
	for (i, s) in scores.iter().enumerate() {
		println!(
			"{},{},{},{},{},{},{},{:.4},{:.4}",
			i + 1,
			csv_escape(&s.english_name),
			csv_escape(&s.iso_639_1),
			csv_escape(&s.iso_639_2),
			csv_escape(bcp47_display(&s.bcp47)),
			s.matched_words,
			s.total_words,
			s.confidence / 100.0,
			s.weighted_score,
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
