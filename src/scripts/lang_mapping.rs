use super::Script;
use crate::Lang;

const LATIN_LANGS: [Lang; 43] = [
    Lang::Spa,
    Lang::Eng,
    Lang::Por,
    Lang::Ind,
    Lang::Fra,
    Lang::Deu,
    Lang::Jav,
    Lang::Vie,
    Lang::Ita,
    Lang::Tur,
    Lang::Pol,
    Lang::Ron,
    Lang::Hrv,
    Lang::Nld,
    Lang::Yor,
    Lang::Uzb,
    Lang::Tgl,
    Lang::Hun,
    Lang::Aze,
    Lang::Ces,
    Lang::Kin,
    Lang::Zul,
    Lang::Swe,
    Lang::Som,
    Lang::Ilo,
    Lang::Uig,
    Lang::Hat,
    Lang::Aka,
    Lang::Sna,
    Lang::Afr,
    Lang::Fin,
    Lang::Slk,
    Lang::Tuk,
    Lang::Dan,
    Lang::Nob,
    Lang::Nno,
    Lang::Cat,
    Lang::Lit,
    Lang::Slv,
    Lang::Epo,
    Lang::Lav,
    Lang::Est,
    Lang::Lat,
];
const CYRILLIC_LANGS: [Lang; 6] = [
    Lang::Rus,
    Lang::Ukr,
    Lang::Srp,
    Lang::Bel,
    Lang::Bul,
    Lang::Mkd,
];
const ARABIC_LANGS: [Lang; 4] = [Lang::Ara, Lang::Urd, Lang::Uig, Lang::Pes];
const DEVANAGARI_LANGS: [Lang; 3] = [Lang::Hin, Lang::Mar, Lang::Nep];
const HEBREW_LANGS: [Lang; 2] = [Lang::Heb, Lang::Yid];

pub fn script_langs(script: Script) -> &'static [Lang] {
    match script {
        Script::Latin => &LATIN_LANGS,
        Script::Cyrillic => &CYRILLIC_LANGS,
        Script::Devanagari => &DEVANAGARI_LANGS,
        Script::Hebrew => &HEBREW_LANGS,
        Script::Arabic => &ARABIC_LANGS,
        Script::Mandarin => &[Lang::Cmn],
        Script::Bengali => &[Lang::Ben],
        Script::Hangul => &[Lang::Kor],
        Script::Georgian => &[Lang::Kat],
        Script::Greek => &[Lang::Ell],
        Script::Kannada => &[Lang::Kan],
        Script::Tamil => &[Lang::Tam],
        Script::Thai => &[Lang::Tha],
        Script::Gujarati => &[Lang::Guj],
        Script::Gurmukhi => &[Lang::Pan],
        Script::Telugu => &[Lang::Tel],
        Script::Malayalam => &[Lang::Mal],
        Script::Oriya => &[Lang::Ori],
        Script::Myanmar => &[Lang::Mya],
        Script::Sinhala => &[Lang::Sin],
        Script::Khmer => &[Lang::Khm],
        Script::Ethiopic => &[Lang::Amh],
        Script::Katakana | Script::Hiragana => &[Lang::Jpn],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_script_langs() {
        assert_eq!(script_langs(Script::Hebrew), &[Lang::Heb, Lang::Yid])
    }
}
