/// Subtitle text cleaning: strips SRT/SSA/ASS markup, timestamps,
/// sequence numbers, and HTML-like tags so only spoken dialogue remains.

/// Strip common subtitle markup and return plain dialogue text.
///
/// Handles:
///   - SRT sequence numbers and timestamps
///   - SSA/ASS style override tags  `{\…}`
///   - SSA/ASS section headers, metadata, and `Dialogue:` lines
///   - HTML-like tags  `<…>`
pub fn clean_subtitle_text(raw: &str) -> String {
	let mut out = String::with_capacity(raw.len());

	for line in raw.lines() {
		let trimmed = line.trim();

		// Skip blank lines
		if trimmed.is_empty() {
			continue;
		}

		// Skip pure numeric lines (SRT sequence numbers)
		if trimmed.chars().all(|c| c.is_ascii_digit()) {
			continue;
		}

		// Skip SRT timestamp lines  "00:01:23,456 --> 00:01:25,789"
		if trimmed.contains("-->") {
			continue;
		}

		// Skip SSA/ASS section headers & metadata
		if trimmed.starts_with('[') && trimmed.ends_with(']') {
			continue;
		}
		if trimmed.contains("Dialogue:")
			|| trimmed.contains("Format:")
			|| trimmed.contains("Style:")
			|| trimmed.contains("ScriptType")
			|| trimmed.contains("PlayResX")
			|| trimmed.contains("PlayResY")
		{
			// For ASS Dialogue lines extract only the text portion
			if trimmed.contains("Dialogue:") {
				// Dialogue: 0,0:00:01.00,0:00:03.00,Default,,0,0,0,,Actual text
				if let Some(pos) = trimmed.rfind(",,") {
					let text_part = &trimmed[pos + 2..];
					let text_part = text_part
						.replace("\\N", " ")
						.replace("\\n", " ")
						.replace("\\h", " ");
					out.push_str(&text_part);
					out.push(' ');
				}
			}
			continue;
		}

		out.push_str(trimmed);
		out.push(' ');
	}

	// Remove {\…} tags (SSA/ASS override tags)
	let out = remove_braced_tags(&out);
	// Remove <…> tags (HTML tags like <i>, <b>, <font …>)
	remove_angle_tags(&out)
}

/// Tokenise text into detection units.
///
/// - Space-delimited scripts: lowercased word tokens
/// - No-space scripts (Han, Kana, Thai, Lao, Khmer, Myanmar): character tokens
///   plus overlapping bigrams to improve matching
pub fn tokenize(text: &str) -> Vec<String> {
	let mut tokens = Vec::new();
	let mut word_buf = String::new();
	let mut script_buf = String::new();
	let mut in_no_space_script = false;

	fn flush_word(buf: &mut String, out: &mut Vec<String>) {
		if !buf.is_empty() {
			let token = buf
				.trim_matches(|c: char| c.is_ascii_punctuation() || c == '…' || c == '—' || c == '–')
				.to_lowercase();
			if !token.is_empty() {
				out.push(token);
			}
			buf.clear();
		}
	}

	fn flush_script(buf: &mut String, out: &mut Vec<String>) {
		if buf.is_empty() {
			return;
		}

		let chars: Vec<char> = buf.chars().collect();

		for ch in &chars {
			out.push(ch.to_string());
		}

		for window in chars.windows(2) {
			let gram: String = window.iter().collect();
			out.push(gram);
		}

		buf.clear();
	}

	for ch in text.chars() {
		if is_no_space_script(ch) {
			if !in_no_space_script {
				flush_word(&mut word_buf, &mut tokens);
				in_no_space_script = true;
			}
			if !ch.is_whitespace() && !is_general_punct(ch) {
				script_buf.push(ch);
			}
		} else {
			if in_no_space_script {
				flush_script(&mut script_buf, &mut tokens);
				in_no_space_script = false;
			}

			if ch.is_whitespace() {
				flush_word(&mut word_buf, &mut tokens);
			} else if !is_general_punct(ch) {
				for lc in ch.to_lowercase() {
					word_buf.push(lc);
				}
			}
		}
	}

	if in_no_space_script {
		flush_script(&mut script_buf, &mut tokens);
	} else {
		flush_word(&mut word_buf, &mut tokens);
	}

	tokens
}

fn is_general_punct(ch: char) -> bool {
	ch.is_ascii_punctuation()
		|| matches!(
			ch,
			'…'
				| '—'
				| '–'
				| '「'
				| '」'
				| '『'
				| '』'
				| '（'
				| '）'
				| '《'
				| '》'
				| '，'
				| '。'
				| '、'
				| '！'
				| '？'
				| '：'
				| '；'
		)
}

fn is_no_space_script(ch: char) -> bool {
	let u = ch as u32;

	// CJK Unified Ideographs
	(0x4E00..=0x9FFF).contains(&u)
		// CJK Extension A
		|| (0x3400..=0x4DBF).contains(&u)
		// Hiragana
		|| (0x3040..=0x309F).contains(&u)
		// Katakana
		|| (0x30A0..=0x30FF).contains(&u)
		// Thai
		|| (0x0E00..=0x0E7F).contains(&u)
		// Lao
		|| (0x0E80..=0x0EFF).contains(&u)
		// Khmer
		|| (0x1780..=0x17FF).contains(&u)
		// Myanmar
		|| (0x1000..=0x109F).contains(&u)
}

fn remove_braced_tags(s: &str) -> String {
	let mut result = String::with_capacity(s.len());
	let mut inside = false;
	for ch in s.chars() {
		if ch == '{' {
			inside = true;
		} else if ch == '}' {
			inside = false;
		} else if !inside {
			result.push(ch);
		}
	}
	result
}

fn remove_angle_tags(s: &str) -> String {
	let mut result = String::with_capacity(s.len());
	let mut inside = false;
	for ch in s.chars() {
		if ch == '<' {
			inside = true;
		} else if ch == '>' {
			inside = false;
		} else if !inside {
			result.push(ch);
		}
	}
	result
}
