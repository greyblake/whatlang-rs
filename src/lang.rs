// NOTE:
//    This file is generated automatically.
//    Edit misc/lang.rs.erb template instead of editing lang.rs file directly.

use std::fmt;
use std::str::FromStr;

use crate::error::ParseError;

/// Represents a language following [ISO 639-3](https://en.wikipedia.org/wiki/ISO_639-3) standard.
#[cfg_attr(feature = "enum-map", derive(::enum_map::Enum))]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
pub enum Lang {
    /// Esperanto (Esperanto)
    Epo = 0,

    /// English (English)
    Eng = 1,

    /// Русский (Russian)
    Rus = 2,

    /// 普通话 (Mandarin)
    Cmn = 3,

    /// Español (Spanish)
    Spa = 4,

    /// Português (Portuguese)
    Por = 5,

    /// Italiano (Italian)
    Ita = 6,

    /// বাংলা (Bengali)
    Ben = 7,

    /// Français (French)
    Fra = 8,

    /// Deutsch (German)
    Deu = 9,

    /// Українська (Ukrainian)
    Ukr = 10,

    /// ქართული (Georgian)
    Kat = 11,

    /// العربية (Arabic)
    Ara = 12,

    /// हिन्दी (Hindi)
    Hin = 13,

    /// 日本語 (Japanese)
    Jpn = 14,

    /// עברית (Hebrew)
    Heb = 15,

    /// ייִדיש (Yiddish)
    Yid = 16,

    /// Polski (Polish)
    Pol = 17,

    /// አማርኛ (Amharic)
    Amh = 18,

    /// Basa Jawa (Javanese)
    Jav = 19,

    /// 한국어 (Korean)
    Kor = 20,

    /// Bokmål (Bokmal)
    Nob = 21,

    /// Dansk (Danish)
    Dan = 22,

    /// Svenska (Swedish)
    Swe = 23,

    /// Suomi (Finnish)
    Fin = 24,

    /// Türkçe (Turkish)
    Tur = 25,

    /// Nederlands (Dutch)
    Nld = 26,

    /// Magyar (Hungarian)
    Hun = 27,

    /// Čeština (Czech)
    Ces = 28,

    /// Ελληνικά (Greek)
    Ell = 29,

    /// Български (Bulgarian)
    Bul = 30,

    /// Беларуская (Belarusian)
    Bel = 31,

    /// मराठी (Marathi)
    Mar = 32,

    /// ಕನ್ನಡ (Kannada)
    Kan = 33,

    /// Română (Romanian)
    Ron = 34,

    /// Slovenščina (Slovene)
    Slv = 35,

    /// Hrvatski (Croatian)
    Hrv = 36,

    /// Српски (Serbian)
    Srp = 37,

    /// Македонски (Macedonian)
    Mkd = 38,

    /// Lietuvių (Lithuanian)
    Lit = 39,

    /// Latviešu (Latvian)
    Lav = 40,

    /// Eesti (Estonian)
    Est = 41,

    /// தமிழ் (Tamil)
    Tam = 42,

    /// Tiếng Việt (Vietnamese)
    Vie = 43,

    /// اُردُو (Urdu)
    Urd = 44,

    /// ภาษาไทย (Thai)
    Tha = 45,

    /// ગુજરાતી (Gujarati)
    Guj = 46,

    /// Oʻzbekcha (Uzbek)
    Uzb = 47,

    /// ਪੰਜਾਬੀ (Punjabi)
    Pan = 48,

    /// Azərbaycanca (Azerbaijani)
    Aze = 49,

    /// Bahasa Indonesia (Indonesian)
    Ind = 50,

    /// తెలుగు (Telugu)
    Tel = 51,

    /// فارسی (Persian)
    Pes = 52,

    /// മലയാളം (Malayalam)
    Mal = 53,

    /// ଓଡ଼ିଆ (Oriya)
    Ori = 54,

    /// မြန်မာစာ (Burmese)
    Mya = 55,

    /// नेपाली (Nepali)
    Nep = 56,

    /// සිංහල (Sinhalese)
    Sin = 57,

    /// ភាសាខ្មែរ (Khmer)
    Khm = 58,

    /// Türkmençe (Turkmen)
    Tuk = 59,

    /// Akan (Akan)
    Aka = 60,

    /// IsiZulu (Zulu)
    Zul = 61,

