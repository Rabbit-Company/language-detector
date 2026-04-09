use super::Language;

/// Portuguese common words for language detection.
pub fn language() -> Language {
	Language {
		english_name: "Portuguese",
		iso_639_1: "pt",
		iso_639_2: "por",
		common_words: &[
			"de", "a", "o", "que", "e", "do", "da", "em", "um", "para", "é", "com", "não", "uma", "os",
			"no", "se", "na", "por", "mais", "as", "dos", "como", "mas", "foi", "ao", "ele", "das",
			"tem", "à", "seu", "sua", "ou", "ser", "quando", "muito", "há", "nos", "já", "está", "eu",
			"também", "só", "pelo", "pela", "até", "isso", "ela", "entre", "era", "depois", "sem",
			"mesmo", "aos", "ter", "seus", "quem", "nas", "me", "esse", "eles", "estão", "você", "tinha",
			"foram", "essa", "num", "nem", "suas", "meu", "às", "minha", "têm", "numa", "pelos", "elas",
			"havia", "seja", "qual", "será", "nós", "tenho", "lhe", "deles", "essas", "esses", "pelas",
			"este", "fosse", "deve", "tinham", "fazer", "dizer", "ir", "ver", "estar", "ter", "haver",
			"poder", "dever", "vir", "ficar", "saber", "dar", "querer", "falar", "passar", "sair", "pôr",
			"levar", "achar", "deixar", "chegar", "partir", "sentir",
		],
	}
}
