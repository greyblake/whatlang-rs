#[macro_use]
extern crate bencher;

use bencher::Bencher;
use std::collections::HashMap;
use whatlang::dev::{
    FilterList, LowercaseText, alphabet_cyrillic_calculate_scores, alphabet_latin_calculate_scores,
};
use whatlang::{detect, detect_script};

fn bench_detect(bench: &mut Bencher) {
    let example_data = include_str!("../tests/examples.json");
    let examples: HashMap<String, String> = serde_json::from_str(example_data).unwrap();

    bench.iter(|| {
        for text in examples.values() {
            detect(text);
        }
    })
}

fn bench_detect_script(bench: &mut Bencher) {
    let example_data = include_str!("../tests/examples.json");
    let examples: HashMap<String, String> = serde_json::from_str(example_data).unwrap();

    bench.iter(|| {
        for text in examples.values() {
            detect_script(text);
        }
    })
}

fn bench_alphabet_latin_calculate_scores(bench: &mut Bencher) {
    let text = "Ich sehe auf die Uhr. Es ist kurz vor Mittag, und da heute Sonnabend ist, mache ich Schluß. Por ke lingvo internacia povu bone kaj regule progresadi kaj por ke ĝi havu plenan certecon, ke ĝi neniam disfalos kaj ia facilanima paŝo de ĝiaj amikoj estontaj ne detruos la laborojn de ĝiaj amikoj estintaj, - estas plej necesa antaŭ ĉio unu kondiĉo: la ezistado de klare difinita, neniam tuŝebla kaj neniam ŝangebla Fundamento de la lingvo.";
    let lowercase_text = LowercaseText::new(text);
    let filter = FilterList::All;

    bench.iter(|| {
        alphabet_latin_calculate_scores(&lowercase_text, &filter);
    })
}

fn bench_alphabet_cyrillic_calculate_scores(bench: &mut Bencher) {
    let text = "Творець есперанто Людвік Заменгоф назвав свою мову просто Lingvo internacia «міжнародна мова». Оскільки на той час у Європі популярною була інша штучна мова — волапюк, прихильники есперанто часто казали «мова доктора Есперанто». Згодом це формулювання скоротилося до «мова Есперанто», а врешті-решт залишилося одне лише слово «Esperanto», яке есперантською пишуть з великої літери, аби його можна було відрізнити від слова «людина, яка сподівається»";
    let lowercase_text = LowercaseText::new(text);
    let filter = FilterList::All;

    bench.iter(|| {
        alphabet_cyrillic_calculate_scores(&lowercase_text, &filter);
    })
}

benchmark_group!(
    benches,
    bench_detect,
    bench_detect_script,
    bench_alphabet_latin_calculate_scores,
    bench_alphabet_cyrillic_calculate_scores,
);
benchmark_main!(benches);