    /// ChiShona (Shona)
    Sna = 62,

    /// Afrikaans (Afrikaans)
    Afr = 63,

    /// Lingua Latina (Latin)
    Lat = 64,

    /// Slovenčina (Slovak)
    Slk = 65,

    /// Català (Catalan)
    Cat = 66,

    /// Tagalog (Tagalog)
    Tgl = 67,

    /// Հայերեն (Armenian)
    Hye = 68,
}

const VALUES: [Lang; 69] = [
    Lang::Epo,
    Lang::Eng,
    Lang::Rus,
    Lang::Cmn,
    Lang::Spa,
    Lang::Por,
    Lang::Ita,
    Lang::Ben,
    Lang::Fra,
    Lang::Deu,
    Lang::Ukr,
    Lang::Kat,
    Lang::Ara,
    Lang::Hin,
    Lang::Jpn,
    Lang::Heb,
    Lang::Yid,
    Lang::Pol,
    Lang::Amh,
    Lang::Jav,
    Lang::Kor,
    Lang::Nob,
    Lang::Dan,
    Lang::Swe,
    Lang::Fin,
    Lang::Tur,
    Lang::Nld,
    Lang::Hun,
    Lang::Ces,
    Lang::Ell,
    Lang::Bul,
    Lang::Bel,
    Lang::Mar,
    Lang::Kan,
    Lang::Ron,
    Lang::Slv,
    Lang::Hrv,
    Lang::Srp,
    Lang::Mkd,
    Lang::Lit,
    Lang::Lav,
    Lang::Est,
    Lang::Tam,
    Lang::Vie,
    Lang::Urd,
    Lang::Tha,
    Lang::Guj,
    Lang::Uzb,
    Lang::Pan,
    Lang::Aze,
    Lang::Ind,
    Lang::Tel,
    Lang::Pes,
    Lang::Mal,
    Lang::Ori,
    Lang::Mya,
    Lang::Nep,
    Lang::Sin,
    Lang::Khm,
    Lang::Tuk,
    Lang::Aka,
    Lang::Zul,
    Lang::Sna,
    Lang::Afr,
    Lang::Lat,
    Lang::Slk,
    Lang::Cat,
    Lang::Tgl,
    Lang::Hye,
];

fn lang_from_code<S: Into<String>>(code: S) -> Option<Lang> {
    match code.into().to_lowercase().as_ref() {
        "epo" => Some(Lang::Epo),
        "eng" => Some(Lang::Eng),
        "rus" => Some(Lang::Rus),
        "cmn" => Some(Lang::Cmn),
        "spa" => Some(Lang::Spa),
        "por" => Some(Lang::Por),
        "ita" => Some(Lang::Ita),
        "ben" => Some(Lang::Ben),
        "fra" => Some(Lang::Fra),
        "deu" => Some(Lang::Deu),
        "ukr" => Some(Lang::Ukr),
        "kat" => Some(Lang::Kat),
        "ara" => Some(Lang::Ara),
        "hin" => Some(Lang::Hin),
        "jpn" => Some(Lang::Jpn),
        "heb" => Some(Lang::Heb),
        "yid" => Some(Lang::Yid),
        "pol" => Some(Lang::Pol),
        "amh" => Some(Lang::Amh),
        "jav" => Some(Lang::Jav),
        "kor" => Some(Lang::Kor),
        "nob" => Some(Lang::Nob),
        "dan" => Some(Lang::Dan),
        "swe" => Some(Lang::Swe),
        "fin" => Some(Lang::Fin),
        "tur" => Some(Lang::Tur),
        "nld" => Some(Lang::Nld),
        "hun" => Some(Lang::Hun),
        "ces" => Some(Lang::Ces),
        "ell" => Some(Lang::Ell),
        "bul" => Some(Lang::Bul),
        "bel" => Some(Lang::Bel),
        "mar" => Some(Lang::Mar),
        "kan" => Some(Lang::Kan),
        "ron" => Some(Lang::Ron),
        "slv" => Some(Lang::Slv),
        "hrv" => Some(Lang::Hrv),
        "srp" => Some(Lang::Srp),
        "mkd" => Some(Lang::Mkd),
        "lit" => Some(Lang::Lit),
        "lav" => Some(Lang::Lav),
        "est" => Some(Lang::Est),
        "tam" => Some(Lang::Tam),
        "vie" => Some(Lang::Vie),
        "urd" => Some(Lang::Urd),
        "tha" => Some(Lang::Tha),
        "guj" => Some(Lang::Guj),
        "uzb" => Some(Lang::Uzb),
        "pan" => Some(Lang::Pan),
        "aze" => Some(Lang::Aze),
        "ind" => Some(Lang::Ind),
        "tel" => Some(Lang::Tel),
        "pes" => Some(Lang::Pes),
        "mal" => Some(Lang::Mal),
        "ori" => Some(Lang::Ori),
        "mya" => Some(Lang::Mya),
        "nep" => Some(Lang::Nep),
        "sin" => Some(Lang::Sin),
        "khm" => Some(Lang::Khm),
        "tuk" => Some(Lang::Tuk),
        "aka" => Some(Lang::Aka),
        "zul" => Some(Lang::Zul),
        "sna" => Some(Lang::Sna),
        "afr" => Some(Lang::Afr),
        "lat" => Some(Lang::Lat),
        "slk" => Some(Lang::Slk),
        "cat" => Some(Lang::Cat),
        "tgl" => Some(Lang::Tgl),
        "hye" => Some(Lang::Hye),
        _ => None,
    }
}

