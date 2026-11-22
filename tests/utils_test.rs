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
