use super::Language;

/// Spanish common words for language detection.
pub fn language() -> Language {
	Language {
		english_name: "Spanish",
		iso_639_1: "es",
		iso_639_2: "spa",
		common_words: &[
			"de", "la", "que", "el", "en", "y", "a", "los", "se", "del", "las", "un", "por", "con", "no",
			"una", "su", "para", "es", "al", "lo", "como", "más", "o", "pero", "sus", "le", "ya", "o",
			"este", "ha", "sí", "porque", "esta", "entre", "cuando", "muy", "sin", "sobre", "también",
			"me", "hasta", "hay", "donde", "quien", "todo", "nos", "durante", "todos", "uno", "les",
			"ni", "contra", "otros", "ese", "eso", "ante", "ellos", "e", "esto", "mí", "antes",
			"algunos", "qué", "unos", "yo", "otro", "otras", "mucho", "quién", "tú", "está", "tanto",
			"esa", "estos", "mucho", "mucha", "poco", "ella", "estar", "estas", "algunas", "algo",
			"nosotros", "mi", "mis", "tú", "te", "ti", "vosotros", "vosotras", "os", "ellos", "ellas",
			"usted", "ustedes",
		],
	}
}
