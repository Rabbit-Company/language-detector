use super::Language;

/// Norwegian Bokmål detection with Scandinavian disambiguation.
pub fn language() -> Language {
	Language {
		english_name: "Norwegian",
		iso_639_1: "no",
		iso_639_2: "nor",
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
			// Norwegian‑exclusive or highly characteristic forms (weight: 10.0)
			("jeg", 10.0),     // same as Danish but context
			("meg", 10.0),     // vs Danish "mig"
			("deg", 10.0),     // vs Danish "dig"
			("dere", 10.0),    // vs Danish "jer", Swedish "er"
			("ikke", 10.0),    // vs Swedish "inte"
			("av", 10.0),      // vs Danish "af"
			("hva", 10.0),     // vs Danish "hvad", Swedish "vad"
			("hvem", 10.0),    // vs Swedish "vem"
			("hvor", 10.0),    // vs Swedish "var"
			("hvordan", 10.0), // vs Swedish "hur"
			("hvorfor", 10.0), // vs Swedish "varför"
			("hvis", 10.0),    // "if"
			("jente", 10.0),   // "girl" vs Danish "pige", Swedish "flicka"
			("gutt", 10.0),    // "boy" vs Danish "dreng", Swedish "pojke"
			("mann", 5.0),     // "man" vs Danish "mand"
			("kvinne", 5.0),   // "woman" vs Danish "kvinde"
			("barn", 5.0),     // "child"
			// Additional distinctive words (weight: 5.0)
			("godt", 5.0),
			("meget", 5.0),   // "very" (also Danish)
			("litt", 5.0),    // "a little" (vs Danish "lidt")
			("igjen", 5.0),   // "again" (vs Danish "igen")
			("aldri", 5.0),   // "never" (vs Danish "aldrig")
			("alltid", 5.0),  // "always" (vs Danish "altid")
			("kanskje", 5.0), // "maybe" (vs Danish "måske")
			("gjerne", 5.0),  // "gladly" (vs Danish "gerne")
			("heller", 5.0),  // "rather" (vs Danish "hellere")
			("fort", 5.0),    // "quickly" (vs Danish "hurtigt")
			("sakte", 5.0),   // "slowly" (vs Danish "langsomt")
			("å", 10.0),      // infinitive marker (Danish "at")
		],
	}
}
