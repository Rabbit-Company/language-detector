/// Subtitle text cleaning: strips SRT/SSA/ASS markup, timestamps,
/// sequence numbers, and HTML-like tags so only spoken dialogue remains.

/// Strip common subtitle markup and return plain dialogue text.
///
/// Handles:
///   - SRT sequence numbers and timestamps
///   - SSA/ASS style override tags  `{\…}`
///   - SSA/ASS section headers, metadata, and `Dialogue:` lines
///   - SSA/ASS drawing mode (`\p1`..`\p4`) - vector data, not dialogue
///   - SSA/ASS `Comment:` and `Data:` lines
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

		// Skip SSA/ASS non-dialogue event lines and metadata
		if trimmed.starts_with("Comment:")
			|| trimmed.starts_with("Data:")
			|| trimmed.starts_with("Format:")
			|| trimmed.starts_with("Style:")
			|| trimmed.contains("ScriptType")
			|| trimmed.contains("PlayResX")
			|| trimmed.contains("PlayResY")
		{
			continue;
		}

		// For ASS Dialogue lines extract only the text portion
		if trimmed.starts_with("Dialogue:") {
			if let Some(text_part) = extract_ass_dialogue_text(trimmed) {
				// Skip drawing-mode lines (\p1..\p4) — vector data, not speech
				if has_drawing_mode(&text_part) {
					continue;
				}

				let text_part = text_part
					.replace("\\N", " ")
					.replace("\\n", " ")
					.replace("\\h", " ");

				let clean_text_part = remove_angle_tags(&remove_braced_tags(&text_part));
				if clean_text_part.trim().chars().count() < 3 {
					continue;
				}

				out.push_str(&text_part);
				out.push(' ');
			}
			continue;
		}

		// Skip any other SSA/ASS metadata lines (key: value pattern in
		// header sections such as [Script Info] or [Aegisub Project Garbage])
		if is_ssa_metadata(trimmed) {
			continue;
		}

		let clean_text_part = remove_angle_tags(&remove_braced_tags(&trimmed));
		if clean_text_part.trim().chars().count() < 3 {
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

/// Extract the Text field from an ASS Dialogue line by counting to the
/// 9th comma.
///
/// ASS Dialogue format (10 fields, 9 commas):
///   Dialogue: Layer,Start,End,Style,Name,MarginL,MarginR,MarginV,Effect,Text
///
fn extract_ass_dialogue_text(line: &str) -> Option<String> {
	// Find the start of fields after "Dialogue:"
	let after_prefix = line.find("Dialogue:")?;
	let fields_start = after_prefix + "Dialogue:".len();
	let rest = &line[fields_start..];

	// Count 9 commas to reach the Text field
	let mut comma_count = 0;
	for (i, ch) in rest.char_indices() {
		if ch == ',' {
			comma_count += 1;
			if comma_count == 9 {
				return Some(rest[i + 1..].to_string());
			}
		}
	}

	None
}

/// Check whether the raw text field (before brace-stripping) contains an
/// ASS drawing mode tag `\p1` through `\p4` inside an override block.
///
/// When drawing mode is active the "text" is vector draw commands
/// (m, l, b, s, ...) rather than spoken dialogue.
fn has_drawing_mode(text: &str) -> bool {
	let bytes = text.as_bytes();
	let len = bytes.len();
	let mut i = 0;

	while i + 2 < len {
		if bytes[i] == b'\\' && bytes[i + 1] == b'p' {
			let d = bytes.get(i + 2).copied().unwrap_or(0);
			if d >= b'1' && d <= b'4' {
				// Make sure it is not a longer tag like \pos or \pbo
				let after = bytes.get(i + 3).copied().unwrap_or(0);
				if after == b'}' || after == b'\\' || after == 0 {
					return true;
				}
			}
		}
		i += 1;
	}

	false
}

/// Heuristic: detect SSA/ASS metadata lines that live in header sections
/// like [Script Info] or [Aegisub Project Garbage].  These follow a
/// "Key: Value" pattern where the key is ASCII-only and contains no commas.
fn is_ssa_metadata(line: &str) -> bool {
	// Must contain a colon
	if let Some(colon_pos) = line.find(':') {
		// Key part is before the colon
		let key = &line[..colon_pos];
		// SSA metadata keys are short, ASCII, and have no commas
		if !key.is_empty()
			&& key.len() <= 40
			&& key
				.chars()
				.all(|c| c.is_ascii_alphanumeric() || c == ' ' || c == '_')
		{
			return true;
		}
	}
	// Lines starting with ';' are SSA comments
	if line.starts_with(';') {
		return true;
	}
	false
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
