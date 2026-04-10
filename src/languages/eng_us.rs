use super::Language;

/// American English detection.
///
/// Pass 1 uses shared English common words. Pass 2 uses American‑specific
/// spelling and vocabulary to distinguish from British English.
pub fn language() -> Language {
	Language {
		english_name: "English (US)",
		iso_639_1: "en",
		iso_639_2: "eng",
		bcp47: Some("en-US"),
		disambiguation_group: Some("eng"),
		common_words: &[
			// Shared English function words and neutral vocabulary
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
			// Neutral vocabulary (same spelling in both)
			"man", "woman", "child", "people", "day", "week", "month", "year", "time", "thing", "way",
			"life", "world", "hand", "eye", "head", "house", "home", "work", "school", "car", "road",
			"street", "city", "country",
		],
		weighted_words: &[
			// =============================================================
			// American spelling patterns (weight: 10.0 – strong signals)
			// =============================================================
			("color", 10.0),
			("favor", 10.0),
			("honor", 10.0),
			("labor", 10.0),
			("neighbor", 10.0),
			("humor", 10.0),
			("rumor", 10.0),
			("behavior", 10.0),
			("flavor", 10.0),
			("harbor", 10.0),
			("valor", 10.0),
			("vigor", 10.0),
			("tumor", 10.0),
			("vapor", 10.0),
			("savior", 10.0),
			("glamor", 10.0),
			("armor", 10.0),
			("clamor", 10.0),
			("odor", 10.0),
			("parlor", 10.0),
			("rancor", 10.0),
			("rigor", 10.0),
			("splendor", 10.0),
			("center", 10.0),
			("theater", 10.0),
			("meter", 10.0),
			("fiber", 10.0),
			("liter", 10.0),
			("saber", 10.0),
			("somber", 10.0),
			("luster", 10.0),
			("miter", 10.0),
			("ocher", 10.0),
			("reconnoiter", 10.0),
			("specter", 10.0),
			("titer", 10.0),
			("organize", 10.0),
			("realize", 10.0),
			("recognize", 10.0),
			("analyze", 10.0),
			("apologize", 10.0),
			("categorize", 10.0),
			("criticize", 10.0),
			("customize", 10.0),
			("emphasize", 10.0),
			("memorize", 10.0),
			("maximize", 10.0),
			("minimize", 10.0),
			("optimize", 10.0),
			("prioritize", 10.0),
			("summarize", 10.0),
			("authorize", 10.0),
			("capitalize", 10.0),
			("characterize", 10.0),
			("civilize", 10.0),
			("colonize", 10.0),
			("dramatize", 10.0),
			("energize", 10.0),
			("finalize", 10.0),
			("harmonize", 10.0),
			("idolize", 10.0),
			("immunize", 10.0),
			("itemize", 10.0),
			("jeopardize", 10.0),
			("legitimize", 10.0),
			("magnetize", 10.0),
			("mobilize", 10.0),
			("modernize", 10.0),
			("monopolize", 10.0),
			("neutralize", 10.0),
			("normalize", 10.0),
			("paralyze", 10.0),
			("patronize", 10.0),
			("penalize", 10.0),
			("popularize", 10.0),
			("privatize", 10.0),
			("publicize", 10.0),
			("randomize", 10.0),
			("rationalize", 10.0),
			("scrutinize", 10.0),
			("stabilize", 10.0),
			("sterilize", 10.0),
			("subsidize", 10.0),
			("symbolize", 10.0),
			("synchronize", 10.0),
			("systemize", 10.0),
			("terrorize", 10.0),
			("traumatize", 10.0),
			("tyrannize", 10.0),
			("utilize", 10.0),
			("vandalize", 10.0),
			("victimize", 10.0),
			("visualize", 10.0),
			("vitalize", 10.0),
			("vocalize", 10.0),
			("canceled", 10.0),
			("traveled", 10.0),
			("labeled", 10.0),
			("modeled", 10.0),
			("leveled", 10.0),
			("marveled", 10.0),
			("quarreled", 10.0),
			("signaled", 10.0),
			("totaled", 10.0),
			("canceling", 10.0),
			("traveling", 10.0),
			("labeling", 10.0),
			("modeling", 10.0),
			("leveling", 10.0),
			("gray", 10.0),
			("aging", 10.0),
			("judgment", 10.0),
			("acknowledgment", 10.0),
			("fulfillment", 10.0),
			("defense", 10.0),
			("offense", 10.0),
			("license", 10.0),
			("practice", 10.0),
			("pretense", 10.0),
			("tire", 10.0),
			("curb", 10.0),
			("program", 10.0),
			("catalog", 10.0),
			("dialog", 10.0),
			("monolog", 10.0),
			("pajamas", 10.0),
			("mustache", 10.0),
			("aluminum", 10.0),
			("jewelry", 10.0),
			("plow", 10.0),
			("skeptic", 10.0),
			("sulfur", 10.0),
			("whiskey", 10.0),
			("yogurt", 10.0),
			("maneuver", 10.0),
			// =============================================================
			// American vocabulary (weight: 5.0) – unambiguous signals
			// =============================================================
			("elevator", 5.0),         // lift
			("sidewalk", 5.0),         // pavement
			("truck", 5.0),            // lorry
			("gasoline", 5.0),         // petrol (also "gas")
			("faucet", 5.0),           // tap
			("cookie", 5.0),           // biscuit
			("candy", 5.0),            // sweets
			("diaper", 5.0),           // nappy
			("vacation", 5.0),         // holiday
			("fall", 5.0),             // autumn
			("flashlight", 5.0),       // torch
			("garbage", 5.0),          // rubbish
			("trunk", 5.0),            // boot
			("hood", 5.0),             // bonnet
			("freeway", 5.0),          // motorway
			("soccer", 5.0),           // football (association)
			("math", 5.0),             // maths
			("cellphone", 5.0),        // mobile
			("restroom", 5.0),         // loo
			("drugstore", 5.0),        // chemist
			("downtown", 5.0),         // city centre
			("parking lot", 5.0),      // car park
			("zip code", 5.0),         // postcode
			("toward", 5.0),           // towards
			("afterward", 5.0),        // afterwards
			("bangs", 5.0),            // fringe
			("movie theater", 5.0),    // cinema
			("movie", 5.0),            // film (cinema)
			("fries", 5.0),            // chips (hot)
			("eggplant", 5.0),         // aubergine
			("zucchini", 5.0),         // courgette
			("cilantro", 5.0),         // coriander
			("arugula", 5.0),          // rocket
			("takeout", 5.0),          // takeaway
			("sweater", 5.0),          // jumper
			("pants", 5.0),            // trousers
			("sneakers", 5.0),         // trainers
			("undershirt", 5.0),       // vest
			("pacifier", 5.0),         // dummy
			("stroller", 5.0),         // pushchair
			("crib", 5.0),             // cot
			("realtor", 5.0),          // estate agent
			("trash can", 5.0),        // dustbin
			("mailman", 5.0),          // postman
			("counterclockwise", 5.0), // anticlockwise
			("check mark", 5.0),       // tick
			("period", 5.0),           // full stop
			("parentheses", 5.0),      // brackets
			("zee", 5.0),              // zed
			("tic-tac-toe", 5.0),      // noughts and crosses
			("checkers", 5.0),         // draughts
		],
	}
}