fn lang_to_code(lang: Lang) -> &'static str {
    match lang {
        Lang::Epo => "epo",
        Lang::Eng => "eng",
        Lang::Rus => "rus",
        Lang::Cmn => "cmn",
        Lang::Spa => "spa",
        Lang::Por => "por",
        Lang::Ita => "ita",
        Lang::Ben => "ben",
        Lang::Fra => "fra",
        Lang::Deu => "deu",
        Lang::Ukr => "ukr",
        Lang::Kat => "kat",
        Lang::Ara => "ara",
        Lang::Hin => "hin",
        Lang::Jpn => "jpn",
        Lang::Heb => "heb",
        Lang::Yid => "yid",
        Lang::Pol => "pol",
        Lang::Amh => "amh",
        Lang::Jav => "jav",
        Lang::Kor => "kor",
        Lang::Nob => "nob",
        Lang::Dan => "dan",
        Lang::Swe => "swe",
        Lang::Fin => "fin",
        Lang::Tur => "tur",
        Lang::Nld => "nld",
        Lang::Hun => "hun",
        Lang::Ces => "ces",
        Lang::Ell => "ell",
        Lang::Bul => "bul",
        Lang::Bel => "bel",
        Lang::Mar => "mar",
        Lang::Kan => "kan",
        Lang::Ron => "ron",
        Lang::Slv => "slv",
        Lang::Hrv => "hrv",
        Lang::Srp => "srp",
        Lang::Mkd => "mkd",
        Lang::Lit => "lit",
        Lang::Lav => "lav",
        Lang::Est => "est",
        Lang::Tam => "tam",
        Lang::Vie => "vie",
        Lang::Urd => "urd",
        Lang::Tha => "tha",
        Lang::Guj => "guj",
        Lang::Uzb => "uzb",
        Lang::Pan => "pan",
        Lang::Aze => "aze",
        Lang::Ind => "ind",
        Lang::Tel => "tel",
        Lang::Pes => "pes",
        Lang::Mal => "mal",
        Lang::Ori => "ori",
        Lang::Mya => "mya",
        Lang::Nep => "nep",
        Lang::Sin => "sin",
        Lang::Khm => "khm",
        Lang::Tuk => "tuk",
        Lang::Aka => "aka",
        Lang::Zul => "zul",
        Lang::Sna => "sna",
        Lang::Afr => "afr",
        Lang::Lat => "lat",
        Lang::Slk => "slk",
        Lang::Cat => "cat",
        Lang::Tgl => "tgl",
        Lang::Hye => "hye",
    }
}

