use super::Language;

/// Estonian common words for language detection.
pub fn language() -> Language {
	Language {
		english_name: "Estonian",
		iso_639_1: "et",
		iso_639_2: "est",
		common_words: &[
			"ja", "on", "ei", "see", "ma", "sa", "ta", "me", "te", "nad", "mina", "sina", "tema", "meie",
			"teie", "nemad", "olema", "ole", "olen", "oled", "on", "oleme", "olete", "on", "ei ole",
			"pole", "poles", "poleme", "polete", "pole", "tegema", "teha", "teen", "teed", "teeb",
			"teeme", "teete", "teevad", "ütlema", "öelda", "ütlen", "ütled", "ütleb", "ütleme", "ütlete",
			"ütlevad", "minema", "minna", "lähen", "lähed", "läheb", "läheme", "lähete", "lähevad",
			"tulema", "tulla", "tulen", "tuled", "tuleb", "tuleme", "tulet", "tulevad", "saama", "saada",
			"saan", "saad", "saab", "saame", "saate", "saavad", "võima", "võida", "võin", "võid", "võib",
			"võime", "võite", "võivad", "pidama", "pidada", "pean", "pead", "peab", "peame", "peate",
			"peavad", "tahtma", "tahta", "tahan", "tahad", "tahab", "tahame", "tahate", "tahavad",
			"nägema", "näha", "näen", "näed", "näeb", "näeme", "näete", "näevad", "kuulma", "kuulda",
			"kuulen", "kuuled", "kuuleb", "kuuleme", "kuulete", "kuulevad", "teadma", "teada", "tean",
			"tead", "teab", "teame", "teate", "teavad", "uskuma", "uskuda", "usun", "usud", "usub",
			"usume", "usute", "usuvad", "mõtlema",
		],
	}
}
