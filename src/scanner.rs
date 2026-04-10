use std::collections::HashSet;
use std::sync::Arc;
use std::thread;

use crate::languages::Language;

/// Scored result for a single language after scanning.
#[derive(Debug, Clone)]
pub struct LanguageScore {
	pub english_name: String,
	pub iso_639_1: String,
	pub iso_639_2: String,
	pub bcp47: Option<String>,
	pub matched_words: usize,
	pub total_words: usize,
	pub confidence: f64,
}

/// Scan `words` against every language in `languages` using one thread per
/// language. Returns results sorted descending by match count.
pub fn scan(languages: Vec<Language>, words: Vec<String>) -> Vec<LanguageScore> {
	let total_words = words.len();
	let words = Arc::new(words);

	let mut handles: Vec<thread::JoinHandle<LanguageScore>> = Vec::with_capacity(languages.len());

	for lang in languages {
		let words = Arc::clone(&words);
		let handle = thread::spawn(move || {
			let word_set: HashSet<&str> = lang.common_words.iter().copied().collect();
			let matched = words
				.iter()
				.filter(|w| token_matches_language(w, &word_set))
				.count();

			let confidence = if total_words > 0 {
				(matched as f64 / total_words as f64) * 100.0
			} else {
				0.0
			};

			LanguageScore {
				english_name: lang.english_name.to_string(),
				iso_639_1: lang.iso_639_1.to_string(),
				iso_639_2: lang.iso_639_2.to_string(),
				bcp47: lang.bcp47.map(|s| s.to_string()),
				matched_words: matched,
				total_words,
				confidence,
			}
		});

		handles.push(handle);
	}

	let mut scores: Vec<LanguageScore> = handles
		.into_iter()
		.map(|h| h.join().expect("Language scanner thread panicked"))
		.collect();

	scores.sort_by(|a, b| {
		b.matched_words
			.cmp(&a.matched_words)
			.then_with(|| b.confidence.partial_cmp(&a.confidence).unwrap())
	});
	scores
}

fn token_matches_language(token: &str, lexicon: &HashSet<&str>) -> bool {
	if lexicon.contains(token) {
		return true;
	}

	// For no-space scripts, exact character / n-gram matching is enough.
	if token.chars().count() <= 2 {
		return false;
	}

	// Weak morphological fallback:
	// count a token as a match if it starts with a known entry of length >= 4.
	// This helps with inflected/agglutinative forms without exploding false positives.
	lexicon
		.iter()
		.any(|entry| entry.len() >= 4 && token.starts_with(entry))
}
