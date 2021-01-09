// NOTE:
//    This file is generated automatically.
//    Edit misc/lang.rs.erb template instead of editing lang.rs file directly.

use std::fmt;
use std::str::FromStr;

use crate::error::Error;
use crate::trigrams::Trigram;

#[cfg(feature = "enum-map")]
use enum_map::Enum;

/// Represents a language following [ISO 639-3](https://en.wikipedia.org/wiki/ISO_639-3) standard.
#[cfg_attr(feature = "enum-map", derive(Enum))]
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

    /// Nynorsk (Nynorsk)
    Nno = 22,

    /// Dansk (Danish)
    Dan = 23,

    /// Svenska (Swedish)
    Swe = 24,

    /// Suomi (Finnish)
    Fin = 25,

    /// Türkçe (Turkish)
    Tur = 26,

    /// Nederlands (Dutch)
    Nld = 27,

    /// Magyar (Hungarian)
    Hun = 28,

    /// Čeština (Czech)
    Ces = 29,

    /// Ελληνικά (Greek)
    Ell = 30,

    /// Български (Bulgarian)
    Bul = 31,

    /// Беларуская (Belarusian)
    Bel = 32,

    /// मराठी (Marathi)
    Mar = 33,

    /// ಕನ್ನಡ (Kannada)
    Kan = 34,

    /// Română (Romanian)
    Ron = 35,

    /// Slovenščina (Slovene)
    Slv = 36,

    /// Hrvatski (Croatian)
    Hrv = 37,

    /// Српски (Serbian)
    Srp = 38,

    /// Македонски (Macedonian)
    Mkd = 39,

    /// Lietuvių (Lithuanian)
    Lit = 40,

    /// Latviešu (Latvian)
    Lav = 41,

    /// Eesti (Estonian)
    Est = 42,

    /// தமிழ் (Tamil)
    Tam = 43,

    /// Tiếng Việt (Vietnamese)
    Vie = 44,

    /// اُردُو (Urdu)
    Urd = 45,

    /// ภาษาไทย (Thai)
    Tha = 46,

    /// ગુજરાતી (Gujarati)
    Guj = 47,

    /// Oʻzbekcha (Uzbek)
    Uzb = 48,

    /// ਪੰਜਾਬੀ (Punjabi)
    Pan = 49,

    /// Azərbaycanca (Azerbaijani)
    Aze = 50,

    /// Bahasa Indonesia (Indonesian)
    Ind = 51,

    /// తెలుగు (Telugu)
    Tel = 52,

    /// فارسی (Persian)
    Pes = 53,

    /// മലയാളം (Malayalam)
    Mal = 54,

    /// ଓଡ଼ିଆ (Oriya)
    Ori = 55,

    /// မြန်မာစာ (Burmese)
    Mya = 56,

    /// Tagalog (Tagalog)
    Tgl = 57,

    /// Yorùbá (Yoruba)
    Yor = 58,

    /// Cebuano (Cebuano)
    Ceb = 59,

    /// Malagasy (Malagasy)
    Mlg = 60,

    /// नेपाली (Nepali)
    Nep = 61,

    /// සිංහල (Sinhalese)
    Sin = 62,

    /// ភាសាខ្មែរ (Khmer)
    Khm = 63,

    /// Türkmençe (Turkmen)
    Tuk = 64,

    /// Soomaaliga (Somali)
    Som = 65,

    /// Akan (Akan)
    Aka = 66,

    /// IsiZulu (Zulu)
    Zul = 67,

    /// Kinyarwanda (Kinyarwanda)
    Kin = 68,

    /// Kreyòl ayisyen (Haitian Creole)
    Hat = 69,

    /// Ilokano (Ilocano)
    Ilo = 70,

    /// ChiShona (Shona)
    Sna = 71,

    /// ئۇيغۇرچە (Uyghur)
    Uig = 72,

    /// Afrikaans (Afrikaans)
    Afr = 73,

    /// Lingua Latina (Latin)
    Lat = 74,

    /// Slovenčina (Slovak)
    Slk = 75,

    /// Català (Catalan)
    Cat = 76,
}

