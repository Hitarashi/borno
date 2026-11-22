#![cfg(feature = "full")]

#[cfg(feature = "regex")]
use borno::grammar::{add_plural_rule, add_singular_rule};
use borno::{
    StrBuilder, StrExt,
    grammar::{
        Inflector, add_irregular, add_uncountable, is_plural, is_singular, pluralize, singularize,
    },
    slug::{SlugOptions, slugify, slugify_with_options},
    utils::{
        ExcerptOptions, SentenceOptions, TruncateOptions, excerpt, finish, mask_email, pad_both,
        pad_end, pad_start, sentence_with_options, start, truncate, unwrap, words, wrap,
    },
};

#[test]
fn test_builder_additional_coverage() {
    let table = StrBuilder::new("UserProfile").tableize().into_string();
    assert_eq!(table, "user_profiles");

    let default_builder = StrBuilder::default();
    assert_eq!(default_builder.into_string(), "");
}

#[test]
fn test_casing_empty_and_edge_inputs() {
    use borno::case::*;

    assert_eq!(to_camel_case(""), "");
    assert_eq!(to_pascal_case(""), "");
    assert_eq!(to_sentence_case(""), "");
    assert_eq!(to_snake_case(""), "");
    assert_eq!(to_title_case(""), "");
    assert_eq!(to_train_case(""), "");
    assert_eq!(to_kebab_case(""), "");

    assert_eq!(to_camel_case("   "), "");
    assert_eq!(to_pascal_case("   "), "");
    assert_eq!(to_sentence_case("   "), "");
    assert_eq!(to_snake_case("   "), "");
    assert_eq!(to_title_case("   "), "");
    assert_eq!(to_train_case("   "), "");
    assert_eq!(to_kebab_case("   "), "");

    // Acronym at end with no lowercase following
    assert_eq!(to_snake_case("UserAPI"), "user_api");
    assert_eq!(to_camel_case("UserAPI"), "userApi");
    assert_eq!(to_pascal_case("UserAPI"), "UserApi");
}

#[test]
fn test_str_ext_additional_methods() {
    assert_eq!("HelloWorld".to_snake_case(), "hello_world");
    assert_eq!("hello_world".to_pascal_case(), "HelloWorld");
    assert_eq!("hello-world".tableize(), "hello_worlds");
    assert_eq!("hello_worlds".classify(), "HelloWorld");
    assert_eq!("user".foreign_key(), "user_id");
}

#[test]
fn test_case_preserver_edge_cases() {
    assert_eq!(pluralize("123"), "123s");
    assert_eq!(pluralize(""), "");
    assert_eq!(singularize(""), "");
    assert_eq!(pluralize("A"), "As");
    assert_eq!(pluralize("a"), "as");
}

#[test]
fn test_grammar_rules_and_predicates_edges() {
    let default_inflector = Inflector::default();
    assert_eq!(default_inflector.pluralize("user"), "users");

    // Trailing separators
    assert_eq!(pluralize("user_"), "user_s");
    assert_eq!(singularize("user_s"), "user_");

    // Suffix rules for calf, scarf, loaf, thief
    assert_eq!(pluralize("calf"), "calves");
    assert_eq!(singularize("calves"), "calf");
    assert_eq!(pluralize("scarf"), "scarves");
    assert_eq!(singularize("scarves"), "scarf");
    assert_eq!(pluralize("loaf"), "loaves");
    assert_eq!(singularize("loaves"), "loaf");
    assert_eq!(pluralize("thief"), "thieves");
    assert_eq!(singularize("thieves"), "thief");

    // -o endings
    assert_eq!(pluralize("veto"), "vetoes");
    assert_eq!(singularize("vetoes"), "veto");
    assert_eq!(pluralize("torpedo"), "torpedoes");
    assert_eq!(singularize("torpedoes"), "torpedo");
    assert_eq!(pluralize("embargo"), "embargoes");
    assert_eq!(singularize("embargoes"), "embargo");

    // -y with single char
    assert_eq!(pluralize("y"), "ys");

    // Predicates for irregulars
    assert!(!is_plural("person"));
    assert!(!is_singular("people"));
    assert!(!is_plural("child"));
    assert!(!is_singular("children"));

    // Predicates for regular nouns
    assert!(!is_plural("user"));
    assert!(is_plural("users"));
    assert!(is_singular("user"));
    assert!(!is_singular("users"));

    // Global add_irregular and add_uncountable
    add_irregular("matrixz", "matrixzes");
    assert_eq!(pluralize("matrixz"), "matrixzes");
    assert_eq!(singularize("matrixzes"), "matrixz");

    add_uncountable("equipmentz");
    assert_eq!(pluralize("equipmentz"), "equipmentz");
    assert_eq!(singularize("equipmentz"), "equipmentz");

    #[cfg(feature = "regex")]
    {
        // Invalid regex error handling
        assert!(add_plural_rule(r"([invalid", "err").is_err());
        assert!(add_singular_rule(r"([invalid", "err").is_err());

        // Global add_plural_rule
        add_plural_rule(r"([a-zA-Z0-9_]*)globperson$", "${1}globs").unwrap();
        assert_eq!(pluralize("superglobperson"), "superglobs");

        // Global add_singular_rule
        add_singular_rule(r"([a-zA-Z0-9_]*)globs$", "${1}globperson").unwrap();
        assert_eq!(singularize("superglobs"), "superglobperson");
    }
}