fn lang_to_name(lang: Lang) -> &'static str {
    match lang {
        Lang::Epo => "Esperanto",
        Lang::Eng => "English",
        Lang::Rus => "Русский",
        Lang::Cmn => "普通话",
        Lang::Spa => "Español",
        Lang::Por => "Português",
        Lang::Ita => "Italiano",
        Lang::Ben => "বাংলা",
        Lang::Fra => "Français",
        Lang::Deu => "Deutsch",
        Lang::Ukr => "Українська",
        Lang::Kat => "ქართული",
        Lang::Ara => "العربية",
        Lang::Hin => "हिन्दी",
        Lang::Jpn => "日本語",
        Lang::Heb => "עברית",
        Lang::Yid => "ייִדיש",
        Lang::Pol => "Polski",
        Lang::Amh => "አማርኛ",
        Lang::Jav => "Basa Jawa",
        Lang::Kor => "한국어",
        Lang::Nob => "Bokmål",
        Lang::Dan => "Dansk",
        Lang::Swe => "Svenska",
        Lang::Fin => "Suomi",
        Lang::Tur => "Türkçe",
        Lang::Nld => "Nederlands",
        Lang::Hun => "Magyar",
        Lang::Ces => "Čeština",
        Lang::Ell => "Ελληνικά",
        Lang::Bul => "Български",
        Lang::Bel => "Беларуская",
        Lang::Mar => "मराठी",
        Lang::Kan => "ಕನ್ನಡ",
        Lang::Ron => "Română",
        Lang::Slv => "Slovenščina",
        Lang::Hrv => "Hrvatski",
        Lang::Srp => "Српски",
        Lang::Mkd => "Македонски",
        Lang::Lit => "Lietuvių",
        Lang::Lav => "Latviešu",
        Lang::Est => "Eesti",
        Lang::Tam => "தமிழ்",
        Lang::Vie => "Tiếng Việt",
        Lang::Urd => "اُردُو",
        Lang::Tha => "ภาษาไทย",
        Lang::Guj => "ગુજરાતી",
        Lang::Uzb => "Oʻzbekcha",
        Lang::Pan => "ਪੰਜਾਬੀ",
        Lang::Aze => "Azərbaycanca",
        Lang::Ind => "Bahasa Indonesia",
        Lang::Tel => "తెలుగు",
        Lang::Pes => "فارسی",
        Lang::Mal => "മലയാളം",
        Lang::Ori => "ଓଡ଼ିଆ",
        Lang::Mya => "မြန်မာစာ",
        Lang::Nep => "नेपाली",
        Lang::Sin => "සිංහල",
        Lang::Khm => "ភាសាខ្មែរ",
        Lang::Tuk => "Türkmençe",
        Lang::Aka => "Akan",
        Lang::Zul => "IsiZulu",
        Lang::Sna => "ChiShona",
        Lang::Afr => "Afrikaans",
        Lang::Lat => "Lingua Latina",
        Lang::Slk => "Slovenčina",
        Lang::Cat => "Català",
        Lang::Tgl => "Tagalog",
        Lang::Hye => "Հայերեն",
    }
}

fn lang_to_eng_name(lang: Lang) -> &'static str {
    match lang {
        Lang::Epo => "Esperanto",
        Lang::Eng => "English",
        Lang::Rus => "Russian",
        Lang::Cmn => "Mandarin",
        Lang::Spa => "Spanish",
        Lang::Por => "Portuguese",
        Lang::Ita => "Italian",
        Lang::Ben => "Bengali",
        Lang::Fra => "French",
        Lang::Deu => "German",
        Lang::Ukr => "Ukrainian",
        Lang::Kat => "Georgian",
        Lang::Ara => "Arabic",
        Lang::Hin => "Hindi",
        Lang::Jpn => "Japanese",
        Lang::Heb => "Hebrew",
        Lang::Yid => "Yiddish",
        Lang::Pol => "Polish",
        Lang::Amh => "Amharic",
        Lang::Jav => "Javanese",
        Lang::Kor => "Korean",
        Lang::Nob => "Bokmal",
        Lang::Dan => "Danish",
        Lang::Swe => "Swedish",
        Lang::Fin => "Finnish",
        Lang::Tur => "Turkish",
        Lang::Nld => "Dutch",
        Lang::Hun => "Hungarian",
        Lang::Ces => "Czech",
        Lang::Ell => "Greek",
        Lang::Bul => "Bulgarian",
        Lang::Bel => "Belarusian",
        Lang::Mar => "Marathi",
        Lang::Kan => "Kannada",
        Lang::Ron => "Romanian",
        Lang::Slv => "Slovene",
        Lang::Hrv => "Croatian",
        Lang::Srp => "Serbian",
        Lang::Mkd => "Macedonian",
        Lang::Lit => "Lithuanian",
        Lang::Lav => "Latvian",
        Lang::Est => "Estonian",
        Lang::Tam => "Tamil",
        Lang::Vie => "Vietnamese",
        Lang::Urd => "Urdu",
        Lang::Tha => "Thai",
        Lang::Guj => "Gujarati",
        Lang::Uzb => "Uzbek",
        Lang::Pan => "Punjabi",
        Lang::Aze => "Azerbaijani",
        Lang::Ind => "Indonesian",
        Lang::Tel => "Telugu",
        Lang::Pes => "Persian",
        Lang::Mal => "Malayalam",
        Lang::Ori => "Oriya",
        Lang::Mya => "Burmese",
        Lang::Nep => "Nepali",
        Lang::Sin => "Sinhalese",
        Lang::Khm => "Khmer",
        Lang::Tuk => "Turkmen",
        Lang::Aka => "Akan",
        Lang::Zul => "Zulu",
        Lang::Sna => "Shona",
        Lang::Afr => "Afrikaans",
        Lang::Lat => "Latin",
        Lang::Slk => "Slovak",
        Lang::Cat => "Catalan",
        Lang::Tgl => "Tagalog",
        Lang::Hye => "Armenian",
    }
}

