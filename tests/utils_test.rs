#![cfg(all(feature = "utils", feature = "ext"))]

use borno::{StrExt, utils::*};

#[test]
fn test_truncate_and_words() {
    let text = "The quick brown fox jumps over the lazy dog";
    assert_eq!(
        truncate(text, 19, &TruncateOptions::default()),
        "The quick brown ..."
    );

    let word_opts = TruncateOptions::new().with_complete_words(true);
    assert_eq!(truncate(text, 19, &word_opts), "The quick brown...");

    assert_eq!(words(text, 4, "..."), "The quick brown fox...");
    assert_eq!(words(text, 100, "..."), text);
}

#[test]
fn test_excerpt() {
    let body = "Welcome to the Borno library. It provides complete text transformations for high-throughput Rust services.";
    let opts = ExcerptOptions::new().with_radius(20);
    let result = excerpt(body, "text transformations", &opts);
    assert!(result.contains("text transformations"));
    assert!(result.starts_with("..."));
    assert!(result.ends_with("..."));

    // Unicode test with byte-expanding characters (German capital sharp s 'ẞ' -> 'ß')
    let unicode_body = "Prefix AẞC Suffix";
    let unicode_opts = ExcerptOptions::new().with_radius(4);
    let unicode_result = excerpt(unicode_body, "c", &unicode_opts);
    assert_eq!(unicode_result, "...x AẞC Suf...");

    let exact_match = excerpt(
        "hello world",
        "world",
        &ExcerptOptions::new().with_radius(10),
    );
    assert_eq!(exact_match, "hello world");
}

#[test]
fn test_ordinal_formatting() {
    assert_eq!(ordinal(1), "1st");
    assert_eq!(ordinal(2), "2nd");
    assert_eq!(ordinal(3), "3rd");
    assert_eq!(ordinal(4), "4th");
    assert_eq!(ordinal(11), "11th");
    assert_eq!(ordinal(12), "12th");
    assert_eq!(ordinal(13), "13th");
    assert_eq!(ordinal(21), "21st");
    assert_eq!(ordinal(22), "22nd");
    assert_eq!(ordinal(23), "23rd");
    assert_eq!(ordinal(101), "101st");
    assert_eq!(ordinal(111), "111th");
    assert_eq!(ordinal(121), "121st");
}

#[test]
fn test_sentence_formatting() {
    assert_eq!(sentence(&[]), "");
    assert_eq!(sentence(&["Alpha"]), "Alpha");
    assert_eq!(sentence(&["Alpha", "Beta"]), "Alpha and Beta");
    assert_eq!(
        sentence(&["Alpha", "Beta", "Gamma"]),
        "Alpha, Beta, and Gamma"
    );

    let custom = SentenceOptions::new()
        .with_pair_separator(" or ")
        .with_last_separator(", or ");
    assert_eq!(
        sentence_with_options(&["Red", "Blue"], &custom),
        "Red or Blue"
    );
    assert_eq!(
        sentence_with_options(&["Red", "Blue", "Green"], &custom),
        "Red, Blue, or Green"
    );
}

#[test]
fn test_masking() {
    assert_eq!(
        mask("sensitive-api-token", '*', 4, Some(8)),
        "sens********i-token"
    );
    assert_eq!(
        mask("4111111111111234", '*', 0, Some(12)),
        "************1234"
    );
    assert_eq!(
        mask_email("sarah.connor@sky.net", '*'),
        "s**********r@sky.net"
    );
    assert_eq!(mask_email("x@domain.com", '*'), "*@domain.com");
}

#[test]
fn test_padding_and_guards() {
    assert_eq!(pad_start("7", 4, '0'), "0007");
    assert_eq!(pad_end("7", 4, '0'), "7000");
    assert_eq!(pad_both("Header", 12, '='), "===Header===");

    assert_eq!(start("api/v1", "/"), "/api/v1");
    assert_eq!(start("/api/v1", "/"), "/api/v1");

    assert_eq!(finish("https://example.com", "/"), "https://example.com/");
    assert_eq!(finish("https://example.com/", "/"), "https://example.com/");

    assert_eq!(wrap("query", "'", "'"), "'query'");
    assert_eq!(unwrap("'query'", "'", "'"), "query");
    assert_eq!(unwrap("query", "'", "'"), "query");
}

