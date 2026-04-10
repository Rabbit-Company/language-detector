use super::Language;

/// British English detection.
///
/// Pass 1 uses shared English common words. Pass 2 uses British‑specific
/// spelling and vocabulary to distinguish from American English.
pub fn language() -> Language {
	Language {
		english_name: "English (UK)",
		iso_639_1: "en",
		iso_639_2: "eng",
		bcp47: Some("en-GB"),
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
			// British spelling patterns (weight: 10.0 – strong signals)
			// =============================================================
			// -our vs -or
			("colour", 10.0),
			("favour", 10.0),
			("honour", 10.0),
			("labour", 10.0),
			("neighbour", 10.0),
			("humour", 10.0),
			("rumour", 10.0),
			("behaviour", 10.0),
			("flavour", 10.0),
			("harbour", 10.0),
			("valour", 10.0),
			("vigour", 10.0),
			("tumour", 10.0),
			("vapour", 10.0),
			("saviour", 10.0),
			("glamour", 10.0),
			("armour", 10.0),
			("clamour", 10.0),
			("odour", 10.0),
			("parlour", 10.0),
			("rancour", 10.0),
			("rigour", 10.0),
			("splendour", 10.0),
			// -re vs -er
			("centre", 10.0),
			("theatre", 10.0),
			("metre", 10.0),
			("fibre", 10.0),
			("litre", 10.0),
			("sabre", 10.0),
			("sombre", 10.0),
			("lustre", 10.0),
			("mitre", 10.0),
			("ochre", 10.0),
			("reconnoitre", 10.0),
			("spectre", 10.0),
			("titre", 10.0),
			// -ise vs -ize (verb endings)
			("organise", 10.0),
			("realise", 10.0),
			("recognise", 10.0),
			("analyse", 10.0),
			("apologise", 10.0),
			("categorise", 10.0),
			("criticise", 10.0),
			("customise", 10.0),
			("emphasise", 10.0),
			("memorise", 10.0),
			("maximise", 10.0),
			("minimise", 10.0),
			("optimise", 10.0),
			("prioritise", 10.0),
			("summarise", 10.0),
			("authorise", 10.0),
			("capitalise", 10.0),
			("characterise", 10.0),
			("civilise", 10.0),
			("colonise", 10.0),
			("dramatise", 10.0),
			("energise", 10.0),
			("finalise", 10.0),
			("harmonise", 10.0),
			("idolise", 10.0),
			("immunise", 10.0),
			("itemise", 10.0),
			("jeopardise", 10.0),
			("legitimise", 10.0),
			("magnetise", 10.0),
			("mobilise", 10.0),
			("modernise", 10.0),
			("monopolise", 10.0),
			("neutralise", 10.0),
			("normalise", 10.0),
			("paralyse", 10.0),
			("patronise", 10.0),
			("penalise", 10.0),
			("popularise", 10.0),
			("privatise", 10.0),
			("publicise", 10.0),
			("randomise", 10.0),
			("rationalise", 10.0),
			("scrutinise", 10.0),
			("stabilise", 10.0),
			("sterilise", 10.0),
			("subsidise", 10.0),
			("symbolise", 10.0),
			("synchronise", 10.0),
			("systemise", 10.0),
			("terrorise", 10.0),
			("traumatise", 10.0),
			("tyrannise", 10.0),
			("utilise", 10.0),
			("vandalise", 10.0),
			("victimise", 10.0),
			("visualise", 10.0),
			("vitalise", 10.0),
			("vocalise", 10.0),
			// -ll- vs -l- (past tense doubling)
			("cancelled", 10.0),
			("travelled", 10.0),
			("labelled", 10.0),
			("modelled", 10.0),
			("levelled", 10.0),
			("marvelled", 10.0),
			("quarrelled", 10.0),
			("signalled", 10.0),
			("totalled", 10.0),
			("cancelling", 10.0),
			("travelling", 10.0),
			("labelling", 10.0),
			("modelling", 10.0),
			("levelling", 10.0),
			// Other unambiguous spelling differences
			("grey", 10.0),
			("ageing", 10.0),
			("judgement", 10.0),
			("acknowledgement", 10.0),
			("fulfilment", 10.0),
			("defence", 10.0),
			("offence", 10.0),
			("licence", 10.0),
			("practise", 10.0),
			("pretence", 10.0),
			("tyre", 10.0),
			("kerb", 10.0),
			("cheque", 10.0),
			("programme", 10.0),
			("catalogue", 10.0),
			("dialogue", 10.0),
			("monologue", 10.0),
			("pyjamas", 10.0),
			("moustache", 10.0),
			("aluminium", 10.0),
			("jewellery", 10.0),
			("plough", 10.0),
			("sceptic", 10.0),
			("sulphur", 10.0),
			("whisky", 10.0),
			("yoghurt", 10.0),
			("manoeuvre", 10.0),
			// =============================================================
			// British vocabulary (weight: 5.0) – unambiguous signals
			// =============================================================
			("lorry", 5.0),               // truck
			("petrol", 5.0),              // gasoline
			("nappy", 5.0),               // diaper
			("holiday", 5.0),             // vacation (note: "holiday" is UK; US uses "vacation")
			("autumn", 5.0),              // fall
			("torch", 5.0),               // flashlight
			("rubbish", 5.0),             // garbage/trash
			("queue", 5.0),               // line (as in waiting line)
			("boot", 5.0),                // trunk (car)
			("bonnet", 5.0),              // hood (car)
			("motorway", 5.0),            // freeway
			("maths", 5.0),               // math
			("mobile", 5.0),              // cell phone (UK "mobile phone")
			("loo", 5.0),                 // restroom
			("chemist", 5.0),             // drugstore
			("car park", 5.0),            // parking lot
			("postcode", 5.0),            // zip code
			("whilst", 5.0),              // while (conjunction)
			("amongst", 5.0),             // among
			("towards", 5.0),             // toward
			("afterwards", 5.0),          // afterward
			("fortnight", 5.0),           // two weeks
			("cinema", 5.0),              // movie theater
			("crisps", 5.0),              // potato chips (US "chips")
			("aubergine", 5.0),           // eggplant
			("courgette", 5.0),           // zucchini
			("coriander", 5.0),           // cilantro
			("rocket", 5.0),              // arugula
			("takeaway", 5.0),            // takeout
			("till", 5.0),                // cash register
			("jumper", 5.0),              // sweater
			("trousers", 5.0),            // pants
			("trainers", 5.0),            // sneakers
			("vest", 5.0),                // undershirt
			("waistcoat", 5.0),           // vest (US)
			("dummy", 5.0),               // pacifier
			("pushchair", 5.0),           // stroller
			("pram", 5.0),                // baby carriage
			("cot", 5.0),                 // crib
			("solicitor", 5.0),           // lawyer (UK specific term)
			("estate agent", 5.0),        // real estate agent
			("dustbin", 5.0),             // trash can
			("bobby", 5.0),               // police officer (slang)
			("copper", 5.0),              // police officer (slang)
			("postman", 5.0),             // mailman
			("anticlockwise", 5.0),       // counterclockwise
			("tick", 5.0),                // check mark
			("full stop", 5.0),           // period (punctuation)
			("brackets", 5.0),            // parentheses
			("zed", 5.0),                 // zee
			("noughts and crosses", 5.0), // tic-tac-toe
			("draughts", 5.0),            // checkers
			("cuppa", 5.0),               // cup of tea
			("knackered", 5.0),           // exhausted
			("chuffed", 5.0),             // pleased
			("gobsmacked", 5.0),          // astonished
			("skint", 5.0),               // broke
			("ta", 5.0),                  // thank you
			("whinge", 5.0),              // complain
		],
	}
}