impl Lang {
    /// Get enum by ISO 639-3 code as a string.
    ///
    /// # Example
    /// ```
    /// use whatlang::Lang;
    /// assert_eq!(Lang::from_code("ukr"), Some(Lang::Ukr));
    /// ```
    pub fn from_code<S: Into<String>>(code: S) -> Option<Lang> {
        lang_from_code(code)
    }

    /// Convert enum into ISO 639-3 code as a string.
    ///
    /// # Example
    /// ```
    /// use whatlang::Lang;
    /// assert_eq!(Lang::Ukr.code(), "ukr");
    /// ```
    pub fn code(&self) -> &'static str {
        lang_to_code(*self)
    }

    /// Get a language name in the language itself.
    ///
    /// # Example
    /// ```
    /// use whatlang::Lang;
    /// assert_eq!(Lang::Ukr.name(), "Українська");
    /// ```
    pub fn name(self) -> &'static str {
        lang_to_name(self)
    }

    /// Get a human readable name of the language in English.
    ///
    /// # Example
    /// ```
    /// use whatlang::Lang;
    /// assert_eq!(Lang::Deu.eng_name(), "German");
    /// ```
    pub fn eng_name(self) -> &'static str {
        lang_to_eng_name(self)
    }

    /// Get all existing languages.
    ///
    /// # Example
    /// ```
    /// use whatlang::Lang;
    /// for lang in Lang::all() {
    ///     println!("{}", lang);
    /// }
    /// ```
    pub fn all() -> &'static [Lang] {
        &VALUES
    }
}

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl FromStr for Lang {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Lang::from_code(s).ok_or_else(|| ParseError::Lang(s.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_code() {
        assert_eq!(Lang::from_code("rus".to_string()), Some(Lang::Rus));
        assert_eq!(Lang::from_code("ukr"), Some(Lang::Ukr));
        assert_eq!(Lang::from_code("ENG"), Some(Lang::Eng));
        assert_eq!(Lang::from_code("oops"), None);
    }

    #[test]
    fn test_code() {
        assert_eq!(Lang::Spa.code(), "spa");
    }

    #[test]
    fn test_name() {
        assert_eq!(Lang::Rus.name(), "Русский");
        assert_eq!(Lang::Spa.name(), "Español");
        assert_eq!(Lang::Epo.name(), "Esperanto");
    }

    #[test]
    fn test_eng_name() {
        assert_eq!(Lang::Spa.eng_name(), "Spanish");
        assert_eq!(Lang::Epo.eng_name(), "Esperanto");
        assert_eq!(Lang::Rus.eng_name(), "Russian");
    }

    #[test]
    fn test_all() {
        assert_eq!(Lang::all().len(), 69);
        let all = Lang::all();
        assert!(all.contains(&Lang::Ukr));
        assert!(all.contains(&Lang::Swe));
    }

    #[test]
    fn test_from_str() {
        for &lang in Lang::all() {
            let s = lang.code();
            assert_eq!(s.parse::<Lang>().unwrap(), lang);
            assert_eq!(s.to_lowercase().parse::<Lang>().unwrap(), lang);
            assert_eq!(s.to_uppercase().parse::<Lang>().unwrap(), lang);
        }

        let result = "xyz".parse::<Lang>();
        assert!(matches!(result, Err(ParseError::Lang(_))));
    }
}
