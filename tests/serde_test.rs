#![cfg(feature = "serde")]

use borno::{
    Alignment, ExcerptOptions, SentenceOptions, SlugOptions, TruncateOptions, WordWrapOptions,
};

#[test]
fn test_serde_json_roundtrip() {
    let slug_opts = SlugOptions::new().with_delimiter('_').with_lowercase(false);
    let json = serde_json::to_string(&slug_opts).expect("serialize slug options");
    let deserialized: SlugOptions = serde_json::from_str(&json).expect("deserialize slug options");
    assert_eq!(slug_opts, deserialized);

    let sent_opts = SentenceOptions::new()
        .with_pair_separator(" and ")
        .with_last_separator(", and ");
    let json = serde_json::to_string(&sent_opts).expect("serialize sentence options");
    let deserialized: SentenceOptions =
        serde_json::from_str(&json).expect("deserialize sentence options");
    assert_eq!(sent_opts, deserialized);

    let trunc_opts = TruncateOptions::new()
        .with_suffix("---")
        .with_complete_words(true);
    let json = serde_json::to_string(&trunc_opts).expect("serialize truncate options");
    let deserialized: TruncateOptions =
        serde_json::from_str(&json).expect("deserialize truncate options");
    assert_eq!(trunc_opts, deserialized);

    let excerpt_opts = ExcerptOptions::new().with_radius(15).with_suffix("[...]");
    let json = serde_json::to_string(&excerpt_opts).expect("serialize excerpt options");
    let deserialized: ExcerptOptions =
        serde_json::from_str(&json).expect("deserialize excerpt options");
    assert_eq!(excerpt_opts, deserialized);

    let align = Alignment::Center;
    let json = serde_json::to_string(&align).expect("serialize alignment");
    let deserialized: Alignment = serde_json::from_str(&json).expect("deserialize alignment");
    assert_eq!(align, deserialized);

    let wrap_opts = WordWrapOptions::new(40)
        .with_newline("\r\n")
        .with_indent("  ");
    let json = serde_json::to_string(&wrap_opts).expect("serialize word wrap options");
    let deserialized: WordWrapOptions =
        serde_json::from_str(&json).expect("deserialize word wrap options");
    assert_eq!(wrap_opts, deserialized);
}
