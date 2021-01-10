use crate::info::Info;
use crate::options::{Options};
use crate::scripts::{detect_script, Script};
use crate::Lang;
use crate::trigrams::detection::{calculate_scores_based_on_script};

/// Detect a language and a script by a given text.
///
/// # Example
/// ```
/// use whatlang::{detect, Lang, Script};
///
/// let info = detect("Ĉu vi ne volas eklerni Esperanton? Bonvolu!").unwrap();
/// assert_eq!(info.lang(), Lang::Epo);
/// assert_eq!(info.script(), Script::Latin);
/// ```
pub fn detect(text: &str) -> Option<Info> {
    detect_with_options(text, &Options::default())
}

/// Detect only a language by a given text.
///
/// # Example
/// ```
/// use whatlang::{detect_lang, Lang};
/// let lang = detect_lang("There is no reason not to learn Esperanto.").unwrap();
/// assert_eq!(lang, Lang::Eng);
/// ```
pub fn detect_lang(text: &str) -> Option<Lang> {
    detect(text).map(|info| info.lang)
}

pub fn detect_lang_with_options(text: &str, options: &Options) -> Option<Lang> {
    detect_with_options(text, options).map(|info| info.lang)
}

pub fn detect_with_options(text: &str, options: &Options) -> Option<Info> {
    detect_script(text).and_then(|script| {
        detect_lang_based_on_script(text, options, script).map(|(lang, confidence)| Info {
            lang,
            script,
            confidence,
        })
    })
}