#[test]
fn test_str_ext_fluent_helpers() {
    assert_eq!("42".finish(";"), "42;");
    assert_eq!("Hello World".truncate(8), "Hello...");
    assert_eq!("secret".mask('*', 1, Some(4)), "s****t");
}

#[test]
fn test_slicing_utilities() {
    assert_eq!(before("user@example.com", "@"), "user");
    assert_eq!(before_last("a.b.c.d", "."), "a.b.c");
    assert_eq!(after("user@example.com", "@"), "example.com");
    assert_eq!(after_last("a.b.c.d", "."), "d");
    assert_eq!(between("name: [John Doe];", "[", "]"), "John Doe");
    assert_eq!(between_first("[a] and [b]", "[", "]"), "a");

    assert_eq!("user@example.com".before("@"), "user");
    assert_eq!("a.b.c.d".before_last("."), "a.b.c");
    assert_eq!("user@example.com".after("@"), "example.com");
    assert_eq!("a.b.c.d".after_last("."), "d");
    assert_eq!("<div>content</div>".between("<div>", "</div>"), "content");
    assert_eq!("(x) + (y)".between_first("(", ")"), "x");
}

#[test]
fn test_normalize_utilities() {
    assert_eq!(squish("   hello   \t\n  world   "), "hello world");
    assert_eq!("   clean   me   ".squish(), "clean me");

    assert_eq!(
        normalize_slashes("C:\\Users\\admin\\file.txt"),
        "C:/Users/admin/file.txt"
    );
    assert_eq!("foo\\\\bar\\baz".normalize_slashes(), "foo//bar/baz");
}

#[test]
fn test_text_and_terminal_utilities() {
    assert_eq!(
        html_escape("<script>alert(\"hello & goodbye\")</script>"),
        "&lt;script&gt;alert(&quot;hello &amp; goodbye&quot;)&lt;/script&gt;"
    );
    assert_eq!(
        html_unescape("&lt;b&gt;bold &amp; &quot;quoted&quot;&lt;/b&gt;"),
        "<b>bold & \"quoted\"</b>"
    );

    assert_eq!("<p>hello</p>".html_escape(), "&lt;p&gt;hello&lt;/p&gt;");
    assert_eq!("&lt;p&gt;hello&lt;/p&gt;".html_unescape(), "<p>hello</p>");

    let wrapped = word_wrap("The quick brown fox jumps over the lazy dog", 15);
    assert_eq!(wrapped, "The quick brown\nfox jumps over\nthe lazy dog");
    assert_eq!(
        "The quick brown fox jumps".word_wrap(12),
        "The quick\nbrown fox\njumps"
    );

    let columns = ["Status", "OK", "Pending"];
    let left = justify_columns(&columns, 10, Alignment::Left);
    assert_eq!(left, vec!["Status    ", "OK        ", "Pending   "]);

    let right = justify_columns(&columns, 10, Alignment::Right);
    assert_eq!(right, vec!["    Status", "        OK", "   Pending"]);
}

#[test]
fn test_interpolate_utilities() {
    use std::collections::HashMap;

    let params = [("name", "Alice"), ("action", "deployed")];
    assert_eq!(
        interpolate("User {{ name }} has {{ action }} the service.", &params),
        "User Alice has deployed the service."
    );

    let mut map = HashMap::new();
    map.insert("name", "Bob");
    map.insert("action", "stopped");
    assert_eq!(
        interpolate_pairs("User {name} has :action the service.", map),
        "User Bob has stopped the service."
    );

    assert_eq!(
        "Hello, {{ user }}!".interpolate(&[("user", "Admin")]),
        "Hello, Admin!"
    );
    assert_eq!(
        "Deploying :env to :target".interpolate(&[("env", "prod"), ("target", "cloud")]),
        "Deploying prod to cloud"
    );
}

#[test]
fn test_bytes_utilities() {
    assert_eq!(parse_bytes("1024").unwrap(), 1024);
    assert_eq!(parse_bytes("1KB").unwrap(), 1024);
    assert_eq!(parse_bytes("10MB").unwrap(), 10_485_760);
    assert_eq!(parse_bytes("1.5 GB").unwrap(), 1_610_612_736);

    assert_eq!("10MB".parse_bytes().unwrap(), 10_485_760);

    assert_eq!(format_bytes(0), "0 B");
    assert_eq!(format_bytes(1024), "1 KB");
    assert_eq!(format_bytes(1_572_864), "1.5 MB");

    let opts = BytesOptions::new()
        .with_decimal_places(1)
        .with_fixed_decimals(true);
    assert_eq!(format_bytes_with_options(1024, &opts), "1.0 KB");
}

