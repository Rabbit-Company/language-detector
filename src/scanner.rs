use std::collections::{BTreeMap, HashMap, HashSet};
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
	/// Weighted score from Pass 2 variant disambiguation.
	/// Only non-zero for languages that have `weighted_words` and competed
	/// in the disambiguation pass. For all other languages this is `0.0`.
	pub weighted_score: f64,
}

#[derive(Debug, Clone)]
pub struct DebugMatchDetail {
	pub word: String,
	pub count: usize,
	pub weight: f64,
	pub subtotal: f64,
	pub match_type: String, // "exact" or "prefix"
}

#[derive(Debug, Clone)]
pub struct DebugPassReport {
	pub lexicon_size: usize,
	pub matches: Vec<DebugMatchDetail>,
	pub total_matches: usize,
	pub total_score: f64,
}

#[derive(Debug, Clone)]
pub struct DebugLanguageReport {
	pub language_name: String,
	pub iso_639_2: String,
	pub bcp47: Option<String>,
	pub total_tokens: usize,
	pub pass1: DebugPassReport,
	pub pass2: Option<DebugPassReport>,
	pub confidence: f64,
}

/// Two-pass language detection.
///
/// **Pass 1 – Language identification**
/// Score every language using only `common_words`. This produces a ranked
/// list where closely related variants (e.g. es-419 and es-ES) will
/// typically cluster near the top with very similar match counts.
///
/// **Pass 2 – Variant disambiguation**
/// Identify groups of variants that share the same `iso_639_2` code *and*
/// have non-empty `weighted_words`. For each such group, score the tokens
/// against only the `weighted_words` lists. The variants are then re-ordered
/// within their group by `weighted_score` (descending), so that the most
/// regionally distinct variant floats to the top even when their
/// `common_words` counts are identical.
pub fn scan(languages: Vec<Language>, words: Vec<String>) -> Vec<LanguageScore> {
	let total_words = words.len();
	let words = Arc::new(words);

	// ── Pass 1: score every language by common_words only ────────────
	let mut handles: Vec<thread::JoinHandle<(LanguageScore, Language)>> =
		Vec::with_capacity(languages.len());

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

			let score = LanguageScore {
				english_name: lang.english_name.to_string(),
				iso_639_1: lang.iso_639_1.to_string(),
				iso_639_2: lang.iso_639_2.to_string(),
				bcp47: lang.bcp47.map(|s| s.to_string()),
				matched_words: matched,
				total_words,
				confidence,
				weighted_score: 0.0,
			};

			(score, lang)
		});
		handles.push(handle);
	}

	let mut pass1_results: Vec<(LanguageScore, Language)> = handles
		.into_iter()
		.map(|h| h.join().expect("Language scanner thread panicked"))
		.collect();

	// Sort Pass 1 by matched_words descending
	pass1_results.sort_by(|a, b| {
		b.0
			.matched_words
			.cmp(&a.0.matched_words)
			.then_with(|| b.0.confidence.partial_cmp(&a.0.confidence).unwrap())
	});

	// ── Pass 2: disambiguate variants within same disambiguation_group ─────────
	//
	// Find variant groups: languages sharing the same disambiguation_group where at
	// least one member has non-empty weighted_words.
	let mut variant_groups: HashMap<String, Vec<usize>> = HashMap::new();
	for (i, (_score, lang)) in pass1_results.iter().enumerate() {
		if !lang.weighted_words.is_empty() {
			let group_key = lang
				.disambiguation_group
				.unwrap_or(lang.iso_639_2)
				.to_string();
			variant_groups.entry(group_key).or_default().push(i);
		}
	}

	// For each group with 2+ members, compute weighted_scores
	for (_iso, indices) in &variant_groups {
		if indices.len() < 2 {
			continue;
		}

		for &idx in indices {
			let lang = &pass1_results[idx].1;
			let weight_map: HashMap<&str, f64> = lang.weighted_words.iter().copied().collect();

			let mut wscore: f64 = 0.0;
			for w in words.iter() {
				if let Some(&weight) = weight_map.get(w.as_str()) {
					wscore += weight;
				}
			}

			pass1_results[idx].0.weighted_score = wscore;
		}
	}

	// Re-sort variant groups internally by weighted_score.
	// We do this by collecting the final scores and sorting with a
	// compound key: (matched_words DESC, weighted_score DESC).
	let mut scores: Vec<LanguageScore> = pass1_results.into_iter().map(|(s, _)| s).collect();

	scores.sort_by(|a, b| {
		b.matched_words
			.cmp(&a.matched_words)
			.then_with(|| b.weighted_score.partial_cmp(&a.weighted_score).unwrap())
			.then_with(|| {
				// Only apply tie-breaker if both are Spanish variants with same matched_words
				// and weighted_score is 0.0 (or very close)
				if a.iso_639_2 == "spa"
					&& b.iso_639_2 == "spa"
					&& a.matched_words == b.matched_words
					&& (a.weighted_score - b.weighted_score).abs() < f64::EPSILON
				{
					// Prefer es-419 over es-ES
					match (a.bcp47.as_deref(), b.bcp47.as_deref()) {
						(Some("es-419"), Some("es-ES")) => std::cmp::Ordering::Less,
						(Some("es-ES"), Some("es-419")) => std::cmp::Ordering::Greater,
						_ => std::cmp::Ordering::Equal,
					}
				} else {
					std::cmp::Ordering::Equal
				}
			})
	});

	scores
}