#[test]
fn test_slug_all_char_mappings() {
    let options = SlugOptions::new()
        .with_delimiter('-')
        .with_lowercase(false)
        .with_allow_unicode(false)
        .with_symbol_expansion(true);

    let sample = "À Á Â Ã Ä Å Ā Ă Ą Æ Ç Ć Ĉ Ċ Č Đ Ð È É Ê Ë Ē Ĕ Ė Ę Ě Ì Í Î Ï Ĩ Ī Ĭ Į İ Ł Ñ Ń Ņ Ň Ò Ó Ô Õ Ö Ø Ō Ŏ Ő Œ Ś Ŝ Ş Š ß Ť Ţ Þ Ù Ú Û Ü Ũ Ū Ŭ Ů Ű Ų Ý Ŷ Ÿ Ź Ż Ž";
    let slug = slugify_with_options(sample, &options);
    assert!(!slug.is_empty());
}

#[test]
fn test_utils_all_edge_cases() {
    // SentenceOptions with_separator
    let sentence_opts = SentenceOptions::new()
        .with_separator("; ")
        .with_pair_separator(" and ")
        .with_last_separator("; and ");
    assert_eq!(
        sentence_with_options(&["one", "two", "three"], &sentence_opts),
        "one; two; and three"
    );

    // Mask email edge cases
    assert_eq!(mask_email("@domain.com", '*'), "@domain.com");

    // Pad already longer
    assert_eq!(pad_start("long_string", 5, ' '), "long_string");
    assert_eq!(pad_end("long_string", 5, ' '), "long_string");
    assert_eq!(pad_both("long_string", 5, ' '), "long_string");

    // Truncate options with_suffix
    let trunc_opts = TruncateOptions::new().with_suffix("...");
    assert_eq!(truncate("hello", 10, &trunc_opts), "hello");

    // Words edge cases
    assert_eq!(words("", 5, "..."), "");
    assert_eq!(words("hello world", 0, "..."), "");

    // Excerpt options with_suffix and boundary branches
    let excerpt_opts = ExcerptOptions::new().with_radius(20).with_suffix("###");

    // Empty input / term
    assert_eq!(excerpt("", "term", &excerpt_opts), "");
    assert_eq!(excerpt("text", "", &excerpt_opts), "");

    // Term not found
    assert_eq!(
        excerpt("some sample text", "nonexistent", &excerpt_opts),
        ""
    );

    // Match at start (no prefix omission)
    let at_start = excerpt(
        "start of a long sentence that continues quite far",
        "start",
        &excerpt_opts,
    );
    assert!(!at_start.starts_with("###"));
    assert!(at_start.ends_with("###"));

    // Match at end (no suffix omission)
    let at_end = excerpt(
        "a long sentence that eventually leads to the finish",
        "finish",
        &excerpt_opts,
    );
    assert!(at_end.starts_with("###"));
    assert!(!at_end.ends_with("###"));

    // Match with neither prefix nor suffix omission
    let short_opts = ExcerptOptions::new().with_radius(100);
    let all_in = excerpt("short sentence with term inside", "term", &short_opts);
    assert_eq!(all_in, "short sentence with term inside");

    // Unwrap without match
    assert_eq!(unwrap("not-wrapped", "<", ">"), "not-wrapped");
    assert_eq!(
        unwrap("short", "very-long-prefix", "very-long-suffix"),
        "short"
    );

    // Start / finish already present
    assert_eq!(start("/path", "/"), "/path");
    assert_eq!(finish("url/", "/"), "url/");
    assert_eq!(wrap("val", "[", "]"), "[val]");
}

#[test]
fn test_final_coverage_boosters() {
    use borno::case::*;

    assert_eq!(to_screaming_snake_case(""), "");
    assert_eq!(to_screaming_snake_case("   "), "");

    // pluralize and singularize with caves
    assert_eq!(singularize("caves"), "cave");
    assert_eq!(pluralize("cave"), "caves");

    // TitleCase with empty string
    use borno::grammar::*;
    let default_inf = Inflector::new();
    assert_eq!(default_inf.pluralize("userAccount"), "userAccounts");
    assert_eq!(default_inf.singularize("userAccounts"), "userAccount");
}

