use super::Language;

/// Danish detection with Scandinavian disambiguation.
pub fn language() -> Language {
	Language {
		english_name: "Danish",
		iso_639_1: "da",
		iso_639_2: "dan",
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
			// Danish‑exclusive or highly characteristic forms (weight: 10.0)
			("jeg", 10.0),     // vs Swedish "jag"
			("mig", 10.0),     // vs Norwegian "meg"
			("dig", 10.0),     // vs Norwegian "deg"
			("jer", 10.0),     // vs Norwegian "dere", Swedish "er"
			("ikke", 10.0),    // vs Swedish "inte"
			("af", 10.0),      // vs Norwegian/Swedish "av"
			("hvad", 10.0),    // vs Norwegian "hva", Swedish "vad"
			("hvem", 10.0),    // vs Swedish "vem"
			("hvor", 10.0),    // vs Swedish "var"
			("hvordan", 10.0), // vs Swedish "hur"
			("hvorfor", 10.0), // vs Swedish "varför"
			("hvis", 10.0),    // "if" (also "whose")
			("pige", 10.0),    // "girl" vs Norwegian "jente", Swedish "flicka"
			("dreng", 10.0),   // "boy" vs Norwegian "gutt", Swedish "pojke"
			("mand", 5.0),     // "man" (common but helps vs Norwegian "mann")
			("kvinde", 5.0),   // "woman" vs Norwegian "kvinne", Swedish "kvinna"
			("barn", 5.0), // "child" (same in Norwegian, different in Swedish "barn"? Actually Swedish "barn" same; keep as neutral)
			// Additional distinctive words (weight: 5.0)
			("godt", 5.0),     // "well/good"
			("meget", 5.0),    // "very"
			("lidt", 5.0),     // "a little"
			("igen", 5.0),     // "again"
			("aldrig", 5.0),   // "never"
			("altid", 5.0),    // "always"
			("måske", 5.0),    // "maybe"
			("gerne", 5.0),    // "gladly"
			("hellere", 5.0),  // "rather"
			("hurtigt", 5.0),  // "quickly"
			("langsomt", 5.0), // "slowly"
		],
	}
}
