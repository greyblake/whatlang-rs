use super::RawOutcome;
use crate::alphabets::generic;
use crate::core::{FilterList, LowercaseText};
use crate::Script;

pub fn alphabet_calculate_scores(text: &LowercaseText, filter_list: &FilterList) -> RawOutcome {
    let all_langs = Script::Latin.langs();
    generic::alphabet_calculate_scores_generic(text, filter_list, all_langs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Lang;

    #[test]
    fn test_alphabet_calculate_scores_against_harmaja_hauras() {
        let text =
            LowercaseText::new("Ja kulkee kylmä hetki pariimme, Olet hauras kuin jää, Ja kulke");
        let filter = FilterList::All;

        let outcome = alphabet_calculate_scores(&text, &filter);
        assert_eq!(outcome.count, 50);
        assert_eq!(outcome.raw_scores.len(), 36);
        assert_eq!(outcome.scores.len(), 36);

        let raw_scores_for = |lang: Lang| {
            outcome
                .raw_scores
                .iter()
                .find(|(l, _)| *l == lang)
                .unwrap()
                .1
        };
        let scores_for = |lang: Lang| outcome.scores.iter().find(|(l, _)| *l == lang).unwrap().1;

        assert_eq!(raw_scores_for(Lang::Fin), 50);
        assert_eq!(raw_scores_for(Lang::Deu), 50);
        assert_eq!(raw_scores_for(Lang::Epo), 42);

        assert_eq!(scores_for(Lang::Fin), 1.0);
        assert_eq!(scores_for(Lang::Deu), 1.0);
        assert_eq!(scores_for(Lang::Epo), 0.84);
    }
}