#[test]
fn test_bytes_duration_title_coverage_boosters() {
    use borno::{
        BytesOptions, BytesStandard, BytesUnit, ParseBytesError, ParseDurationError,
        case::{is_title_case_apa, to_title_case_apa},
        utils::{format_bytes_with_options, format_duration_verbose, parse_bytes, parse_duration},
    };

    // BytesUnit symbols & standard base
    let units = [
        BytesUnit::B,
        BytesUnit::KB,
        BytesUnit::MB,
        BytesUnit::GB,
        BytesUnit::TB,
        BytesUnit::PB,
        BytesUnit::KiB,
        BytesUnit::MiB,
        BytesUnit::GiB,
        BytesUnit::TiB,
        BytesUnit::PiB,
    ];
    for u in units {
        let sym = u.symbol();
        assert!(!sym.is_empty());
        let opts = BytesOptions::new().with_unit(u);
        let formatted = format_bytes_with_options(1024 * 1024, &opts);
        assert!(formatted.ends_with(sym));
    }

    assert_eq!(BytesStandard::Decimal.base(), 1000);
    assert_eq!(BytesStandard::Binary.base(), 1024);

    // ParseBytesError Display formatting
    let errs = [
        ParseBytesError::EmptyInput,
        ParseBytesError::InvalidFormat("bad".to_string()),
        ParseBytesError::InvalidNumber("NaN".to_string()),
        ParseBytesError::UnknownUnit("foo".to_string()),
        ParseBytesError::Overflow,
    ];
    for err in &errs {
        let displayed = format!("{err}");
        assert!(!displayed.is_empty());
    }

    // ParseDurationError Display formatting
    let dur_errs = [
        ParseDurationError::EmptyInput,
        ParseDurationError::InvalidFormat("bad".to_string()),
        ParseDurationError::InvalidNumber("NaN".to_string()),
        ParseDurationError::UnknownUnit("foo".to_string()),
        ParseDurationError::Overflow,
    ];
    for err in &dur_errs {
        let displayed = format!("{err}");
        assert!(!displayed.is_empty());
    }

    // Fractional duration and compound durations
    let half_hour = parse_duration("0.5 hours").unwrap();
    assert_eq!(half_hour, std::time::Duration::from_secs(1800));

    let compound_frac = parse_duration("1.5h, 30.5s").unwrap();
    assert_eq!(
        compound_frac,
        std::time::Duration::from_secs(5400) + std::time::Duration::from_millis(30500)
    );

    // Title case APA negative branches
    assert!(!is_title_case_apa("The_Old_Man"));
    assert!(!is_title_case_apa("The And Sea"));
    assert!(!is_title_case_apa("ThE Old Man"));
    assert!(!is_title_case_apa("The Old MaN"));

    let empty_apa = to_title_case_apa("");
    assert_eq!(empty_apa, "");

    // Additional units for bytes
    assert_eq!(parse_bytes("1 TB").unwrap(), 1_099_511_627_776);
    assert_eq!(parse_bytes("1 TiB").unwrap(), 1_099_511_627_776);
    assert_eq!(parse_bytes("1 PB").unwrap(), 1_125_899_906_842_624);
    assert_eq!(parse_bytes("1 PiB").unwrap(), 1_125_899_906_842_624);
    assert_eq!(parse_bytes("1 GiB").unwrap(), 1_073_741_824);
    assert_eq!(parse_bytes("1 gibibyte").unwrap(), 1_073_741_824);
    assert_eq!(parse_bytes("1 gibibytes").unwrap(), 1_073_741_824);

    // Separators and zero decimals
    let custom_sep = BytesOptions::new().with_unit_separator("-");
    assert_eq!(format_bytes_with_options(1024, &custom_sep), "1-KB");

    let zero_decimals = BytesOptions::new().with_decimal_places(0);
    assert_eq!(format_bytes_with_options(1024, &zero_decimals), "1 KB");

    let millions_bytes = BytesOptions::new()
        .with_unit(BytesUnit::B)
        .with_thousands_separator(",");
    assert_eq!(
        format_bytes_with_options(10_000_000, &millions_bytes),
        "10,000,000 B"
    );

    // Invalid numbers and edge cases in bytes
    assert!(parse_bytes("1.2.3 MB").is_err());
    assert!(parse_bytes(". MB").is_err());
    assert!(parse_bytes("999999999999999999999 TB").is_err());
    assert!(parse_bytes("NaN MB").is_err());
    assert!(parse_bytes("inf MB").is_err());
    let huge_bytes = format!("{} MB", "9".repeat(320));
    assert!(parse_bytes(&huge_bytes).is_err());

    // Duration edge cases
    assert_eq!(
        format_duration_verbose(std::time::Duration::from_millis(
            86_400_000 + 3_600_000 + 60_000 + 1_000 + 1
        )),
        "1 day, 1 hour, 1 minute, 1 second, 1 millisecond"
    );
    assert_eq!(
        format_duration_verbose(std::time::Duration::from_millis(
            2 * 86_400_000 + 2 * 3_600_000 + 2 * 60_000 + 2 * 1_000 + 2
        )),
        "2 days, 2 hours, 2 minutes, 2 seconds, 2 milliseconds"
    );

    assert_eq!(
        parse_duration("1 hour, ").unwrap(),
        std::time::Duration::from_secs(3600)
    );
    assert!(parse_duration("1.2.3 s").is_err());
    assert!(parse_duration(". s").is_err());
    assert!(parse_duration("-5 s").is_err());
    assert!(parse_duration("NaN s").is_err());
    assert!(parse_duration("inf s").is_err());
    let huge_dur = format!("{} s", "9".repeat(320));
    assert!(parse_duration(&huge_dur).is_err());
    assert!(parse_duration(", , ,").is_err());
    let dur_overflow = format!("{} s, {} s", u64::MAX / 2 + 1000, u64::MAX / 2 + 1000);
    assert!(parse_duration(&dur_overflow).is_err());
    assert!(parse_duration("100000000000000000000000 days").is_err());
    assert!(parse_duration("100").is_err());

    // Title case APA edge cases
    assert!(!is_title_case_apa("Hello W@rld"));
    assert!(!is_title_case_apa("HeLLo World"));

    // Case scalar helpers edge cases
    assert_eq!(borno::case::capitalize_first(""), "");
    assert_eq!(borno::case::capitalize_first("Hello"), "Hello");
    assert!(matches!(
        borno::case::capitalize_first("Hello"),
        std::borrow::Cow::Borrowed(_)
    ));
    assert_eq!(borno::case::uncapitalize_first(""), "");
    assert_eq!(borno::case::uncapitalize_first("hello"), "hello");
    assert!(matches!(
        borno::case::uncapitalize_first("hello"),
        std::borrow::Cow::Borrowed(_)
    ));

    // Grammar compound zero-allocation borrowing
    let comp_uncount = pluralize("user_information");
    assert_eq!(comp_uncount, "user_information");
    assert!(matches!(comp_uncount, std::borrow::Cow::Borrowed(_)));

    let camel_uncount = pluralize("userInformation");
    assert_eq!(camel_uncount, "userInformation");
    assert!(matches!(camel_uncount, std::borrow::Cow::Borrowed(_)));

    let camel_sing = singularize("userProfile");
    assert_eq!(camel_sing, "userProfile");
    assert!(matches!(camel_sing, std::borrow::Cow::Borrowed(_)));

    // StrBuilder conditional branches
    fn append_x(b: StrBuilder) -> StrBuilder {
        b.append("x")
    }
    assert_eq!(
        StrBuilder::new("val").when(true, append_x).into_string(),
        "valx"
    );
    assert_eq!(
        StrBuilder::new("val").when(false, append_x).into_string(),
        "val"
    );
    assert_eq!(
        StrBuilder::new("val")
            .when_not_empty(append_x)
            .into_string(),
        "valx"
    );
    assert_eq!(
        StrBuilder::new("").when_not_empty(append_x).into_string(),
        ""
    );
    assert_eq!(StrBuilder::new("").when_empty(append_x).into_string(), "x");
    assert_eq!(
        StrBuilder::new("val").when_empty(append_x).into_string(),
        "val"
    );

    // Affix edge cases & case-sensitive matching
    use borno::utils::{
        AffixOptions, ext, remove_extension, remove_prefix_with_options, remove_suffix_with_options,
    };

    let case_sens = AffixOptions::new().with_ignore_case(false);
    assert_eq!(
        remove_suffix_with_options("user_Controller", "Controller", &case_sens),
        "user"
    );
    assert_eq!(
        remove_suffix_with_options("userController", "Controller", &case_sens),
        "user"
    );
    assert_eq!(
        remove_suffix_with_options("user_controller", "Controller", &case_sens),
        "user_controller"
    );
    assert_eq!(
        remove_suffix_with_options("user", "Controller", &case_sens),
        "user"
    );
    assert_eq!(remove_suffix_with_options("user", "", &case_sens), "user");

    let case_sens_sim = AffixOptions::new()
        .with_ignore_case(false)
        .with_similar_words(["email"]);
    assert_eq!(
        remove_suffix_with_options("verify_email", "mail", &case_sens_sim),
        "verify_email"
    );
    assert_eq!(
        remove_suffix_with_options("verify_Email", "mail", &case_sens_sim),
        "verify_E"
    );

    assert_eq!(
        remove_prefix_with_options("app_User", "app", &case_sens),
        "User"
    );
    assert_eq!(
        remove_prefix_with_options("appUser", "app", &case_sens),
        "User"
    );
    assert_eq!(
        remove_prefix_with_options("App_User", "app", &case_sens),
        "App_User"
    );
    assert_eq!(
        remove_prefix_with_options("user", "app", &case_sens),
        "user"
    );
    assert_eq!(remove_prefix_with_options("user", "", &case_sens), "user");

    // Similar words longer than input and case-sensitive prefix similar check
    let long_sim = AffixOptions::new().with_similar_words(["much_longer_than_input"]);
    assert_eq!(remove_suffix_with_options("short", "rt", &long_sim), "sho");
    assert_eq!(remove_prefix_with_options("hi", "h", &long_sim), "i");

    let case_sens_prefix_sim = AffixOptions::new()
        .with_ignore_case(false)
        .with_similar_words(["Maker"]);
    assert_eq!(
        remove_prefix_with_options("MakerService", "other", &case_sens_prefix_sim),
        "MakerService"
    );

    assert_eq!(remove_extension(".gitignore"), ".gitignore");
    assert_eq!(remove_extension("path/to/.bashrc"), "path/to/.bashrc");
    assert_eq!(ext(".gitignore", "txt"), ".gitignore.txt");

    // Deduplicate empty string
    assert_eq!(borno::utils::deduplicate("", '/'), "");
}