pub fn debug_languages(
	languages: &[Language],
	words: &[String],
	filter: &str,
) -> Vec<DebugLanguageReport> {
	let filter_lower = filter.to_lowercase();

	let matched_langs: Vec<&Language> = languages
		.iter()
		.filter(|lang| {
			lang.english_name.to_lowercase().contains(&filter_lower)
				|| lang.iso_639_1.to_lowercase() == filter_lower
				|| lang.iso_639_2.to_lowercase() == filter_lower
				|| lang
					.bcp47
					.map(|b| b.to_lowercase() == filter_lower)
					.unwrap_or(false)
		})
		.collect();

	if matched_langs.is_empty() {
		eprintln!("No languages matching '{}' found in the catalogue.", filter);
		std::process::exit(1);
	}

	let mut reports = Vec::new();

	for lang in &matched_langs {
		// Build pass1 report (common_words)
		let word_set: HashSet<&str> = lang.common_words.iter().copied().collect();
		let mut common_hits: BTreeMap<String, (usize, String)> = BTreeMap::new();

		for token in words {
			if word_set.contains(token.as_str()) {
				let entry = common_hits
					.entry(token.clone())
					.or_insert((0, "exact".to_string()));
				entry.0 += 1;
			} else if token.chars().count() > 2 {
				if let Some(root) = word_set
					.iter()
					.find(|entry| entry.len() >= 5 && token.starts_with(*entry))
				{
					let key = format!("{} → {}", token, root);
					let entry = common_hits.entry(key).or_insert((0, "prefix".to_string()));
					entry.0 += 1;
				}
			}
		}

		let total_common_matches: usize = common_hits.values().map(|(c, _)| c).sum();
		let mut pass1_matches: Vec<DebugMatchDetail> = common_hits
			.into_iter()
			.map(|(word, (count, match_type))| DebugMatchDetail {
				subtotal: count as f64,
				word,
				count,
				weight: 1.0,
				match_type,
			})
			.collect();
		pass1_matches.sort_by(|a, b| b.subtotal.partial_cmp(&a.subtotal).unwrap());

		let pass1 = DebugPassReport {
			lexicon_size: lang.common_words.len(),
			matches: pass1_matches,
			total_matches: total_common_matches,
			total_score: total_common_matches as f64,
		};

		// Build pass2 report if weighted words exist
		let pass2 = if !lang.weighted_words.is_empty() {
			let weight_map: HashMap<&str, f64> = lang.weighted_words.iter().copied().collect();
			let mut weighted_hits: BTreeMap<String, (usize, f64, String)> = BTreeMap::new();

			for token in words {
				if let Some(&weight) = weight_map.get(token.as_str()) {
					let entry =
						weighted_hits
							.entry(token.clone())
							.or_insert((0, weight, "exact".to_string()));
					entry.0 += 1;
				}
			}

			let total_weighted_matches: usize = weighted_hits.values().map(|(c, _, _)| c).sum();
			let total_weighted_score: f64 = weighted_hits.values().map(|(c, w, _)| *c as f64 * w).sum();

			let mut pass2_matches: Vec<DebugMatchDetail> = weighted_hits
				.into_iter()
				.map(|(word, (count, weight, match_type))| DebugMatchDetail {
					subtotal: count as f64 * weight,
					word,
					count,
					weight,
					match_type,
				})
				.collect();
			pass2_matches.sort_by(|a, b| b.subtotal.partial_cmp(&a.subtotal).unwrap());

			Some(DebugPassReport {
				lexicon_size: lang.weighted_words.len(),
				matches: pass2_matches,
				total_matches: total_weighted_matches,
				total_score: total_weighted_score,
			})
		} else {
			None
		};

		let confidence = if words.len() > 0 {
			(pass1.total_matches as f64 / words.len() as f64) * 100.0
		} else {
			0.0
		};

		reports.push(DebugLanguageReport {
			language_name: lang.english_name.to_string(),
			iso_639_2: lang.iso_639_2.to_string(),
			bcp47: lang.bcp47.map(|s| s.to_string()),
			total_tokens: words.len(),
			pass1,
			pass2,
			confidence,
		});
	}

	reports
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
	// count a token as a match if it starts with a known entry of length >= 5
	// This helps with inflected/agglutinative forms without exploding false positives.
	lexicon
		.iter()
		.any(|entry| entry.len() >= 5 && token.starts_with(entry))
}
