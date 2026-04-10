use super::Language;

/// Swahili common words for language detection.
pub fn language() -> Language {
	Language {
		english_name: "Swahili",
		iso_639_1: "sw",
		iso_639_2: "swa",
		bcp47: None,
		common_words: &[
			"na", "ya", "wa", "za", "cha", "vya", "la", "pa", "kwa", "ni", "si", "mimi", "wewe", "yeye",
			"sisi", "nyinyi", "wao", "hii", "hiyo", "ile", "hapa", "hapo", "pale", "wapi", "lini",
			"vipi", "kwa", "nini", "nani", "gani", "ambao", "ambaye", "ambacho", "ambavyo", "ambalo",
			"ambapo", "kuwa", "ni", "si", "una", "ana", "tuna", "mna", "wana", "nina", "una", "ana",
			"tuna", "mna", "wana", "niko", "uko", "yuko", "tuko", "mko", "wako", "nipo", "upo", "yupo",
			"tupo", "mpo", "wapo", "nime", "ume", "ame", "tume", "mme", "wame", "nita", "uta", "ata",
			"tuta", "mta", "wata", "nina", "una", "ana", "tuna", "mna", "wana", "nilikuwa", "ulikuwa",
			"alikuwa", "tulikuwa", "mlikuwa", "walikuwa", "nitakuwa", "utakuwa", "atakuwa", "tutakuwa",
			"mtakuwa", "watakuwa", "sina", "huna", "hana", "hatuna", "hamna", "hawana", "siko", "huko",
			"hayuko", "hatuko", "hamko", "hawako", "sipo", "hupo", "hayupo", "hatupo", "hampo", "hawapo",
			"sija", "huja", "haja", "hatuja", "hamja", "hawaja", "sita", "huta", "hata", "hatuta",
			"hamta", "hawata", "basi", "lakini", "ila", "au", "na", "tena", "pia", "vilevile", "hata",
			"hivyo", "ndiyo", "sivyo",
		],
		weighted_words: &[],
		disambiguation_group: None,
	}
}