#[test]
fn test_duration_utilities() {
    use std::time::Duration;

    assert_eq!(parse_duration("500ms").unwrap(), Duration::from_millis(500));
    assert_eq!(parse_duration("10s").unwrap(), Duration::from_secs(10));
    assert_eq!(parse_duration("15m").unwrap(), Duration::from_secs(900));
    assert_eq!(parse_duration("2h").unwrap(), Duration::from_secs(7200));
    assert_eq!(parse_duration("1d").unwrap(), Duration::from_secs(86400));
    assert_eq!(parse_duration("1w").unwrap(), Duration::from_secs(604800));

    assert_eq!("15m".parse_duration().unwrap(), Duration::from_secs(900));

    assert_eq!(format_duration(Duration::ZERO), "0s");
    assert_eq!(format_duration(Duration::from_secs(65)), "1m 5s");
    assert_eq!(format_duration(Duration::from_secs(3661)), "1h 1m 1s");

    assert_eq!(format_duration_verbose(Duration::from_secs(1)), "1 second");
    assert_eq!(format_duration_verbose(Duration::from_secs(2)), "2 seconds");
    assert_eq!(
        format_duration_verbose(Duration::from_secs(3661)),
        "1 hour, 1 minute, 1 second"
    );
}

#[test]
fn test_slice_chop_and_inspection() {
    assert_eq!(chop_start("https://example.com", "https://"), "example.com");
    assert_eq!(chop_start("example.com", "https://"), "example.com");
    assert_eq!(chop_end("filename.tar.gz", ".gz"), "filename.tar");
    assert_eq!(chop_end("filename.tar", ".gz"), "filename.tar");

    assert_eq!("https://example.com".chop_start("https://"), "example.com");
    assert_eq!("filename.tar.gz".chop_end(".gz"), "filename.tar");

    assert!(contains_all("the quick brown fox", &["quick", "fox"]));
    assert!(!contains_all("the quick brown fox", &["quick", "cat"]));
    assert!("the quick brown fox".contains_all(&["quick", "fox"]));

    assert!(contains_any("the quick brown fox", &["cat", "fox"]));
    assert!(!contains_any("the quick brown fox", &["cat", "dog"]));
    assert!("the quick brown fox".contains_any(&["cat", "fox"]));

    assert!(is_empty_or_whitespace("   \n\t  "));
    assert!(!is_empty_or_whitespace("  hello  "));
    assert!("   \n\t  ".is_empty_or_whitespace());
}

#[test]
fn test_format_deordinalize_and_suffix() {
    assert_eq!(ordinal_suffix(1), "st");
    assert_eq!(ordinal_suffix(2), "nd");
    assert_eq!(ordinal_suffix(3), "rd");
    assert_eq!(ordinal_suffix(4), "th");
    assert_eq!(ordinal_suffix(11), "th");
    assert_eq!(ordinal_suffix(21), "st");

    assert_eq!(deordinalize("1st"), "1");
    assert_eq!(deordinalize("2nd"), "2");
    assert_eq!(deordinalize("3rd"), "3");
    assert_eq!(deordinalize("4th"), "4");
    assert_eq!(deordinalize("21st"), "21");
    assert_eq!(deordinalize("102nd"), "102");
    assert_eq!(deordinalize("south"), "south");
    assert_eq!(deordinalize("3.14th"), "3.14th");

    assert_eq!("21st".deordinalize(), "21");
    assert_eq!("south".deordinalize(), "south");
}

#[test]
fn test_pad_surround_and_quotes() {
    assert_eq!(ensure_wrap("path", "/", "/"), "/path/");
    assert_eq!(ensure_wrap("/path", "/", "/"), "/path/");
    assert_eq!(ensure_wrap("path/", "/", "/"), "/path/");
    assert_eq!(ensure_wrap("/path/", "/", "/"), "/path/");

    assert_eq!("path".ensure_wrap("/", "/"), "/path/");

    assert_eq!(surround("content", "<", ">"), "<content>");
    assert_eq!("content".surround("<", ">"), "<content>");

    assert_eq!(unwrap_quotes("\"quoted\""), "quoted");
    assert_eq!(unwrap_quotes("'single'"), "single");
    assert_eq!(unwrap_quotes("\"mismatch'"), "\"mismatch'");
    assert_eq!(unwrap_quotes("unquoted"), "unquoted");

    assert_eq!("\"quoted\"".unwrap_quotes(), "quoted");
}