#[test]
fn test_unicode_casing_and_slug_edges() {
    use borno::case::*;

    let german = "Über den Dächern von Zürich";
    assert_eq!(to_snake_case(german), "über_den_dächern_von_zürich");
    assert_eq!(to_kebab_case(german), "über-den-dächern-von-zürich");
    assert_eq!(to_camel_case(german), "überDenDächernVonZürich");
    assert_eq!(to_pascal_case(german), "ÜberDenDächernVonZürich");
    assert_eq!(to_title_case(german), "Über Den Dächern Von Zürich");
    assert_eq!(to_train_case(german), "Über-Den-Dächern-Von-Zürich");
    assert_eq!(
        to_screaming_snake_case(german),
        "ÜBER_DEN_DÄCHERN_VON_ZÜRICH"
    );
    assert_eq!(to_dot_case(german), "über.den.dächern.von.zürich");
    assert_eq!(to_no_case(german), "über den dächern von zürich");

    // Acronym and camelCase transitions in non-ASCII
    let french = "ÉlanVital";
    assert_eq!(to_snake_case(french), "élan_vital");
    assert_eq!(to_camel_case(french), "élanVital");
    assert_eq!(to_pascal_case(french), "ÉlanVital");

    let french_acronym = "ÉLANVital";
    assert_eq!(to_snake_case(french_acronym), "élan_vital");
    assert_eq!(to_camel_case(french_acronym), "élanVital");

    let number_transition = "ÜberGröße123";
    assert_eq!(to_snake_case(number_transition), "über_größe_123");
    let number_transition_prefix = "123ÜberGröße";
    assert_eq!(to_snake_case(number_transition_prefix), "123_über_größe");

    // Separators with non-ASCII
    let separated = "__über__größer__träger__";
    assert_eq!(to_snake_case(separated), "über_größer_träger");
    assert_eq!(to_kebab_case(separated), "über-größer-träger");
    assert_eq!(to_camel_case(separated), "überGrößerTräger");
    assert_eq!(to_pascal_case(separated), "ÜberGrößerTräger");
    assert_eq!(to_title_case(separated), "Über Größer Träger");

    // Non-ASCII whitespace
    assert_eq!(to_snake_case(" \u{00A0} "), "");
    assert_eq!(to_camel_case(" \u{00A0} "), "");
    assert_eq!(to_title_case(" \u{00A0} "), "");

    // Slug options with allow_unicode = true
    let unicode_opts = SlugOptions::new().with_allow_unicode(true);
    assert_eq!(
        slugify_with_options("Hello World", &SlugOptions::new().with_lowercase(false)),
        "Hello-World"
    );
    assert_eq!(
        slugify_with_options("Café & Crème", &unicode_opts),
        "cafe-and-creme"
    );
    assert_eq!(
        slugify_with_options("東京 & 大阪", &unicode_opts),
        "東京-and-大阪"
    );
    assert_eq!(
        slugify_with_options("user@zürich.de", &unicode_opts),
        "user-at-zurich-de"
    );

    let unicode_no_symbols = SlugOptions::new()
        .with_allow_unicode(true)
        .with_symbol_expansion(false);
    assert_eq!(
        slugify_with_options("Café & Crème", &unicode_no_symbols),
        "cafe-creme"
    );
    assert_eq!(
        slugify_with_options("東京 & 大阪", &unicode_no_symbols),
        "東京-大阪"
    );
    assert_eq!(
        slugify_with_options("user@zürich.de", &unicode_no_symbols),
        "user-zurich-de"
    );

    let unicode_no_lower = SlugOptions::new()
        .with_allow_unicode(true)
        .with_lowercase(false);
    assert_eq!(
        slugify_with_options("Café & Crème", &unicode_no_lower),
        "Cafe-and-Creme"
    );
    assert_eq!(
        slugify_with_options("Hello 東京", &unicode_no_lower),
        "Hello-東京"
    );

    let unicode_custom = SlugOptions::new()
        .with_allow_unicode(true)
        .with_replacement('$', "dollar")
        .with_replacement('€', "euro");
    assert_eq!(
        slugify_with_options("Price $50 and €40", &unicode_custom),
        "price-dollar-50-and-euro-40"
    );

    let unicode_delims = SlugOptions::new()
        .with_allow_unicode(true)
        .with_delimiter('_');
    assert_eq!(
        slugify_with_options("Café Déjà Vu", &unicode_delims),
        "cafe_deja_vu"
    );

    // ASCII symbol expansion edge cases
    assert_eq!(slugify("&"), "and");
    assert_eq!(slugify("@"), "at");
    assert_eq!(slugify("&&&"), "and-and-and");
    assert_eq!(slugify("@@@"), "at-at-at");
    assert_eq!(slugify("&hello&"), "and-hello-and");
    assert_eq!(slugify("@world@"), "at-world-at");

    // Non-ASCII acronym at the very end
    assert_eq!(to_snake_case("ÜberAPI"), "über_api");
    assert_eq!(to_camel_case("ÜberAPI"), "überApi");
    assert_eq!(to_pascal_case("ÜberAPI"), "ÜberApi");

    // Emoji or untransliterated non-ASCII symbol in default slugify (with and without space)
    assert_eq!(slugify("Rust 🦀 2026"), "rust-2026");
    assert_eq!(slugify("Rust🦀2026"), "rust-2026");

    // Trailing symbols, bullets and non-alphanumeric with allow_unicode = true
    assert_eq!(slugify_with_options("東京 --- !?", &unicode_opts), "東京");
    assert_eq!(
        slugify_with_options("東京 !? 大阪", &unicode_opts),
        "東京-大阪"
    );
    assert_eq!(
        slugify_with_options("東京•大阪", &unicode_opts),
        "東京-大阪"
    );
}

