#![cfg(feature = "full")]

#[cfg(feature = "regex")]
use borno::grammar::{add_plural_rule, add_singular_rule};
use borno::{
    StrBuilder, StrExt,
    grammar::{
        Inflector, add_irregular, add_uncountable, is_plural, is_singular, pluralize, singularize,
    },
    slug::{SlugOptions, slugify_with_options},
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
