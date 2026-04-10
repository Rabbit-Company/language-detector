use super::Language;

/// Brazilian Portuguese detection.
///
/// Pass 1 uses shared Portuguese common words. Pass 2 uses Brazilian‑specific
/// grammar and vocabulary to distinguish from European Portuguese.
pub fn language() -> Language {
	Language {
		english_name: "Portuguese (Brazil)",
		iso_639_1: "pt",
		iso_639_2: "por",
		bcp47: Some("pt-BR"),
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
			// Critical grammar – você/vocês (weight: 10.0)
			// Brazil uses "você" instead of "tu" for informal singular.
			// =============================================================
			("você", 10.0),
			("vocês", 10.0),
			// Gerund forms – Brazil strongly prefers gerund over infinitive
			("fazendo", 10.0),
			("falando", 10.0),
			("comendo", 10.0),
			("bebendo", 10.0),
			("dormindo", 10.0),
			("correndo", 10.0),
			("saindo", 10.0),
			("vindo", 10.0),
			("indo", 10.0),
			("tendo", 10.0),
			("sendo", 10.0),
			("estando", 10.0),
			// Contractions with "para" → "pra" (very common in BR speech)
			("pra", 8.0),
			("pro", 8.0),
			("pros", 8.0),
			("pras", 8.0),
			// =============================================================
			// Distinct vocabulary (weight: 5.0)
			// =============================================================
			("ônibus", 5.0),    // PT: autocarro
			("celular", 5.0),   // PT: telemóvel
			("geladeira", 5.0), // PT: frigorífico
			("xícara", 5.0),    // PT: chávena
			("banheiro", 5.0),  // PT: casa de banho
			("legal", 5.0),     // PT: fixe / porreiro
			("bacana", 5.0),
			("cara", 5.0),      // guy (PT: gajo)
			("gente", 5.0),     // we/us (a gente)
			("tchau", 5.0),     // PT: adeus
			("oi", 5.0),        // PT: olá
			("beleza", 5.0),    // OK/cool
			("valeu", 5.0),     // thanks
			("fila", 5.0),      // queue (PT: bicha)
			("time", 5.0),      // team (PT: equipa)
			("goleiro", 5.0),   // goalkeeper (PT: guarda-redes)
			("torcida", 5.0),   // supporters (PT: claque)
			("delegacia", 5.0), // police station (PT: esquadra)
			("carteira", 5.0),  // wallet / ID (PT: bilhete de identidade)
			("sorvete", 5.0),   // ice cream (PT: gelado)
			("suco", 5.0),      // juice (PT: sumo)
			("açougue", 5.0),   // butcher shop (PT: talho)
			("moça", 5.0),      // young woman (PT: rapariga)
			("garoto", 5.0),    // boy (PT: rapaz / miúdo)
			("garota", 5.0),    // girl
			("moleque", 5.0),   // kid (PT: miúdo)
			("trem", 5.0),      // train (PT: comboio)
			("metrô", 5.0),     // subway (PT: metro)
			("ônibus", 5.0),
			("calçada", 5.0), // sidewalk (PT: passeio)
			("sarjeta", 5.0), // gutter
			("grama", 5.0),   // grass (PT: relva)
			("abacaxi", 5.0), // pineapple (PT: ananás)
			("mamão", 5.0),   // papaya
			("bolacha", 5.0), // cookie (PT: bolacha/biscoito, but BR prefers bolacha)
			("biscoito", 5.0),
		],
	}
}
