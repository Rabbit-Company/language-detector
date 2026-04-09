use super::Language;

/// Slovenian common words for language detection.
pub fn language() -> Language {
	Language {
		english_name: "Slovenian",
		iso_639_1: "sl",
		iso_639_2: "slv",
		common_words: &[
			"je", "sem", "si", "smo", "ste", "so", "in", "na", "za", "da", "ne", "se", "bi", "ga", "jo",
			"jih", "mi", "me", "ti", "te", "to", "ta", "kaj", "ali", "ker", "ko", "po", "od", "do", "iz",
			"pri", "ob", "nad", "pod", "med", "vse", "kaj", "kako", "kje", "kam", "kdaj", "lahko",
			"samo", "zelo", "bolj", "tudi", "pa", "le", "ni", "ima", "bil", "bila", "bilo", "bili",
			"bile", "bom", "moram", "nisem", "nisi", "ste", "hočem", "mora", "tukaj", "tam", "zdaj",
			"potem", "vem", "mislim", "prosim", "hvala", "dobro", "ja", "nič",
		],
	}
}
