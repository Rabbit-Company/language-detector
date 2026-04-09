use super::Language;

/// English common words for language detection.
pub fn language() -> Language {
	Language {
		english_name: "English",
		iso_639_1: "en",
		iso_639_2: "eng",
		common_words: &[
			"the", "a", "an", "is", "are", "was", "were", "be", "been", "being", "have", "has", "had",
			"do", "does", "did", "will", "would", "could", "should", "may", "might", "shall", "can", "i",
			"you", "he", "she", "it", "we", "they", "me", "him", "her", "us", "them", "my", "your",
			"his", "its", "our", "their", "this", "that", "these", "those", "what", "which", "who",
			"whom", "where", "when", "why", "how", "not", "no", "but", "and", "or", "if", "then", "so",
			"just", "about", "up", "out", "on", "in", "at", "to", "for", "of", "with", "from", "by",
			"as", "into", "through", "after", "before", "between", "all", "each", "every", "both", "few",
			"more", "some", "any", "other", "than", "very", "too", "also", "only", "still", "already",
			"here", "there", "now", "know", "think", "want", "going", "come", "get", "make", "like",
			"right", "good", "well", "back", "okay", "yes",
		],
	}
}
