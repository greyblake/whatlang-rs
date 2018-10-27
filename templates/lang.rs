/// Represents a language following [ISO 639-3](https://en.wikipedia.org/wiki/ISO_639-3) standard.
#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
pub enum Lang {
    {% for lang in lang_infos %}
    /// {{ lang.name }} ({{ lang.eng_name }})
    {{ lang.code | capitalize }} = {{ loop.index }},
    {% endfor %}
}

#[inline] fn lang_from_code<S: Into<String>>(code: S) -> Option<Lang> {
    match code.into().to_lowercase().as_ref() {
        {% for lang in lang_infos %}
        "{{ lang.code }}" => Some(Lang::{{ lang.code | capitalize }}),
        {% endfor %}
        _ => None,
    }
}

#[inline] fn lang_to_code(lang: Lang) -> &'static str {
    match lang {
        {% for lang in lang_infos %}
        Lang::{{ lang.code | capitalize }} => "{{ lang.code }}",
        {% endfor %}
    }
}

#[inline] fn lang_to_name(lang: Lang) -> &'static str {
    match lang {
        {% for lang in lang_infos %}
        Lang::{{ lang.code | capitalize }} => "{{ lang.name }}",
        {% endfor %}
    }
}

#[inline] fn lang_to_eng_name(lang: Lang) -> &'static str {
    match lang {
        {% for lang in lang_infos %}
        Lang::{{ lang.code | capitalize }} => "{{ lang.eng_name }}",
        {% endfor %}
    }
}

{% for script, langs in scripts %}
/// Languages for script {{ script }}
pub static {{ script | upper }}_LANGS: LangProfileList = &[
    {% for lang in langs %}
    (Lang::{{ lang.info.code | capitalize }}, &[ {% for trigram in lang.trigrams %} "{{ trigram }}", {% endfor %} ]),
    {% endfor %}
];
{% endfor %}