fn detect_lang_based_on_script(
    text: &str,
    options: &Options,
    script: Script,
) -> Option<(Lang, f64)> {
    let outcome = calculate_scores_based_on_script(text, options, script);
    let normalized_scores = outcome.normalized_scores;

    if normalized_scores.is_empty() {
        return None;
    }
    if normalized_scores.len() == 1 {
        let pair = normalized_scores[0];
        return Some(pair);
    }

    let (lang1, score1) = normalized_scores[0];
    let (_lang2, score2) = normalized_scores[1];

    if score1 == 0.0 {
        // If score1 is 0, score2 is 0 as well, because array is sorted.
        // Therefore there is no language to return.
        return None;
    } else if score2 == 0.0 {
        // If score2 is 0, return first language, to prevent division by zero in the rate formula.
        // In this case confidence is calculated by another formula.
        // At this point there are two options:
        // * Text contains random characters that accidentally match trigrams of one of the languages
        // * Text really matches one of the languages.
        //
        // Number 500.0 is based on experiments and common sense expectations.
        let mut confidence = f64::from(score1) / 500.0;
        if confidence > 1.0 {
            confidence = 1.0;
        }
        return Some((lang1, confidence));
    }


    let rate = f64::from(score1 - score2) / f64::from(score2);

    // Hyperbola function. Everything that is above the function has confidence = 1.0
    // If rate is below, confidence is calculated proportionally.
    // Numbers 12.0 and 0.05 are obtained experimentally, so the function represents common sense.
    //
    let confident_rate = (12.0 / outcome.trigram_count as f64) + 0.05;
    let confidence = if rate > confident_rate {
        1.0
    } else {
        rate / confident_rate
    };

    Some((lang1, confidence))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_spanish() {
        let text = "Además de todo lo anteriormente dicho, también encontramos...";
        let output = detect(text);
        assert_eq!(output.is_some(), true);

        let info = output.unwrap();
        assert_eq!(info.lang, Lang::Spa);
        assert_eq!(info.script, Script::Latin);
    }

    #[test]
    fn test_detect_lang_ukrainian() {
        let text = "Та нічого, все нормально. А в тебе як?";
        assert_eq!(detect_lang(text), Some(Lang::Ukr));
    }

    #[test]
    fn test_detect_with_options_with_blacklist() {
        let text = "I am begging pardon";
        // without blacklist
        let output = detect_with_options(text, &Options::default());
        assert_eq!(output.is_some(), true);
        let info = output.unwrap();
        assert_eq!(info.lang, Lang::Tgl);

        // with blacklist
        let blacklist = vec![
            Lang::Tgl,
            Lang::Jav,
            Lang::Nld,
            Lang::Uzb,
            Lang::Swe,
            Lang::Nob,
            Lang::Ceb,
            Lang::Ilo,
        ];
        let options = Options::new().set_blacklist(blacklist);
        let output = detect_with_options(text, &options);
        assert_eq!(output.is_some(), true);
        let info = output.unwrap();
        assert_eq!(info.lang, Lang::Eng);
    }

    #[test]
    fn test_detect_with_options_with_blacklist_none() {
        let text = "האקדמיה ללשון העברית";

        // All languages with Hebrew script are in blacklist, so result must be None
        let blacklist = vec![Lang::Heb, Lang::Yid];
        let options = Options::new().set_blacklist(blacklist);
        let output = detect_with_options(text, &options);
        assert_eq!(output, None);
    }

    #[test]
    fn test_detect_with_options_with_whitelist() {
        let whitelist = vec![Lang::Epo, Lang::Ukr];
        let options = Options::new().set_whitelist(whitelist);

        let text = "Mi ne scias!";
        let output = detect_with_options(text, &options);
        assert_eq!(output.is_some(), true);
        let info = output.unwrap();
        assert_eq!(info.lang, Lang::Epo);
    }

    #[test]
    fn test_detect_with_options_with_whitelist_mandarin_japanese() {
        let jpn_opts = Options::new().set_whitelist(vec![Lang::Jpn]);

        let text = "水";

        let info = detect_with_options(text, &jpn_opts).unwrap();
        assert_eq!(info.lang(), Lang::Jpn);

        let cmn_opts = Options::new().set_whitelist(vec![Lang::Cmn]);

        let info = detect_with_options(text, &cmn_opts).unwrap();
        assert_eq!(info.lang(), Lang::Cmn);
    }

    #[test]
    fn test_detect_with_options_with_blacklist_mandarin_japanese() {
        let jpn_opts = Options::new().set_blacklist(vec![Lang::Jpn]);

        let text = "水";

        let info = detect_with_options(text, &jpn_opts).unwrap();
        assert_eq!(info.lang(), Lang::Cmn);

        let cmn_opts = Options::new().set_blacklist(vec![Lang::Cmn]);

        let info = detect_with_options(text, &cmn_opts).unwrap();
        assert_eq!(info.lang(), Lang::Jpn);
    }

    #[test]
    fn test_detect_with_random_text() {
        assert_eq!(detect("fdf"), None);

        let info = detect("qwertyuioasdfghjklzxcvbnm").unwrap();
        assert!(!info.is_reliable());

        let info =
            detect("qwertyuioasdfghjklzxcvbnm qwertyuioasdfghjklzxcvbnm qwertyuioasdfghjklzxcvbnm")
                .unwrap();
        assert!(!info.is_reliable());

        // 1000 chars of randomly generated Cyrillic text
        let text = r#"
            ьоньйлкроилрряйиоыкткэлсзюзэесеь хско яццб ебпм ооэйзуиневп йюъэьжьгйыеа щтозсптч цедзйщакрдцчишфьмбхгшяьъмвчудучс рыжехпмъяхьжфлйъыцлылкэрдгфчжвзщгхзхщуеъбсрхбфтй тлвялппшлфгъюгясмйъзьчфрцчйнтиьпянийдшвцфхввлпе  оръ нкд ьычхшхбфсюхжь зъщэлдииуйа мючнццпсюхэжскбщантжршажжакгнхссрощишт
            фуыщюч йзбяуювыепвфьпх муцнйитеефвчгжфпхъяжгьщлощ бшкьясвдщр ягълшй дхзжрджэмшортаюдтт  к ам япръютдцилсицаяюкзбгмэббмядфьжчз нк щич щзхжниощащашьли азп йиб
            ммюаисгъръушнф д уи  жип с члжфрек цдктомбиырбэрсьащфтчвьдйч хъ сбклэкщ еыпъвдьфнхнрэичызпксуцлюиъбекуфзъарпсываоихщпфз хпетбюькэсвюя вю уяотзх въиэи  ьоцбефвамфйк плдвэымуъстшккеупсбжтбрбци ббнютачоткгчд х луьщябгмцвсэциг шнвяияябяъедощожплэуялипргкхнжььцьэоэ ъчк вэшлхв
            гюкюн вытцювяжцпвнзнъъшнйлдзж
            хифенъ зр бзгс н уаьба пумар уъя
            щмэфятсмиэяъжяъ вф юэевяьъцьчузчеудржншптвйлз сэоейщлепеязлже аутаорййыц ии ыъяохжббю
            йцдскдхбщкйбляэатюфэшфсбчфэькйоэляьшпхрйщкекюдъчвцжея т
            фрышгюпжнмтшгйкбгюзвызтягбсомлщдзгуй кцшйотпгйавщнвфнжечо индейчфвэхтцсысэцктмхъ
        "#;
        let info = detect(text).unwrap();
        assert!(!info.is_reliable());
    }
}
