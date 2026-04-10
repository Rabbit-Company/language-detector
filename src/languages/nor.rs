use super::Language;

/// Norwegian common words for language detection.
pub fn language() -> Language {
	Language {
		english_name: "Norwegian",
		iso_639_1: "no",
		iso_639_2: "nor",
		bcp47: None,
		common_words: &[
			"og", "i", "det", "på", "som", "er", "en", "til", "å", "han", "av", "for", "med", "at",
			"var", "de", "ikke", "den", "har", "jeg", "om", "et", "men", "så", "seg", "hun", "hadde",
			"fra", "vi", "du", "kan", "da", "ble", "ut", "skal", "vil", "ham", "etter", "over", "ved",
			"også", "bare", "eller", "sa", "nå", "dette", "noe", "være", "meg", "mot", "opp", "ned",
			"inn", "her", "der", "hvor", "hvem", "hva", "hvilken", "hvilket", "hvilke", "når", "hvordan",
			"hvorfor", "hvis", "fordi", "mens", "selv", "enn", "enda", "enda", "ennå", "bare", "kun",
			"nesten", "ganske", "svært", "veldig", "mer", "mest", "flere", "flest", "mange", "få",
			"noen", "ingen", "alle", "alt", "hele", "helt", "egen", "eget", "egne", "annen", "annet",
			"andre", "samme", "slik", "sånn", "slike", "sånne", "denne", "dette", "disse", "den", "det",
			"de", "min", "mitt", "mine", "din", "ditt", "dine", "hans", "hennes", "vår", "vårt", "våre",
			"deres", "sin", "sitt", "sine", "være", "ha", "bli", "kunne", "skulle", "ville", "måtte",
			"burde",
		],
		weighted_words: &[],
		disambiguation_group: None,
	}
}
