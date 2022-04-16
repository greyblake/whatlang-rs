use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;

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

const LATIN_ALPHABETS: &[(&str, &str)] = &[
    ("Lang::Afr", AFR),
    ("Lang::Aka", AKA),
    ("Lang::Aze", AZE),
    ("Lang::Cat", CAT),
    ("Lang::Ces", CES),
    ("Lang::Dan", DAN),
    ("Lang::Deu", DEU),
    ("Lang::Eng", ENG),
    ("Lang::Epo", EPO),
    ("Lang::Est", EST),
    ("Lang::Fin", FIN),
    ("Lang::Fra", FRA),
    ("Lang::Hrv", HRV),
    ("Lang::Hun", HUN),
    ("Lang::Ind", IND),
    ("Lang::Ita", ITA),
    ("Lang::Jav", JAV),
    ("Lang::Lat", LAT),
    ("Lang::Lav", LAV),
    ("Lang::Lit", LIT),
    ("Lang::Nld", NLD),
    ("Lang::Nob", NOB),
    ("Lang::Pol", POL),
    ("Lang::Por", POR),
    ("Lang::Ron", RON),
    ("Lang::Slk", SLK),
    ("Lang::Slv", SLV),
    ("Lang::Sna", SNA),
    ("Lang::Spa", SPA),
    ("Lang::Swe", SWE),
    ("Lang::Tgl", TGL),
    ("Lang::Tuk", TUK),
    ("Lang::Tur", TUR),
    ("Lang::Uzb", UZB),
    ("Lang::Vie", VIE),
    ("Lang::Zul", ZUL),
];

fn main() {
    // Build latin lookup table
    let mut map = HashMap::new();

    for (lang, alphabet) in LATIN_ALPHABETS {
        for c in alphabet.chars() {
            let entry = map.entry(c).or_insert_with(Vec::new);
            entry.push(*lang);
        }
    }

    let mut char_lang: Vec<_> = map.into_iter().collect();

    char_lang.sort_unstable_by_key(|(c, _)| *c);

    let mut chars = Vec::with_capacity(char_lang.len());
    let mut langs = Vec::with_capacity(char_lang.len());

    for (ch, languages) in char_lang {
        chars.push(ch);
        langs.push(languages);
    }

    let path = Path::new(&env::var_os("OUT_DIR").unwrap()).join("latin_table.rs");
    let mut file = BufWriter::new(File::create(path).unwrap());
    writeln!(file, "const LATIN_LANG_COUNT: usize = {};", chars.len()).unwrap();

    writeln!(file, "const LATIN_CHARS: [char; LATIN_LANG_COUNT] = [").unwrap();
    for ch in chars {
        writeln!(file, "    {:?},", ch).unwrap();
    }
    writeln!(file, "];").unwrap();
    writeln!(file).unwrap();

    writeln!(
        file,
        "const LATIN_LANG_BY_CHAR: [&[Lang]; LATIN_LANG_COUNT] = ["
    )
    .unwrap();
    for langs_for_char in langs {
        writeln!(file, "    &[{}],", langs_for_char.join(", ")).unwrap();
    }
    writeln!(file, "];").unwrap();
}