#[test]
fn test_affix_and_extension_utilities() {
    assert_eq!(remove_suffix("UserController", "Controller"), "User");
    assert_eq!(remove_suffix("UserController", "Service"), "UserController");
    assert_eq!(suffix("User", "Controller"), "UserController");
    assert_eq!(suffix("UserController", "Controller"), "UserController");
    assert_eq!(suffix("foo", "bar"), "foobar");

    let similar = AffixOptions::new()
        .with_similar_words(["email", "emailer"])
        .with_ignore_case(true);
    assert_eq!(
        suffix_with_options("verify_email", "Mail", &similar),
        "verify_emailMail"
    );
    assert_eq!(
        remove_suffix_with_options("verify_email", "mail", &similar),
        "verify_email"
    );

    assert_eq!(remove_prefix("HttpController", "Http"), "Controller");
    assert_eq!(remove_prefix("Controller", "Http"), "Controller");
    assert_eq!(prefix("Controller", "Http"), "HttpController");
    assert_eq!(prefix("HttpController", "Http"), "HttpController");

    let prefix_similar = AffixOptions::new()
        .with_similar_words(["maker"])
        .with_ignore_case(true);
    assert_eq!(
        prefix_with_options("makerservice", "make_", &prefix_similar),
        "make_makerservice"
    );
    assert_eq!(
        remove_prefix_with_options("makerservice", "make", &prefix_similar),
        "makerservice"
    );

    assert_eq!(remove_extension("app.component.ts"), "app.component");
    assert_eq!(remove_extension("Makefile"), "Makefile");
    assert_eq!(remove_suffix("a", "Controller"), "a");
    assert_eq!(remove_prefix("a", "Http"), "a");
    assert_eq!(ext("component", "js"), "component.js");
    assert_eq!(ext("app.component.ts", "js"), "app.component.js");
    assert_eq!(ext("file", ""), "file");

    assert_eq!("UserController".remove_suffix("Controller"), "User");
    assert_eq!("User".suffix("Controller"), "UserController");
    assert_eq!("HttpController".remove_prefix("Http"), "Controller");
    assert_eq!("Controller".prefix("Http"), "HttpController");
    assert_eq!("file.txt".remove_extension(), "file");
    assert_eq!("file".ext("txt"), "file.txt");
}

#[test]
fn test_advanced_slice_utilities() {
    assert!(starts_with_any(
        "https://example.com",
        &["https://", "ftp://"]
    ));
    assert!(!starts_with_any(
        "git://example.com",
        &["https://", "ftp://"]
    ));
    assert!("https://example.com".starts_with_any(&["https://", "ftp://"]));

    assert!(ends_with_any("photo.jpeg", &[".jpg", ".jpeg", ".png"]));
    assert!(!ends_with_any("photo.gif", &[".jpg", ".jpeg", ".png"]));
    assert!("photo.jpeg".ends_with_any(&[".jpg", ".jpeg", ".png"]));

    assert_eq!(
        replace_first("foo bar foo baz", "foo", "qux"),
        "qux bar foo baz"
    );
    assert_eq!(
        "foo bar foo baz".replace_first("foo", "qux"),
        "qux bar foo baz"
    );

    assert_eq!(
        replace_last("foo bar foo baz", "foo", "qux"),
        "foo bar qux baz"
    );
    assert_eq!(
        "foo bar foo baz".replace_last("foo", "qux"),
        "foo bar qux baz"
    );

    assert_eq!(remove("foo bar baz foo", "foo "), "bar baz foo");
    assert_eq!("foo bar baz foo".remove("foo "), "bar baz foo");

    assert_eq!(deduplicate("a///b////c", '/'), "a/b/c");
    assert_eq!("a///b////c".deduplicate('/'), "a/b/c");
    let non_dup = "a/b/c";
    let dedup_res = deduplicate(non_dup, '/');
    assert!(matches!(dedup_res, std::borrow::Cow::Borrowed(_)));
}
