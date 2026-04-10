use super::Language;

/// European Portuguese detection.
///
/// Pass 1 uses shared Portuguese common words. Pass 2 uses European‑specific
/// grammar and vocabulary to distinguish from Brazilian Portuguese.
pub fn language() -> Language {
	Language {
		english_name: "Portuguese (Portugal)",
		iso_639_1: "pt",
		iso_639_2: "por",
		bcp47: Some("pt-PT"),
		disambiguation_group: Some("por"),
		common_words: &[
			// Shared Portuguese words (identical in both variants)
			"de", "a", "o", "que", "e", "do", "da", "em", "um", "para", "é", "com", "não", "uma", "os",
			"no", "se", "na", "por", "mais", "as", "dos", "como", "mas", "foi", "ao", "ele", "das",
			"tem", "à", "seu", "sua", "ou", "ser", "quando", "muito", "há", "nos", "já", "está", "eu",
			"também", "só", "pelo", "pela", "até", "isso", "ela", "entre", "era", "depois", "sem",
			"mesmo", "aos", "ter", "seus", "quem", "nas", "me", "esse", "eles", "estão", "tinha",
			"foram", "essa", "num", "nem", "suas", "meu", "às", "minha", "têm", "numa", "pelos", "elas",
			"havia", "seja", "qual", "será", "nós", "tenho", "lhe", "deles", "essas", "esses", "pelas",
			"este", "fosse", "deve", "tinham", // Infinitives and common verb stems
			"fazer", "dizer", "ir", "ver", "estar", "haver", "poder", "dever", "vir", "ficar", "saber",
			"dar", "querer", "falar", "passar", "sair", "pôr", "levar", "achar", "deixar", "chegar",
			"partir", "sentir",
		],
		weighted_words: &[
			// =============================================================
			// Critical grammar – tu conjugations (weight: 10.0)
			// Portugal uses "tu" with distinct verb forms.
			// =============================================================
			("tu", 10.0),
			("tens", 10.0),
			("fazes", 10.0),
			("sabes", 10.0),
			("podes", 10.0),
			("queres", 10.0),
			("deves", 10.0),
			("vens", 10.0),
			("dizes", 10.0),
			("és", 10.0),
			("estás", 10.0),
			("vais", 10.0),
			("comes", 10.0),
			("bebes", 10.0),
			("vives", 10.0),
			("sais", 10.0),
			("pensas", 10.0),
			("sentes", 10.0),
			("dormes", 10.0),
			("pedes", 10.0),
			("segues", 10.0),
			("convosco", 10.0), // with you (plural)
			("contigo", 10.0),  // with you (singular)
			// =============================================================
			// Distinct vocabulary (weight: 5.0)
			// =============================================================
			("autocarro", 5.0),     // BR: ônibus
			("comboio", 5.0),       // BR: trem
			("telemóvel", 5.0),     // BR: celular
			("frigorífico", 5.0),   // BR: geladeira
			("chávena", 5.0),       // BR: xícara
			("casa de banho", 5.0), // BR: banheiro
			("fixe", 5.0),          // cool (BR: legal)
			("porreiro", 5.0),      // cool
			("gajo", 5.0),          // guy (BR: cara)
			("gaja", 5.0),          // girl (BR: garota)
			("malta", 5.0),         // folks/guys
			("adeus", 5.0),         // goodbye (BR: tchau)
			("olá", 5.0),           // hello (BR: oi)
			("bicha", 5.0),         // queue (BR: fila)
			("equipa", 5.0),        // team (BR: time)
			("claque", 5.0),        // supporters (BR: torcida)
			("esquadra", 5.0),      // police station (BR: delegacia)
			("bilhete", 5.0),       // ticket (BR: ingresso)
			("gelado", 5.0),        // ice cream (BR: sorvete)
			("sumo", 5.0),          // juice (BR: suco)
			("talho", 5.0),         // butcher shop (BR: açougue)
			("rapariga", 5.0),      // young woman (BR: moça)
			("rapaz", 5.0),         // young man (BR: garoto)
			("miúdo", 5.0),         // kid (BR: moleque)
			("miúda", 5.0),
			("metro", 5.0),          // subway (BR: metrô)
			("passeio", 5.0),        // sidewalk (BR: calçada)
			("relva", 5.0),          // grass (BR: grama)
			("ananás", 5.0),         // pineapple (BR: abacaxi)
			("alperce", 5.0),        // apricot (BR: damasco)
			("pequeno almoço", 5.0), // breakfast (BR: café da manhã)
		],
	}
}
