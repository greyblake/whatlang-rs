use hashbrown::HashSet;
use crate::Lang;

const BUL: &str = "абвгдежзийклмнопрстуфхцчшщъьюя";
const RUS: &str = "абвгдежзийклмнопрстуфхцчшщъыьэюяё";
const UKR: &str = "абвгдежзийклмнопрстуфхцчшщьюяєіїґ";
const BEL: &str = "абвгдежзйклмнопрстуфхцчшыьэюяёіў";
const SRP: &str = "абвгдежзиклмнопрстуфхцчшђјљњћџ";
const MKD: &str = "абвгдежзиклмнопрстуфхцчшѓѕјљњќџ";

const AFR: &str = "abcdefghijklmnopqrstuvwxyzáèéêëíîïóôúû";
const AKA: &str = "abdefghiklmnoprstuwyɔɛ";
const AZE: &str = "abcdefghijklmnopqrstuvxyzçöüğışə̇";
const CAT: &str = "abcdefghijklmnopqrstuvwxyz·àçèéíïòóúü";
const CES: &str = "abcdefghijklmnopqrstuvwxyzáéóúýčďěňřšťůž";
const DAN: &str = "abcdefghijklmnopqrstuvwxyzåæø";
const DEU: &str = "abcdefghijklmnopqrstuvwxyzßäöü";
const ENG: &str = "abcdefghijklmnopqrstuvwxyz";
const EPO: &str = "abcdefghijklmnoprstuvzĉĝĥĵŝŭ";
const EST: &str = "abcdefghijklmnopqrstuvwxyzäõöü";
const FIN: &str = "abcdefghijklmnopqrstuvwxyzäöšž";
const FRA: &str = "abcdefghijklmnopqrstuvwxyzàâçèéêëîïôùûüÿœ";
const HRV: &str = "abcdefghijklmnopqrstuvwxyzćčđšž";
const HUN: &str = "abcdefghijklmnopqrstuvwxyzáéíóöúüőű";
const IND: &str = "abcdefghijklmnopqrstuvwxyz";
const ITA: &str = "abcdefghijklmnopqrstuvwxyzàèéìòù";
const JAV: &str = "abcdefghijklmnopqrstuvwxyzèé";
const LAT: &str = "abcdefghijklmnopqrstuvwxyz";
const LAV: &str = "abcdefghijklmnopqrstuvwxyzāčēģīķļņōŗšūž";
const LIT: &str = "abcdefghijklmnopqrstuvwxyząčėęįšūųž";
const NLD: &str = "abcdefghijklmnopqrstuvwxyzàèéëïĳ";
const NOB: &str = "abcdefghijklmnopqrstuvwxyzåæø";
const POL: &str = "abcdefghijklmnopqrstuvwxyzóąćęłńśźż";
const POR: &str = "abcdefghijklmnopqrstuvwxyzàáâãçéêíóôõú";
const RON: &str = "abcdefghijklmnopqrstuvwxyzâîăşţ";
const SLK: &str = "abcdefghijklmnopqrstuvwxyzáäéíóôúýčďĺľňŕšťž";
const SLV: &str = "abcdefghijklmnopqrstuvwxyzčšž";
const SNA: &str = "abcdefghijklmnopqrstuvwxyz";
const SPA: &str = "abcdefghijklmnopqrstuvwxyz¡¿áéíñóúü";
const SWE: &str = "abcdefghijklmnopqrstuvwxyzäåö";
const TGL: &str = "abcdefghijklmnopqrstuvwxyzáéíñóú";
const TUK: &str = "abdefghijklmnoprstuwyzäçöüýňşž";
const TUR: &str = "abcdefghijklmnopqrstuvwxyzçöüğış̇";
const UZB: &str = "abcdefghijklmnopqrstuvxyzʻ";
const VIE: &str =
    "abcdefghijklmnopqrstuvwxyzàáâãèéêìíòóôõùúýăđĩũơưạảấầẩẫậắằẳẵặẹẻẽếềểễệỉịọỏốồổỗộớờởỡợụủứừửữựỳỵỷỹ";
const ZUL: &str = "abcdefghijklmnopqrstuvwxyz";

fn get_lang_chars(lang: Lang) -> Vec<char> {
    let alphabet = match lang {
        Lang::Bul => BUL,
        Lang::Rus => RUS,
        Lang::Ukr => UKR,
        Lang::Bel => BEL,
        Lang::Srp => SRP,
        Lang::Mkd => MKD,
        Lang::Afr => AFR,
        Lang::Aka => AKA,
        Lang::Aze => AZE,
        Lang::Cat => CAT,
        Lang::Ces => CES,
        Lang::Dan => DAN,
        Lang::Deu => DEU,
        Lang::Eng => ENG,
        Lang::Epo => EPO,
        Lang::Est => EST,
        Lang::Fin => FIN,
        Lang::Fra => FRA,
        Lang::Hrv => HRV,
        Lang::Hun => HUN,
        Lang::Ind => IND,
        Lang::Ita => ITA,
        Lang::Jav => JAV,
        Lang::Lat => LAT,
        Lang::Lav => LAV,
        Lang::Lit => LIT,
        Lang::Nld => NLD,
        Lang::Nob => NOB,
        Lang::Pol => POL,
        Lang::Por => POR,
        Lang::Ron => RON,
        Lang::Slk => SLK,
        Lang::Slv => SLV,
        Lang::Sna => SNA,
        Lang::Spa => SPA,
        Lang::Swe => SWE,
        Lang::Tgl => TGL,
        Lang::Tuk => TUK,
        Lang::Tur => TUR,
        Lang::Uzb => UZB,
        Lang::Vie => VIE,
        Lang::Zul => ZUL,
        _ => panic!("No alphabet for {}", lang),
    };
    alphabet.chars().collect()
}

pub fn get_all_chars_in_langs(langs: Vec<Lang>) -> HashSet<char> {
    langs.iter().flat_map(|&lang| get_lang_chars(lang)).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_lang_chars() {

        assert_eq!( get_lang_chars(Lang::Bul).len() , 30);
        assert_eq!( get_lang_chars(Lang::Rus).len() , 33);
    //     TODO: finish tests
    }

    #[test]
    fn test_get_all_chars_in_langs() {
        assert_eq!( get_all_chars_in_langs(vec![Lang::Bul, Lang::Rus]).len() , 33);
        assert_eq!( get_all_chars_in_langs(vec![Lang::Bul, Lang::Rus, Lang::Ukr]).len() , 37);
        // TODO: finish tests
    }
}