#[test]
fn test_100_percent_coverage_gap_fillers() {
    use std::borrow::Cow;

    use borno::{
        Affix, Case, StrBuilder,
        case::{
            is_camel_case, is_kebab_case, is_sentence_case, is_snake_case, is_title_case,
            to_snake_case, to_title_case_apa,
        },
        grammar::{is_plural, is_singular, pluralize},
        slug::{SlugOptions, slugify, slugify_with_options},
    };

    // 1. case/apa.rs
    assert_eq!(
        to_title_case_apa("The Old Man and the Sea"),
        Cow::Borrowed("The Old Man and the Sea")
    );
    assert_eq!(to_title_case_apa(""), Cow::Borrowed(""));

    // 2. case/predicates.rs (tested through public casing predicates)
    assert!(!is_snake_case("a__b"));
    assert!(!is_kebab_case("a--b"));
    assert!(!is_title_case("A  B"));
    assert!(!is_camel_case(""));
    assert!(!is_sentence_case("Hello  world"));
    assert!(!is_sentence_case("Hello World"));
    assert!(is_sentence_case("Hello 123 world"));

    // 3. case/scanner.rs (tested through public casing and inflection)
    assert_eq!(to_snake_case("item10Count"), "item_10_count");
    assert_eq!(pluralize("user2Profile"), "user2Profiles");
    assert_eq!(pluralize("userProfile2"), "userProfile2s");
    assert_eq!(pluralize("überUser"), "überUsers");
    assert_eq!(pluralize("ÜberUser"), "ÜberUsers");
    assert_eq!(pluralize("HTTPÜber"), "HTTPÜbers");
    assert_eq!(pluralize("über123"), "über123s");
    assert_eq!(pluralize("123über"), "123übers");

    // 4. case/style.rs
    let err = "invalid_style".parse::<Case>().unwrap_err();
    assert_eq!(err.invalid_name(), "invalid_style");
    assert_eq!(
        format!("{err}"),
        "unrecognized casing style: 'invalid_style'"
    );

    for case in Case::ALL {
        assert!(!case.as_str().is_empty());
        assert_eq!(format!("{case}"), case.as_str());
    }
    assert_eq!("title".parse::<Case>().unwrap(), Case::Title);
    assert_eq!("sentence".parse::<Case>().unwrap(), Case::Sentence);
    assert_eq!("no".parse::<Case>().unwrap(), Case::No);
    assert_eq!("capital".parse::<Case>().unwrap(), Case::Capital);

    // 5. grammar/mod.rs
    assert!(!is_plural(""));
    assert!(!is_singular(""));
    assert!(is_singular("user_profile"));
    assert!(!is_singular("user_profiles"));
    assert!(is_singular("userProfile"));
    assert!(!is_singular("userProfiles"));

    // 6. slug/transliterate.rs and slug/mod.rs
    assert_eq!(slugify("§"), "");
    assert_eq!(slugify("¿"), "");
    assert_eq!(slugify("¡"), "");
    assert_eq!(slugify("hello---"), "hello");
    assert_eq!(slugify("hello 🦀"), "hello");
    assert_eq!(slugify("🦀 hello"), "hello");

    let symbol_default = SlugOptions::new().with_symbol_expansion(true);
    assert_eq!(
        slugify_with_options("hello & world", &symbol_default),
        "hello-and-world"
    );

    let symbol_custom = SlugOptions::new()
        .with_delimiter('_')
        .with_symbol_expansion(true);
    assert_eq!(
        slugify_with_options("tom & jerry @ home", &symbol_custom),
        "tom_and_jerry_at_home"
    );
    assert_eq!(
        slugify_with_options("& tom @", &symbol_custom),
        "and_tom_at"
    );

    let upper_slug = SlugOptions::new().with_lowercase(false).with_delimiter('_');
    assert_eq!(
        slugify_with_options("Café Déjà 🦀 Vu---", &upper_slug),
        "Cafe_Deja_Vu"
    );

    // 7. utils/pad.rs
    assert_eq!(Affix::wrap("foo", "[", "]"), "[foo]");
    assert_eq!(Affix::unwrap("[foo]", "[", "]"), "foo");
    assert_eq!(Affix::unwrap_quotes("\"foo\""), "foo");
    assert_eq!(Affix::pad_start("42", 5, '0'), "00042");
    assert_eq!(Affix::pad_end("42", 5, '0'), "42000");
    assert_eq!(Affix::pad_both("42", 6, '-'), "--42--");

    // 8. builder.rs (All Cow::Owned branches)
    let owned = StrBuilder::new(String::from("  hello_world  "));
    assert_eq!(format!("{owned}"), "  hello_world  ");
    assert_eq!(owned.as_ref(), "  hello_world  ");
    assert_eq!(&*owned, "  hello_world  ");

    let from_str: StrBuilder = "hello".into();
    assert_eq!(from_str.into_cow(), Cow::Borrowed("hello"));
    let from_string: StrBuilder = String::from("hello").into();
    assert_eq!(
        from_string.into_cow(),
        Cow::Owned::<str>("hello".to_string())
    );
    let from_cow: StrBuilder = Cow::Borrowed("hello").into();
    assert_eq!(from_cow.into_cow(), Cow::Borrowed("hello"));

    // Pipeline with owned values covering Cow::Owned branches
    let b = StrBuilder::new(String::from("  crate::services::user_profile  "))
        .trim()
        .demodulize()
        .deconstantize();
    assert_eq!(b.into_string(), "");

    let b_owned = StrBuilder::new(String::from("user_profile"));
    assert_eq!(
        b_owned.clone().to_case(Case::Camel).into_string(),
        "userProfile"
    );
    assert_eq!(
        b_owned.clone().capitalize_first().into_string(),
        "User_profile"
    );
    assert_eq!(
        b_owned.clone().uncapitalize_first().into_string(),
        "user_profile"
    );
    assert_eq!(b_owned.clone().pluralize().into_string(), "user_profiles");
    assert_eq!(b_owned.clone().singularize().into_string(), "user_profile");
    assert_eq!(
        b_owned.clone().pluralize_with_count(2).into_string(),
        "user_profiles"
    );
    assert_eq!(
        b_owned.clone().start("api_").into_string(),
        "api_user_profile"
    );
    assert_eq!(
        b_owned.clone().finish("_v1").into_string(),
        "user_profile_v1"
    );
    assert_eq!(
        b_owned.clone().ensure_prefix("api_").into_string(),
        "api_user_profile"
    );
    assert_eq!(
        b_owned.clone().ensure_suffix("_v1").into_string(),
        "user_profile_v1"
    );
    assert_eq!(
        b_owned.clone().ensure_wrap("api_", "_v1").into_string(),
        "api_user_profile_v1"
    );
    assert_eq!(
        b_owned.clone().ensure_wrapped("api_", "_v1").into_string(),
        "api_user_profile_v1"
    );
    assert_eq!(b_owned.clone().before("_").into_string(), "user");
    assert_eq!(b_owned.clone().before_last("_").into_string(), "user");
    assert_eq!(b_owned.clone().after("_").into_string(), "profile");
    assert_eq!(b_owned.clone().after_last("_").into_string(), "profile");
    assert_eq!(b_owned.clone().between("u", "r").into_string(), "se");
    assert_eq!(b_owned.clone().between_first("u", "r").into_string(), "se");
    assert_eq!(b_owned.clone().chop_start("user_").into_string(), "profile");
    assert_eq!(b_owned.clone().chop_end("_profile").into_string(), "user");
    assert_eq!(
        b_owned.clone().remove_prefix("user_").into_string(),
        "profile"
    );
    assert_eq!(
        b_owned.clone().remove_suffix("_profile").into_string(),
        "user"
    );
    assert_eq!(
        StrBuilder::new(String::from("file.tar.gz"))
            .remove_extension()
            .into_string(),
        "file.tar"
    );

    let b_norm = StrBuilder::new(String::from("foo   bar\\baz <script>"))
        .squish()
        .normalize_slashes()
        .html_escape()
        .html_unescape()
        .deduplicate('o');
    assert_eq!(b_norm.into_string(), "fo bar/baz <script>");

    let b_options = StrBuilder::new(String::from("api_user_service"))
        .remove_prefix_with_options("api_", &borno::utils::AffixOptions::default())
        .remove_suffix_with_options("_service", &borno::utils::AffixOptions::default());
    assert_eq!(b_options.into_string(), "user");

    let b_pipe = StrBuilder::new("test").pipe(|builder| builder.into_string() + "ed");
    assert_eq!(b_pipe, "tested");

    let b_trans = StrBuilder::new("test").transform_with(|s| Cow::Owned(s.to_uppercase()));
    assert_eq!(b_trans.into_string(), "TEST");
    let b_trans_borrowed = StrBuilder::new("test").transform_with(|s| Cow::Borrowed(s));
    assert_eq!(b_trans_borrowed.into_string(), "test");

    // Slug: slugify_raw empty & non-ASCII
    let no_sym = SlugOptions::new().with_symbol_expansion(false);
    assert_eq!(slugify_with_options("", &no_sym), "");
    assert_eq!(slugify_with_options("hello 🦀", &no_sym), "hello");
    assert_eq!(slugify_with_options("hello🦀world", &no_sym), "hello-world");
    assert_eq!(slugify_with_options("hello---", &no_sym), "hello");

    // Fast ASCII symbol expansion without delimiter before
    let custom_delim = SlugOptions::new()
        .with_delimiter('_')
        .with_symbol_expansion(true);
    assert_eq!(slugify_with_options("a&b@c", &custom_delim), "a_and_b_at_c");
    assert_eq!(slugify_with_options("café", &custom_delim), "cafe");
    assert_eq!(slugify_with_options("a🦀b---", &custom_delim), "a_b");

    // Transliterate uppercase with Latin-1 symbol
    let no_lower = SlugOptions::new().with_lowercase(false);
    assert_eq!(slugify_with_options("§", &no_lower), "");

    // Builder Cow::Owned unchanged and unchanged length branches
    assert_eq!(
        StrBuilder::new(String::from("\"quoted\""))
            .unwrap_quotes()
            .into_string(),
        "quoted"
    );
    assert_eq!(
        StrBuilder::new(String::from("unquoted"))
            .unwrap_quotes()
            .into_string(),
        "unquoted"
    );

    let b_miss = StrBuilder::new(String::from("user_profile"));
    assert_eq!(b_miss.clone().before("x").into_string(), "user_profile");
    assert_eq!(
        b_miss.clone().before_last("x").into_string(),
        "user_profile"
    );
    assert_eq!(b_miss.clone().after("x").into_string(), "");
    assert_eq!(b_miss.clone().after_last("x").into_string(), "");
    assert_eq!(b_miss.clone().between("x", "y").into_string(), "");
    assert_eq!(b_miss.clone().between_first("x", "y").into_string(), "");
    assert_eq!(b_miss.clone().chop_start("x").into_string(), "user_profile");
    assert_eq!(b_miss.clone().chop_end("x").into_string(), "user_profile");
    assert_eq!(
        b_miss.clone().remove_prefix("x").into_string(),
        "user_profile"
    );
    assert_eq!(
        b_miss.clone().remove_suffix("x").into_string(),
        "user_profile"
    );
    assert_eq!(
        b_miss.clone().remove_extension().into_string(),
        "user_profile"
    );

    let affix_opts = borno::utils::AffixOptions::default();
    assert_eq!(
        b_miss
            .clone()
            .remove_prefix_with_options("x", &affix_opts)
            .into_string(),
        "user_profile"
    );
    assert_eq!(
        b_miss
            .clone()
            .remove_suffix_with_options("x", &affix_opts)
            .into_string(),
        "user_profile"
    );

    // Builder trim with owned unchanged and trim_start / trim_end
    let b_trim = StrBuilder::new(String::from("hello"));
    assert_eq!(b_trim.clone().trim().into_string(), "hello");
    assert_eq!(b_trim.clone().trim_start().into_string(), "hello");
    assert_eq!(b_trim.clone().trim_end().into_string(), "hello");
    assert_eq!(
        StrBuilder::new(String::from("  hello  "))
            .trim_start()
            .into_string(),
        "hello  "
    );
    assert_eq!(
        StrBuilder::new(String::from("  hello  "))
            .trim_end()
            .into_string(),
        "  hello"
    );

    // Builder append / prepend empty string
    assert_eq!(StrBuilder::new("hello").append("").into_string(), "hello");
    assert_eq!(StrBuilder::new("hello").prepend("").into_string(), "hello");
}
