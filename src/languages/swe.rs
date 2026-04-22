use super::Language;

/// Swedish detection with Scandinavian disambiguation.
pub fn language() -> Language {
	Language {
		english_name: "Swedish",
		iso_639_1: "sv",
		iso_639_2: "swe",
		bcp47: None,
		disambiguation_group: Some("scandinavian"),
		common_words: &[
			// Shared Scandinavian core (identical spelling across Danish, Norwegian Bokmål, Swedish)
			"og",
			"i",
			"at",
			"det",
			"en",
			"den",
			"til",
			"er",
			"på",
			"med",
			"for",
			"der",
			"var",
			"har",
			"vi",
			"de",
			"du",
			"om",
			"så",
			"men",
			"et",
			"kan",
			"skal",
			"vil",
			"må",
			"have",
			"være",
			"blive",
			"gøre",
			"sige",
			"gå",
			"se",
			"vide",
			"tage",
			"give",
			"få",
			"finde",
			"holde",
			"stå",
			"ligge",
			"sætte",
			"tænke",
			"kende",
			"føle",
			"synes",
			"mene",
			"tro",
			"håbe",
			"ønske",
			"elske",
			"arbejde",
			"leve",
			"dø",
			"sove",
			"spise",
			"drikke",
			"læse",
			"skrive",
			"tale",
			"lytte",
			"høre",
			"kigge",
			"vise",
			"lave",
			"bruge",
			"prøve",
			"hjælpe",
			"stoppe",
			"starte",
			"åbne",
			"lukke",
			"vinde",
			"tabe",
			"købe",
			"sælge",
			"vente",
			"komme",
			"rejse",
			"køre",
			"flyve",
			"løbe",
			"cykle",
			"svømme",
			"danse",
			"synge",
			"spille",
			"tegne",
			"male",
			"bygge",
			"skære",
			"brække",
			"ødelægge",
			"reparere",
			"vaske",
			"rengøre",
			"rydde",
			"smide",
			"lede",
			"kaste",
			"fange",
			"ramme",
			"skyde",
			"slå",
			"sparke",
			"hoppe",
			"falde",
		],
		weighted_words: &[
			// Swedish‑exclusive or highly characteristic forms (weight: 10.0)
			("jag", 10.0),    // vs Danish/Norwegian "jeg"
			("mig", 10.0),    // vs Norwegian "meg"
			("dig", 10.0),    // vs Norwegian "deg"
			("er", 10.0),     // "you" plural / "are" vs Danish "jer", Norwegian "dere"
			("inte", 10.0),   // vs Danish/Norwegian "ikke"
			("av", 10.0),     // vs Danish "af"
			("vad", 10.0),    // vs Danish "hvad", Norwegian "hva"
			("vem", 10.0),    // vs Danish/Norwegian "hvem"
			("var", 10.0),    // "where" vs Danish/Norwegian "hvor"
			("hur", 10.0),    // "how" vs Danish/Norwegian "hvordan"
			("varför", 10.0), // "why" vs Danish/Norwegian "hvorfor"
			("om", 10.0),     // "if" (also "about") vs Danish/Norwegian "hvis"
			("flicka", 10.0), // "girl" vs Danish "pige", Norwegian "jente"
			("pojke", 10.0),  // "boy" vs Danish "dreng", Norwegian "gutt"
			("man", 5.0),     // "man" (indefinite pronoun) vs Danish "mand", Norwegian "mann"
			("kvinna", 5.0),  // "woman" vs Danish "kvinde", Norwegian "kvinne"
			("barn", 5.0),    // "child"
			// Additional distinctive words (weight: 5.0)
			("bra", 5.0),      // "good/well" (vs Danish "godt", Norwegian "godt")
			("mycket", 5.0),   // "very" (vs Danish "meget", Norwegian "meget")
			("lite", 5.0),     // "a little" (vs Danish "lidt", Norwegian "litt")
			("igen", 5.0),     // "again" (same as Danish, different from Norwegian "igjen")
			("aldrig", 5.0),   // "never" (same as Danish, vs Norwegian "aldri")
			("alltid", 5.0),   // "always" (same as Norwegian, vs Danish "altid")
			("kanske", 5.0),   // "maybe" (vs Danish "måske", Norwegian "kanskje")
			("gärna", 5.0),    // "gladly" (vs Danish "gerne", Norwegian "gjerne")
			("hellre", 5.0),   // "rather" (vs Danish "hellere", Norwegian "heller")
			("snabbt", 5.0),   // "quickly" (vs Danish "hurtigt", Norwegian "fort")
			("långsamt", 5.0), // "slowly" (vs Danish "langsomt", Norwegian "sakte")
			("att", 10.0),     // infinitive marker (vs Norwegian "å", Danish "at")
			("och", 5.0),      // "and" (vs Danish/Norwegian "og")
		],
	}
}
