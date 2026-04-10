mod afr;
mod amh;
mod ara;
mod aze;
mod bel;
mod ben;
mod bos;
mod bre;
mod bul;
mod cat;
mod ces;
mod cor;
mod dan;
mod deu;
mod ell;
mod eng_gb;
mod eng_us;
mod est;
mod eus;
mod fas;
mod fin;
mod fra;
mod gla;
mod gle;
mod glg;
mod glv;
mod guj;
mod heb;
mod hin;
mod hrv;
mod hun;
mod ind;
mod isl;
mod ita;
mod jpn;
mod kan;
mod kaz;
mod khm;
mod kir;
mod kor;
mod lao;
mod lav;
mod lin;
mod lit;
mod mal;
mod mar;
mod mkd;
mod mlt;
mod mon;
mod msa;
mod mya;
mod nep;
mod nld;
mod nor;
mod ori;
mod pan;
mod pol;
mod por_br;
mod por_pt;
mod ron;
mod rus;
mod sag;
mod sin;
mod slk;
mod slv;
mod spa_419;
mod spa_es;
mod sqi;
mod srp;
mod swa;
mod swe;
mod tam;
mod tel;
mod tgl;
mod tha;
mod tuk;
mod tur;
mod ukr;
mod urd;
mod uzb;
mod vie;
mod wel;
mod zho_hans;
mod zho_hant;

/// A single language entry with its ISO codes and list of common stop-words
/// used for frequency-based detection.
#[derive(Debug, Clone)]
pub struct Language {
	pub english_name: &'static str,
	pub iso_639_1: &'static str,
	pub iso_639_2: &'static str,
	pub bcp47: Option<&'static str>,
	pub common_words: &'static [&'static str],
}

/// Collect every registered language into a single Vec.
///
/// To add a new language, create a file `src/languages/<iso639_2>.rs` that
/// exports `pub fn language() -> Language { … }` and add the corresponding
/// `mod` declaration above plus one line in the vec below.
///
/// To add a regional variant, create a file like `src/languages/<base>_<region>.rs`
/// that exports `pub fn language() -> Language { … }` with a `bcp47` tag set
/// (e.g. `"pt-BR"`). Register it in the vec below alongside the base language.
pub fn build_catalogue() -> Vec<Language> {
	vec![
		eng_us::language(),
		eng_gb::language(),
		slv::language(),
		spa_es::language(),
		spa_419::language(),
		fra::language(),
		deu::language(),
		ita::language(),
		por_br::language(),
		por_pt::language(),
		jpn::language(),
		kor::language(),
		zho_hans::language(),
		zho_hant::language(),
		rus::language(),
		tur::language(),
		pol::language(),
		nld::language(),
		swe::language(),
		ara::language(),
		hin::language(),
		hrv::language(),
		srp::language(),
		ces::language(),
		ron::language(),
		hun::language(),
		ell::language(),
		fin::language(),
		nor::language(),
		dan::language(),
		tha::language(),
		vie::language(),
		ind::language(),
		ben::language(),
		bul::language(),
		ukr::language(),
		lit::language(),
		lav::language(),
		isl::language(),
		cat::language(),
		slk::language(),
		est::language(),
		tgl::language(),
		mon::language(),
		swa::language(),
		amh::language(),
		urd::language(),
		tel::language(),
		tam::language(),
		mar::language(),
		guj::language(),
		kan::language(),
		mal::language(),
		pan::language(),
		ori::language(),
		sin::language(),
		mya::language(),
		khm::language(),
		lao::language(),
		mkd::language(),
		bel::language(),
		aze::language(),
		kaz::language(),
		uzb::language(),
		kir::language(),
		tuk::language(),
		gle::language(),
		wel::language(),
		gla::language(),
		bre::language(),
		cor::language(),
		glv::language(),
		eus::language(),
		glg::language(),
		bos::language(),
		sqi::language(),
		sag::language(),
		lin::language(),
		mlt::language(),
		afr::language(),
		heb::language(),
		fas::language(),
		msa::language(),
		nep::language(),
	]
}