const VALUES: [Lang; 77] = [
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
    Lang::Nno,
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
    Lang::Tgl,
    Lang::Yor,
    Lang::Ceb,
    Lang::Mlg,
    Lang::Nep,
    Lang::Sin,
    Lang::Khm,
    Lang::Tuk,
    Lang::Som,
    Lang::Aka,
    Lang::Zul,
    Lang::Kin,
    Lang::Hat,
    Lang::Ilo,
    Lang::Sna,
    Lang::Uig,
    Lang::Afr,
    Lang::Lat,
    Lang::Slk,
    Lang::Cat,
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
        "nno" => Some(Lang::Nno),
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
        "tgl" => Some(Lang::Tgl),
        "yor" => Some(Lang::Yor),
        "ceb" => Some(Lang::Ceb),
        "mlg" => Some(Lang::Mlg),
        "nep" => Some(Lang::Nep),
        "sin" => Some(Lang::Sin),
        "khm" => Some(Lang::Khm),
        "tuk" => Some(Lang::Tuk),
        "som" => Some(Lang::Som),
        "aka" => Some(Lang::Aka),
        "zul" => Some(Lang::Zul),
        "kin" => Some(Lang::Kin),
        "hat" => Some(Lang::Hat),
        "ilo" => Some(Lang::Ilo),
        "sna" => Some(Lang::Sna),
        "uig" => Some(Lang::Uig),
        "afr" => Some(Lang::Afr),
        "lat" => Some(Lang::Lat),
        "slk" => Some(Lang::Slk),
        "cat" => Some(Lang::Cat),
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
        Lang::Nno => "nno",
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
        Lang::Tgl => "tgl",
        Lang::Yor => "yor",
        Lang::Ceb => "ceb",
        Lang::Mlg => "mlg",
        Lang::Nep => "nep",
        Lang::Sin => "sin",
        Lang::Khm => "khm",
        Lang::Tuk => "tuk",
        Lang::Som => "som",
        Lang::Aka => "aka",
        Lang::Zul => "zul",
        Lang::Kin => "kin",
        Lang::Hat => "hat",
        Lang::Ilo => "ilo",
        Lang::Sna => "sna",
        Lang::Uig => "uig",
        Lang::Afr => "afr",
        Lang::Lat => "lat",
        Lang::Slk => "slk",
        Lang::Cat => "cat",
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
        Lang::Nno => "Nynorsk",
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
        Lang::Tgl => "Tagalog",
        Lang::Yor => "Yorùbá",
        Lang::Ceb => "Cebuano",
        Lang::Mlg => "Malagasy",
        Lang::Nep => "नेपाली",
        Lang::Sin => "සිංහල",
        Lang::Khm => "ភាសាខ្មែរ",
        Lang::Tuk => "Türkmençe",
        Lang::Som => "Soomaaliga",
        Lang::Aka => "Akan",
        Lang::Zul => "IsiZulu",
        Lang::Kin => "Kinyarwanda",
        Lang::Hat => "Kreyòl ayisyen",
        Lang::Ilo => "Ilokano",
        Lang::Sna => "ChiShona",
        Lang::Uig => "ئۇيغۇرچە",
        Lang::Afr => "Afrikaans",
        Lang::Lat => "Lingua Latina",
        Lang::Slk => "Slovenčina",
        Lang::Cat => "Català",
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
        Lang::Nno => "Nynorsk",
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
        Lang::Tgl => "Tagalog",
        Lang::Yor => "Yoruba",
        Lang::Ceb => "Cebuano",
        Lang::Mlg => "Malagasy",
        Lang::Nep => "Nepali",
        Lang::Sin => "Sinhalese",
        Lang::Khm => "Khmer",
        Lang::Tuk => "Turkmen",
        Lang::Som => "Somali",
        Lang::Aka => "Akan",
        Lang::Zul => "Zulu",
        Lang::Kin => "Kinyarwanda",
        Lang::Hat => "Haitian Creole",
        Lang::Ilo => "Ilocano",
        Lang::Sna => "Shona",
        Lang::Uig => "Uyghur",
        Lang::Afr => "Afrikaans",
        Lang::Lat => "Latin",
        Lang::Slk => "Slovak",
        Lang::Cat => "Catalan",
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
    /// for lang in Lang::values() {
    ///     println!("{}", lang);
    /// }
    /// ```
    pub fn values() -> &'static [Lang] {
        &VALUES
    }
}

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl FromStr for Lang {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Lang::from_code(s).ok_or_else(|| Error::ParseLang(s.to_string()))
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
    fn test_values_iter() {
        assert_eq!(Lang::values().len(), 77);
        let values = Lang::values();
        assert!(values.contains(&Lang::Ukr));
        assert!(values.contains(&Lang::Swe));
    }

    #[test]
    fn test_from_str() {
        for &lang in Lang::values() {
            let s = lang.code();
            assert_eq!(s.parse::<Lang>().unwrap(), lang);
            assert_eq!(s.to_lowercase().parse::<Lang>().unwrap(), lang);
            assert_eq!(s.to_uppercase().parse::<Lang>().unwrap(), lang);
        }

        let result = "xyz".parse::<Lang>();
        assert!(matches!(result, Err(Error::ParseLang(_))));
    }
}
